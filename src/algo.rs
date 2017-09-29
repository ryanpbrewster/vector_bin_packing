use {Bin, Problem, Solution};
use item::Item;

pub fn trivial<T: Item>(problem: &Problem<T>) -> Solution<T> {
    let mut bins = vec![Bin::new(); problem.num_bins];
    for &item in problem.items.iter() {
        bins[0].push(item)
    }
    Solution { bins }
}

pub fn greedy<T: Item>(problem: &Problem<T>) -> Solution<T> {
    let mut bins = vec![Bin::new(); problem.num_bins];
    for &item in &problem.items {
        let best_bin = bins.iter()
            .enumerate()
            .min_by_key(|&(_, bin)| {
                let new_total: T = bin.total + item;
                new_total.score(problem.metric) as i32
            })
            .map(|(idx, _)| idx)
            .expect("problem.num_bins must be > 0");
        bins[best_bin].push(item);
    }
    Solution { bins }
}

#[cfg(test)]
mod test {
    use ::*;

    #[test]
    fn smoke_test() {
        let problem = Problem {
            items: vec![5, 3, 2],
            num_bins: 2,
            metric: Metric::LInf,
        };

        let trivial_solution = algo::trivial(&problem);
        assert_eq!(trivial_solution.score(problem.metric), 10);

        let greedy_solution = algo::greedy(&problem);
        assert_eq!(greedy_solution.score(problem.metric), 5);
    }
}
