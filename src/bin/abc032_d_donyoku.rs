use proconio::input;

// 貪欲法！！vが大きいやつから、w超えるまで順に取る。
fn main() {
    input! {
        n:usize,
        limit:usize,
        mut vw:[(usize,usize);n],
    }
    vw.sort_by(|a, b| b.cmp(a));
    let mut value = 0;
    let mut weight = 0;
    for (v, w) in vw {
        if weight + w > limit {
            continue;
        }
        value += v;
        weight += w;
    }
    println!("{}", value);
}
