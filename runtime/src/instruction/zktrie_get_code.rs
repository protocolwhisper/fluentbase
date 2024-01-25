use crate::RuntimeContext;
use fluentbase_rwasm::{common::Trap, Caller};
use fluentbase_types::ExitCode;

pub struct ZkTrieGetCode;

impl ZkTrieGetCode {
    pub fn fn_handler<T>(
        mut caller: Caller<'_, RuntimeContext<T>>,
        key20_offset: u32,
        output_offset: u32,
        output_len: u32,
    ) -> Result<(), Trap> {
        let address = caller.read_memory(key20_offset, 20).to_vec();
        let code =
            Self::fn_impl(caller.data(), &address, output_len).map_err(|err| err.into_trap())?;
        caller.write_memory(output_offset, &code);
        Ok(())
    }

    pub fn fn_impl<T>(
        context: &RuntimeContext<T>,
        key: &[u8],
        output_len: u32,
    ) -> Result<Vec<u8>, ExitCode> {
        let zktrie = context.zktrie.clone().unwrap();
        let code = zktrie.borrow().get_code(key).unwrap_or_default();
        if code.len() <= output_len as usize {
            Ok(code)
        } else {
            Err(ExitCode::OutputOverflow)
        }
    }
}
