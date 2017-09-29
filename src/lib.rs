use std::cmp;

pub trait Item: Clone {}

mod items {
    use Item;
    impl Item for i32 {}
}

#[derive(Debug)]
pub struct Problem<T: Item> {
    pub items: Vec<T>,
    pub num_bins: usize,
}

#[derive(Debug)]
pub struct Solution<T: Item> {
    pub bins: Vec<Vec<T>>,
}

pub trait Metric {
    fn for_bin<T: Item>(bin: &[T]) -> f64;

    fn for_solution<T: Item>(solution: &Solution<T>) -> f64 {
        solution
            .bins
            .iter()
            .map(|bin| Self::for_bin(bin))
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .expect("Metric::for_solution called on solution with 0 bins")
    }
}

pub mod metrics {
    use Metric;
    pub struct L0;
    impl Metric for L0 {
        fn for_bin<T>(bin: &[T]) -> f64 {
            0.0
        }
    }
}

pub mod algo;
