pub mod food_preparation {
    // We refer this from the cargo.toml
    extern crate rand as r;

    pub fn prepare_food() {
        println!("Preparing food to {} people", r::random::<u8>())
    }
}
