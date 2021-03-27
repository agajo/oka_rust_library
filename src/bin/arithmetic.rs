use num::integer::binomial;
use num::integer::div_ceil;
use num::integer::gcd;
use num::integer::mod_floor;
use num::integer::Roots;

fn main() {
    // 整数平方根 ans^2 <= target < (ans+1)^2
    println!("{}, {}, {}", 24.sqrt(), 25.sqrt(), 26.sqrt());

    // 最大公約数
    println!("{}", gcd(24, 36));

    // 必ず非負のmod
    println!("{} {}", (-34i64).rem_euclid(7), mod_floor(-34, 7));

    // 不足注目の割り算
    println!("{}", div_ceil(17, 6));

    // nCr
    println!("{}", binomial(5, 3));
}
