use crate::Shape;

pub struct Positions<const N: usize> {
	start: usize,
	shape: [usize; N],
}

impl<const N: usize> Positions<N> {
	pub fn new(shape: [usize; N]) -> Self {
		Self { start: 0, shape }
	}
}

impl<const N: usize> Iterator for Positions<N> {
	type Item = [usize; N];

	fn next(&mut self) -> Option<Self::Item> {
		let mut it = self.start..self.shape.capacity();

		let next = it.next();

		self.start = it.start;

		let next = next?;

		self.shape.index_to_position(next)
	}
}
