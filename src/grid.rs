use std::ops::Index;

pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

impl Resolution {
    pub fn area(&self) -> usize { self.width * self.height }
    pub fn slice(&self) -> [usize; 2] { [self.width, self.height] }
}

pub struct Grid<T> {
    resolution: Resolution,
    values: Vec<T>,
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize { return self.resolution.width }
    pub fn height(&self) -> usize { return self.resolution.height }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index<'a>(&'a self, (x, y): (usize, usize)) -> &'a T {
        &self.values[x + y * self.resolution.width ]
    }
}

impl<T: Clone> Grid<T> {
    pub fn from_buffer(resolution: Resolution, buffer: &Vec<T>) -> Grid<T> {
        Grid {
            resolution,
            values: buffer.to_owned(),
        }
    }
    pub fn fill(resolution: Resolution, value: T) -> Grid<T> {
        let area = resolution.area();
        Grid {
            resolution,
            values: vec![value; area]
        }
    }
}