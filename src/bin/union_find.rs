use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
use std::collections::HashMap;

// ABC183 F にACするものです。そのためにいくつか追加でやってます。
// 連結成分の個数がほしいだけなら、union後、find(i)==i になるiの個数を数えればOK。
fn main() {
    input!(
        n: usize,
        q: usize,
        c: [Usize1; n],
        queries: [(Usize1, Usize1, Usize1); q]
    );
    let mut uf = UnionFind::new(n);
    let mut infos: Vec<HashMap<usize, usize>> = (0usize..n)
        .map(|i| {
            let mut nhm = HashMap::new();
            nhm.insert(c[i], 1);
            nhm
        })
        .collect();
    for query in queries {
        if query.0 == 0 {
            let x = query.1;
            let y = query.2;
            let px = uf.find_mut(x);
            let py = uf.find_mut(y);
            let is_merged = uf.union(x, y);
            if is_merged {
                if uf.find_mut(x) == px {
                    // infos[py]を借用すると、そのあとinfos[px].insertする時に可変借用で怒られる。
                    // それを回避するため、あらかじめcloneしてから値を読む。
                    let ip = infos[py].clone();
                    for (key, value) in ip {
                        let v = match infos[px].get(&key) {
                            Some(z) => z,
                            None => &0,
                        } + value;
                        infos[px].insert(key, v);
                    }
                } else {
                    let ip = infos[px].clone();
                    for (key, value) in ip {
                        let v = match infos[py].get(&key) {
                            Some(z) => z,
                            None => &0,
                        } + value;
                        infos[py].insert(key, v);
                    }
                }
            }
        } else {
            let root = uf.find(query.1);
            let ans = match infos[root].get(&query.2) {
                Some(z) => z,
                None => &0,
            };
            println!("{}", ans);
        }
    }
}
