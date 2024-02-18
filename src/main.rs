// use crate::access::get_data;

mod access;
mod types;

fn main() {
    // access::get_data("https://www.rust-lang.org");
    let data = access::get_data("https://www.yodobashi.com/product/100000001007496605/");

    println!("{:?}", data);
}
