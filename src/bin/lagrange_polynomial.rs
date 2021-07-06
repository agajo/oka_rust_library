use num::{One, Zero};
use std::iter::Product;
use std::ops::{Div, Mul, Sub};

fn main() {
    let f = lag_poly_a(vec![(0.0, 0.0), (1.0, 1.0), (-1.0, 1.0)]);
    println!("{}", f(-3.0));
    let mut finv = vec![1.0; 10];
    for i in 1..10 {
        finv[i] = finv[i - 1] / (i as f64);
    }
    let f = lag_poly_b(vec![0.0, 1.0, 8.0, 27.0], |x| x as f64, finv);
    println!("{}", f(-6.0));
}

// pointsが連番でなくてもいいバージョン。遅い。
// 通る点をn個受け取って、n-1次多項式と等しい関数を返す
// T(得たい関数のドメインの方。グラフのx座標の方)について。
// Tは引き算を含むので、usize型を渡してはダメ！！
// Tは割り算を含むので、isize型を渡してもダメ！！
// Tに渡せるのはf64か、剰余類で正確に割り算できるMnに限ると思われる。
// 普通はU=Tになると思われる。
fn lag_poly_a<T, U>(points: Vec<(T, U)>) -> impl Fn(T) -> U
where
    T: Copy + Product + Sub<Output = T> + Div<Output = U>,
    U: Copy + Zero + Mul<Output = U>,
{
    let n = points.len();
    move |x: T| {
        let mut ans: U = U::zero();
        for k in 0..n {
            let bk = points[k].1;
            let ak = points[k].0;
            let numerator: T = points
                .iter()
                .enumerate()
                .filter(|t| t.0 != k)
                .map(|t| x - t.1 .0)
                .product();
            let denominator: T = points
                .iter()
                .enumerate()
                .filter(|t| t.0 != k)
                .map(|t| ak - t.1 .0)
                .product();
            ans = ans + bk * (numerator / denominator);
        }
        ans
    }
}

// pointsが0から始まる連番のバージョン。速い。
// 通る点をn個受け取って、n-1次多項式と等しい関数を返す
// T(得たい関数のドメインの方。グラフのx座標の方)について。
// Tは引き算を含むので、usize型を渡してはダメ！！
// Tは割り算を含むので、isize型を渡してもダメ！！
// Tに渡せるのはf64か、剰余類で正確に割り算できるMnに限ると思われる。
// usize_to_tは (|x| x as f64) か (|x| Mn(x)) だろう。
// finvは前計算して用意してください。階乗の逆数。
fn lag_poly_b<T, F>(points: Vec<T>, usize_to_t: F, finv: Vec<T>) -> impl Fn(T) -> T
where
    T: Copy + One + Sub<Output = T> + Zero + Mul<Output = T>,
    F: Fn(usize) -> T,
{
    let n = points.len();
    move |x: T| {
        let mut numerators_from_left = vec![T::one(); n];
        let mut numerators_from_right = vec![T::one(); n];
        for i in 0..(n - 1) {
            numerators_from_left[i + 1] = numerators_from_left[i] * (x - usize_to_t(i));
        }
        for i in 0..(n - 1) {
            numerators_from_right[i + 1] = numerators_from_right[i] * (x - usize_to_t(n - 1 - i));
        }
        let mut ans: T = T::zero();
        for k in 0..n {
            let bk = points[k];
            let adding = bk
                * numerators_from_left[k]
                * numerators_from_right[n - 1 - k]
                * finv[k]
                * finv[n - 1 - k];
            if (n - 1 - k) % 2 == 0 {
                ans = ans + adding;
            } else {
                ans = ans - adding;
            }
        }
        ans
    }
}
