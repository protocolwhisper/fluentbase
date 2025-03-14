use super::inner_evm_context::InnerEvmContext;
use crate::{
    db::Database,
    interpreter::{CallInputs, Contract, Gas, InstructionResult, Interpreter, InterpreterResult},
    primitives::{Address, Bytes, EVMError, Env, HashSet, U256},
    return_ok,
    ContextPrecompiles,
    FrameOrResult,
    CALL_STACK_LIMIT,
};
use core::{
    fmt,
    ops::{Deref, DerefMut},
};
use fluentbase_sdk::Account;
use fluentbase_types::ExitCode;
use std::boxed::Box;

/// EVM context that contains the inner EVM context and precompiles.
pub struct EvmContext<DB: Database> {
    /// Inner EVM context.
    pub inner: InnerEvmContext<DB>,
    /// Precompiles that are available for evm.
    pub precompiles: ContextPrecompiles<DB>,
}

impl<DB: Database + Clone> Clone for EvmContext<DB> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            precompiles: ContextPrecompiles::default(),
        }
    }
}

impl<DB> fmt::Debug for EvmContext<DB>
where
    DB: Database + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EvmContext")
            .field("inner", &self.inner)
            .field("precompiles", &self.inner)
            .finish_non_exhaustive()
    }
}

impl<DB: Database> Deref for EvmContext<DB> {
    type Target = InnerEvmContext<DB>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<DB: Database> DerefMut for EvmContext<DB> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<DB: Database> EvmContext<DB> {
    /// Create new context with database.
    pub fn new(db: DB) -> Self {
        Self {
            inner: InnerEvmContext::new(db),
            precompiles: ContextPrecompiles::default(),
        }
    }

    /// Creates a new context with the given environment and database.
    #[inline]
    pub fn new_with_env(db: DB, env: Box<Env>) -> Self {
        Self {
            inner: InnerEvmContext::new_with_env(db, env),
            precompiles: ContextPrecompiles::default(),
        }
    }

    /// Sets the database.
    ///
    /// Note that this will ignore the previous `error` if set.
    #[inline]
    pub fn with_db<ODB: Database>(self, db: ODB) -> EvmContext<ODB> {
        EvmContext {
            inner: self.inner.with_db(db),
            precompiles: ContextPrecompiles::default(),
        }
    }

    /// Sets precompiles
    #[inline]
    pub fn set_precompiles(&mut self, precompiles: ContextPrecompiles<DB>) {
        // set warm loaded addresses.
        self.journaled_state.warm_preloaded_addresses =
            precompiles.addresses().copied().collect::<HashSet<_>>();
        self.precompiles = precompiles;
    }

    pub fn load_jzkt_account(&mut self, address: Address) -> Result<Account, EVMError<ExitCode>> {
        let (account, _) = self.load_account(address)?;
        let mut jzkt_account: Account = account.info.clone().into();
        jzkt_account.address = address;
        Ok(jzkt_account)
    }

    /// Call precompile contract
    #[inline]
    pub(crate) fn call_precompile(
        &mut self,
        address: Address,
        input_data: &Bytes,
        gas: Gas,
    ) -> Option<InterpreterResult> {
        let out = self
            .precompiles
            .call(address, input_data, gas.limit(), &mut self.inner)?;

        let mut result = InterpreterResult {
            result: InstructionResult::Ok,
            gas,
            output: Bytes::new(),
        };

        match out {
            Ok((gas_used, data)) => {
                if result.gas.record_cost(gas_used) {
                    result.result = InstructionResult::Ok;
                    result.output = data;
                } else {
                    result.result = InstructionResult::OutOfFuel;
                }
            }
            Err(e) => {
                result.result = if e == crate::precompile::Error::OutOfGas {
                    InstructionResult::OutOfFuel
                } else {
                    InstructionResult::PrecompileError
                };
            }
        }
        Some(result)
    }

    /// Make call frame
    #[inline]
    pub fn make_call_frame(
        &mut self,
        inputs: &CallInputs,
    ) -> Result<FrameOrResult, EVMError<ExitCode>> {
        let gas = Gas::new(inputs.gas_limit);

        let return_result = |instruction_result: InstructionResult| {
            Ok(FrameOrResult::new_call_result(
                InterpreterResult {
                    result: instruction_result,
                    gas,
                    output: Bytes::new(),
                },
                inputs.return_memory_offset.clone(),
            ))
        };

        // Check depth
        if self.journaled_state.depth() > CALL_STACK_LIMIT {
            return return_result(InstructionResult::CallDepthOverflow);
        }

        let (account, _) = self
            .inner
            .journaled_state
            .load_code(inputs.contract, &mut self.inner.db)?;
        let code_hash = account.info.code_hash();
        let bytecode = account.info.code.clone().unwrap_or_default();

        // Create subroutine checkpoint
        let checkpoint = self.journaled_state.checkpoint();

        // Touch address. For "EIP-158 State Clear", this will erase empty accounts.
        if inputs.transfer.value == U256::ZERO {
            self.load_account(inputs.context.address)?;
            self.journaled_state.touch(&inputs.context.address);
        }

        // Transfer value from caller to called account
        if let Some(result) = self.inner.journaled_state.transfer(
            &inputs.transfer.source,
            &inputs.transfer.target,
            inputs.transfer.value,
            &mut self.inner.db,
        )? {
            self.journaled_state.checkpoint_revert(checkpoint);
            return return_result(result);
        }

        if let Some(result) = self.call_precompile(inputs.contract, &inputs.input, gas) {
            if matches!(result.result, return_ok!()) {
                self.journaled_state.checkpoint_commit();
            } else {
                self.journaled_state.checkpoint_revert(checkpoint);
            }
            Ok(FrameOrResult::new_call_result(
                result,
                inputs.return_memory_offset.clone(),
            ))
        } else if !bytecode.is_empty() {
            let contract = Box::new(Contract::new_with_context(
                inputs.input.clone(),
                bytecode,
                code_hash,
                &inputs.context,
            ));
            // Create interpreter and executes call and push new CallStackFrame.
            Ok(FrameOrResult::new_call_frame(
                inputs.return_memory_offset.clone(),
                checkpoint,
                Interpreter::new(contract, gas.limit(), inputs.is_static),
            ))
        } else {
            self.journaled_state.checkpoint_commit();
            return_result(InstructionResult::Ok)
        }
    }
}

/// Test utilities for the [`EvmContext`].
#[cfg(any(test, feature = "test-utils"))]
pub(crate) mod test_utils {
    use super::*;
    use crate::{
        db::{CacheDB, EmptyDB},
        interpreter::{CallContext, CallScheme, Transfer},
        journaled_state::JournaledState,
        primitives::{address, AccountInfo, Address, Bytes, Env, HashSet, SpecId, B256, U256},
        InnerEvmContext,
    };
    use std::boxed::Box;

