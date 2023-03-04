#![cfg_attr(not(feature = "std"), no_std)]

pub mod student;
pub mod professor;
pub mod university;

use codec::Decode;
use codec::Encode;
use codec::MaxEncodedLen;
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

/// type to uniquely represent any item
/// this better to be a had bytes
#[derive(Decode, Encode, TypeInfo, Debug, Eq, PartialEq, MaxEncodedLen, Clone)]
pub struct UniqId {}
