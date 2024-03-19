use rwasm::{
    core::{Trap, TrapCode},
    engine::bytecode::FuncIdx,
};

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(strum_macros::EnumIter))]
pub enum ExitCode {
    // warning: when adding new codes don't forget to add them to impls below
    Ok = 0,
    Panic = -71,
    // fluentbase error codes
    ExecutionHalted = -1001,
    NotSupportedCall = -1003,
    TransactError = -1004,
    OutputOverflow = -1005,
    InputDecodeFailure = -1006,
    PoseidonError = -1007,
    PersistentStorageError = -1008,
    WriteProtection = -1009,
    CreateError = -1010,
    PreimageUnavailable = -1011,
    InsufficientBalance = -1012,
    CreateCollision = -1013,
    ContractSizeLimit = -1014,
    StorageSlotOverflow = -1015,
    CallDepthOverflow = -1016,
    FatalExternalError = -1017,
    CompilationError = -1018,
    OverflowPayment = -1019,
    EVMCreateError = -1020,
    EVMCallError = -1021,
    EVMNotFound = -1022,
    // trap error codes
    UnreachableCodeReached = -2006,
    MemoryOutOfBounds = -2007,
    TableOutOfBounds = -2008,
    IndirectCallToNull = -2009,
    IntegerDivisionByZero = -2010,
    IntegerOverflow = -2011,
    BadConversionToInteger = -2012,
    StackOverflow = -2013,
    BadSignature = -2014,
    OutOfFuel = -2015,
    GrowthOperationLimited = -2016,
    UnknownError = -2017,
    UnresolvedFunction = -2018,
}

impl ExitCode {
    pub fn is_ok(&self) -> bool {
        *self == Self::Ok
    }

    pub fn into_i32(self) -> i32 {
        self as i32
    }

    pub fn into_trap(self) -> Trap {
        Trap::i32_exit(self as i32)
    }
}

impl From<TrapCode> for ExitCode {
    fn from(value: TrapCode) -> Self {
        match value {
            TrapCode::UnreachableCodeReached => ExitCode::UnreachableCodeReached,
            TrapCode::MemoryOutOfBounds => ExitCode::MemoryOutOfBounds,
            TrapCode::TableOutOfBounds => ExitCode::TableOutOfBounds,
            TrapCode::IndirectCallToNull => ExitCode::IndirectCallToNull,
            TrapCode::IntegerDivisionByZero => ExitCode::IntegerDivisionByZero,
            TrapCode::IntegerOverflow => ExitCode::IntegerOverflow,
            TrapCode::BadConversionToInteger => ExitCode::BadConversionToInteger,
            TrapCode::StackOverflow => ExitCode::StackOverflow,
            TrapCode::BadSignature => ExitCode::BadSignature,
            TrapCode::OutOfFuel => ExitCode::OutOfFuel,
            TrapCode::GrowthOperationLimited => ExitCode::GrowthOperationLimited,
            TrapCode::UnresolvedFunction => ExitCode::UnresolvedFunction,
        }
    }
}

impl Into<Trap> for ExitCode {
    fn into(self) -> Trap {
        self.into_trap()
    }
}

impl From<i32> for ExitCode {
    fn from(value: i32) -> ExitCode {
        match value {
            0 => ExitCode::Ok,
            -71 => ExitCode::Panic,
            // fluentbase error codes
            -1001 => ExitCode::ExecutionHalted,
            -1003 => ExitCode::NotSupportedCall,
            -1004 => ExitCode::TransactError,
            -1005 => ExitCode::OutputOverflow,
            -1006 => ExitCode::InputDecodeFailure,
            -1007 => ExitCode::PoseidonError,
            -1008 => ExitCode::PersistentStorageError,
            -1009 => ExitCode::WriteProtection,
            -1010 => ExitCode::CreateError,
            -1011 => ExitCode::PreimageUnavailable,
            -1012 => ExitCode::InsufficientBalance,
            -1013 => ExitCode::CreateCollision,
            -1014 => ExitCode::ContractSizeLimit,
            -1015 => ExitCode::StorageSlotOverflow,
            -1016 => ExitCode::CallDepthOverflow,
            -1017 => ExitCode::FatalExternalError,
            -1018 => ExitCode::CompilationError,
            -1019 => ExitCode::OverflowPayment,
            -1020 => ExitCode::EVMCreateError,
            -1021 => ExitCode::EVMCallError,
            -1022 => ExitCode::EVMNotFound,
            // trap error codes
            -2006 => ExitCode::UnreachableCodeReached,
            -2007 => ExitCode::MemoryOutOfBounds,
            -2008 => ExitCode::TableOutOfBounds,
            -2009 => ExitCode::IndirectCallToNull,
            -2010 => ExitCode::IntegerDivisionByZero,
            -2011 => ExitCode::IntegerOverflow,
            -2012 => ExitCode::BadConversionToInteger,
            -2013 => ExitCode::StackOverflow,
            -2014 => ExitCode::BadSignature,
            -2015 => ExitCode::OutOfFuel,
            -2016 => ExitCode::GrowthOperationLimited,
            -2017 => ExitCode::UnknownError,
            -2018 => ExitCode::UnresolvedFunction,
            // this is 100% unknown error
            _ => ExitCode::UnresolvedFunction,
        }
    }
}

