extern crate vector_bin_packing;
use vector_bin_packing::{Problem, Metric};
use vector_bin_packing::{item, algo};

extern crate rand;
use rand::{SeedableRng, XorShiftRng, Rng};

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "dist",
            about = "Generate a distribution of greed Vector Bin Packing solutions.")]
struct Opt {
    #[structopt(long = "seed", help = "Seed the PRNG", default_value = "default")]
    seed: String,

    #[structopt(long = "num-items", help = "Number of items", default_value = "20")]
    num_items: usize,

    #[structopt(long = "num-bins", help = "Number of bins", default_value = "3")]
    num_bins: usize,

    #[structopt(long = "num-trials", help = "Number of trials", default_value = "3")]
    num_trials: usize,
}

fn main() {
    let opt = Opt::from_args();

    let mut prng = XorShiftRng::from_seed([42, 42, 42, 42]);
    let items = prng.gen_iter::<item::Vnode>().take(opt.num_items).collect();

    let mut problem = Problem {
        items,
        num_bins: opt.num_bins,
        metric: Metric::LInf,
    };

    for _ in 0..opt.num_trials {
        prng.shuffle(&mut problem.items);
        let greedy_solution = algo::greedy(&problem);
        println!("{}", greedy_solution.score(problem.metric));
    }
}
