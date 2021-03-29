use itertools::Itertools;

// itertoolsの使い方をいろいろ確認します。
// itertools-num は別物です。

fn main() {
    // 組み合わせ全列挙 (indexだけでなく、実際の要素を指定した個数組み合わせてくれる)
    let result = (0..5usize).combinations(3);
    result.for_each(|v| {
        println!("{:?}", v);
    });
    let v = vec!["ore".to_string(), "aitsu".to_string(), "senko".to_string()];
    let result = v.iter().combinations(2);
    result.for_each(|v| {
        println!("{:?}", v);
    });

    // 同じものを選んでもいい組み合わせ全列挙
    let result = (0..3usize).combinations_with_replacement(3);
    result.for_each(|v| {
        println!("{:?}", v);
    });
}
