
extern crate num;
extern crate vecmath;

mod traits;
mod vec2;
mod vec3;
mod vec4;

pub mod prelude {
	pub use traits::*;
}

pub use prelude::*;
