use std::default;

// Webサイトから取得したデータ
#[derive(Debug, Default, Clone)]
pub struct WebData {
    /// 商品名
    pub name: String,
    /// 商品のURL
    pub url: String,
    /// 価格
    pub price: u64,
    /// ポイント
    pub point: u64,
    /// ポイント(%)
    pub point_ratio: u64,
    /// 在庫状況
    pub status: StockStatus,
    /// 製造メーカ
    pub maker: String,
}

// TODO: 各種ステータスに対する処理を実装する
#[derive(Debug, Clone, Default)]
pub enum StockStatus {
    /// 在庫あり
    Sufficient,
    /// 在庫僅少
    Limited,
    // お取り寄せ
    #[default]
    BackOrder,
    // 販売休止中 (違うところに表示される)
    // 販売終了 (違うところに表示される)
    // 店舗のみ（店頭でのみ販売しています）
    // 予定数の販売を終了しました
}

impl StockStatus {
    pub fn from_string(string: &str) -> StockStatus {
        if string.starts_with("在庫あり") {
            return StockStatus::Sufficient;
        }

        StockStatus::BackOrder
    }
}
