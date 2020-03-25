// Relatives can be included directly
// Both of the following work the same.
// super refers to the parent  directory.
// https://doc.rust-lang.org/rust-by-example/mod/super.html
use super::sink;
// create refers to the root directory.
// use crate::house::bathroom::sink;

pub fn use_toilet() {
    println!("💩");
    sink::wash_hands()
}
