use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::AttachAxis;
use crate::Axies;
use crate::AxiesMut;
use crate::DetachAxis;
use crate::MutableArrayBase;
use crate::ReadOnlyArrayBase;
use crate::Shape;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct View<'a, H, L, const AXIS: usize> {
	src: &'a H,
	axis: usize,
	_shape: PhantomData<L>,
}

impl<'a, H, L, const AXIS: usize> View<'a, H, L, AXIS> {
	pub(crate) unsafe fn new_unchecked(src: &'a H, axis: usize) -> Self {
		Self {
			src,
			axis,
			_shape: PhantomData,
		}
	}
}

impl<'a, H, L, const AXIS: usize> View<'a, H, L, AXIS>
where
	H: ReadOnlyArrayBase + Axies<AXIS, Axis = L>,
	H::Shape: DetachAxis<AXIS, Output = L>,
	L: Shape + AttachAxis<AXIS, Output = H::Shape>,
{
	pub fn iter(&self) -> impl Iterator<Item = &H::Item> {
		Iter::new(self)
	}
}

unsafe impl<'a, H, L, const AXIS: usize> ReadOnlyArrayBase for View<'a, H, L, AXIS>
where
	H: ReadOnlyArrayBase + Axies<AXIS, Axis = L>,
	H::Shape: DetachAxis<AXIS, Output = L>,
	L: Shape + AttachAxis<AXIS, Output = H::Shape>,
{
	type Item = H::Item;
	type Shape = L;

	fn shape(&self) -> Self::Shape {
		self.src.shape().detach_axis()
	}

	fn item(&self, position: L) -> Option<&H::Item> {
		let position = position.attach_axis(self.axis);

		self.src.item(position)
	}
}

unsafe impl<'a, H, L, const AXIS: usize, const N: usize> Axies<N> for View<'a, H, L, AXIS>
where
	H: ReadOnlyArrayBase + Axies<AXIS, Axis = L>,
	H::Shape: DetachAxis<AXIS, Output = L>,
	L: Shape + AttachAxis<AXIS, Output = H::Shape> + DetachAxis<N>,
{
	type Axis = <L as DetachAxis<N>>::Output;
}

#[repr(C)]
pub struct ViewMut<'a, H, L, const AXIS: usize> {
	///
	/// # Safety
	///
	/// `src` has all the same invariants as `&'a mut H`
	/// but it is allowed to alias with references returned by
	/// `Self::item[|_mut]`
	///
	/// the only reason we don't use a mutable reference
	/// is because `Self::item` would not compile because:
	/// ```ignore
	/// let view: ViewMut<'a, ..> = ..;
	///
	/// let a: &'a H = view.item(..); // returns a reference;
	/// let b: &'a mut H = view.src;
	///
	/// // `a` and `b` would alias
	/// ```
	/// with `src` being `NonNull` no aliasing happens
	///
	src: NonNull<H>,
	axis: usize,
	_logical: PhantomData<&'a mut H>,
	_shape: PhantomData<L>,
}

impl<'a, H, L, const AXIS: usize> ViewMut<'a, H, L, AXIS> {
	fn as_ref(&self) -> &H {
		unsafe {
			// SAFETY: `self.src` is a valid reference as per its invariant
			self.src.as_ref()
		}
	}
	fn as_mut(&mut self) -> &mut H {
		unsafe {
			// SAFETY: `self.src` is a valid mutable reference as per its invariant
			self.src.as_mut()
		}
	}
}

impl<'a, H, L, const AXIS: usize> ViewMut<'a, H, L, AXIS>
where
	H: ReadOnlyArrayBase + Axies<AXIS, Axis = L>,
	H::Shape: DetachAxis<AXIS, Output = L>,
	L: Shape + AttachAxis<AXIS, Output = H::Shape>,
{
	pub fn iter(&self) -> Iter<'a, H, L, AXIS> {
		Iter::new(unsafe {
			// SAFETY: both structs have same layout
			// TODO add method to do this
			&*(self as *const _ as *const _)
		})
	}
	pub fn iter_mut(&'a mut self) -> IterMut<'a, H, L, AXIS>
	where
		H: MutableArrayBase,
	{
		IterMut::new(self)
	}
}

/// # Safety
///
/// because `Self` is roughly `&'a mut H` if it is `Send` so is `Self`
///
unsafe impl<'a, H, L, const AXIS: usize> Send for ViewMut<'a, H, L, AXIS> where &'a mut H: Send {}

/// # Safety
///
/// because `Self` is roughly `&'a mut H` if it is `Sync` so is `Self`
///
unsafe impl<'a, H, L, const AXIS: usize> Sync for ViewMut<'a, H, L, AXIS> where &'a mut H: Sync {}

impl<'a, H, L, const AXIS: usize> ViewMut<'a, H, L, AXIS> {
	pub(crate) unsafe fn new_unchecked(src: &'a mut H, axis: usize) -> Self {
		Self {
			src: NonNull::from(src),
			axis,
			_logical: PhantomData,
			_shape: PhantomData,
		}
	}
}

