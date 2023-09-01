use crate::{
    state_circuit::tag::RwTableTag,
    trace_step::{GadgetError, TraceStep},
};
use fluentbase_rwasm::{common::UntypedValue, RwOp};

#[derive(Clone, Copy, Debug)]
pub enum RwRow {
    /// Start
    Start { rw_counter: usize },
    /// Stack
    Stack {
        rw_counter: usize,
        is_write: bool,
        call_id: usize,
        stack_pointer: usize,
        value: UntypedValue,
    },
    /// Global
    Global {
        rw_counter: usize,
        is_write: bool,
        call_id: usize,
        global_index: usize,
        value: UntypedValue,
    },
    /// Memory
    Memory {
        rw_counter: usize,
        is_write: bool,
        call_id: usize,
        memory_address: u64,
        byte: u8,
    },
}

pub fn rw_rows_from_trace(
    res: &mut Vec<RwRow>,
    trace: &TraceStep,
    call_id: usize,
) -> Result<(), GadgetError> {
    let rw_ops = trace.instr().get_rw_ops();
    let mut stack_reads = 0;
    let mut stack_writes = 0;
    for rw_op in rw_ops.iter() {
        match rw_op {
            RwOp::StackWrite => {
                let addr = trace.next_nth_stack_addr(stack_writes)?;
                let value = trace.next_nth_stack_value(stack_writes)?;
                res.push(RwRow::Stack {
                    rw_counter: res.len(),
                    is_write: true,
                    call_id,
                    stack_pointer: addr as usize,
                    value,
                });
                stack_writes += 1
            }
            RwOp::StackRead => {
                let addr = trace.curr_nth_stack_addr(stack_reads)?;
                let value = trace.curr_nth_stack_value(stack_reads)?;
                res.push(RwRow::Stack {
                    rw_counter: res.len(),
                    is_write: false,
                    call_id,
                    stack_pointer: addr as usize,
                    value,
                });
                stack_reads += 1;
            }
            RwOp::GlobalWrite(_) => {}
            RwOp::GlobalRead(_) => {}
            RwOp::MemoryWrite(_) => {}
            RwOp::MemoryRead(_) => {}
            RwOp::TableWrite => {}
            RwOp::TableRead => {}
        }
    }
    Ok(())
}

impl RwRow {
    pub fn value(&self) -> UntypedValue {
        match self {
            Self::Stack { value, .. } => *value,
            Self::Global { value, .. } => *value,
            Self::Memory { byte, .. } => UntypedValue::from(*byte),
            _ => unreachable!("{:?}", self),
        }
    }

    pub fn stack_value(&self) -> UntypedValue {
        match self {
            Self::Stack { value, .. } => *value,
            _ => unreachable!("{:?}", self),
        }
    }

    pub(crate) fn global_value(&self) -> (UntypedValue, usize) {
        match self {
            Self::Global {
                value,
                global_index,
                ..
            } => (*value, *global_index),
            _ => unreachable!(),
        }
    }

    pub fn memory_value(&self) -> u8 {
        match self {
            Self::Memory { byte, .. } => *byte,
            _ => unreachable!("{:?}", self),
        }
    }

    pub fn rw_counter(&self) -> usize {
        match self {
            Self::Start { rw_counter }
            | Self::Memory { rw_counter, .. }
            | Self::Stack { rw_counter, .. }
            | Self::Global { rw_counter, .. } => *rw_counter,
            _ => 0,
        }
    }

    pub fn is_write(&self) -> bool {
        match self {
            Self::Start { .. } => false,
            Self::Memory { is_write, .. }
            | Self::Stack { is_write, .. }
            | Self::Global { is_write, .. } => *is_write,
            _ => false,
        }
    }

    pub fn tag(&self) -> RwTableTag {
        match self {
            Self::Start { .. } => RwTableTag::Start,
            Self::Memory { .. } => RwTableTag::Memory,
            Self::Stack { .. } => RwTableTag::Stack,
            Self::Global { .. } => RwTableTag::Global,
        }
    }

    pub fn id(&self) -> Option<usize> {
        match self {
            Self::Stack { call_id, .. }
            | Self::Global { call_id, .. }
            | Self::Memory { call_id, .. } => Some(*call_id),
            Self::Start { .. } => None,
        }
    }

    pub fn address(&self) -> Option<u32> {
        match self {
            Self::Memory { memory_address, .. } => Some(*memory_address as u32),
            Self::Stack { stack_pointer, .. } => Some(*stack_pointer as u32),
            Self::Global { global_index, .. } => Some(*global_index as u32),
            Self::Start { .. } => None,
        }
    }
}