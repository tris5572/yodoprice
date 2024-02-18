use data::AppData;

mod access;
mod data;
mod types;

fn main() {
    // access::get_data("https://www.rust-lang.org");
    // let data = access::get_data("https://www.yodobashi.com/product/100000001007496605/");
    // println!("{:?}", data);

    let app_data = AppData::from_file();
    println!("{:?}", app_data.histories);
}
