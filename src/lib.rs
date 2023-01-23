mod view;

pub use view::View;
pub use view::ViewMut;

pub fn subd<const N: usize>(extents: [usize; N], dim: usize) -> usize {
	extents.into_iter().take(dim).product()
}

pub fn position_to_index<const N: usize>(
	extents: [usize; N],
	position: [usize; N],
) -> Option<usize> {
	(0..N).try_fold(0, |acc, i| {
		let stride = extents[i];
		let coordinate = position[i];

		if coordinate >= stride {
			return None;
		}

		Some(acc + coordinate * subd(extents, i))
	})
}

pub fn index_to_position<const N: usize>(extents: [usize; N], index: usize) -> Option<[usize; N]> {
	let capacity = extents.into_iter().product();

	if index >= capacity {
		return None;
	}

	let prev = 0;

	Some(std::array::from_fn(|i| {
		(index - prev) / subd(extents, i) % extents[i]
	}))
}

pub unsafe trait Shape: Sized {
	const DIMENSIONS: usize;
	const ZERO: Self;

	fn capacity(&self) -> usize;
	fn get(&self, axis: usize) -> usize;
	fn set(&mut self, axis: usize, value: usize);
	fn position_to_index(&self, position: Self) -> Option<usize>;
	fn index_to_position(&self, index: usize) -> Option<Self>;
}

unsafe impl<const N: usize> Shape for [usize; N] {
	const DIMENSIONS: usize = N;
	const ZERO: Self = [0; N];

	fn capacity(&self) -> usize {
		subd(*self, Self::DIMENSIONS)
	}
	fn get(&self, axis: usize) -> usize {
		self[axis]
	}
	fn set(&mut self, axis: usize, value: usize) {
		self[axis] = value
	}
	fn position_to_index(&self, position: Self) -> Option<usize> {
		position_to_index(*self, position)
	}
	fn index_to_position(&self, index: usize) -> Option<Self> {
		index_to_position(*self, index)
	}
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Singularity<T>(pub T);

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Axis<T, const D: usize, const N: usize>(pub [T; N]);

pub type Array0<T> = Singularity<T>;
pub type Array1<T, const A: usize> = Axis<Array0<T>, 1, A>;
pub type Array2<T, const A: usize, const B: usize> = Axis<Array1<T, A>, 2, B>;
pub type Array3<T, const A: usize, const B: usize, const C: usize> = Axis<Array2<T, A, B>, 3, C>;
pub type Array4<T, const A: usize, const B: usize, const C: usize, const D: usize> =
	Axis<Array3<T, A, B, C>, 4, D>;

pub unsafe trait Array: Sized {
	type Item;
	type Shape: Shape;

	const SHAPE: Self::Shape;

	fn shape(&self) -> Self::Shape {
		Self::SHAPE
	}

	fn as_ptr(&self) -> *const Self::Item {
		self as *const Self as *const Self::Item
	}

	fn as_mut_ptr(&mut self) -> *mut Self::Item {
		self as *mut Self as *mut Self::Item
	}

	fn as_slice(&self) -> &[Self::Item] {
		unsafe { std::slice::from_raw_parts(self.as_ptr(), Self::SHAPE.capacity()) }
	}

	fn as_mut_slice(&mut self) -> &mut [Self::Item] {
		unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), Self::SHAPE.capacity()) }
	}

	fn item(&self, position: Self::Shape) -> Option<&Self::Item> {
		self.as_slice()
			.get(Self::SHAPE.position_to_index(position)?)
	}
	fn item_mut(&mut self, position: Self::Shape) -> Option<&mut Self::Item> {
		self.as_mut_slice()
			.get_mut(Self::SHAPE.position_to_index(position)?)
	}
}

pub unsafe trait Higher {
	type Higher<const N: usize>;
}

pub unsafe trait Lower {
	type Lower;

	fn lower(&self, axis: usize) -> Option<&Self::Lower>;
	fn lower_mut(&mut self, axis: usize) -> Option<&mut Self::Lower>;
}

pub unsafe trait Axies<const AXIS: usize>: Array {
	type Axis: Shape;

	const SHAPE: Self::Axis;

	fn axis(&self, axis: usize) -> Option<View<Self, Self::Axis, AXIS>> {
		if axis >= <Self as Array>::SHAPE.get(AXIS) {
			return None;
		}

		Some(unsafe { View::new_unchecked(self, axis) })
	}
	fn axis_mut(&mut self, axis: usize) -> Option<ViewMut<Self, Self::Axis, AXIS>> {
		if axis >= <Self as Array>::SHAPE.get(AXIS) {
			return None;
		}

		Some(unsafe { ViewMut::new_unchecked(self, axis) })
	}

