use std::fs::File;
use std::io::prelude::*;

// Cargo.tomlでpackageのnameを確認し、
// use package_name::svg_movie::*;
// のようにする必要がある！

pub struct SvgMovie {
    frame_count: usize,
    directory_name: String,
}

impl SvgMovie {
    pub fn new() -> SvgMovie {
        let directory_name = "out_txt".to_string();
        let _ = std::fs::create_dir(&directory_name);
        SvgMovie {
            frame_count: 0,
            directory_name: directory_name,
        }
    }

    // Vec<u8>として作ったバッファにwriteln!で行を書き込んでいき
    // このメソッドに渡すと出力結果として書き出されます。
    pub fn export_frame(&mut self, buffer: Vec<u8>) {
        let mut file = File::create(format!(
            "{}/{:>04}.txt",
            self.directory_name, self.frame_count
        ))
        .unwrap();
        file.write_all(&buffer).unwrap();
        self.frame_count += 1;
    }
}
