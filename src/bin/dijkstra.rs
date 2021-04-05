use num::integer::div_ceil;
use proconio::{input, marker::Usize1};
use std::{cmp::Reverse, collections::BinaryHeap};

// ABC192 E を解きます。

fn dijkstra(
    targets: &Vec<Vec<(usize, usize, usize)>>,
    n: usize,
    start: usize,
) -> Vec<Option<usize>> {
    // (cost, city) を持ちます
    let mut min_costs = vec![None; n];
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), start));
    while let Some((cost, city)) = que.pop() {
        if let None = min_costs[city] {
            min_costs[city] = Some(cost.0);
            for (target, t, k) in &targets[city] {
                que.push((Reverse(k * div_ceil(cost.0, *k) + t), *target))
            }
        }
    }
    min_costs
}

fn main() {
    input!(
        n: usize,
        m: usize,
        x: Usize1,
        y: Usize1,
        abtk: [(Usize1, Usize1, usize, usize); m]
    );
    let mut targets = vec![vec![]; n];
    for (a, b, t, k) in abtk {
        targets[a].push((b, t, k));
        targets[b].push((a, t, k));
    }
    let min_costs = dijkstra(&targets, n, x);
    if let Some(c) = min_costs[y] {
        println!("{}", c);
    } else {
        println!("-1");
    }
}
