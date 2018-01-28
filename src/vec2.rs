
use std::ops::*;

use traits::*;

/// A 2D vector.
#[derive(Copy, Clone, Debug, Default)]
pub struct Vec2<T: Sized>{
	pub x: T,
	pub y: T
}

/// Constructs a Vec2 from individual components.
pub fn vec2<T: Sized + Clone>(x: T, y: T) -> Vec2<T> {
	Vec2{ x, y }
}

impl<T: Sized + Clone> Vec2<T> {
	/// Creates a new vector from an array of components
	pub fn new(vals: [T; 2]) -> Self {
		Self {
			x: vals[0].clone(),
			y: vals[1].clone()
		}
	}

	/// Returns an array containing all the elements of the vector.
	pub fn as_array(self) -> [T; 2] {
		[self.x, self.y]
	}
}

impl<T: Sized + Clone> Index<usize> for Vec2<T> {
	type Output = T;

	fn index(&self, idx: usize) -> &Self::Output {
		match idx {
			0 => &self.x,
			1 => &self.y,
			_ => panic!("Accessed out of range index on a vector")
		}
	}
}
impl<T: Sized + Clone> IndexMut<usize> for Vec2<T> {
	fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
			match idx {
				0 => &mut self.x,
				1 => &mut self.y,
				_ => panic!("Accessed out of range index on a vector")
			}
	}
}

impl<T: Sized + Clone> HasPerElementOps for Vec2<T> {
	type ElemType = T;

	fn apply_op<U: Fn(&T) -> T>(&self, func: U) -> Self {
		vec2(
			func(&self[0]),
			func(&self[1]))
	}
}
impl<T: Sized + Clone> HasPerElementBinOps for Vec2<T> {
	type ElemType = T;

	fn apply_bin_op<U: Fn(&T, T) -> T>(&self, rhs: Self, func: U) -> Self {
		vec2(
			func(&self[0], rhs[0].clone()),
			func(&self[1], rhs[1].clone())
		)
	}
}

impl<T> Add for Vec2<T>
	where T: Add<Output = T> + Sized + Clone
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		vec2(
			self[0].clone() + rhs[0].clone(),
			self[1].clone() + rhs[1].clone()
		)
	}
}
impl<T> Sub for Vec2<T>
	where T: Sub<Output = T> + Sized + Clone
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		vec2(
			self[0].clone() - rhs[0].clone(),
			self[1].clone() - rhs[1].clone()
		)
	}
}
impl<T> Mul for Vec2<T>
	where T: Mul<Output = T> + Sized + Clone
{
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		vec2(
			self[0].clone() * rhs[0].clone(),
			self[1].clone() * rhs[1].clone()
		)
	}
}
impl<T> Div for Vec2<T>
	where T: Div<Output = T> + Sized + Clone
{
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		vec2(
			self[0].clone() / rhs[0].clone(),
			self[1].clone() / rhs[1].clone()
		)
	}
}

impl<T> Add<T> for Vec2<T>
	where T: Add<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn add(self, rhs: T) -> Self {
		vec2(
			self[0].clone() + rhs.clone(),
			self[1].clone() + rhs
		)
	}
}
impl<T> Sub<T> for Vec2<T>
	where T: Sub<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn sub(self, rhs: T) -> Self {
		vec2(
			self[0].clone() - rhs.clone(),
			self[1].clone() - rhs
		)
	}
}
impl<T> Mul<T> for Vec2<T>
	where T: Mul<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self {
		vec2(
			self[0].clone() * rhs.clone(),
			self[1].clone() * rhs
		)
	}
}
impl<T> Div<T> for Vec2<T>
	where T: Div<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn div(self, rhs: T) -> Self {
		vec2(
			self[0].clone() / rhs.clone(),
			self[1].clone() / rhs
		)
	}
}

impl<T> Rem<T> for Vec2<T>
	where T: Rem<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn rem(self, rhs: T) -> Self {
		vec2(
			self[0].clone() % rhs.clone(),
			self[1].clone() % rhs
		)
	}
}

impl<T> Rem for Vec2<T>
	where T: Rem<Output = T> + Sized + Clone
{
	type Output = Self;

	fn rem(self, rhs: Self) -> Self {
		vec2(
			self[0].clone() % rhs[0].clone(),
			self[1].clone() % rhs[1].clone()
		)
	}
}

impl<T> HasDot for Vec2<T>
	where T: Mul<Output = T> + Add<Output = T> + Sized + Clone
{
	type Output = T;

	fn dot(&self, rhs: Self) -> T {
		let [x, y] = (self.clone() * rhs).as_array();
		return x + y;
	}
}

impl<T> HasX for Vec2<T> 
	where T: Sized + Clone
{
	type Output = T;

	fn x(&self) -> &T {
		&self[0]
	}
}
impl<T> HasY for Vec2<T>
	where T: Sized + Clone
{
	fn y(&self) -> &T {
		&self[1]
	}
}
