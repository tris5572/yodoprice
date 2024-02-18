use std::io::Write;

use chrono::Utc;
use data::{OnePrice, ProductHistory};

mod access;
mod data;
mod types;

fn main() {
    // access::get_data("https://www.rust-lang.org");
    // let data = access::get_data("https://www.yodobashi.com/product/100000001007496605/");
    // println!("{:?}", data);

    write_file_test();
}

// データ書き込みテスト
fn write_file_test() {
    let data1 = ProductHistory {
        name: "テストデータ1".to_string(),
        url: "http://test-data-01/".to_string(),
        history: vec![OnePrice {
            price: 111,
            point: 1,
            point_ratio: 11,
            datetime: Utc::now(),
        }],
        maker: "メーカ1".to_string(),
    };
    let data2 = ProductHistory {
        name: "テストデータ2".to_string(),
        url: "http://test-data-02/".to_string(),
        history: vec![OnePrice {
            price: 222,
            point: 2,
            point_ratio: 22,
            datetime: Utc::now(),
        }],
        maker: "メーカ2".to_string(),
    };

    let file_name = "test_data.json";
    let result = output(file_name, &vec![data1, data2]);
    match result {
        Ok(_) => {}
        Err(err) => println!("ファイル書き込みエラー: {}", err),
    }
}

fn output(file_name: &str, data: &Vec<ProductHistory>) -> std::io::Result<()> {
    // データをシリアライズ
    let serialized = serde_json::to_string(&data).unwrap();

    // 実行ファイルがある場所をカレントディレクトリに設定
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    std::env::set_current_dir(exe_dir).unwrap();

    let mut file = std::fs::File::create(file_name)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
