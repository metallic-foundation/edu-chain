#![cfg_attr(not(feature = "std"), no_std)]

pub mod exam;
pub mod lecture;
pub mod professor;
pub mod scholarship;
pub mod student;
pub mod university;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

/// type to uniquely represent any item
/// this better to be a had bytes
#[derive(Decode, Encode, TypeInfo, Debug, Eq, PartialEq, MaxEncodedLen, Clone)]
pub struct UniqId {}

/// Structure to represent the IPFS link
#[derive(Decode, Encode, TypeInfo, Debug, Eq, PartialEq, MaxEncodedLen, Clone)]
pub struct IpfsLink {}
