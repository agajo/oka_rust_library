use proconio::input;
// ABC144 E に提出してACしたものです。

// 二分探索で特定の整数を見つけます。欲しい整数が一意に定まるように定義しておくこと！
// LLONG_MAXからはじめて範囲を狭めていくので、オーバーフロー注意！

// TODO: 引数は問題によって違うよ
fn bs(n: usize, k: isize, a: &Vec<isize>, f: &Vec<isize>, is_minimizing: bool) -> isize {
    // ここから答えを二分探索
    // conditionを満たす最小値が欲しい場合:
    //   left < 目的の整数 <= right を保ったまま範囲を狭め、最後rightを返す。
    // conditionを満たす最大値が欲しい場合:
    //   left <= 目的の整数 < right を保ったまま範囲を狭め、最後leftを返す。

    let condition = |mid| {
        // TODO: 値がmidの時条件を満たしているかどうか書く
        let mut k_needs = 0;
        for i in 0..n {
            if a[i] * f[i] > mid {
                k_needs += (a[i] * f[i] - mid + f[i] - 1) / f[i];
            }
        }
        k_needs <= k
    };

    // leftの初期値注意。
    // left = 0だと、rightが0になれないので、最小値を求めていて答えに0がありうる場合はleftの初期値は-1。
    // 答えに負の数もありうる時はleftの初期値はLLONG_MIN。
    // overflow対策で、midで割り算する前にmid!=0を確認してね。
    let mut left: isize = -1;
    let mut right: isize = std::isize::MAX;

    while left + 1 != right {
        let mid = (left + right) / 2;
        let ans_is_in_left_range: bool = if is_minimizing {
            condition(mid)
        } else {
            !condition(mid)
        };
        if ans_is_in_left_range {
            right = mid;
        } else {
            left = mid;
        }
    }
    return if is_minimizing { right } else { left };
}

fn main() {
    // TODO: 変数・入出力を正しく
    input! {n:usize,k:isize,mut a:[isize;n],mut f:[isize;n]}
    a.sort();
    f.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let ans = bs(n, k, &a, &f, true);
    println!("{}", ans);
}
