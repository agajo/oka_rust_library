use proconio::{input, marker::Usize1};

// メモ化再帰の基本的なやり方。これに提出してACするものです。
// https://atcoder.jp/contests/dp/tasks/dp_g

fn longest(from: usize, targets: &Vec<Vec<usize>>, memo: &mut Vec<Option<usize>>) -> usize {
    match memo[from] {
        Some(l) => l,
        None => {
            if targets[from].is_empty() {
                memo[from] = Some(0);
                0
            } else {
                let ans = targets[from]
                    .iter()
                    .map(|&t| longest(t, targets, memo) + 1)
                    .max()
                    .unwrap();
                memo[from] = Some(ans);
                ans
            }
        }
    }
}

fn main() {
    input!(n: usize, m: usize, xy: [(Usize1, Usize1); m]);
    let mut targets: Vec<Vec<usize>> = vec![vec![]; n];
    for (x, y) in xy {
        targets[x].push(y);
    }
    let targets = targets;

    let mut memo: Vec<Option<usize>> = vec![None; n];

    let result = (0..n)
        .map(|i| longest(i, &targets, &mut memo))
        .max()
        .unwrap();
    println!("{}", result);
}
