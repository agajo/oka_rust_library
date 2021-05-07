use proconio::input;
// ABC144 E に提出してACしたものです。

// 二分探索で特定の整数を見つけます。欲しい整数が一意に定まるように定義しておくこと！
// LLONG_MAXからはじめて範囲を狭めていくので、オーバーフロー注意！

// TODO: 引数は問題によって違うよ
fn bs(n: usize, k: isize, a: &Vec<isize>, f: &Vec<isize>) -> isize {
    // ここから答えを二分探索
    // left < 目的の整数 <= right を常に保ったまま、範囲を狭めます。
    // ここの等号不等号を逆にしなければならないことがある！
    // その場合、出力するものをleftに変えること。コードを変えるのはそこだけだけど。

    // leftの初期値注意。
    // left = 0だと、rightが0になれないので、答えに0がありうる場合はleftの初期値は-1。
    // 答えに負の数もありうる時はleftの初期値はLLONG_MIN。
    // overflow対策でmidで割り算する前にmid!=0を確認してね。
    let mut left: isize = -1;
    let mut right: isize = std::isize::MAX;

    while left + 1 != right {
        let mid = (left + right) / 2;

        // l < 目的の整数 <= mid (ansIsInLeftRange)なのか
        // mid < 目的の整数 <= right (!ansIsInLeftRange)なのかを判定します。
        // TODO: その判定方法は問題によって違う
        // ここの判定によって、等号不等号がどっちになるのか決まる！ここを先に決めてから判断すること！

        let mut k_needs = 0;
        for i in 0..n {
            if a[i] * f[i] > mid {
                k_needs += (a[i] * f[i] - mid + f[i] - 1) / f[i];
            }
        }
        let ans_is_in_left_range: bool = k_needs <= k;
        if ans_is_in_left_range {
            right = mid;
        } else {
            left = mid;
        }
    }
    // TODO: 等号不等号を逆にした場合は、leftを出力
    return right;
    // return left;
}

fn main() {
    // TODO: 変数・入出力を正しく
    input! {n:usize,k:isize,mut a:[isize;n],mut f:[isize;n]}
    a.sort();
    f.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let ans = bs(n, k, &a, &f);
    println!("{}", ans);
}