    /// Mock caller address.
    pub const MOCK_CALLER: Address = address!("0000000000000000000000000000000000000000");

    /// Creates `CallInputs` that calls a provided contract address from the mock caller.
    pub fn create_mock_call_inputs(to: Address) -> CallInputs {
        CallInputs {
            contract: to,
            transfer: Transfer {
                source: MOCK_CALLER,
                target: to,
                value: U256::ZERO,
            },
            input: Bytes::new(),
            gas_limit: 0,
            context: CallContext {
                address: MOCK_CALLER,
                caller: MOCK_CALLER,
                code_address: MOCK_CALLER,
                apparent_value: U256::ZERO,
                scheme: CallScheme::Call,
            },
            is_static: false,
            return_memory_offset: 0..0,
        }
    }

    /// Creates an evm context with a cache db backend.
    /// Additionally loads the mock caller account into the db,
    /// and sets the balance to the provided U256 value.
    pub fn create_cache_db_evm_context_with_balance(
        env: Box<Env>,
        mut db: CacheDB<EmptyDB>,
        balance: U256,
    ) -> EvmContext<CacheDB<EmptyDB>> {
        db.insert_account_info(
            MOCK_CALLER,
            AccountInfo {
                nonce: 0,
                balance,
                code_hash: B256::default(),
                rwasm_code_hash: Default::default(),
                code: None,
                rwasm_code: None,
            },
        );
        create_cache_db_evm_context(env, db)
    }

    /// Creates a cached db evm context.
    pub fn create_cache_db_evm_context(
        env: Box<Env>,
        db: CacheDB<EmptyDB>,
    ) -> EvmContext<CacheDB<EmptyDB>> {
        EvmContext {
            inner: InnerEvmContext {
                env,
                journaled_state: JournaledState::new(SpecId::CANCUN, HashSet::new()),
                db,
                error: Ok(()),
                #[cfg(feature = "optimism")]
                l1_block_info: None,
            },
            precompiles: ContextPrecompiles::default(),
        }
    }

