extern crate nalgebra as na;
use na::{DMatrix, DVector};

// Xとuを元に、Xv=u にできるだけ近くなるvを求めます
// 定数項はナシ。原点を絶対に通る。
fn linear_regression(x: &Vec<Vec<f64>>, u: &Vec<f64>) -> Option<Vec<f64>> {
    if x.len() != u.len() {
        panic!("the data size and the answer size are different!");
    }
    let m = x.len();
    let n = x[0].len();
    let x = DMatrix::from_fn(m, n, |r, c| x[r][c] as f64);
    let xt = x.transpose();
    let xtx = &xt * x;
    let u = DVector::from_vec(u.iter().map(|k| *k as f64).collect());
    let xtu = xt * u;

    let w = xtx.lu().solve(&xtu);
    if w == None {
        return None;
    } else {
        let w: Vec<f64> = w.unwrap().iter().map(|x| *x).collect();
        return Some(w);
    }
}

fn main() {
    let x = vec![
        vec![1.0, 2.0, 10.0],
        vec![0.0, 5.0, 1.0],
        vec![3.0, 8.0, 9.0],
    ];
    let u = vec![3495.0, 1306.0, 4585.0];
    let w = linear_regression(&x, &u);
    println!("{:?}", w); // 正解は[100, 200, 300]
}
