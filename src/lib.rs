#[repr(C)]
struct Singularity<T>(pub T);

#[repr(C)]
struct Axis<T, const D: usize, const N: usize>([T; N]);

type Array0<T> = Singularity<T>;
type Array1<T, const A: usize> = Axis<Array0<T>, 1, A>;
type Array2<T, const A: usize, const B: usize> = Axis<Array1<T, A>, 2, B>;
type Array3<T, const A: usize, const B: usize, const C: usize> = Axis<Array2<T, A, B>, 3, C>;
type Array4<T, const A: usize, const B: usize, const C: usize, const D: usize> = Axis<Array3<T, A, B, C>, 4, D>;

unsafe trait Array {
    type Item;
    type Shape;
    type Higher<const N: usize>;

    const SHAPE: Self::Shape;
    const CAPACITY: usize;
    const DIMENSIONS: usize;
}

unsafe trait Lower {
    type Lower;


    fn lower(&self, axis: usize) -> Option<&Self::Lower>;
    fn lower_mut(&mut self, axis: usize) -> Option<&mut Self::Lower>;
}

unsafe trait Axies<const AXIS: usize>: Array {
    type Axis: Array;
}

unsafe impl<T> Array for Array0<T> {
    type Item = T;
    type Shape = [usize; 0];
    type Higher<const N: usize> = Array1<T, N>;

    const SHAPE: Self::Shape = [];
    const CAPACITY: usize = 1;
    const DIMENSIONS: usize = 0;
}

unsafe impl<T, const A: usize> Array for Array1<T, A> {
    type Item = T;
    type Shape = [usize; 1];
    type Higher<const B: usize> = Array2<T, A, B>;

    const SHAPE: Self::Shape = [A];
    const CAPACITY: usize = 1 * A;
    const DIMENSIONS: usize = 1;
}

unsafe impl<T, const A: usize, const B: usize> Array for Array2<T, A, B> {
    type Item = T;
    type Shape = [usize; 2];
    type Higher<const C: usize> = Array3<T, A, B, C>;

    const SHAPE: Self::Shape = [A, B];
    const CAPACITY: usize = 1 * A * B;
    const DIMENSIONS: usize = 2;
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Array for Array3<T, A, B, C> {
    type Item = T;
    type Shape = [usize; 3];
    type Higher<const D: usize> = Array4<T, A, B, C, D>;

    const SHAPE: Self::Shape = [A, B, C];
    const CAPACITY: usize = 1 * A * B * C;
    const DIMENSIONS: usize = 3;
}

unsafe impl<T, const A: usize> Lower for Array1<T, A> {
    type Lower = Array0<T>;

    fn lower(&self, axis: usize) -> Option<&Self::Lower> { self.0.get(axis) }
    fn lower_mut(&mut self, axis: usize) -> Option<&mut Self::Lower> { self.0.get_mut(axis) }
}

unsafe impl<T, const A: usize, const B: usize> Lower for Array2<T, A, B> {
    type Lower = Array1<T, A>;

    fn lower(&self, axis: usize) -> Option<&Self::Lower> { self.0.get(axis) }
    fn lower_mut(&mut self, axis: usize) -> Option<&mut Self::Lower> { self.0.get_mut(axis) }
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Lower for Array3<T, A, B, C> {
    type Lower = Array2<T, A, B>;

    fn lower(&self, axis: usize) -> Option<&Self::Lower> { self.0.get(axis) }
    fn lower_mut(&mut self, axis: usize) -> Option<&mut Self::Lower> { self.0.get_mut(axis) }
}

unsafe impl<T, const A: usize, const B: usize, const C: usize, const D: usize> Lower for Array4<T, A, B, C, D> {
    type Lower = Array3<T, A, B, C>;

    fn lower(&self, axis: usize) -> Option<&Self::Lower> { self.0.get(axis) }
    fn lower_mut(&mut self, axis: usize) -> Option<&mut Self::Lower> { self.0.get_mut(axis) }
}

unsafe impl<T, const A: usize> Axies<0> for Array1<T, A> {
    type Axis = Array0<T>;
}

unsafe impl<T, const A: usize, const B: usize> Axies<0> for Array2<T, A, B> {
    type Axis = Array1<T, A>;
}

unsafe impl<T, const A: usize, const B: usize> Axies<1> for Array2<T, A, B> {
    type Axis = Array1<T, B>;
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Axies<0> for Array3<T, A, B, C> {
    type Axis = Array2<T, B, C>;
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Axies<1> for Array3<T, A, B, C> {
    type Axis = Array2<T, A, C>;
}

unsafe impl<T, const A: usize, const B: usize, const C: usize> Axies<2> for Array3<T, A, B, C> {
    type Axis = Array2<T, A, B>;
}
