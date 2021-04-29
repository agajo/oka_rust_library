fn main() {
    const N: i64 = 314;
    const N2: i64 = 300;
    const S: &str = "6283";
    let s: String = "3.1415".to_string();
    let v: Vec<char> = vec!['1', '4', '1', '4'];
    const F: f64 = 1.732;
    const C: char = 'j';
    const CH: char = '7';
    const N3: isize = 7;

    // &str -> String -> i64
    let n0: i64 = S.to_string().parse::<i64>().unwrap();
    println!("{}", n0);

    // i64 -> String
    let s0: String = N.to_string();
    println!("{}", s0);

    // Vec<char> -> Iter -> String
    let s0: String = v.iter().collect(); // collect::<String>() としてもOK
    println!("{}", s0);

    // &str -> String -> Chars<'_> -> Vec<char>
    let v0: Vec<char> = S.to_string().chars().collect(); // collect::<Vec<_>>() としてもOK
    println!("{}", v0.iter().collect::<String>());

    // char→isize
    let n3: isize = CH.to_digit(10).unwrap() as isize;
    let n4: isize = (CH as u8 - b'0') as isize;
    println!("{}, {}", n3, n4);

    // isize->char
    let c0: char = std::char::from_digit(N3 as u32, 10).unwrap();
    println!("{}", c0);

    // i64 -> f64
    let f0: f64 = N as f64 / N2 as f64;
    println!("{}", f0);

    // f64 -> i64
    let n0: i64 = F as i64; // asで整数にすると切り捨てられる
    let n1: i64 = F.round() as i64; // 四捨五入ならこう。
    println!("{}, {}", n0, n1);

    // String -> f64
    let f0: f64 = s.parse().unwrap();
    println!("{}", f0);

    // アルファベット → 何番目(0-indexed)か
    let n0 = C as u8 - b'a'; // b'a' は 'a' as u8 でも可
    println!("{}, {}, {}, {}", C, C as u8, n0, b'j' - b'a');

    // 何番目(0-indexed)か → アルファベット
    let c0 = (10 + b'a') as char;
    println!("{}", c0);
}
