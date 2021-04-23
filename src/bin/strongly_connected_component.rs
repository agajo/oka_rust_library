use num::integer::binomial;
use petgraph::algo::tarjan_scc;
use petgraph::prelude::*;
use proconio::{input, marker::Usize1};

// これに通るものです。
// https://atcoder.jp/contests/typical90/tasks/typical90_u

fn main() {
    input! {
        _:usize,
        m:usize,
        mut ab:[(Usize1,Usize1);m],
    }
    // Graph<ノード重みなし、エッジ重みなし、有向、ノードのインデックスはusize型>
    let g: Graph<(), (), Directed, usize> = Graph::from_edges(ab);
    // IntoNeighborsは、 &Graph 型に実装されている！
    // 所有権渡す必要がないのは明らかだが…。
    let result = tarjan_scc(&g);
    // 結果は成分のDAG(トポロジカルソート済み！)になっている！
    let ans: usize = result
        .iter()
        .filter(|v| v.len() >= 2)
        .map(|v| binomial(v.len(), 2))
        .sum();
    println!("{}", ans);
}
