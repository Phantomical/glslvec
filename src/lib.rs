
#![feature(slice_patterns)]

extern crate vecmath;

mod traits;
mod vec2;
mod vec3;
mod vec4;
mod trait_impls;
mod functions;

pub mod prelude {
	pub use traits::*;
	pub use trait_impls::*;
}

pub use prelude::{HasX, HasY, HasZ, HasW};
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
pub use functions::*;