impl Into<i32> for ExitCode {
    fn into(self) -> i32 {
        self as i32
    }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "std", derive(strum_macros::EnumIter))]
pub enum SysFuncIdx {
    #[default]
    UNKNOWN = 0x0000,

    // crypto
    CRYPTO_KECCAK256 = 0x0101,
    CRYPTO_POSEIDON = 0x0102,
    CRYPTO_POSEIDON2 = 0x0103,
    CRYPTO_ECRECOVER = 0x0104,

    // SYS host
    SYS_HALT = 0x0001,
    SYS_STATE = 0x0002,
    SYS_READ = 0x0003,
    SYS_INPUT_SIZE = 0x0004,
    SYS_WRITE = 0x0005,
    SYS_OUTPUT_SIZE = 0x0006,
    SYS_READ_OUTPUT = 0x0007,
    SYS_EXEC = 0x0008,
    SYS_EXEC_HASH = 0x0009,
    SYS_FORWARD_OUTPUT = 0x000a,

    // jzkt
    JZKT_OPEN = 0x0701,
    JZKT_CHECKPOINT = 0x0702,
    JZKT_GET = 0x0703,
    JZKT_UPDATE = 0x0704,
    JZKT_UPDATE_PREIMAGE = 0x0705,
    JZKT_REMOVE = 0x0706,
    JZKT_COMPUTE_ROOT = 0x0707,
    JZKT_EMIT_LOG = 0x0708,
    JZKT_COMMIT = 0x0709,
    JZKT_ROLLBACK = 0x070A,
    JZKT_STORE = 0x070B,
    JZKT_LOAD = 0x070C,
    JZKT_PREIMAGE_SIZE = 0x070D,
    JZKT_PREIMAGE_COPY = 0x070E,
}

impl SysFuncIdx {
    pub fn fuel_cost(&self) -> u32 {
        match self {
            SysFuncIdx::SYS_HALT => 1,
            SysFuncIdx::SYS_STATE => 1,
            SysFuncIdx::SYS_READ => 1,
            SysFuncIdx::SYS_INPUT_SIZE => 1,
            SysFuncIdx::SYS_WRITE => 1,
            SysFuncIdx::CRYPTO_KECCAK256 => 1,
            SysFuncIdx::CRYPTO_POSEIDON => 1,
            SysFuncIdx::CRYPTO_POSEIDON2 => 1,
            SysFuncIdx::CRYPTO_ECRECOVER => 1,
            SysFuncIdx::JZKT_OPEN => 1,
            SysFuncIdx::JZKT_UPDATE => 1,
            SysFuncIdx::JZKT_GET => 1,
            SysFuncIdx::JZKT_COMPUTE_ROOT => 1,
            SysFuncIdx::JZKT_ROLLBACK => 1,
            SysFuncIdx::JZKT_COMMIT => 1,
            _ => 1, //unreachable!("not configured fuel for opcode: {:?}", self),
        }
    }
}

impl Into<u32> for SysFuncIdx {
    fn into(self) -> u32 {
        self as u32
    }
}

impl Into<FuncIdx> for SysFuncIdx {
    fn into(self) -> FuncIdx {
        (self as u32).into()
    }
}
