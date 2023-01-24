use crate::Axies;
use crate::AxiesMut;
use crate::MutableArrayBase;
use crate::ReadOnlyArrayBase;
use crate::View;
use crate::ViewMut;

#[derive(Debug, Default, Copy, Clone)]
pub struct Array<A> {
	array: A,
}

impl<A> Array<A> {
	pub fn new(array: A) -> Self {
		Self { array }
	}
	pub fn into_inner(self) -> A {
		self.array
	}
}

impl<A> Array<A>
where
	A: ReadOnlyArrayBase,
{
	pub fn item(&self, position: A::Shape) -> Option<&A::Item> {
		self.array.item(position)
	}
	pub fn axis<const AXIS: usize>(&self, index: usize) -> Option<Array<View<A, A::Axis, AXIS>>>
	where
		A: Axies<AXIS>,
	{
		self.array.axis(index).map(Array::new)
	}
	pub fn shape(&self) -> A::Shape {
		self.array.shape()
	}
}

impl<A> Array<A>
where
	A: MutableArrayBase,
{
	pub fn item_mut(&mut self, position: A::Shape) -> Option<&mut A::Item> {
		self.array.item_mut(position)
	}
	pub fn axis_mut<const AXIS: usize>(
		&mut self,
		index: usize,
	) -> Option<Array<ViewMut<A, A::Axis, AXIS>>>
	where
		A: AxiesMut<AXIS>,
	{
		self.array.axis_mut(index).map(Array::new)
	}
}
