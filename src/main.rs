extern crate vector_bin_packing;
use vector_bin_packing::{Problem, Solution};
use vector_bin_packing::algo;

fn main() {
    let problem = Problem {
        items: vec![5, 3, 2],
        num_bins: 2,
    };

    let solution = algo::trivial(&problem);
    println!("{:?}", solution);
}