    /// Returns a new `EvmContext` with an empty journaled state.
    pub fn create_empty_evm_context(env: Box<Env>, db: EmptyDB) -> EvmContext<EmptyDB> {
        EvmContext {
            inner: InnerEvmContext {
                env,
                journaled_state: JournaledState::new(SpecId::CANCUN, HashSet::new()),
                db,
                error: Ok(()),
                #[cfg(feature = "optimism")]
                l1_block_info: None,
            },
            precompiles: ContextPrecompiles::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db::{CacheDB, EmptyDB},
        interpreter::InstructionResult,
        primitives::{address, AccountInfo, Bytecode, Bytes, Env, U256},
        Frame,
        FrameOrResult,
        JournalEntry,
    };
    use std::boxed::Box;
    use test_utils::*;

    // Tests that the `EVMContext::make_call_frame` function returns an error if the
    // call stack is too deep.
    #[test]
    fn test_make_call_frame_stack_too_deep() {
        let env = Env::default();
        let db = EmptyDB::default();
        let mut context = test_utils::create_empty_evm_context(Box::new(env), db);
        context.journaled_state.depth = CALL_STACK_LIMIT as usize + 1;
        let contract = address!("dead10000000000000000000000000000001dead");
        let call_inputs = test_utils::create_mock_call_inputs(contract);
        let res = context.make_call_frame(&call_inputs);
        let Ok(FrameOrResult::Result(err)) = res else {
            panic!("Expected FrameOrResult::Result");
        };
        assert_eq!(
            err.interpreter_result().result,
            InstructionResult::CallDepthOverflow
        );
    }

    // Tests that the `EVMContext::make_call_frame` function returns an error if the
    // transfer fails on the journaled state. It also verifies that the revert was
    // checkpointed on the journaled state correctly.
    #[test]
    fn test_make_call_frame_transfer_revert() {
        let env = Env::default();
        let db = EmptyDB::default();
        let mut evm_context = test_utils::create_empty_evm_context(Box::new(env), db);
        let contract = address!("dead10000000000000000000000000000001dead");
        let mut call_inputs = test_utils::create_mock_call_inputs(contract);
        call_inputs.transfer.value = U256::from(1);
        let res = evm_context.make_call_frame(&call_inputs);
        let Ok(FrameOrResult::Result(result)) = res else {
            panic!("Expected FrameOrResult::Result");
        };
        assert_eq!(
            result.interpreter_result().result,
            InstructionResult::InsufficientBalance
        );
        let checkpointed = vec![vec![JournalEntry::AccountLoaded { address: contract }]];
        assert_eq!(evm_context.journaled_state.journal, checkpointed);
        assert_eq!(evm_context.journaled_state.depth, 0);
    }

    #[test]
    fn test_make_call_frame_missing_code_context() {
        let env = Env::default();
        let cdb = CacheDB::new(EmptyDB::default());
        let bal = U256::from(3_000_000_000_u128);
        let mut context = create_cache_db_evm_context_with_balance(Box::new(env), cdb, bal);
        let contract = address!("dead10000000000000000000000000000001dead");
        let call_inputs = test_utils::create_mock_call_inputs(contract);
        let res = context.make_call_frame(&call_inputs);
        let Ok(FrameOrResult::Result(result)) = res else {
            panic!("Expected FrameOrResult::Result");
        };
        assert_eq!(result.interpreter_result().result, InstructionResult::Ok);
    }

    #[test]
    fn test_make_call_frame_succeeds() {
        let env = Env::default();
        let mut cdb = CacheDB::new(EmptyDB::default());
        let bal = U256::from(3_000_000_000_u128);
        let by = Bytecode::new_raw(Bytes::from(vec![0x60, 0x00, 0x60, 0x00]));
        let contract = address!("dead10000000000000000000000000000001dead");
        cdb.insert_account_info(
            contract,
            AccountInfo {
                nonce: 0,
                balance: bal,
                code_hash: by.clone().hash_slow(),
                rwasm_code_hash: Default::default(),
                code: Some(by),
                rwasm_code: None,
            },
        );
        let mut evm_context = create_cache_db_evm_context_with_balance(Box::new(env), cdb, bal);
        let call_inputs = test_utils::create_mock_call_inputs(contract);
        let res = evm_context.make_call_frame(&call_inputs);
        let Ok(FrameOrResult::Frame(Frame::Call(call_frame))) = res else {
            panic!("Expected FrameOrResult::Frame(Frame::Call(..))");
        };
        assert_eq!(call_frame.return_memory_range, 0..0,);
    }
}
