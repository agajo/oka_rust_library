use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

// ターミナルからcargoコマンドを実行し、公式ビジュアライザを引数を変えながら連続実行します。
// 入力データを固定し、outディレクトリにある出力結果を一つずつ画像化します。
// 公式ビジュアライザの出力ファイル名が固定の場合、上書きさせないために改造が必須！！
// out_svg/xxxx.svg という名前で出力されるように。

fn main() {
    // TODO: このframesはフォルダ調べて取得できるはずだ
    let frames = 100;
    for i in 0..frames {
        let mut command = Command::new("cargo");
        command.args(&[
            "run",
            "--release",
            "--bin",
            "vis",
            "in/0000.txt", // TODO: 入力ファイルはこれ固定でいいのか？
            &format!("out/{:>04}.txt", i),
        ]);
        let _ = command.spawn();
    }
    let mut file = File::create("out_svg/frames.txt").unwrap();
    file.write_all(frames.to_string().as_bytes()).unwrap();
}
