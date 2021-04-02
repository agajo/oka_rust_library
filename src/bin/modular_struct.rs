use num::{pow, One};
use std::fmt::{Display, Formatter, Result};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

fn main() {
    let x = Mn(4);
    let y = Mn(10);
    println!(
        "x:{}, y:{}, x+y:{}, x*y:{}, x-y:{},y-x:{}",
        x,
        y,
        x + y,
        x * y,
        x - y,
        y - x,
    );
    println!("x³:{}, x¹⁰⁰⁰⁰⁰⁰⁰⁰:{}", x.pow(3), x.pow(100000000));
    println!("3≡10:{}", Mn(3) == Mn(10));
    println!("1+2+...+10:{}", (1..=10).map(|v| Mn(v)).sum::<Mn>());

    // ここからmodular_combinationの使い方
    // TODO: max_nを変更
    let max_n = 10usize.pow(6);
    let mut fac = vec![Mn(0); max_n];
    let mut finv = vec![Mn(0); max_n];
    init_modular_tables(&mut fac, &mut finv);
    let a = modular_combination(6, 4, &fac, &finv);
    let b = modular_permutation(6, 4, &fac, &finv);
    let c = modular_combination(6, 0, &fac, &finv);

    println!("6C4:{}, 6P4:{}, 6C0:{}", a, b, c);
}

// --------------ここから、構造体Mn(Modular Number)の実装--------------

// TODO: CHANGE MODULAR HERE!!
const MOD: usize = 7;
// const MOD: usize = 998_244_353;
// const MOD: usize = 1_000_000_007;

// 中身がMOD未満であることは保証「「しません」」！！
// 表示時は保証します。
#[derive(Debug, Copy, Clone)]
struct Mn(usize);

impl One for Mn {
    fn one() -> Self {
        Self(1)
    }
}

impl Mn {
    // これを10^7回以上呼ぶ場合はメモ化しましょう！！
    fn pow(&self, exp: usize) -> Mn {
        pow(*self, exp)
    }
}

impl Display for Mn {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0.rem_euclid(MOD))
    }
}

impl PartialEq for Mn {
    fn eq(&self, other: &Self) -> bool {
        self.0.rem_euclid(MOD) == other.0.rem_euclid(MOD)
    }
}

impl Add for Mn {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0).rem_euclid(MOD))
    }
}

impl AddAssign for Mn {
    fn add_assign(&mut self, other: Self) {
        *self = Self((self.0 + other.0).rem_euclid(MOD));
    }
}

impl Mul for Mn {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self((self.0 * other.0).rem_euclid(MOD))
    }
}

impl MulAssign for Mn {
    fn mul_assign(&mut self, other: Self) {
        *self = Self((self.0 * other.0).rem_euclid(MOD));
    }
}

impl Neg for Mn {
    type Output = Self;
    fn neg(self) -> Self {
        Mn(MOD - self.0.rem_euclid(MOD))
    }
}

impl Sub for Mn {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(if self.0 >= other.0 {
            self.0 - other.0
        } else {
            MOD - (other.0 - self.0).rem_euclid(MOD)
        })
    }
}

impl SubAssign for Mn {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(if self.0 >= other.0 {
            self.0 - other.0
        } else {
            MOD - (other.0 - self.0).rem_euclid(MOD)
        });
    }
}

impl Sum for Mn {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self(0), |x, y| x + y)
    }
}

// --------------ここまで、構造体Mn(Modular Number)の実装--------------

// --------------ここから二項係数(nCk)の話-----------------------------
// 構造体Mnからコピペする必要あり！

// テーブルを作る前処理
// 計算量 O(n)
fn init_modular_tables(fac: &mut Vec<Mn>, finv: &mut Vec<Mn>) {
    let max_n = fac.len();
    let mut inv = vec![Mn(0); max_n];
    fac[0] = Mn(1);
    fac[1] = Mn(1);
    finv[0] = Mn(1);
    finv[1] = Mn(1);
    inv[1] = Mn(1);
    for i in 2..max_n {
        let mi = Mn(i);
        fac[i] = fac[i - 1] * mi;
        // 何でこれで順にinv[i]が出るんや2020-06-30
        inv[i] = Mn(MOD) - inv[MOD % i] * Mn(MOD / i);
        finv[i] = finv[i - 1] * inv[i];
    }
}

// 二項係数計算 nCk
// 先にinitModularTables()を呼んでおくこと！
fn modular_combination(n: usize, k: usize, fac: &Vec<Mn>, finv: &Vec<Mn>) -> Mn {
    if fac[5] != Mn(120) {
        println!("call initModularTables() first!!!!!!!!");
        return Mn(0);
    }
    if n < k {
        return Mn(0);
    }
    fac[n] * finv[k] * finv[n - k]
}

// 順列計算 nPk
// 先にinitModularTables()を呼んでおくこと！
fn modular_permutation(n: usize, k: usize, fac: &Vec<Mn>, finv: &Vec<Mn>) -> Mn {
    if fac[5] != Mn(120) {
        println!("call initModularTables() first!!!!!!!!");
        return Mn(0);
    }
    if n < k {
        return Mn(0);
    }
    fac[n] * finv[n - k]
}

// 離散対数、離散平方根についてはまた必要になったら用意する。2020-06-30
