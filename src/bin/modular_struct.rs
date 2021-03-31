use num::{pow, One};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

fn main() {
    let x = Mn::new(4);
    let y = Mn::new(10);
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
    // TODO: max_nを変更
    let max_n = 10usize.pow(6);
    let mut fac = vec![Mn::new(0); max_n];
    let mut finv = vec![Mn::new(0); max_n];
    init_modular_tables(&mut fac, &mut finv);
    let a = modular_combination(6, 4, &fac, &finv);
    let b = modular_permutation(6, 4, &fac, &finv);
    let c = modular_combination(6, 0, &fac, &finv);

    println!("6C4:{}, 6P4:{}, 6C0:{}", a, b, c);
}

// --------------ここから、構造体Mn(Modular Number)の実装--------------
#[derive(Debug, Copy, Clone, PartialEq)]
struct Mn {
    v: usize,
    m: usize,
}

impl One for Mn {
    fn one() -> Self {
        Self {
            v: 1,
            // TODO: CHANGE MODULAR HERE!!
            m: 7,
            // m:998_244_353,
            // m:1_000_000_007,
        }
    }
}

impl Mn {
    fn new(value: usize) -> Mn {
        let m = Mn::one().m;
        let value = value.rem_euclid(m);
        Mn { v: value, m: m }
    }
    // これを10^7回以上呼ぶ場合はメモ化しましょう！！
    fn pow(&self, exp: usize) -> Mn {
        pow(*self, exp)
    }
}

impl Display for Mn {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.v)
    }
}

impl Add for Mn {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            v: (self.v + other.v).rem_euclid(self.m),
            m: self.m,
        }
    }
}

impl AddAssign for Mn {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            v: (self.v + other.v).rem_euclid(self.m),
            m: self.m,
        };
    }
}

impl Mul for Mn {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            v: (self.v * other.v).rem_euclid(self.m),
            m: self.m,
        }
    }
}

impl MulAssign for Mn {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            v: (self.v * other.v).rem_euclid(self.m),
            m: self.m,
        };
    }
}

impl Neg for Mn {
    type Output = Self;
    fn neg(self) -> Self {
        Mn {
            v: if self.v == 0 { 0 } else { self.m - self.v },
            m: self.m,
        }
    }
}

impl Sub for Mn {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            v: if self.v >= other.v {
                self.v - other.v
            } else {
                self.m - (other.v - self.v)
            },

            m: self.m,
        }
    }
}

impl SubAssign for Mn {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            v: if self.v >= other.v {
                self.v - other.v
            } else {
                self.m - (other.v - self.v)
            },
            m: self.m,
        };
    }
}

// --------------ここまで、構造体Mn(Modular Number)の実装--------------

// --------------ここから二項係数(nCk)の話-----------------------------
// 構造体Mnからコピペする必要あり！

// テーブルを作る前処理
// 計算量 O(n)
fn init_modular_tables(fac: &mut Vec<Mn>, finv: &mut Vec<Mn>) {
    let max_n = fac.len();
    let mut inv = vec![Mn::new(0); max_n];
    fac[0] = Mn::new(1);
    fac[1] = Mn::new(1);
    finv[0] = Mn::new(1);
    finv[1] = Mn::new(1);
    inv[1] = Mn::new(1);
    for i in 2..max_n {
        let mi = Mn::new(i);
        let m = Mn::one().m;
        fac[i] = fac[i - 1] * mi;
        // 何でこれで順にinv[i]が出るんや2020-06-30
        inv[i] = Mn::new(m) - inv[m % i] * Mn::new(m / i);
        finv[i] = finv[i - 1] * inv[i];
    }
}

// 二項係数計算 nCk
// 先にinitModularTables()を呼んでおくこと！
fn modular_combination(n: usize, k: usize, fac: &Vec<Mn>, finv: &Vec<Mn>) -> Mn {
    if fac[5] != Mn::new(120) {
        println!("call initModularTables() first!!!!!!!!");
        return Mn::new(0);
    }
    if n < k {
        return Mn::new(0);
    }
    fac[n] * finv[k] * finv[n - k]
}

// 順列計算 nPk
// 先にinitModularTables()を呼んでおくこと！
fn modular_permutation(n: usize, k: usize, fac: &Vec<Mn>, finv: &Vec<Mn>) -> Mn {
    if fac[5] != Mn::new(120) {
        println!("call initModularTables() first!!!!!!!!");
        return Mn::new(0);
    }
    if n < k {
        return Mn::new(0);
    }
    fac[n] * finv[n - k]
}

// 離散対数、離散平方根についてはまた必要になったら用意する。2020-06-30
