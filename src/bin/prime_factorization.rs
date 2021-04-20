use std::collections::BTreeMap;
// =========素因数分解Aここから下をコピペ============
// 試し割りによる方法。O(√n)です。m回やるならO(m√n)
// 返り値は{素数、その個数}のBTreeMap
// HashMapでない理由は、素数を小さい順に見る、とかしたいかもしれないから。
fn prime_factorization_a(n: usize) -> BTreeMap<usize, usize> {
    let mut ans = BTreeMap::new();
    let mut k = n;
    for i in 2..n {
        if i > n / i {
            break;
        }

        let mut count = 0;
        while k % i == 0 {
            count += 1;
            k /= i;
        }
        if count > 0 {
            ans.insert(i, count);
        }
    }
    if k != 1 {
        ans.insert(k, 1);
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
// 返り値は{素数、その個数}のBTreeMap
fn prime_factorization_b(n: usize, min_divisors: &Vec<usize>) -> BTreeMap<usize, usize> {
    if n > min_divisors.len() - 1 {
        println!("error! n must be <= minDivisors.size()-1");
    } else if n <= 1 {
        return BTreeMap::new();
    }
    let mut result = BTreeMap::new();
    let mut x = n;
    let mut last_divisor = min_divisors[n];
    let mut count = 0;
    while x != 1 {
        if min_divisors[x] == last_divisor {
            count += 1;
            x = x / min_divisors[x];
        } else {
            result.insert(last_divisor, count);
            last_divisor = min_divisors[x];
            count = 0;
        }
    }
    result.insert(last_divisor, count);
    result
}
// =========素因数分解Bここまで============

fn main() {
    let result = prime_factorization_a(1);
    println!("1 → {}", result.len()); // 1を素因数分解すると、空ベクトルが返る。
    let result = prime_factorization_a(96);
    for (p, count) in result {
        println!("{}:{}", p, count);
    }
    let min_divisors = make_min_divisors(1000000);
    let result = prime_factorization_a(1);
    println!("1 → {}", result.len()); // 1を素因数分解すると、空ベクトルが返る。
    let result = prime_factorization_b(96, &min_divisors);
    for (p, count) in result {
        println!("{}:{}", p, count);
    }
}
