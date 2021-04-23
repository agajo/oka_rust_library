use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng; // Cargo.tomlにfeatureの記述が必要
use rand::SeedableRng;

fn f(x: f64) -> f64 {
    2.0 * x.powf(4.0) + x.powf(3.0) - 5.0 * x.powf(2.0) - -0.5 * x + 3.0
}

// 1変数関数を相手に超テキトーに作った焼きなまし法です。そのうちなんとかしましょう。2021-04-23
fn yakinama(now: f64, now_value: f64, t: f64) -> (f64, f64) {
    let mut sr = SmallRng::from_entropy();
    let uniform = Uniform::new(-2.0, 2.0);
    let d = uniform.sample(&mut sr);
    let next = now + d;
    let next_value = f(next);
    // f64は NaN とか Inf とかいう意味不明な値を持てるので、その時は採用しない
    if next_value.is_nan() || next_value.is_infinite() {
        return (now, now_value);
    }
    // 改善してたら必ず採用
    if next_value < now_value {
        return (next, next_value);
    }
    // 改善してなくても、tまでは悪化を許容
    if (next_value - now_value) < t {
        return (next, next_value);
    }
    (now, now_value)
}

fn main() {
    let mut sr = SmallRng::from_entropy();
    let unif = Uniform::new(-100.0, 100.0);
    let mut now = unif.sample(&mut sr);
    let mut now_value = f(now);
    let mut t = 100.0;
    for _ in 0..10usize.pow(6) {
        let nows = yakinama(now, now_value, t);
        now = nows.0;
        now_value = nows.1;
        // 毎ステップ、tを定数倍小さくします。指数関数的に小さくなる。
        t *= 0.99;
    }
    println!("{}:{}", now, now_value);
}
