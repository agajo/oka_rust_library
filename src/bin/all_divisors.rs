fn all_divisors(n: i64) -> Vec<i64> {
    let mut result_increacing = Vec::<i64>::new();
    let mut result_decreacing = Vec::<i64>::new();
    for i in 1..n {
        if i > n / i {
            break;
        }
        if n % i == 0 {
            result_increacing.push(i);
            if i != n / i {
                result_decreacing.push(n / i);
            }
        }
    }
    for x in result_decreacing.iter().rev() {
        result_increacing.push(*x);
    }
    result_increacing
}

fn main() {
    let v = all_divisors(400);
    for x in v {
        println!("{}", x);
    }
}
