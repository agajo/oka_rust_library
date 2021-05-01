use proconio::input;
use std::collections::BinaryHeap;

// ビームサーチ
fn main() {
    input! {
        n:usize,
        limit:usize,
        mut vw:[(usize,usize);n],
    }
    vw.sort_by(|a, b| b.cmp(a));
    let mut best_score = 0;
    let initial_state = (0, 0, vec![]);
    let mut states = BinaryHeap::new();
    states.push(initial_state);
    while !states.is_empty() {
        states = next_beam_states(20, n, limit, states, &vw);
        if !states.is_empty() {
            best_score = std::cmp::max(best_score, states.peek().unwrap().0);
        }
    }
    println!("{}", best_score);
}

fn next_beam_states(
    beam_width: usize,
    n: usize,
    limit: usize,
    mut states: BinaryHeap<(usize, usize, Vec<usize>)>,
    vw: &Vec<(usize, usize)>,
) -> BinaryHeap<(usize, usize, Vec<usize>)> {
    let mut result = BinaryHeap::new();
    for _ in 0..std::cmp::min(beam_width, states.len()) {
        let (score, cost, state) = states.pop().unwrap();
        let last = match state.last() {
            None => 0,
            Some(x) => *x + 1,
        };
        for i in last..n {
            let new_score = score + vw[i].0;
            let new_cost = cost + vw[i].1;
            if new_cost <= limit {
                let mut new_state = state.clone();
                new_state.push(i);
                result.push((new_score, new_cost, new_state));
            }
        }
    }
    result
}
