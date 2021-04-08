// BIT
// Binary Indexed Tree
// Fenwick Tree
// 更新と出力の両方が頻繁にあり、普通にやっても累積和でやってもだめな場合の折衷案として使えます。

// Inverse Semigroup で使えますが、普通は群でしょう。
// 可逆でなく、単なるモノイドの場合はセグメントツリーを使うこと。

// 0番は空けて、1-indexedです。

// 3 1 4 1 5 9 2 6 5 3 5を1回更新して1回区間和を出す例題になってます。

// 更新メソッド
// incrementは増分。新しい値そのものではないので注意！
fn update_tree(tree: &mut Vec<usize>, index: usize, increment: usize) {
    let mut index = index;
    while index < tree.len() {
        // TODO: 群の演算。問題によって違う。
        tree[index] = tree[index] + increment;
        // 1110 = 1110 + 0010 → 10000 みたいな。一番下位の1を足す。
        index += (index as isize & -(index as isize)) as usize;
    }
}

// 累積和取得メソッド。区間和の取得はこれの差で。
fn get_sum(tree: &Vec<usize>, index: usize) -> usize {
    let mut index = index;
    let mut result = 0;
    while index > 0 {
        // TODO: 群の演算。問題によって違う。
        result = result + tree[index];
        index -= (index as isize & -(index as isize)) as usize;
    }
    result
}

fn main() {
    // TODO: 入力を受け取る。
    let n = 11;
    let a = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    // 構築。
    let mut tree = vec![0; n + 1];

    for i in 1..=n {
        update_tree(&mut tree, i, a[i - 1]);
    }

    // TODO: クエリの処理
    update_tree(&mut tree, 7, 100);
    let result = get_sum(&tree, 9) - get_sum(&tree, 4);
    println!("{}", result); // 127
    println!("{}", get_sum(&tree, 0)); // 0
}
