use proconio::input;
use std::cmp::{max, min};

// セグメントツリー。セグ木。
// 更新と出力の両方が頻繁にあり、普通にやっても累積和でやってもだめな場合の折衷案として使えます。
// モノイドで使えます。
// 可逆な場合(つまり群の場合)はBinary Indexed Treeの方がいいかも？

// 0番は空けて、1-indexedです。

// このコードは、ABC185のF問題に提出してACしたものです。

fn monoid_sum(left: usize, right: usize) -> usize {
    // TODO: 問題によってここは変わるはずです。和なのかminなのかxorなのか…。
    // 結合法則が成り立つ演算。
    left ^ right
}
// TODO: monoid_sumの単位元を入れます。
const IDENTITY_ELEMENT: usize = 0;

// targetIndexは0始まりです
fn update_tree(tree: &mut Vec<usize>, taget_index: usize, input_value: usize) {
    // tree_sizeよりもtree.len()のほうが1大きいので注意
    let leaf_begin = tree.len() / 2;
    let target_index_in_tree = leaf_begin + taget_index;
    // TODO: ここの更新内容は問題によって違うはず。
    tree[target_index_in_tree] = monoid_sum(tree[target_index_in_tree], input_value);
    let mut t = target_index_in_tree / 2;
    while t >= 1 {
        tree[t] = monoid_sum(tree[t * 2], tree[t * 2 + 1]);
        t = t / 2;
    }
}

// left included. right included.
fn get_range_result(
    tree: &Vec<usize>,
    scope_tree: &Vec<(usize, usize)>,
    target_index_in_tree: usize,
    left: usize,
    right: usize,
) -> usize {
    if left > right {
        return IDENTITY_ELEMENT;
    }
    // tree_sizeよりもtree.len()のほうが1大きいので注意
    if scope_tree[target_index_in_tree].0 == left && right == scope_tree[target_index_in_tree].1 {
        return tree[target_index_in_tree];
    } else {
        return monoid_sum(
            get_range_result(
                tree,
                scope_tree,
                target_index_in_tree * 2,
                max(scope_tree[target_index_in_tree * 2].0, left),
                min(scope_tree[target_index_in_tree * 2].1, right),
            ),
            get_range_result(
                tree,
                scope_tree,
                target_index_in_tree * 2 + 1,
                max(scope_tree[target_index_in_tree * 2 + 1].0, left),
                min(scope_tree[target_index_in_tree * 2 + 1].1, right),
            ),
        );
    }
}

fn main() {
    // TODO: 入力やqueryの内容も問題によって違うから直してね
    input! {
        n:usize,
        q:usize,
        a:[usize;n],
        txy:[(usize,usize,usize);q],
    }

    // セグメントツリーを構築します。BITと違い、updateTreeを使って構築すると効率が悪いので別でやります。
    let mut tree_size = 1;
    let mut leaf_begin = 1;
    while tree_size < a.len() * 2 {
        tree_size = tree_size * 2 + 1;
        leaf_begin *= 2;
    }
    // tree配列は1からtree_sizeまでを使います。
    let mut tree = vec![0; tree_size + 1];
    // 各ノードがsummarizeの責任を持つleafの範囲を0-indexedで持ちます。
    let mut scope_tree = vec![(0, 0); tree_size + 1];
    for i in 0..tree_size / 2 + 1
    // ちなみにtree_size/2+1はleaf_beginと一致する
    {
        if i < a.len() {
            tree[leaf_begin + i] = a[i];
        } else {
            tree[leaf_begin + i] = IDENTITY_ELEMENT;
        }
        scope_tree[leaf_begin + i] = (i, i);
    }
    for i in (1..=leaf_begin - 1).rev() {
        tree[i] = monoid_sum(tree[i * 2], tree[i * 2 + 1]);
        scope_tree[i] = (scope_tree[i * 2].0, scope_tree[i * 2 + 1].1);
    }

    // queryを処理していきます
    // TODO: 入力やqueryの内容も問題によって違うから直してね
    for i in 0..q {
        let t = txy[i].0;
        let x = txy[i].1 - 1;
        let y = txy[i].2;
        if t == 1 {
            // updateTreeのtargetIndexは0始まり
            update_tree(&mut tree, x, y);
        } else {
            // // left included. right included.
            println!("{}", get_range_result(&tree, &scope_tree, 1, x, y - 1));
        }
    }
}
