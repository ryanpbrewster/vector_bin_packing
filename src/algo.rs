use {Item, Problem, Solution};

pub fn trivial<T: Item>(problem: &Problem<T>) -> Solution<T> {
    let mut bins = vec![Vec::new(); problem.num_bins];
    bins[0].clone_from(&problem.items);
    Solution { bins }
}

pub fn greedy<T: Item>(problem: &Problem<T>) -> Solution<T> {
    let mut bins = vec![Vec::new(); problem.num_bins];
    bins[0].clone_from(&problem.items);
    Solution { bins }
}
