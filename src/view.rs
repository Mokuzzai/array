use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::AttachAxis;
use crate::DetachAxis;
use crate::Axies;
use crate::MutableArrayBase;
use crate::ReadOnlyArrayBase;
use crate::Shape;

pub struct View<'a, H, L, const AXIS: usize> {
	src: &'a H,
	axis: usize,
	_logical: PhantomData<&'a H>,
	_shape: PhantomData<L>,
}

impl<'a, H, L, const AXIS: usize> View<'a, H, L, AXIS> {
	pub(crate) unsafe fn new_unchecked(src: &'a H, axis: usize) -> Self {
		Self {
			src,
			axis,
			_logical: PhantomData,
			_shape: PhantomData,
		}
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


pub struct ViewMut<'a, H, L, const AXIS: usize> {
	///
	/// # Safety
	///
	/// `src` has all the same invariants as `&'a mut H`
	/// but it is allowed to alias with references returned by
	/// `Self::item[|_mut]`
	///
	/// the only reason we don't use a mutable reference
	/// is because `Self::item[|_mut]` would not compile because:
	/// ```ignore
	/// let view: ViewMut<'a, ..> = ..;
	///
	/// let a: &'a mut H = view.item_mut(..); // returns a mutable reference;
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
		unsafe { self.src.as_ref() }.shape().detach_axis()
	}

	fn item(&self, position: L) -> Option<&H::Item> {
		let position = position.attach_axis(self.axis);

		unsafe {
			// SAFETY: `self.src` is a valid reference as per its invariant
			self.src.as_ref().item(position)
		}
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

		unsafe {
			// SAFETY: `self.src` is a valid mutable reference as per its invariant
			self.src.as_mut().item_mut(position)
		}
	}
}
