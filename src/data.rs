use chrono::serde::ts_seconds;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

use crate::types::WebData;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// #[derive(Debug, Serialize, Deserialize)]
// pub struct AllHistory {
//     pub histories: Vec<ProductHistory>,
// }

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// 1つの製品における価格等の履歴データ
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductHistory {
    pub name: String,
    pub url: String,
    pub history: Vec<OnePrice>,
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
    fn from_web_data(data: WebData) -> Self {
        Self {
            price: data.price,
            point: data.point,
            point_ratio: data.point_ratio,
            datetime: Utc::now(),
        }
    }
}
