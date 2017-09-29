#[derive(Debug)]
pub struct Problem<T: Clone> {
    pub items: Vec<T>,
    pub num_bins: usize,
}

#[derive(Debug)]
pub struct Solution<T: Clone> {
    pub bins: Vec<Vec<T>>,
}

pub mod algo;
