use itertools::Itertools;

// itertoolsの使い方をいろいろ確認します。
// itertools-num は別物です。

fn main() {
    // 組み合わせ全列挙 (indexだけでなく、実際の要素を指定した個数組み合わせてくれる)
    let result = (0..5usize).combinations(3);
    result.for_each(|v| {
        print!("{:?}", v);
    });
    print!("\n");
    let v = vec!["ore".to_string(), "aitsu".to_string(), "senko".to_string()];
    let result = v.iter().combinations(2);
    result.for_each(|v| {
        print!("{:?}", v);
    });
    print!("\n");

    // 同じものを選んでもいい組み合わせ全列挙
    let result = (0..3usize).combinations_with_replacement(3);
    result.for_each(|v| {
        print!("{:?}", v);
    });
    print!("\n");

    // 順列全列挙
    let v = vec![2, 1, 2, 3];
    let result = v.iter().permutations(2);
    for x in result {
        print!("{:?}", x);
    }
    print!("\n");
}
