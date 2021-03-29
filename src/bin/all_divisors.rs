fn all_divisors(n: usize) -> Vec<usize> {
    let mut result_increacing = Vec::<usize>::new();
    let mut result_decreacing = Vec::<usize>::new();
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
