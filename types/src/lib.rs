#![cfg_attr(not(feature = "std"), no_std)]

pub mod exam;
pub mod lecture;
pub mod primitives;
pub mod professor;
pub mod scholarship;
pub mod student;
pub mod university;
pub mod validator;

pub use primitives::*;