	fn attach_axis(position: Self::Axis, axis: usize) -> <Self as Array>::Shape;
}

unsafe impl<T> Array for Array0<T> {
	type Item = T;
	type Shape = [usize; 0];

	const SHAPE: Self::Shape = [];
}

unsafe impl<T, const A: usize> Array for Array1<T, A> {
	type Item = T;
	type Shape = [usize; 1];

	const SHAPE: Self::Shape = [A];
}

unsafe impl<T, const A: usize, const B: usize> Array for Array2<T, A, B> {
	type Item = T;
	type Shape = [usize; 2];

	const SHAPE: Self::Shape = [A, B];
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Array for Array3<T, A, B, C> {
	type Item = T;
	type Shape = [usize; 3];

	const SHAPE: Self::Shape = [A, B, C];
}

unsafe impl<T> Higher for Array0<T> {
	type Higher<const N: usize> = Array1<T, N>;
}

unsafe impl<T, const A: usize> Higher for Array1<T, A> {
	type Higher<const B: usize> = Array2<T, A, B>;
}

unsafe impl<T, const A: usize, const B: usize> Higher for Array2<T, A, B> {
	type Higher<const C: usize> = Array3<T, A, B, C>;
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Higher for Array3<T, A, B, C> {
	type Higher<const D: usize> = Array4<T, A, B, C, D>;
}

unsafe impl<T, const A: usize> Lower for Array1<T, A> {
	type Lower = Array0<T>;

	fn lower(&self, axis: usize) -> Option<&Self::Lower> {
		self.0.get(axis)
	}
	fn lower_mut(&mut self, axis: usize) -> Option<&mut Self::Lower> {
		self.0.get_mut(axis)
	}
}

unsafe impl<T, const A: usize, const B: usize> Lower for Array2<T, A, B> {
	type Lower = Array1<T, A>;

	fn lower(&self, axis: usize) -> Option<&Self::Lower> {
		self.0.get(axis)
	}
	fn lower_mut(&mut self, axis: usize) -> Option<&mut Self::Lower> {
		self.0.get_mut(axis)
	}
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Lower for Array3<T, A, B, C> {
	type Lower = Array2<T, A, B>;

	fn lower(&self, axis: usize) -> Option<&Self::Lower> {
		self.0.get(axis)
	}
	fn lower_mut(&mut self, axis: usize) -> Option<&mut Self::Lower> {
		self.0.get_mut(axis)
	}
}

unsafe impl<T, const A: usize, const B: usize, const C: usize, const D: usize> Lower
	for Array4<T, A, B, C, D>
{
	type Lower = Array3<T, A, B, C>;

	fn lower(&self, axis: usize) -> Option<&Self::Lower> {
		self.0.get(axis)
	}
	fn lower_mut(&mut self, axis: usize) -> Option<&mut Self::Lower> {
		self.0.get_mut(axis)
	}
}

unsafe impl<T, const A: usize> Axies<0> for Array1<T, A> {
	type Axis = [usize; 0];

	const SHAPE: Self::Axis = [];

	fn attach_axis([]: Self::Axis, axis: usize) -> <Self as Array>::Shape {
		[axis]
	}
}

unsafe impl<T, const A: usize, const B: usize> Axies<0> for Array2<T, A, B> {
	type Axis = [usize; 1];

	const SHAPE: Self::Axis = [B];

	fn attach_axis([b]: Self::Axis, axis: usize) -> <Self as Array>::Shape {
		[axis, b]
	}
}

unsafe impl<T, const A: usize, const B: usize> Axies<1> for Array2<T, A, B> {
	type Axis = [usize; 1];

	const SHAPE: Self::Axis = [A];

	fn attach_axis([a]: Self::Axis, axis: usize) -> <Self as Array>::Shape {
		[a, axis]
	}
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Axies<0> for Array3<T, A, B, C> {
	type Axis = [usize; 2];

	const SHAPE: Self::Axis = [B, C];

	fn attach_axis([b, c]: Self::Axis, axis: usize) -> <Self as Array>::Shape {
		[axis, b, c]
	}
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Axies<1> for Array3<T, A, B, C> {
	type Axis = [usize; 2];

	const SHAPE: Self::Axis = [A, C];

	fn attach_axis([a, c]: Self::Axis, axis: usize) -> <Self as Array>::Shape {
		[a, axis, c]
	}
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Axies<2> for Array3<T, A, B, C> {
	type Axis = [usize; 2];

	const SHAPE: Self::Axis = [A, B];

	fn attach_axis([a, b]: Self::Axis, axis: usize) -> <Self as Array>::Shape {
		[a, b, axis]
	}
}
