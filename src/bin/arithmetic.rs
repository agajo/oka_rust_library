use num::integer::{binomial, div_ceil, div_floor, gcd, mod_floor, Roots};

fn main() {
    // 整数平方根 ans^2 <= target < (ans+1)^2
    println!("{}, {}, {}", 24.sqrt(), 25.sqrt(), 26.sqrt());

    // 最大公約数
    println!("{}", gcd(24, 36));

    // 必ず非負のmod
    println!("{} {}", (-34i64).rem_euclid(7), mod_floor(-34, 7));

    // 不足注目の割り算の商
    println!("{}", div_ceil(17, 6));

    // 不足(-m < ans <= 0)
    println!("{}", 17 - 6 * div_ceil(17, 6));

    // あまりを捨てる、不足を埋める
    println!("{}, {}", 10 * div_floor(17, 10), 10 * div_ceil(17, 10));

    // nCr
    println!("{}", binomial(5, 3));

    // popcount
    println!("{}", 7u64.count_ones());
}
