fn main() {
    static N: i64 = 314;
    static S: &str = "6283";
    let v: Vec<char> = vec!['1', '4', '1', '4'];
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
}
