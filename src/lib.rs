extern crate num;
extern crate rand;

pub mod algo;
pub mod item;

use item::Item;

#[derive(Debug)]
pub struct Problem<T: Item> {
    pub items: Vec<T>,
    pub num_bins: usize,
    pub metric: Metric,
}

#[derive(Debug)]
pub struct Solution<T: Item> {
    pub bins: Vec<Bin<T>>,
}

#[derive(Debug, Clone)]
pub struct Bin<T: Item> {
    items: Vec<T>,
    pub total: T,
}
impl<T: Item> Bin<T> {
    pub fn new() -> Bin<T> {
        Bin {
            items: Vec::new(),
            total: T::zero(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
        self.total += item;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Metric {
    L1,
    L2,
    LInf,
}

impl<T: Item> Solution<T> {
    pub fn score(&self, metric: Metric) -> i32 {
        self.bins
            .iter()
            .map(|bin| bin.total.score(metric))
            .max()
            .expect("Metric::for_solution called on solution with 0 bins")
    }
}
