//! Bindings to Direct3D 11 via the ANGLE OpenGL-to-Direct3D translation layer on Windows.

pub mod adapter;
pub mod context;
pub mod device;
pub mod surface;

pub(crate) loader;

mod error;