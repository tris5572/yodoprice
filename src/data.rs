use std::io::Write;

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::WebData;

const DATA_FILE_NAME: &str = "data.json";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Serialize, Deserialize)]
pub struct AppData {
    /// 全登録アイテムの履歴
    pub histories: Vec<ProductHistory>,
}

impl AppData {
    /// データをファイルから読み込む。
    pub fn from_file() -> Self {
        let data = read_data_file();
        let histories = data.unwrap_or_default();
        Self { histories }
    }

    /// データをファイルへ出力する。
    pub fn write_file(&self) -> std::io::Result<()> {
        write_data_file(&self.histories)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// 1つの製品における価格等の履歴データ
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductHistory {
    /// このアプリで独自に割り振ったID
    pub id: String,
    /// 製品名
    pub name: String,
    /// 自分で設定した名称
    pub custom_name: Option<String>,
    /// 製品のURL
    pub url: String,
    /// 価格の履歴
    pub history: Vec<OnePrice>,
    /// 製造メーカ
    pub maker: String,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// 1回分の価格データ
#[derive(Debug, Serialize, Deserialize)]
pub struct OnePrice {
    pub price: u64,
    pub point: u64,
    pub point_ratio: u64,
    #[serde(with = "ts_seconds")]
    pub datetime: DateTime<Utc>,
}

impl OnePrice {
    /// サイトから取得したデータを元に、1回分の価格データを生成する。
    /// 日時は現時刻を割り当てる。
    fn from_web_data(data: WebData) -> Self {
        Self {
            price: data.price,
            point: data.point,
            point_ratio: data.point_ratio,
            datetime: Utc::now(),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// ファイル操作系
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

fn read_data_file() -> std::io::Result<Vec<ProductHistory>> {
    // 実行ファイルがある場所をカレントディレクトリに設定
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    std::env::set_current_dir(exe_dir).unwrap();

    let input = std::fs::read_to_string(DATA_FILE_NAME)?;
    let data = serde_json::from_str(&input).unwrap();

    Ok(data)
}

fn write_data_file(data: &Vec<ProductHistory>) -> std::io::Result<()> {
    // データをシリアライズ
    let serialized = serde_json::to_string(&data).unwrap();

    // 実行ファイルがある場所をカレントディレクトリに設定
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    std::env::set_current_dir(exe_dir).unwrap();

    let mut file = std::fs::File::create(DATA_FILE_NAME)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