unsafe impl<'a, H, L, const AXIS: usize> ReadOnlyArrayBase for ViewMut<'a, H, L, AXIS>
where
	H: ReadOnlyArrayBase + Axies<AXIS, Axis = L>,
	H::Shape: DetachAxis<AXIS, Output = L>,
	L: Shape + AttachAxis<AXIS, Output = H::Shape>,
{
	type Item = H::Item;
	type Shape = L;

	fn shape(&self) -> Self::Shape {
		self.as_ref().shape().detach_axis()
	}

	fn item(&self, position: L) -> Option<&H::Item> {
		let position = position.attach_axis(self.axis);

		self.as_ref().item(position)
	}
}

unsafe impl<'a, H, L, const AXIS: usize> MutableArrayBase for ViewMut<'a, H, L, AXIS>
where
	H: MutableArrayBase + Axies<AXIS, Axis = L>,
	H::Shape: DetachAxis<AXIS, Output = L>,
	L: Shape + AttachAxis<AXIS, Output = H::Shape>,
{
	fn item_mut(&mut self, position: L) -> Option<&mut H::Item> {
		let position = position.attach_axis(self.axis);

		self.as_mut().item_mut(position)
	}
}

unsafe impl<'a, H, L, const AXIS: usize, const N: usize> Axies<N> for ViewMut<'a, H, L, AXIS>
where
	H: ReadOnlyArrayBase + Axies<AXIS, Axis = L>,
	H::Shape: DetachAxis<AXIS, Output = L>,
	L: Shape + AttachAxis<AXIS, Output = H::Shape> + DetachAxis<N>,
{
	type Axis = <L as DetachAxis<N>>::Output;
}

unsafe impl<'a, H, L, const AXIS: usize, const N: usize> AxiesMut<N> for ViewMut<'a, H, L, AXIS>
where
	H: ReadOnlyArrayBase + Axies<AXIS, Axis = L>,
	H::Shape: DetachAxis<AXIS, Output = L>,
	L: Shape + AttachAxis<AXIS, Output = H::Shape> + DetachAxis<N>,
{
}

pub struct Iter<'a, H, L, const AXIS: usize> {
	src: &'a View<'a, H, L, AXIS>,
	index: usize,
}

impl<'a, H, L, const AXIS: usize> Iter<'a, H, L, AXIS>
where
	H: ReadOnlyArrayBase + Axies<AXIS, Axis = L>,
	H::Shape: DetachAxis<AXIS, Output = L>,
	L: Shape + AttachAxis<AXIS, Output = H::Shape>,
{
	fn new(src: &'a View<'a, H, L, AXIS>) -> Self {
		Self { src, index: 0 }
	}
}

impl<'a, H, L, const AXIS: usize> Iterator for Iter<'a, H, L, AXIS>
where
	H: ReadOnlyArrayBase + Axies<AXIS, Axis = L>,
	H::Shape: DetachAxis<AXIS, Output = L>,
	L: Shape + AttachAxis<AXIS, Output = H::Shape>,
{
	type Item = &'a H::Item;

	fn next(&mut self) -> Option<Self::Item> {
		let mut it = self.index..self.src.shape().capacity();

		let next = it.next();

		self.index = it.start;

		let next = next?;

		let position = self.src.shape().index_to_position(next)?;

		self.src.item(position)
	}
}

pub struct IterMut<'a, H, L, const AXIS: usize> {
	src: NonNull<ViewMut<'a, H, L, AXIS>>,
	index: usize,
}

impl<'a, H, L, const AXIS: usize> IterMut<'a, H, L, AXIS>
where
	H: ReadOnlyArrayBase + Axies<AXIS, Axis = L>,
	H::Shape: DetachAxis<AXIS, Output = L>,
	L: Shape + AttachAxis<AXIS, Output = H::Shape>,
{
	fn new(src: &'a mut ViewMut<'a, H, L, AXIS>) -> Self {
		Self {
			src: NonNull::from(src),
			index: 0,
		}
	}
}

impl<'a, H, L: 'a, const AXIS: usize> Iterator for IterMut<'a, H, L, AXIS>
where
	H: MutableArrayBase + Axies<AXIS, Axis = L>,
	H::Shape: DetachAxis<AXIS, Output = L>,
	L: Shape + AttachAxis<AXIS, Output = H::Shape>,
{
	type Item = &'a mut H::Item;

	fn next(&mut self) -> Option<Self::Item> {
		let shape = unsafe { self.src.as_ref().shape() };

		let mut it = self.index..shape.capacity();

		let next = it.next();

		self.index = it.start;

		let next = next?;

		let position = shape.index_to_position(next)?;

		unsafe { self.src.as_mut().item_mut(position) }
	}
}

const _: () = {
	use core::fmt::*;

	impl<'a, H, L, const AXIS: usize> Debug for View<'a, H, L, AXIS>
	where
		H: ReadOnlyArrayBase + Axies<AXIS, Axis = L>,
		H::Shape: DetachAxis<AXIS, Output = L>,
		H::Item: Debug,
		L: Shape + AttachAxis<AXIS, Output = H::Shape>,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_list().entries(self.iter()).finish()
		}
	}

	impl<'a, H, L, const AXIS: usize> Debug for ViewMut<'a, H, L, AXIS>
	where
		H: ReadOnlyArrayBase + Axies<AXIS, Axis = L>,
		H::Shape: DetachAxis<AXIS, Output = L>,
		H::Item: Debug,
		L: Shape + AttachAxis<AXIS, Output = H::Shape>,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_list().entries(self.iter()).finish()
		}
	}
};
