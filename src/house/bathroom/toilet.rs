// Relatives can be included directly
// Both of the following work the same.
// super refers to the current directory.
use super::sink;
// create refers to the root directory.
// use crate::house::bathroom::sink;

pub fn use_toilet() {
    println!("💩");
    sink::wash_hands()
}
