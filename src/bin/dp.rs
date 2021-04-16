use proconio::input;
use std::cmp::min;

// Levenshtein distanceを例に、DPを実装しておきます。
// 正しさの確認は ABC185 E で。

// 今回は、dp[i][j]は「sをi文字消し、tを左にj文字追加した状態までのコスト」とする。
fn fill_dp_table(dp: &mut Vec<Vec<usize>>, s: &Vec<usize>, t: &Vec<usize>) {
    let n = dp.len();
    let m = dp[0].len();
    // TODO: 初期値を埋めます。
    for i in 0..n {
        dp[i][0] = i;
    }
    for j in 0..m {
        dp[0][j] = j;
    }

    // 残りをすべて埋めていきます。
    // TODO: 埋める範囲はこれでいい？
    for i in 1..n {
        for j in 1..m {
            // TODO: 漸化式に従って、埋めていく処理。
            // 足す場合も比べる場合も、ここは縦にキレイに並ぶように書くと良い。
            let mut ans = std::usize::MAX;
            ans = min(ans, dp[i - 1][j] + 1);
            ans = min(ans, dp[i][j - 1] + 1);
            ans = min(ans, dp[i - 1][j - 1] + 1);
            if s[i - 1] == t[j - 1] {
                ans = min(ans, dp[i - 1][j - 1]);
            }
            dp[i][j] = ans;
        }
    }
}

fn main() {
    // TODO: 入力を正しく受け取る
    input! {
        source_size:usize,
        target_size:usize,
        source:[usize;source_size],
        target:[usize;target_size],
    }

    // TODO: DP Tableの大きさを指定
    let mut dp = vec![vec![0; target_size + 1]; source_size + 1];

    // TODO: DP表を埋めます。引数合わせて。
    fill_dp_table(&mut dp, &source, &target);

    // TODO: 出力はこれでいい？
    println!("{}", dp[source_size][target_size]);
}
