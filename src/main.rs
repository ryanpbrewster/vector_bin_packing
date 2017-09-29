extern crate vector_bin_packing;
use vector_bin_packing::{Problem, Metric};
use vector_bin_packing::algo;

fn main() {
    let problem = Problem {
        items: vec![5, 3, 2],
        num_bins: 2,
        metric: Metric::L0,
    };

    let trivial_solution = algo::trivial(&problem);
    println!(
        "trivial ({}): {:?}",
        trivial_solution.score(problem.metric),
        trivial_solution
    );

    let greedy_solution = algo::greedy(&problem);
    println!(
        "greedy ({}): {:?}",
        greedy_solution.score(problem.metric),
        greedy_solution
    );
}
