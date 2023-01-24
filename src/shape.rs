use crate::Positions;

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

	type Positions: Iterator<Item = Self>;

	fn capacity(&self) -> usize;
	fn get(&self, axis: usize) -> usize;
	fn set(&mut self, axis: usize, value: usize);
	fn position_to_index(&self, position: Self) -> Option<usize>;
	fn index_to_position(&self, index: usize) -> Option<Self>;

	fn positions(&self) -> Self::Positions;
}

unsafe impl<const N: usize> Shape for [usize; N] {
	const DIMENSIONS: usize = N;
	const ZERO: Self = [0; N];

	type Positions = Positions<N>;

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

	fn positions(&self) -> Self::Positions {
		Positions::new(*self)
	}
}

pub unsafe trait AttachAxis<const AXIS: usize>: Shape {
	type Output: Shape;

	fn attach_axis(self, value: usize) -> Self::Output;
}

pub unsafe trait DetachAxis<const AXIS: usize>: Shape {
	type Output: Shape;

	fn detach_axis(self) -> Self::Output;
}

unsafe impl AttachAxis<0> for [usize; 0] {
	type Output = [usize; 1];

	fn attach_axis(self, value: usize) -> Self::Output {
		let [] = self;

		[value]
	}
}

unsafe impl AttachAxis<0> for [usize; 1] {
	type Output = [usize; 2];

	fn attach_axis(self, value: usize) -> Self::Output {
		let [b] = self;

		[value, b]
	}
}

unsafe impl AttachAxis<1> for [usize; 1] {
	type Output = [usize; 2];

	fn attach_axis(self, value: usize) -> Self::Output {
		let [a] = self;

		[a, value]
	}
}

unsafe impl AttachAxis<0> for [usize; 2] {
	type Output = [usize; 3];

	fn attach_axis(self, value: usize) -> Self::Output {
		let [b, c] = self;

		[value, b, c]
	}
}

unsafe impl AttachAxis<1> for [usize; 2] {
	type Output = [usize; 3];

	fn attach_axis(self, value: usize) -> Self::Output {
		let [a, c] = self;

		[a, value, c]
	}
}

unsafe impl AttachAxis<2> for [usize; 2] {
	type Output = [usize; 3];

	fn attach_axis(self, value: usize) -> Self::Output {
		let [a, b] = self;

		[a, b, value]
	}
}

unsafe impl DetachAxis<0> for [usize; 1] {
	type Output = [usize; 0];

	fn detach_axis(self) -> Self::Output {
		let [_] = self;

		[]
	}
}

unsafe impl DetachAxis<0> for [usize; 2] {
	type Output = [usize; 1];

	fn detach_axis(self) -> Self::Output {
		let [_, b] = self;

		[b]
	}
}

unsafe impl DetachAxis<1> for [usize; 2] {
	type Output = [usize; 1];

	fn detach_axis(self) -> Self::Output {
		let [a, _] = self;

		[a]
	}
}

unsafe impl DetachAxis<0> for [usize; 3] {
	type Output = [usize; 2];

	fn detach_axis(self) -> Self::Output {
		let [_, b, c] = self;

		[b, c]
	}
}

unsafe impl DetachAxis<1> for [usize; 3] {
	type Output = [usize; 2];

	fn detach_axis(self) -> Self::Output {
		let [a, _, c] = self;

		[a, c]
	}
}

unsafe impl DetachAxis<3> for [usize; 3] {
	type Output = [usize; 2];

	fn detach_axis(self) -> Self::Output {
		let [a, b, _] = self;

		[a, b]
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_attach() {
		let a = [3, 5];

		let a = AttachAxis::<1>::attach_axis(a, 4);

		assert_eq!(a, [3, 4, 5]);

	}
}
