use proconio::input;

// このコードは、ABC179のE問題に提出したもの
// https://atcoder.jp/contests/abc179/tasks/abc179_e

fn solve_loop(
    max_place: usize,
    start_at: usize,
    score_of_start: usize,
    max_step: usize,
    m: usize,
) -> usize {
    // (step, score)を持ちます
    let mut status_at: Vec<Option<(usize, usize)>> = vec![None; max_place];

    // スタート地点への移動
    let mut now_at = start_at;
    let mut step = 0usize;
    let mut score = score_of_start;
    let mut loop_found = false;

    // ここでは、歩き過ぎないようチェック
    // while文に入ると移動するので、まだ移動できる場合にだけtrue
    while step < max_step {
        if status_at[now_at] != None && loop_found == false {
            // この瞬間、「ループのスタート地点に移動済み」で記録上書き前
            let (step_here, score_here) = status_at[now_at].unwrap();
            let period_step = step - step_here;
            let period_score = score - score_here;
            // あと一歩も動かずに終わることがないようにする。最後1ループちょうど残ってても自力で歩く。
            // 1引いてから周期で割ればいいね
            let remaining_loops = (max_step - step - 1) / period_step;
            step += remaining_loops * period_step;
            score += remaining_loops * period_score;
            loop_found = true;
            // あとは歩数の限界まで普通に歩く。
        } else {
            // 普通に一歩歩く
            // 記録(didVisitとstepAtとscoreAtを更新)
            status_at[now_at] = Some((step, score));
            // 移動(現在地を変え、歩数とスコアを移動後のものに更新)
            // TODO: スコアも移動先も問題によって違う！書き換えて！
            now_at = (now_at * now_at).rem_euclid(m);
            score += now_at;
            step += 1;
        }
    }
    score
}

fn main() {
    input!(n: usize, x: usize, m: usize);

    // TODO: 到達しうる場所のインデックスの最大値+1(通常は到達しうる場所の数)
    let max_place = m;
    // TODO: スタート地点
    let start_at = x;
    // TODO: スタート地点に移動後のスコア
    let score_of_start = x;
    // TODO: 最大移動可能回数
    let max_step = n - 1;

    let score = solve_loop(max_place, start_at, score_of_start, max_step, m);
    // TODO: 出力するものはこれでよい？
    println!("{}", score);
}
