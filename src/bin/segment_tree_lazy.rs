use proconio::{input, marker::Usize1};
fn main() {
    // これ用のコードです。
    // https://atcoder.jp/contests/typical90/tasks/typical90_ac
    input! {
        w:usize,
        n:usize,
        lr:[(Usize1,Usize1);n],
    }
    let mut tree = SegTree::from_vec(
        vec![0; w],
        |x, y| x.max(y),
        0,
        |x, y| x.max(y),
        |x, y| x.max(y),
        0,
    );
    for (l, r) in lr {
        let r = r + 1;
        let max_height = tree.get_range_result(l, r);
        let new_height = max_height + 1;
        println!("{}", new_height);
        tree.update_range(l, r, new_height);
    }
}

// ==========ここから構造体「遅延セグメントツリー」を定義します=========

// これを大いに参考にしています。
// https://atcoder.github.io/ac-library/production/document_ja/lazysegtree.html

// S: モノイドの型。 F: モノイド準同型S→Sを確定させる値の型。
struct SegTree<S, F, G, H, I> {
    seg_tree: Vec<S>,
    lazy_tree: Vec<F>,
    op: G,
    e: S,
    mapping: H,
    composition: I,
    id: F,
}

impl<S, F, G, H, I> SegTree<S, F, G, H, I>
where
    S: Clone + Copy,
    F: Clone + Copy + Eq,
    G: Fn(S, S) -> S,
    H: Fn(F, S) -> S,
    I: Fn(F, F) -> F,
{
    pub fn from_vec(
        vec: Vec<S>,
        op: G,
        e: S,
        mapping: H,
        composition: I,
        id: F,
    ) -> SegTree<S, F, G, H, I> {
        let n = vec.len();

        // 必要な大きさを確保
        let mut tree_size = 1;
        while tree_size < n + n - 1 {
            tree_size = tree_size * 2 + 1;
        }
        let tree_size = tree_size;
        let leaf_begin_index = tree_size / 2;
        let mut seg_tree = vec![e; tree_size];
        let lazy_tree = vec![id; tree_size];

        // 葉を初期化
        for i in 0..n {
            seg_tree[leaf_begin_index + i] = vec[i];
        }

        // 親たちを構築
        if leaf_begin_index >= 1 {
            for i in (0..=leaf_begin_index - 1).rev() {
                seg_tree[i] = op(seg_tree[i * 2 + 1], seg_tree[i * 2 + 2]);
            }
        }

        SegTree {
            seg_tree: seg_tree,
            lazy_tree: lazy_tree,
            op: op,
            e: e,
            mapping: mapping,
            composition: composition,
            id: id,
        }
    }

    fn evaluate(&mut self, index: usize) {
        if self.id != self.lazy_tree[index] {
            if index < self.seg_tree.len() / 2 {
                self.lazy_tree[index * 2 + 1] =
                    (self.composition)(self.lazy_tree[index], self.lazy_tree[index * 2 + 1]);
                self.lazy_tree[index * 2 + 2] =
                    (self.composition)(self.lazy_tree[index], self.lazy_tree[index * 2 + 2]);
            }
            self.seg_tree[index] = (self.mapping)(self.lazy_tree[index], self.seg_tree[index]);
            self.lazy_tree[index] = self.id;
        }
    }

    fn update_range(&mut self, a: usize, b: usize, x: F) {
        self.update_query(a, b, 0, 0, (self.seg_tree.len() + 1) / 2, x);
    }

    fn update_query(&mut self, a: usize, b: usize, index: usize, l: usize, r: usize, x: F) {
        self.evaluate(index);
        if a <= l && r <= b {
            self.lazy_tree[index] = (self.composition)(x, self.lazy_tree[index]);
            self.evaluate(index);
        } else if a < r && l < b {
            self.update_query(a, b, index * 2 + 1, l, (l + r) / 2, x);
            self.update_query(a, b, index * 2 + 2, (l + r) / 2, r, x);
            self.seg_tree[index] =
                (self.op)(self.seg_tree[index * 2 + 1], self.seg_tree[index * 2 + 2]);
        }
    }

    // [a,b) の範囲の結果を返します
    fn get_range_result(&mut self, a: usize, b: usize) -> S {
        self.query(a, b, 0, 0, (self.seg_tree.len() + 1) / 2)
    }
    // [a,b)の範囲の結果をノードkに問い合わせます。
    // ノードkの担当範囲が[l,r)になるようにl,rを調整しながら再帰的に問い合わせます。
    // kからl,rは原理的には計算可能ですが、計算が手間なので毎回渡してあげることにします。
    fn query(&mut self, a: usize, b: usize, index: usize, l: usize, r: usize) -> S {
        self.evaluate(index);
        if r <= a || b <= l {
            return self.e;
        } else if a <= l && r <= b {
            return self.seg_tree[index];
        } else {
            let p = self.query(a, b, index * 2 + 1, l, (l + r) / 2);
            let q = self.query(a, b, index * 2 + 2, (l + r) / 2, r);
            return (self.op)(p, q);
        }
    }
}

// ==========構造体の定義ここまで==============================
