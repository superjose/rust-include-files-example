pub mod dessert;
pub mod main_dish;
use super::diner::main_dish::lasagna;

pub fn eat() {
    lasagna::eat_lasagna();
    let candy = dessert::candy::Candy::new_chocolate(10);
    dessert::candy::eat_dessert(candy);
}
