use std::fmt::Debug;
use std::ops::{Add, AddAssign};
use num::Zero;
use rand::{Rand, Rng};

use Metric;

pub trait Item: Clone + Copy + Zero + Add + AddAssign + Debug {
    fn score(&self, metric: Metric) -> i32;
}

impl Item for i32 {
    fn score(&self, metric: Metric) -> i32 {
        match metric {
            Metric::L1 => *self,
            Metric::L2 => *self,
            Metric::LInf => *self,
        }
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Vnode {
    cpu: i32,
    heap_size: i32,
}

impl Add for Vnode {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vnode {
            cpu: self.cpu + rhs.cpu,
            heap_size: self.heap_size + rhs.heap_size,
        }
    }
}

impl AddAssign<Vnode> for Vnode {
    fn add_assign(&mut self, rhs: Vnode) {
        self.cpu += rhs.cpu;
        self.heap_size += rhs.heap_size;
    }
}

impl Zero for Vnode {
    fn zero() -> Self {
        Vnode {
            cpu: 0,
            heap_size: 0,
        }
    }

    fn is_zero(&self) -> bool {
        self.cpu.is_zero() && self.heap_size.is_zero()
    }
}

use std::cmp;
impl Item for Vnode {
    fn score(&self, metric: Metric) -> i32 {
        match metric {
            Metric::L1 => self.cpu + self.heap_size,
            Metric::L2 => self.cpu * self.cpu + self.heap_size * self.heap_size,
            Metric::LInf => cmp::max(self.cpu, self.heap_size),
        }
    }
}

impl Rand for Vnode {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        Vnode {
            cpu: rng.gen_range(0, 100),
            heap_size: rng.gen_range(0, 100),
        }
    }
}
