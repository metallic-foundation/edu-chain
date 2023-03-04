#![cfg_attr(not(feature = "std"), no_std)]

pub mod student;
pub mod professor;
pub mod university;

/// type to uniquely represent any item
/// this better to be a had bytes
type UniqId = ();
