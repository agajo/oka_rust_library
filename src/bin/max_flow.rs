use proconio::input;
use std::cmp::min;
use std::collections::{HashMap, VecDeque};

// 最大流問題を解きます。(最小カット、二部マッチングにも使える)
// アルゴリズムは
// targetsは、HashMap。キーは相手、valueは流量。のVector。
fn max_flow(targets: &Vec<HashMap<usize, usize>>, start: usize, terminate: usize) -> usize {
    let n = targets.len();
    // 残余グラフです。
    let mut targets = targets.clone();
    let mut ans = 0usize;
    loop {
        // 残余グラフに対し、DFS(メモ化再帰)でルート(増分)を見つける
        let mut done = vec![false; n];
        let mut route = Vec::new();
        let level = get_level(&targets, start);
        let addition = find_route(
            &mut targets,
            start,
            terminate,
            &mut route,
            &mut done,
            &level,
        );
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
                let &now_capa = targets[route[i]].get(&route[i + 1]).unwrap();
                if now_capa > addition {
                    targets[route[i]].insert(route[i + 1], now_capa - addition);
                } else if now_capa == addition {
                    targets[route[i]].remove(&route[i + 1]);
                } else {
                    targets[route[i]].remove(&route[i + 1]);
                    let &back = match targets[route[i + 1]].get(&route[i]) {
                        Some(x) => x,
                        None => &0,
                    };
                    targets[route[i + 1]].insert(route[i], back + addition - now_capa);
                }
            }
        }
    }
    ans
}

// 通るノードを並べたVecを更新し、流量増分を返します
// メモdoneの中身は「未調査」か「調査済み(今調べてるルートの途中も含む)」かどちらか
fn find_route(
    targets: &Vec<HashMap<usize, usize>>,
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
    for (&tar, &capa) in targets[start].iter() {
        if level[start] == None
            || level[tar] == None
            || level[start].unwrap() >= level[tar].unwrap()
        {
            continue;
        }
        let flow = find_route(&targets, tar, terminate, route, done, level);
        if flow > 0 {
            return min(flow, capa);
        }
    }
    route.remove(route.len() - 1);
    return 0;
}

// 残余グラフを受け取って、各ノードまでの最短距離を記録したものを返します
fn get_level(targets: &Vec<HashMap<usize, usize>>, s: usize) -> Vec<Option<usize>> {
    let n = targets.len();
    let mut level = vec![None; n];
    let mut que = VecDeque::new();
    que.push_back(s);
    level[s] = Some(0);
    while !que.is_empty() {
        let now = que.pop_front().unwrap();
        for (&tar, &capa) in targets[now].iter() {
            if level[tar] == None && capa > 0 {
                level[tar] = Some(level[now].unwrap() + 1);
                que.push_back(tar);
            }
        }
    }
    level
}

// 最大フロー、最小カット、二部マッチングは全部これで解けるはず。
fn main() {
    // ARC074 F をやってみます。
    input! {h:usize,w:usize,a:[String;h]}
    let a: Vec<Vec<char>> = a.iter().map(|s| s.chars().collect()).collect();
    let mut targets: Vec<HashMap<usize, usize>> = vec![HashMap::new(); h + w];
    let mut s: usize = 0;
    let mut t: usize = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'o' {
                targets[i].insert(h + j, 1);
                targets[h + j].insert(i, 1);
            } else if a[i][j] == 'S' {
                s = i;
                targets[i].insert(h + j, std::usize::MAX / 2);
                targets[h + j].insert(i, std::usize::MAX / 2);
            } else if a[i][j] == 'T' {
                t = i;
                targets[i].insert(h + j, std::usize::MAX / 2);
                targets[h + j].insert(i, std::usize::MAX / 2);
            }
        }
    }
    let s = s;
    let t = t;
    let ans = max_flow(&targets, s, t);
    println!(
        "{}",
        if ans > std::usize::MAX / 4 {
            "-1".to_string()
        } else {
            ans.to_string()
        }
    );
}
