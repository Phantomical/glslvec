
use std::ops::*;

use traits::*;

/// A 4D vector.
#[derive(Copy, Clone, Debug, Default)]
pub struct Vec4<T: Sized>{
	pub x: T,
	pub y: T,
	pub z: T,
	pub w: T
}

/// Constructs a Vec4 from individual components.
pub fn vec4<T: Sized + Clone>(x: T, y: T, z: T, w: T) -> Vec4<T> {
	Vec4{ x, y, z, w }
}

impl<T: Sized + Clone> Vec4<T> {
	/// Creates a new vector from an array of components
	pub fn new(vals: [T; 4]) -> Self {
		Vec4{
			x: vals[0].clone(),
			y: vals[1].clone(),
			z: vals[2].clone(),
			w: vals[3].clone()
		}
	}

	/// Returns an array containing all the elements of the vector.
	pub fn as_array(self) -> [T; 4] {
		[self.x, self.y, self.z, self.w]
	}
}
impl<T: Sized + Clone + Zero + One> Vec4<T> {
	/// Returns a vector containing only zeros
	pub fn zero() -> Self {
		Self::new([T::zero(), T::zero(), T::zero(), T::zero()])
	}

	/// Returns a vector with x equal to 1 and all
	/// other elements equal to 0.
	pub fn unit_x() -> Self {
		Self::new([T::one(), T::zero(), T::zero(), T::zero()])
	}
	/// Returns a vector with y equal to 1 and all
	/// other elements equal to 0.
	pub fn unit_y() -> Self {
		Self::new([T::zero(), T::one(), T::zero(), T::zero()])
	}
	/// Returns a vector with z equal to 1 and all
	/// other elements equal to 0.
	pub fn unit_z() -> Self {
		Self::new([T::zero(), T::zero(), T::one(), T::zero()])
	}
	/// Returns a vector with w equal to 1 and all
	/// other elements equal to 0.
	pub fn unit_w() -> Self {
		Self::new([T::zero(), T::zero(), T::zero(), T::one()])
	}
}

impl<T: Sized + Clone> Index<usize> for Vec4<T> {
	type Output = T;

	fn index(&self, idx: usize) -> &Self::Output {
		match idx {
			0 => &self.x,
			1 => &self.y,
			2 => &self.z,
			3 => &self.w,
			_ => panic!("Accessed invalid vector index")
		}
	}
}
impl<T: Sized + Clone> IndexMut<usize> for Vec4<T> {
	fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
		match idx {
			0 => &mut self.x,
			1 => &mut self.y,
			2 => &mut self.z,
			3 => &mut self.w,
			_ => panic!("Accessed invalid vector index")
		}
	}
}

impl<T: Sized + Clone> HasPerElementOps for Vec4<T> {
	type ElemType = T;

	fn apply_op<U: Fn(&T) -> T>(&self, func: U) -> Self {
		Vec4::new([
			func(&self[0]),
			func(&self[1]),
			func(&self[2]),
			func(&self[3])
		])
	}
}
impl<T: Sized + Clone> HasPerElementBinOps for Vec4<T> {
	type ElemType = T;

	fn apply_bin_op<U: Fn(&T, T) -> T>(&self, rhs: Self, func: U) -> Self {
		Vec4::new([
			func(&self[0], rhs[0].clone()),
			func(&self[1], rhs[1].clone()),
			func(&self[2], rhs[2].clone()),
			func(&self[3], rhs[3].clone())
		])
	}
}

impl<T> Add for Vec4<T>
	where T: Add<Output = T> + Sized + Clone
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Vec4::new([
			self[0].clone() + rhs[0].clone(),
			self[1].clone() + rhs[1].clone(),
			self[2].clone() + rhs[2].clone(),
			self[3].clone() + rhs[3].clone()
		])
	}
}
impl<T> Sub for Vec4<T>
	where T: Sub<Output = T> + Sized + Clone
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		Vec4::new([
			self[0].clone() - rhs[0].clone(),
			self[1].clone() - rhs[1].clone(),
			self[2].clone() - rhs[2].clone(),
			self[3].clone() - rhs[3].clone()
		])
	}
}
impl<T> Mul for Vec4<T>
	where T: Mul<Output = T> + Sized + Clone
{
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		Vec4::new([
			self[0].clone() * rhs[0].clone(),
			self[1].clone() * rhs[1].clone(),
			self[2].clone() * rhs[2].clone(),
			self[3].clone() * rhs[3].clone()
		])
	}
}
impl<T> Div for Vec4<T>
	where T: Div<Output = T> + Sized + Clone
{
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		Vec4::new([
			self[0].clone() / rhs[0].clone(),
			self[1].clone() / rhs[1].clone(),
			self[2].clone() / rhs[2].clone(),
			self[3].clone() / rhs[3].clone()
		])
	}
}

impl<T> Add<T> for Vec4<T>
	where T: Add<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn add(self, rhs: T) -> Self {
		Vec4::new([
			self[0].clone() + rhs.clone(),
			self[1].clone() + rhs.clone(),
			self[2].clone() + rhs.clone(),
			self[3].clone() + rhs
		])
	}
}
impl<T> Sub<T> for Vec4<T>
	where T: Sub<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn sub(self, rhs: T) -> Self {
		Vec4::new([
			self[0].clone() - rhs.clone(),
			self[1].clone() - rhs.clone(),
			self[2].clone() - rhs.clone(),
			self[3].clone() - rhs
		])
	}
}
impl<T> Mul<T> for Vec4<T>
	where T: Mul<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self {
		Vec4::new([
			self[0].clone() * rhs.clone(),
			self[1].clone() * rhs.clone(),
			self[2].clone() * rhs.clone(),
			self[3].clone() * rhs
		])
	}
}
impl<T> Div<T> for Vec4<T>
	where T: Div<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn div(self, rhs: T) -> Self {
		Vec4::new([
			self[0].clone() / rhs.clone(),
			self[1].clone() / rhs.clone(),
			self[2].clone() / rhs.clone(),
			self[3].clone() / rhs
		])
	}
}

impl<T> Rem<T> for Vec4<T>
	where T: Rem<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn rem(self, rhs: T) -> Self {
		Vec4::new([
			self[0].clone() % rhs.clone(),
			self[1].clone() % rhs.clone(),
			self[2].clone() % rhs.clone(),
			self[3].clone() % rhs
		])
	}
}

impl<T> Rem for Vec4<T>
	where T: Rem<Output = T> + Sized + Clone
{
	type Output = Self;

	fn rem(self, rhs: Self) -> Self {
		Vec4::new([
			self[0].clone() % rhs[0].clone(),
			self[1].clone() % rhs[1].clone(),
			self[2].clone() % rhs[2].clone(),
			self[3].clone() % rhs[3].clone()
		])
	}
}

impl<T> HasDot for Vec4<T>
	where T: Mul<Output = T> + Add<Output = T> + Sized + Clone
{
	type Output = T;

	fn dot(&self, rhs: Self) -> T {
		let [x, y, z, w] = (self.clone() * rhs).as_array();
		return x + y + z + w;
	}
}

impl<T> Neg for Vec4<T>
	where T: Neg<Output = T> + Sized + Clone
{
	type Output = Self;

	fn neg(self) -> Self {
		vec4(
			-self.x,
			-self.y,
			-self.z,
			-self.w)
	}
}

