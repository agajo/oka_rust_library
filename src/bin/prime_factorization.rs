// =========素因数分解Aここから下をコピペ============
// 試し割りによる方法。O(√n)です。m回やるならO(m√n)
// 返り値は{素数、その個数}のvector
fn prime_factorization_a(n: i64) -> Vec<(i64, i64)> {
    let mut ans: Vec<(i64, i64)> = Vec::new();
    let mut k = n;
    for i in 2..n {
        if i > n / i {
            break;
        }

        let mut count: i64 = 0;
        while k % i == 0 {
            count += 1;
            k /= i;
        }
        if count > 0 {
            ans.push((i, count));
        }
    }
    if k != 1 {
        ans.push((k, 1));
    }
    ans
}

// =========素因数分解Aここまで============

// =========素因数分解Bここから下をコピペ============
// 事前に(1でない)最小約数を記録した配列を作る。
// 事前計算O(nlogn)、分解一回はO(logn)
// m回やるならO((n+m)logn)
// nが10^6を超えている時は使えない！！！！
fn make_min_divisors(n: usize) -> Vec<usize> {
    let mut mins: Vec<usize> = Vec::new();
    for i in 0..n + 1 {
        mins.push(i);
    }
    for i in 2..n {
        if i > n / i {
            break;
        }

        if mins[i] % i == 0 {
            let mut j = 0;
            while i + j <= n {
                if mins[i + j] == i + j {
                    mins[i + j] = i;
                }
                j += i;
            }
        }
    }
    mins
}

// minDivisorsをコピーせずに使い回すため、参照渡し
// 返り値は{素数、その個数}のvector
fn prime_factorization_b(n: usize, min_divisors: &Vec<usize>) -> Vec<(usize, usize)> {
    if n > min_divisors.len() - 1 {
        println!("error! n must be <= minDivisors.size()-1");
    } else if n <= 1 {
        return vec![];
    }
    let mut result: Vec<(usize, usize)> = Vec::new();
    let mut x = n;
    let mut last_divisor = min_divisors[n];
    let mut count = 0;
    while x != 1 {
        if min_divisors[x] == last_divisor {
            count += 1;
            x = x / min_divisors[x];
        } else {
            result.push((last_divisor, count));
            last_divisor = min_divisors[x];
            count = 0;
        }
    }
    result.push((last_divisor, count));
    result
}
// =========素因数分解Bここまで============

fn main() {
    let result = prime_factorization_a(96);
    println!("{}", result.len()); // 1を素因数分解すると、空ベクトルが返る。
    for x in result {
        println!("{}:{}", x.0, x.1);
    }
    let min_divisors = make_min_divisors(1000000);
    let result = prime_factorization_b(96, &min_divisors);
    println!("{}", result.len()); // 1を素因数分解すると、空ベクトルが返る。
    for x in result {
        println!("{}:{}", x.0, x.1);
    }
}
