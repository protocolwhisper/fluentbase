#![cfg_attr(not(feature = "std"), no_std)]
#![feature(inherent_associated_types)]

extern crate alloc;
extern crate core;

pub use crate::{
    buffer::{BufferDecoder, BufferEncoder},
    encoder::{Encoder, FieldEncoder},
};

mod buffer;
mod encoder;
mod evm;
mod hash;
mod macros;
mod primitive;
mod serde;
#[cfg(test)]
mod tests;
mod vec;