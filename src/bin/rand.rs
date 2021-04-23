use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng; // Cargo.tomlにfeatureの記述が必要
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::time::Instant;

fn main() {
    // thread_rngを使い回すより SmallRng::from_entropy() を使い回すほうが20倍速い！！
    let start = Instant::now();
    let mut x: usize = 0;
    let mut sr = SmallRng::from_entropy();
    for _ in 0..10usize.pow(7) {
        x = sr.gen();
    }
    let end = start.elapsed();
    println!("{},time: {}, sr.gen()", x, end.as_millis());

    let mut sr = SmallRng::from_entropy();

    // 指定範囲の整数から一つ選ぶ
    let uniform = Uniform::new(10, 20);
    let x = uniform.sample(&mut sr);
    println!("{}", x);

    // Vecからn個選ぶ (一度選ばれたものは選ばれない。順番もランダムになる)
    // use rand::seq::IteratorRandom
    let v = vec![1, 2, 3, 4, 5];
    let x = v.iter().choose_multiple(&mut sr, 4);
    println!("{:?}", x);

    // 指定範囲の実数から一つ選ぶ(n個ほしければこれをn回やる)
    let uniform = Uniform::new(10.0, 20.0);
    let x = uniform.sample(&mut sr);
    println!("{}", x);

    // Vecをシャッフルする
    // use rand::seq::SliceRandom
    let mut v = vec![1, 2, 3, 4, 5];
    v.shuffle(&mut sr);
    println!("{:?}", v);
}
