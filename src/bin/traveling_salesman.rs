use proconio::input;
// ABC180のE問題に提出してACしたコードです。
// 巡回セールスマン問題を解くものです。

fn main() {
    input! {
        number_of_cities:usize,
        xyz:[(isize,isize,isize);number_of_cities],
    }

    // distance
    let mut distance: Vec<Vec<usize>> = vec![vec![0; number_of_cities]; number_of_cities];

    for city_from in 0..number_of_cities {
        for city_to in 0..number_of_cities {
            // TODO: 都市city_fromからcity_toへの距離。問題によって違うはず。
            // 都市は0〜(number_of_cities-1) になっていること。
            distance[city_from][city_to] = ((xyz[city_from].0 - xyz[city_to].0).abs()
                + (xyz[city_from].1 - xyz[city_to].1).abs()
                + 0isize.max(xyz[city_to].2 - xyz[city_from].2))
                as usize;
        }
    }

    // dp[state][last] = 「訪問済み都市状態がstateで、最後に訪れた都市がlast」という状態になれる最小コスト
    let mut dp = vec![vec![std::usize::MAX; number_of_cities]; 1 << number_of_cities];
    // 都市一つも訪れてない状態を作るコストは0にします。多分意味ないけど。
    for last in 0..number_of_cities {
        dp[0][last] = 0;
    }
    // TODO: 「最初の都市としてコスト0でいける都市1つだけに訪れたことがある状態」を作るコストを0とします。
    // スタートが決まっていない場合
    // for last in 0.. number_of_cities
    // {
    //     dp[1 << last][last] = 0;
    // }
    // 都市0からスタートする場合
    dp[1 << 0][0] = 0;

    // 状態列挙しながら、そこから次の都市を全列挙しながら、その場合の次の状態におけるコストを更新していきます
    for state in 1..(1 << number_of_cities) {
        for last in 0..number_of_cities {
            // lastが行ったことない場所だったらおかしいのでcontinue
            if (state & (1 << last)) == 0 {
                continue;
            }
            for next_city in 0..number_of_cities {
                // 訪問済みの場所に行こうとしてたら意味ないのでcontinue
                if (state & 1 << next_city) > 0 {
                    continue;
                }
                // ここから作る次の状態がより良い場合、更新します
                if dp[state][last] < std::usize::MAX
                    && dp[state | 1 << next_city][next_city]
                        > dp[state][last] + distance[last][next_city]
                {
                    dp[state | 1 << next_city][next_city] =
                        dp[state][last] + distance[last][next_city];
                }
            }
        }
    }

    let mut result = std::usize::MAX;
    for last in 0..number_of_cities {
        // TODO: 都市0に戻らない場合、+dist[last][0]は不要
        // スタートがどこでもよくて一周する場合は、0からスタートして0に戻ることにすればOK
        if result > dp[(1 << number_of_cities) - 1][last] + distance[last][0] {
            result = dp[(1 << number_of_cities) - 1][last] + distance[last][0];
        }
    }
    println!("{}", result);
}
