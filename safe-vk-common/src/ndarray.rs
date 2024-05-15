use super::keyboard::Button;

pub struct Shape(Vec<usize>);

impl From<(usize, usize)> for Shape {
    fn from(d12: (usize, usize)) -> Self {
        Self(vec![d12.0, d12.1])
    }
}

impl Shape {
    pub fn dims(&self) -> &[usize] {
        &self.0
    }
}

pub trait NdArray<T> {
    fn shape(&self) -> Shape;

    fn slice(&self) -> Vec<Vec<&Button<T>>>;
}

impl<T: serde::Serialize, const N: usize, const M: usize> NdArray<T> for &[[Button<T>; N]; M] {
    fn shape(&self) -> Shape {
        Shape::from((M, N))
    }

    fn slice(&self) -> Vec<Vec<&Button<T>>> {
        self.iter().map(|row| row.iter().collect()).collect()
    }
}

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.dims())
    }
}
