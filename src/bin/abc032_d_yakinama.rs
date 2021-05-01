use proconio::input;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng; // Cargo.tomlにfeatureの記述が必要
use rand::SeedableRng;
use std::collections::HashSet;
use std::time::Instant;

// 最初から焼きなまし法(貪欲法で得た解を焼きなまし法で改善するのではなく！)
fn main() {
    input! {
        n:usize,
        limit:usize,
        vw:[(usize,usize);n],
    }
    let start_time = Instant::now();

    let mut selected = HashSet::new();
    let mut remains = HashSet::new();
    for i in 0..n {
        remains.insert(i);
    }
    let mut score = 0usize;
    let mut cost = 0usize;
    let mut temperature = 1000000000.0;
    loop {
        let (selected2, remains2, score2, cost2) =
            next_state(&selected, &remains, score, cost, &vw);
        if score < score2 + temperature as usize && cost2 <= limit {
            selected = selected2;
            remains = remains2;
            score = score2;
            cost = cost2;
        }
        let elapsed_duration = start_time.elapsed();
        if elapsed_duration.as_millis() > 1900 {
            break;
        }
        temperature *= 0.995;
    }
    println!("{}", score);
}

// 近傍は、一個追加・一個捨てる・一個入れ替え のどれか
// スコアは差分だけ計算する。
// 今の状態とスコアとコストを渡すと、新しい状態とスコアとコストを一個返す(limitオーバーの場合は0点)
fn next_state(
    selected: &HashSet<usize>,
    remains: &HashSet<usize>,
    score: usize,
    cost: usize,
    vw: &Vec<(usize, usize)>,
) -> (HashSet<usize>, HashSet<usize>, usize, usize) {
    // SeedableRngも必要！！
    let mut sr = SmallRng::from_entropy();
    // 指定範囲の整数から一つ選ぶ
    let zero_three = Uniform::new(0, 3);
    let x = zero_three.sample(&mut sr);
    let mut selected = selected.clone();
    let mut remains = remains.clone();
    let mut score = score;
    let mut cost = cost;
    if (x == 0 || x == 2) && !remains.is_empty() {
        // 一つ追加
        let remains2: Vec<usize> = remains.iter().map(|x| *x).collect();
        let dist = Uniform::new(0, remains2.len());
        let y = dist.sample(&mut sr);
        let index = remains2[y];
        selected.insert(index);
        remains.remove(&index);
        score += vw[index].0;
        cost += vw[index].1;
    } else if (x == 1 || x == 2) && !selected.is_empty() {
        // 一つ捨てる
        let selected2: Vec<usize> = selected.iter().map(|x| *x).collect();
        let dist = Uniform::new(0, selected2.len());
        let y = dist.sample(&mut sr);
        let index = selected2[y];
        selected.remove(&index);
        remains.insert(index);
        score -= vw[index].0;
        cost -= vw[index].1;
    }
    (selected, remains, score, cost)
}
