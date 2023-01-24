mod array;
mod shape;
mod view;

pub use shape::DetachAxis;
pub use shape::AttachAxis;
pub use shape::Shape;

pub use crate::array::Array;

pub use view::View;
pub use view::ViewMut;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Singularity<T>(pub T);

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Axis<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> Default for Axis<T, N>
where
	T: Default,
{
	fn default() -> Self {
		Self(std::array::from_fn(|_| Default::default()))
	}
}

pub type Axis0<T> = Singularity<T>;
pub type Axis1<T, const A: usize> = Axis<Axis0<T>, A>;
pub type Axis2<T, const A: usize, const B: usize> = Axis<Axis1<T, A>, B>;
pub type Axis3<T, const A: usize, const B: usize, const C: usize> = Axis<Axis2<T, A, B>, C>;
pub type Axis4<T, const A: usize, const B: usize, const C: usize, const D: usize> =
	Axis<Axis3<T, A, B, C>, D>;

pub type Array0<T> = Array<Axis0<T>>;
pub type Array1<T, const _0: usize> = Array<Axis1<T, _0>>;
pub type Array2<T, const _0: usize, const _1: usize> = Array<Axis2<T, _0, _1>>;
pub type Array3<T, const _0: usize, const _1: usize, const _2: usize> = Array<Axis3<T, _0, _1, _2>>;
pub type Array4<T, const _0: usize, const _1: usize, const _2: usize, const _3: usize> =
	Array<Axis4<T, _0, _1, _2, _3>>;

pub type Cubic2<T, const _0: usize> = Array2<T, _0, _0>;
pub type Cubic3<T, const _0: usize> = Array3<T, _0, _0, _0>;
pub type Cubic4<T, const _0: usize> = Array4<T, _0, _0, _0, _0>;

/// Base trait implemented by all [`Array`]s
///
/// # Safety
///
/// The safety requirements of this trait are unspecified and implementing it is unsafe
///
pub unsafe trait ReadOnlyArrayBase: Sized {
	type Item;
	type Shape: Shape;

	// const SHAPE: Self::Shape;

	fn shape(&self) -> Self::Shape;

	fn item(&self, position: Self::Shape) -> Option<&Self::Item>;
}

pub unsafe trait MutableArrayBase: ReadOnlyArrayBase {
	fn item_mut(&mut self, position: Self::Shape) -> Option<&mut Self::Item>;
}

/// Allow indexing into an `Array`
///
/// # Safety
///
/// The safety requirements of this trait are unspecified and implementing it is unsafe
///
pub unsafe trait Axies<const AXIS: usize>: ReadOnlyArrayBase {
	type Axis: Shape;

	fn axis(&self, axis: usize) -> Option<View<Self, Self::Axis, AXIS>> {
		if axis >= self.shape().get(AXIS) {
			return None;
		}

		Some(unsafe { View::new_unchecked(self, axis) })
	}
}

/// Allow indexing into an `Array`
///
/// # Safety
///
/// The safety requirements of this trait are unspecified and implementing it is unsafe
///
pub unsafe trait AxiesMut<const AXIS: usize>: Axies<AXIS> {
	fn axis_mut(&mut self, axis: usize) -> Option<ViewMut<Self, Self::Axis, AXIS>> {
		if axis >= self.shape().get(AXIS) {
			return None;
		}

		Some(unsafe { ViewMut::new_unchecked(self, axis) })
	}
}

macro_rules! impl_array {
	( $Self:ident, $D:expr, $($A:ident),*) => {
		unsafe impl<T, $(const $A: usize),*> ReadOnlyArrayBase for $Self<T, $($A),*> {
			type Item = T;
			type Shape = [usize; $D];

			fn shape(&self) -> Self::Shape {
				[$($A),*]
			}

			fn item(&self, position: Self::Shape) -> Option<&Self::Item> {
				let shape = self.shape();

				unsafe { std::slice::from_raw_parts(self as *const Self as *const Self::Item, shape.capacity()).get(shape.position_to_index(position)?) }
			}
		}

		unsafe impl<T, $(const $A: usize),*> MutableArrayBase for $Self<T, $($A),*> {
			fn item_mut(&mut self, position: Self::Shape) -> Option<&mut Self::Item> {
				let shape = self.shape();

				unsafe { std::slice::from_raw_parts_mut(self as *mut Self as *mut Self::Item, shape.capacity()).get_mut(shape.position_to_index(position)?) }
			}
		}
	}
}

impl_array! { Axis0, 0, }
impl_array! { Axis1, 1, _0 }
impl_array! { Axis2, 2, _0, _1 }
impl_array! { Axis3, 3, _0, _1, _3 }
impl_array! { Axis4, 4, _0, _1, _3, _4 }

unsafe impl<T, const A: usize> Axies<0> for Axis1<T, A> {
	type Axis = [usize; 0];

	// const SHAPE: Self::Axis = [];
}

unsafe impl<T, const A: usize, const B: usize> Axies<0> for Axis2<T, A, B> {
	type Axis = [usize; 1];

	// const SHAPE: Self::Axis = [B];
}

unsafe impl<T, const A: usize, const B: usize> Axies<1> for Axis2<T, A, B> {
	type Axis = [usize; 1];

	// const SHAPE: Self::Axis = [A];
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Axies<0> for Axis3<T, A, B, C> {
	type Axis = [usize; 2];

	// const SHAPE: Self::Axis = [B, C];
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Axies<1> for Axis3<T, A, B, C> {
	type Axis = [usize; 2];

	// const SHAPE: Self::Axis = [A, C];
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Axies<2> for Axis3<T, A, B, C> {
	type Axis = [usize; 2];

	// const SHAPE: Self::Axis = [A, B];
}

unsafe impl<T, const A: usize> AxiesMut<0> for Axis1<T, A> {}

unsafe impl<T, const A: usize, const B: usize> AxiesMut<0> for Axis2<T, A, B> {}

unsafe impl<T, const A: usize, const B: usize> AxiesMut<1> for Axis2<T, A, B> {}

unsafe impl<T, const A: usize, const B: usize, const C: usize> AxiesMut<0> for Axis3<T, A, B, C> {}

unsafe impl<T, const A: usize, const B: usize, const C: usize> AxiesMut<1> for Axis3<T, A, B, C> {}

unsafe impl<T, const A: usize, const B: usize, const C: usize> AxiesMut<2> for Axis3<T, A, B, C> {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn some_test() {
		let some_array = Array3::<char, 10, 11, 12>::default();

		let view1 = some_array.axis::<1>(4).unwrap();

		assert!(view1.shape() == [10, 12]);

		let view2 = view1.axis::<0>(4).unwrap();

		assert!(view2.shape() == [12]);
	}


}

