// This is fine here because public re-exports are not picked up by rustc
#![allow(unused_imports)]

pub mod vector3;

pub use vector3::Vec3;
