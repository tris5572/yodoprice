use std::io::Write;
use std::sync::Mutex;

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::access::get_data;
use crate::types::{AlreadyExistsError, StockStatus, WebData};

const DATA_FILE_NAME: &str = "data.json";

/// アプリケーション全体のデータ。
pub static APP_STATE: Mutex<AppData> = Mutex::new(AppData { histories: vec![] });

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Default, Serialize, Deserialize)]
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

    /// URLから製品を追加する。
    /// URLにアクセスできないときや、すでに登録済みのときはエラーを返す。
    pub fn add_from_url(&mut self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = get_data(url)?;

        // すでに登録されているURLと重複チェックし、重複する場合はエラーを返す。
        let url_list = self.url_list();
        if url_list.iter().any(|v| v == url) {
            return Err(Box::new(AlreadyExistsError {
                message: "指定されたURLはすでに登録されています".to_string(),
            }));
        }

        // 新規追加する。
        let product = ProductHistory::from_web_data(data);
        self.histories.push(product);

        Ok(())
    }

    // 登録されている全製品の価格を取得し、更新する。
    // TODO: リターン後の表示のため、戻り値の型を Vec<Result<(), String>>に変える。
    pub fn update_all(&mut self) -> Result<(), Vec<String>> {
        let mut array = vec![];

        for url in self.url_list() {
            let result = get_data(&url);
            match result {
                Ok(data) => {
                    let item = self.borrow_mut_product_by_url(&url);
                    if let Some(v) = item {
                        v.add_web_data(data)
                    }
                }
                Err(_) => array.push(format!("データを取得できませんでした{}", url)),
            }
        }
        Ok(())
    }

    // 登録されている製品のURLの一覧を返す。
    pub fn url_list(&self) -> Vec<String> {
        let array: Vec<_> = self.histories.iter().map(|v| v.url.clone()).collect();
        array
    }

    // 指定されたURLを持つ価格履歴の借用を返す。
    pub fn borrow_product_by_url(&self, url: &str) -> Option<&ProductHistory> {
        self.histories.iter().find(|&v| v.url == url)
    }

    // 指定されたURLを持つ価格履歴の可変借用を返す。
    pub fn borrow_mut_product_by_url(&mut self, url: &str) -> Option<&mut ProductHistory> {
        self.histories.iter_mut().find(|v| v.url == url)
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

impl ProductHistory {
    /// サイトから取得したデータを元に、新しい製品データを生成する。
    fn from_web_data(data: WebData) -> Self {
        let price = OnePrice::from_web_data(data.clone());
        // TODO: IDを生成する
        Self {
            id: "".to_string(),
            name: data.name,
            custom_name: None,
            url: data.url,
            history: vec![price],
            maker: data.maker,
        }
    }

    fn add_web_data(&mut self, data: WebData) {
        let item = OnePrice::from_web_data(data);
        self.history.push(item);
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// 1回分の価格データ
#[derive(Debug, Serialize, Deserialize)]
pub struct OnePrice {
    pub price: u64,
    pub point: u64,
    pub point_ratio: u64,
    pub status: StockStatus,
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
            status: StockStatus::default(),
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
    let serialized = serde_json::to_string_pretty(&data).unwrap();

    // 実行ファイルがある場所をカレントディレクトリに設定
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    std::env::set_current_dir(exe_dir).unwrap();

    let mut file = std::fs::File::create(DATA_FILE_NAME)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
