
#![feature(slice_patterns)]

extern crate vecmath;

mod traits;
mod vec2;
mod vec3;
mod vec4;
mod trait_impls;

pub mod prelude {
	pub use traits::*;
	pub use trait_impls::*;
}

pub use prelude::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
