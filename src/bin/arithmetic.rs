use num::integer::{binomial, div_ceil, gcd, mod_floor, Integer, Roots};

fn main() {
    // 整数平方根 √target-1 < ans <= √target < ans+1
    println!("{}, {}, {}", 24.sqrt(), 25.sqrt(), 26.sqrt());

    // 最大公約数
    println!("{}", gcd(24, 36));

    // 必ず非負のmod
    println!("{} {}", (-34i64).rem_euclid(7), mod_floor(-34, 7));

    // 不足注目の割り算の商
    println!("{}", div_ceil(17, 6));

    // 不足(-m < ans <= 0)
    println!(
        "{},{}",
        17 - 6 * div_ceil(17, 6),
        17 - 17.next_multiple_of(&6)
    );

    // あまりを捨てる、不足を埋める
    // num::integer::Integer をuseする
    println!("{}, {}", 17.prev_multiple_of(&10), 17.next_multiple_of(&10));

    // nCr
    println!("{}", binomial(5, 3));

    // popcount
    println!("{}", 7u64.count_ones());

    // Bezoutの等式
    // num::integer::Integer をuseする
    let ans = isize::extended_gcd(&6, &10);
    println!("6*{} + 10*{} = {}", ans.x, ans.y, ans.gcd);
}
