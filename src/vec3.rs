
use std::ops::*;

use traits::*;

/// A 3D vector.
#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3<T: Sized>{
	pub x: T,
	pub y: T,
	pub z: T
}

/// Constructs a Vec3 from individual components.
pub fn vec3<T: Sized + Clone>(x: T, y: T, z: T) -> Vec3<T> {
	Vec3::new([x, y, z])
}

impl<T: Sized + Clone> Vec3<T> {
	/// Creates a new vector from an array of components
	pub fn new(vals: [T; 3]) -> Self {
		Self{ 
			x: vals[0].clone(), 
			y: vals[1].clone(), 
			z: vals[2].clone()
		}
	}

	/// Returns an array containing all the elements of the vector.
	pub fn as_array(self) -> [T; 3] {
		[ self.x, self.y, self.z ]
	}
}

impl<T: Sized + Clone> Index<usize> for Vec3<T> {
	type Output = T;

	fn index(&self, idx: usize) -> &Self::Output {
		match idx {
			0 => &self.x,
			1 => &self.y,
			2 => &self.z,
			_ => panic!("Accessed vector out of bounds.")
		}
	}
}
impl<T: Sized + Clone> IndexMut<usize> for Vec3<T> {
	fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
		match idx {
			0 => &mut self.x,
			1 => &mut self.y,
			2 => &mut self.z,
			_ => panic!("Accessed vector out of bounds.")
		}
	}
}

impl<T: Sized + Clone> HasPerElementOps for Vec3<T> {
	type ElemType = T;

	fn apply_op<U: Fn(&T) -> T>(&self, func: U) -> Self {
		Vec3::new([
			func(&self[0]),
			func(&self[1]),
			func(&self[2])
		])
	}
}
impl<T: Sized + Clone> HasPerElementBinOps for Vec3<T> {
	type ElemType = T;

	fn apply_bin_op<U: Fn(&T, T) -> T>(&self, rhs: Self, func: U) -> Self {
		Vec3::new([
			func(&self[0], rhs[0].clone()),
			func(&self[1], rhs[1].clone()),
			func(&self[2], rhs[2].clone())
		])
	}
}

impl<T> Add for Vec3<T>
	where T: Add<Output = T> + Sized + Clone
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Vec3::new([
			self[0].clone() + rhs[0].clone(),
			self[1].clone() + rhs[1].clone(),
			self[2].clone() + rhs[2].clone()
		])
	}
}
impl<T> Sub for Vec3<T>
	where T: Sub<Output = T> + Sized + Clone
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		Vec3::new([
			self[0].clone() - rhs[0].clone(),
			self[1].clone() - rhs[1].clone(),
			self[2].clone() - rhs[2].clone()
		])
	}
}
impl<T> Mul for Vec3<T>
	where T: Mul<Output = T> + Sized + Clone
{
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		Vec3::new([
			self[0].clone() * rhs[0].clone(),
			self[1].clone() * rhs[1].clone(),
			self[2].clone() * rhs[2].clone()
		])
	}
}
impl<T> Div for Vec3<T>
	where T: Div<Output = T> + Sized + Clone
{
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		Vec3::new([
			self[0].clone() / rhs[0].clone(),
			self[1].clone() / rhs[1].clone(),
			self[2].clone() / rhs[2].clone()
		])
	}
}

impl<T> Add<T> for Vec3<T>
	where T: Add<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn add(self, rhs: T) -> Self {
		Vec3::new([
			self[0].clone() + rhs.clone(),
			self[1].clone() + rhs.clone(),
			self[2].clone() + rhs
		])
	}
}
impl<T> Sub<T> for Vec3<T>
	where T: Sub<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn sub(self, rhs: T) -> Self {
		Vec3::new([
			self[0].clone() - rhs.clone(),
			self[1].clone() - rhs.clone(),
			self[2].clone() - rhs
		])
	}
}
impl<T> Mul<T> for Vec3<T>
	where T: Mul<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self {
		Vec3::new([
			self[0].clone() * rhs.clone(),
			self[1].clone() * rhs.clone(),
			self[2].clone() * rhs
		])
	}
}
impl<T> Div<T> for Vec3<T>
	where T: Div<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn div(self, rhs: T) -> Self {
		Vec3::new([
			self[0].clone() / rhs.clone(),
			self[1].clone() / rhs.clone(),
			self[2].clone() / rhs
		])
	}
}

impl<T> Rem<T> for Vec3<T>
	where T: Rem<Output = T> + Sized + Clone 
{
	type Output = Self;

	fn rem(self, rhs: T) -> Self {
		Vec3::new([
			self[0].clone() % rhs.clone(),
			self[1].clone() % rhs.clone(),
			self[2].clone() % rhs
		])
	}
}

impl<T> Rem for Vec3<T>
	where T: Rem<Output = T> + Sized + Clone
{
	type Output = Self;

	fn rem(self, rhs: Self) -> Self {
		Vec3::new([
			self[0].clone() % rhs[0].clone(),
			self[1].clone() % rhs[1].clone(),
			self[2].clone() % rhs[2].clone()
		])
	}
}

impl<T> HasDot for Vec3<T>
	where T: Mul<Output = T> + Add<Output = T> + Sized + Clone
{
	type Output = T;

	fn dot(&self, rhs: Self) -> T {
		let [x, y, z] = (self.clone() * rhs).as_array();
		return x + y + z;
	}
}

impl<T> HasCross for Vec3<T>
	where T: Mul<Output = T> + Sub<Output = T> + Sized + Clone + Copy
{
	fn cross(&self, rhs: Self) -> Self {
		vec3(
			self.y * rhs.z - self.z * rhs.y,
			self.z * rhs.x - self.x * rhs.z,
			self.x * rhs.y - self.y * rhs.x)
	}
}

impl<T> HasX for Vec3<T> 
	where T: Sized + Clone
{
	type Output = T;

	fn x(&self) -> &T {
		&self[0]
	}
}
impl<T> HasY for Vec3<T>
	where T: Sized + Clone
{
	fn y(&self) -> &T {
		&self[1]
	}
}
impl<T> HasZ for Vec3<T>
	where T: Sized + Clone
{
	fn z(&self) -> &T {
		&self[2]
	}
}
