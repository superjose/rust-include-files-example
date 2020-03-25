// This can be done in Rust
// If you use this method, Rust can get confused if you include a house.rs file at the
// root level directory.
// Ex: /src/house.rs
//
// As this happened, the Rust team in the 2018 release allowed you to
// create a house.rs file in /src/house.rs and export the modules that were
// found in the next level directory.
//
// That example can be found in bathroom.rs, kitchen.rs. It's preferred to use it this way
// as this prevents the namespace collision.
//
// Traditional examples (including mod.rs) can be found in src/house/diner
pub mod bathroom;
pub mod diner;
pub mod kitchen;
