
use vecmath::*;

use std::ops::*;

use traits::*;

#[derive(Copy, Clone, Debug)]
pub struct Vec3<T: Sized + Clone>(pub Vector3<T>);

impl<T: Sized + Clone> Index<usize> for Vec3<T> {
	type Output = T;

	fn index(&self, idx: usize) -> &Self::Output {
		&self.0[idx]
	}
}
impl<T: Sized + Clone> IndexMut<usize> for Vec3<T> {
	fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
		&mut self.0[idx]
	}
}

impl<T: Sized + Clone> HasPerElementOps for Vec3<T> {
	type ElemType = T;

	fn apply_op<U: Fn(&T) -> T>(&self, func: U) -> Self {
		Vec3([
			func(&self[0]),
			func(&self[1]),
			func(&self[2])
		])
	}
}
impl<T: Sized + Clone> HasPerElementBinOps for Vec3<T> {
	type ElemType = T;

	fn apply_bin_op<U: Fn(&T, T) -> T>(&self, rhs: Self, func: U) -> Self {
		Vec3([
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
		Vec3([
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
		Vec3([
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
		Vec3([
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
		Vec3([
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
		Vec3([
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
		Vec3([
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
		Vec3([
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
		Vec3([
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
		Vec3([
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
		Vec3([
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
		let [x, y, z] = (self.clone() * rhs).0;
		return x + y + z;
	}
}

impl<T> HasCross for Vec3<T>
	where T: Mul<Output = T> + Sub<Output = T> + Sized + Clone + Copy
{
	fn cross(&self, rhs: Self) -> Self {
		Vec3(vec3_cross(self.0, rhs.0))
	}
}
