use petgraph::prelude::*;
use proconio::input;
use std::cmp::min;
use std::collections::VecDeque;

// 最大流問題を解きます。(最小カット、二部マッチング、燃やす埋めるにも使える)
// アルゴリズムはDinic。
// targetsは、HashMap。キーは相手、valueは流量。のVector。
fn max_flow(graph: &Graph<(), usize, Directed, usize>, start: usize, terminate: usize) -> usize {
    let n = graph.node_count();
    // 残余グラフです。
    let mut graph = graph.clone();
    let mut ans = 0usize;
    loop {
        let level = get_level(&graph, start);
        // 無限に流せると発覚したか、ルートを探し尽くした場合、終了
        if ans > std::usize::MAX / 4 || level[terminate] == None {
            break;
        }
        loop {
            // 残余グラフに対し、DFS(メモ化再帰)でルート(増分)を見つける
            let mut done = vec![false; n];
            let mut route = Vec::new();
            let addition = find_route(&mut graph, start, terminate, &mut route, &mut done, &level);
            if addition == 0 {
                // 見つからなかったら終了
                break;
            } else if addition > std::usize::MAX / 4 {
                // 無限増やせる場合は当然答えは無限。
                ans = std::usize::MAX / 2;
                break;
            } else {
                // 見つかった場合、増分を答えに加え、次の残余グラフを作る
                if ans > std::usize::MAX - addition {
                    ans = std::usize::MAX / 2;
                } else {
                    ans += addition;
                }
                for i in 0..(route.len() - 1) {
                    let edge = graph
                        .edges_connecting(NodeIndex::from(route[i]), NodeIndex::from(route[i + 1]))
                        .next()
                        .unwrap();
                    let edge_id = edge.id();
                    let now_capa = *edge.weight();
                    if now_capa > addition {
                        graph.update_edge(
                            NodeIndex::from(route[i]),
                            NodeIndex::from(route[i + 1]),
                            now_capa - addition,
                        );
                    } else if now_capa == addition {
                        graph.remove_edge(edge_id);
                    } else {
                        graph.remove_edge(edge_id);
                        if let Some(back_edge) = graph
                            .edges_connecting(
                                NodeIndex::from(route[i + 1]),
                                NodeIndex::from(route[i]),
                            )
                            .next()
                        {
                            let back_edge_weight = *back_edge.weight();
                            graph.update_edge(
                                NodeIndex::from(route[i + 1]),
                                NodeIndex::from(route[i]),
                                back_edge_weight + addition - now_capa,
                            );
                        } else {
                            graph.add_edge(
                                NodeIndex::from(route[i + 1]),
                                NodeIndex::from(route[i]),
                                addition - now_capa,
                            );
                        }
                    }
                }
            }
        }
    }
    ans
}

// 通るノードを並べたVecを更新し、流量増分を返します
// メモdoneの中身は「未調査」か「調査済み(今調べてるルートの途中も含む)」かどちらか
fn find_route(
    graph: &Graph<(), usize, Directed, usize>,
    start: usize,
    terminate: usize,
    route: &mut Vec<usize>,
    done: &mut Vec<bool>,
    level: &Vec<Option<usize>>,
) -> usize {
    // 調査済みだったら即返す
    if done[start] {
        return 0;
    }
    done[start] = true;
    route.push(start);
    if start == terminate {
        return std::usize::MAX / 2;
    }
    for edge in graph.edges(NodeIndex::from(start)) {
        let tar = edge.target().index();
        let capa = *edge.weight();
        if level[start] == None
            || level[tar] == None
            || level[start].unwrap() >= level[tar].unwrap()
            || level[terminate].unwrap() < level[tar].unwrap()
        {
            continue;
        }
        let flow = find_route(&graph, tar, terminate, route, done, level);
        if flow > 0 {
            return min(flow, capa);
        }
    }
    route.remove(route.len() - 1);
    return 0;
}

// 残余グラフを受け取って、各ノードまでの最短距離を記録したものを返します
fn get_level(graph: &Graph<(), usize, Directed, usize>, s: usize) -> Vec<Option<usize>> {
    let n = graph.node_count();
    let mut level = vec![None; n];
    let mut que = VecDeque::new();
    que.push_back(s);
    level[s] = Some(0);
    while !que.is_empty() {
        let now = que.pop_front().unwrap();
        for edge in graph.edges(NodeIndex::from(now)) {
            let tar = edge.target().index();
            let capa = *edge.weight();
            if level[tar] == None && capa > 0 {
                level[tar] = Some(level[now].unwrap() + 1);
                que.push_back(tar);
            }
        }
    }
    level
}

// 最大フロー、最小カット、二部マッチング、燃やす埋めるは全部これで解けるはず。
fn main() {
    // ARC074 F をやってみます。
    input! {h:usize,w:usize,a:[String;h]}
    let a: Vec<Vec<char>> = a.iter().map(|s| s.chars().collect()).collect();
    let mut graph: Graph<(), usize, Directed, usize> = Graph::default();
    for _ in 0..(h + w) {
        graph.add_node(());
    }
    let mut s: usize = 0;
    let mut t: usize = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'o' {
                graph.add_edge(NodeIndex::from(i), NodeIndex::from(h + j), 1);
                graph.add_edge(NodeIndex::from(h + j), NodeIndex::from(i), 1);
            } else if a[i][j] == 'S' {
                s = i;
                graph.add_edge(
                    NodeIndex::from(i),
                    NodeIndex::from(h + j),
                    std::usize::MAX / 2,
                );
                graph.add_edge(
                    NodeIndex::from(h + j),
                    NodeIndex::from(i),
                    std::usize::MAX / 2,
                );
            } else if a[i][j] == 'T' {
                t = i;
                graph.add_edge(
                    NodeIndex::from(i),
                    NodeIndex::from(h + j),
                    std::usize::MAX / 2,
                );
                graph.add_edge(
                    NodeIndex::from(h + j),
                    NodeIndex::from(i),
                    std::usize::MAX / 2,
                );
            }
        }
    }
    let s = s;
    let t = t;
    let ans = max_flow(&graph, s, t);
    println!(
        "{}",
        if ans > std::usize::MAX / 4 {
            "-1".to_string()
        } else {
            ans.to_string()
        }
    );
}
