use proconio::input;
use std::time::Instant;

// chokudaiサーチ
fn main() {
    input! {
        n:usize,
        limit:usize,
        mut vw:[(usize,usize);n],
    }
    let start_time = Instant::now();
    vw.sort_by(|a, b| b.cmp(a));
    let mut best_score = 0;
    let initial_state = (0, 0, vec![]);
    let mut states_vec: Vec<Vec<(usize, usize, Vec<usize>)>> = vec![Vec::new(); 201];
    states_vec.get_mut(0).unwrap().push(initial_state);
    // 頭から順に、Vecたちを見ていきながら、一番上を取り出して、
    // そこから遷移可能な状態を重複がないように次のVecに突っ込むことを繰り返す。
    // 頭から見ていく時、しばらく空の可能性がある。最後の方も空の可能性がある。
    'outer: loop {
        // Vecたちが全部空(この場合全探索完了)か、時間切れで終了。
        let mut empty_so_far = true;
        for i in 0..states_vec.len() {
            let elapsed_duration = start_time.elapsed();
            if elapsed_duration.as_millis() > 1900 {
                break 'outer;
            }
            if empty_so_far == false && states_vec[i].is_empty() {
                continue 'outer;
            }
            if !states_vec[i].is_empty() {
                empty_so_far = false;
                let top_state = states_vec[i].remove(0);
                best_score = std::cmp::max(best_score, top_state.0);
                if i < states_vec.len() - 1 {
                    let next_states = next_beam_states(n, limit, top_state, &vw);
                    for st in next_states {
                        states_vec[i + 1].push(st);
                    }
                    states_vec[i + 1].sort_by(|a, b| b.cmp(a));
                    // 上位500個は残し、あとは捨てます。ビーム幅は最大でも500ということ。
                    // これをやらないと持つ情報量が膨大になる。
                    states_vec[i + 1].truncate(500);
                }
            }
        }
        if empty_so_far {
            break 'outer;
        }
    }
    println!("{}", best_score);
}

fn next_beam_states(
    n: usize,
    limit: usize,
    state: (usize, usize, Vec<usize>),
    vw: &Vec<(usize, usize)>,
) -> Vec<(usize, usize, Vec<usize>)> {
    let mut result = Vec::new();
    let (score, cost, state) = state;
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
    result
}
