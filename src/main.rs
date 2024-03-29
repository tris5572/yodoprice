use std::io::Write;

use data::{AppData, APP_STATE};

use crate::util::{commafy, omitted_string};

mod access;
mod data;
mod types;
mod util;

fn main() {
    // access::get_data("https://www.rust-lang.org");
    // let data = access::get_data("https://www.yodobashi.com/product/100000001007496605/");
    // println!("{:?}", data);

    // ファイルからデータを読み込んで初期化する。
    // Mutexのロックを解除するために別のブロックにしている。
    {
        let mut app_state = APP_STATE.lock().unwrap();
        *app_state = AppData::from_file();
    }

    main_loop();
}

/// アプリのメインループ。
pub fn main_loop() {
    loop {
        print!("-> ");
        std::io::stdout().flush().unwrap();
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("stdinからの入力に失敗しました");
        let input = buf.trim();

        if input.starts_with("quit") || input == "q" || input.starts_with("q ") {
            // 終了
            break;
        } else if input.starts_with("help") || input == "h" {
            // ヘルプを表示
            print_help();
        } else if input.starts_with("add ") || input.starts_with("a ") {
            // 追加
            command_add(input);
            save_file();
        } else if input.starts_with("update") || input == "u" {
            // 更新
            update_all();
            save_file();
        } else if input.starts_with("list") || input == "l" {
            // 一覧表示
            print_list();
        } else if input == "add" || input == "a" {
            println!("追加するためには、URLも一緒に入力してください。");
        }
    }
}

/// ヘルプを表示する。
pub fn print_help() {
    println!("yodopriceは、ヨドバシ.comの価格の履歴を取得・表示するアプリです。");
    println!(" help / h  ヘルプを表示します。(この画面)");
    println!(" add <url> / a   指定されたURLの製品を追加します。");
    println!(" update / u   登録されている製品の価格を取得・更新します。");
}

pub fn command_add(input: &str) {
    let buf: Vec<_> = input.split_whitespace().collect();
    if buf.len() <= 1 {
        println!("URLを指定してください。");
        return;
    }
    let url = buf[1];
    let mut app_state = APP_STATE.lock().unwrap();
    let result = (*app_state).add_from_url(url);
    match result {
        Ok(_) => println!("登録が正常に完了しました"),
        Err(e) => println!("登録時にエラーが発生しました({})", e),
    }
}

pub fn update_all() {
    let mut app_state = APP_STATE.lock().unwrap();
    let _result = (*app_state).update_all();
    // TODO: 更新結果のメッセージを表示する。最安値が更新された、エラーが発生した、等。
}

// 製品の一覧を表示する。
pub fn print_list() {
    let app_state = APP_STATE.lock().unwrap();
    let products = &app_state.histories;
    let mut i = 1;

    for product in products {
        print!("{}: {}  ", i, omitted_string(&product.name));
        let (high, low, now) = product.high_low_now();
        println!(
            "￥{} - (￥{}/￥{})",
            commafy(now),
            commafy(high),
            commafy(low)
        );
        i += 1;
    }
}

// データをログファイルへ書き込む。
pub fn save_file() {
    let app_state = APP_STATE.lock().unwrap();
    let _ = (*app_state).write_file();
}
