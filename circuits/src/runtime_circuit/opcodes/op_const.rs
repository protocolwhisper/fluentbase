use crate::{
    constraint_builder::AdviceColumn,
    runtime_circuit::{constraint_builder::OpConstraintBuilder, opcodes::ExecutionGadget},
    util::Field,
};
use fluentbase_rwasm::engine::bytecode::Instruction;
use halo2_proofs::{circuit::Region, plonk::Error};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub(crate) struct ConstGadget<F: Field> {
    value: AdviceColumn,
    _pd: PhantomData<F>,
}

impl<F: Field> ExecutionGadget<F> for ConstGadget<F> {
    const NAME: &'static str = "WASM_CONST";

    fn configure(cb: &mut OpConstraintBuilder<F>) -> Self {
        let value = cb.query_cell();
        cb.stack_push(value.current());
        Self {
            value,
            _pd: Default::default(),
        }
    }

    fn assign_exec_step(&self, region: &mut Region<'_, F>, offset: usize) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn push_gadget_simple() {}
}