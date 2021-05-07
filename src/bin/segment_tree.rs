use proconio::{input, marker::Usize1};

// セグメントツリー。セグ木。
// 更新と出力の両方が頻繁にあり、普通にやっても累積和でやってもだめな場合の折衷案として使えます。
// モノイドで使えます。
// 可逆な場合(つまり群の場合)はBinary Indexed Treeの方がいいかも？

// このコードは、ABC185のF問題に提出してACしたものです。

fn main() {
    // TODO: 入力やqueryの内容も問題によって違うから直してね
    input! {
        n:usize,
        q:usize,
        a:[usize;n],
        txy:[(usize,Usize1,usize);q],
    }

    let mut tree = SegTree::from_vec(a, 0, |a, b| a ^ b);

    // queryを処理していきます
    // TODO: 入力やqueryの内容も問題によって違うから直してね
    for (t, x, y) in txy {
        if t == 1 {
            tree.add_to_element(x, y);
        } else {
            // // left included. right excluded.
            println!("{}", tree.get_range_result(x, y));
        }
    }
}

// ==========ここから構造体「セグメントツリー」を定義します=========

#[derive(Debug, Clone)]
struct SegTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    seg_tree: Vec<T>,
    identity_element: T,
    monoid_sum: F,
}

impl<T, F> SegTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    pub fn from_vec(vec: Vec<T>, identity_element: T, monoid_sum: F) -> SegTree<T, F> {
        let n = vec.len();

        // 必要な大きさを確保
        let mut tree_size = 1;
        while tree_size < n + n - 1 {
            tree_size = tree_size * 2 + 1;
        }
        let tree_size = tree_size;
        let leaf_begin_index = tree_size / 2;
        let mut seg_tree = vec![identity_element; tree_size];

        // 葉を初期化
        for i in 0..n {
            seg_tree[leaf_begin_index + i] = vec[i];
        }

        // 親たちを構築
        if leaf_begin_index >= 1 {
            for i in (0..=leaf_begin_index - 1).rev() {
                seg_tree[i] = monoid_sum(seg_tree[i * 2 + 1], seg_tree[i * 2 + 2]);
            }
        }

        SegTree {
            seg_tree: seg_tree,
            identity_element: identity_element,
            monoid_sum: monoid_sum,
        }
    }

    fn update_ancestors(&mut self, index: usize) {
        let leaf_begin_index = self.seg_tree.len() / 2;
        let mut t = leaf_begin_index + index;
        while t > 0 {
            t = (t - 1) / 2;
            self.seg_tree[t] =
                (self.monoid_sum)(self.seg_tree[t * 2 + 1], self.seg_tree[t * 2 + 2]);
        }
    }

    fn update_element(&mut self, index: usize, new_value: T) {
        let leaf_begin_index = self.seg_tree.len() / 2;
        self.seg_tree[leaf_begin_index + index] = new_value;
        self.update_ancestors(index);
    }

    fn add_to_element(&mut self, index: usize, adding_value: T) {
        let leaf_begin_index = self.seg_tree.len() / 2;
        self.seg_tree[leaf_begin_index + index] =
            (self.monoid_sum)(self.seg_tree[leaf_begin_index + index], adding_value);
        self.update_ancestors(index);
    }
    // [a,b) の範囲の結果を返します
    fn get_range_result(&self, a: usize, b: usize) -> T {
        self.query(a, b, 0, 0, (self.seg_tree.len() + 1) / 2)
    }
    // [a,b)の範囲の結果をノードkに問い合わせます。
    // ノードkの担当範囲が[l,r)になるようにl,rを調整しながら再帰的に問い合わせます。
    // kからl,rは原理的には計算可能ですが、計算が手間なので毎回渡してあげることにします。
    fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            return self.identity_element;
        } else if a <= l && r <= b {
            return self.seg_tree[k];
        } else {
            return (self.monoid_sum)(
                self.query(a, b, k * 2 + 1, l, (l + r) / 2),
                self.query(a, b, k * 2 + 2, (l + r) / 2, r),
            );
        }
    }
}

// ==========構造体の定義ここまで==============================
