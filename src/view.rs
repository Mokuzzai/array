use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::Array;
use crate::Axies;
use crate::Shape;

pub struct View<'a, H, L, const AXIS: usize> {
	src: NonNull<H>,
	axis: usize,
	_logical: PhantomData<&'a L>,
}

impl<'a, H, L, const AXIS: usize> View<'a, H, L, AXIS> {
	pub unsafe fn new_unchecked(src: &H, axis: usize) -> Self {
		Self {
			src: NonNull::from(src),
			axis,
			_logical: PhantomData,
		}
	}
}

impl<'a, H, L, const AXIS: usize> View<'a, H, L, AXIS>
where
	H: Array + Axies<AXIS, Axis = L>,
	L: Shape,
{
	pub fn item(&self, position: L) -> Option<&H::Item> {
		let position = H::attach_axis(position, self.axis);

		unsafe { self.src.as_ref().item(position) }
	}
}

pub struct ViewMut<'a, H, L, const AXIS: usize> {
	src: NonNull<H>,
	axis: usize,
	_logical: PhantomData<&'a mut L>,
}

impl<'a, H, L, const AXIS: usize> ViewMut<'a, H, L, AXIS> {
	pub unsafe fn new_unchecked(src: &mut H, axis: usize) -> Self {
		Self {
			src: NonNull::from(src),
			axis,
			_logical: PhantomData,
		}
	}
}

impl<'a, H, L, const AXIS: usize> ViewMut<'a, H, L, AXIS>
where
	H: Array + Axies<AXIS, Axis = L>,
	L: Shape,
{
	pub fn item(&self, position: L) -> Option<&H::Item> {
		let position = H::attach_axis(position, self.axis);

		unsafe { self.src.as_ref().item(position) }
	}
	pub fn item_mut(&mut self, position: L) -> Option<&mut H::Item> {
		let position = H::attach_axis(position, self.axis);

		unsafe { self.src.as_mut().item_mut(position) }
	}
}
