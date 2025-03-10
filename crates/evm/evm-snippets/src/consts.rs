pub const U64_MSBIT_IS_1: u64 = 0x8000000000000000;
pub const U8_MSBIT_IS_1: u8 = 0x80;
pub const U64_ALL_BITS_ARE_1: u64 = 0xffffffffffffffff;
pub const U64_ALL_BITS_ARE_0: u64 = 0;
pub const U64_MAX_VAL: u64 = 0xffffffffffffffff;
pub const U64_ALL_BITS_1_EXCEPT_MSB: u64 = 0xffffffffffffffff - U64_MSBIT_IS_1;
pub const BYTE_MAX_VAL: u64 = 0xff;
pub const BYTE_SIGN_BIT_MASK: u64 = 0b10000000;
pub const U64_LSBYTE_MASK: u64 = 0xff;
pub const U64_LOW_PART_MASK: u64 = 0xffffffff;
pub const BITS_IN_BYTE: u64 = 8;
pub const U64_BYTES_COUNT: u64 = 8;
pub const U256_BYTES_COUNT: u64 = 32;
pub const U64_BITS_COUNT: u64 = 64;
pub const U64_HALF_BITS_COUNT: u64 = U64_BITS_COUNT / 2;
// TODO fix hardcode
pub(crate) const GAS_LIMIT_HARDCODED: u32 = 10_000_000;
