
use num::traits::*;
use vecmath::*;

use std::ops::Fn;

pub trait HasDot {
	type Output;

	fn dot(&self, other: Self) -> Self::Output;
}
pub trait HasCross {
	fn cross(&self, other: Self) -> Self;
}
pub trait HasLength {
	type Output;

	fn length(&self) -> Self::Output;

	fn magnitude(&self) -> Self::Output {
		return self.length();
	}

	fn normalize(&self) -> Self;
}
pub trait HasDistance {
	type Output;

	fn distance(&self, rhs: Self) -> Self::Output;
}

pub trait HasFaceForward {
	fn faceforward(&self, I: Self, Nref: Self) -> Self;
}
pub trait HasReflect {
	fn reflect(&self, N: Self) -> Self;
}
pub trait HasRefract {
	type ElemType;

	fn refract(&self, N: Self, eta: Self::ElemType) -> Self;
}

pub trait HasX {
	type Output;

	fn x(&self) -> Self::Output;
}
pub trait HasY: HasX {
	fn y(&self) -> Self::Output;
}
pub trait HasZ: HasY {
	fn z(&self) -> Self::Output;
}
pub trait HasW: HasZ {
	fn w(&self) -> Self::Output;
}

pub trait HasPerElementOps {
	type ElemType;

	fn apply_op(&self, func: Fn(Self::ElemType) -> Self::ElemType) -> Self;
}

pub trait HasAbs {
	fn abs(&self) -> Self;
}

pub trait HasTrig {
	fn sin(&self) -> Self;
	fn cos(&self) -> Self;
	fn tan(&self) -> Self;

	fn asin(&self) -> Self;
	fn acos(&self) -> Self;
	fn atan(&self) -> Self;
}

pub trait HasExponential {
	fn pow(&self, exponent: Self) -> Self;
	fn log(&self) -> Self;
	fn exp(&self) -> Self;
	fn log2(&self) -> Self;
	fn exp2(&self) -> Self;
}

pub trait HasSqrt {
	fn sqrt(&self) -> Self;
	fn inv_sqrt(&self) -> Self;
}

pub trait HasSign {
	fn sign(&self) -> Self;
}

pub trait HasFloor {
	fn floor(&self) -> Self;
}
pub trait HasCeil {
	fn ceil(&self) -> Self;
}
pub trait HasFract {
	fn fract(&self) -> Self;
}
pub trait HasMod {
	fn modulus(&self, modulus: Self) -> Self;
}

pub trait HasMinMax {
	fn min(&self, rhs: Self) -> Self;
	fn max(&self, rhs: Self) -> Self;

	fn clamp(&self, min: Self, max: Self) -> Self;
}

pub trait HasMix {
	type ElemType;

	fn mix(&self, rhs: Self, f: Self::ElemType) -> Self;
}

pub trait HasStep {
	// NOTE: GLSL function arguments are the other way around
	fn step(&self, edge: Self) -> Self;
}

pub trait HasSmoothStep {
	fn smoothstep(&self, edge0: Self, edge1: Self) -> Self;
}
