pub fn wash_face() {
    /**
     * The following functions can't be used as for Rust they're on a
     * differnet module and they're not exposed.
     * utensils::use_soap();
     * utensils::use_water();
     */
    // You can use this one as it's publicly exposed.
    utensils::clean_face();
}

pub fn wash_hands() {
    utensils::wash_hands();
}

// Can never be accessed outside this file, since
// it's private
mod utensils {
    fn use_soap() {
        println!("Using soap...");
    }
    fn use_water() {
        println!("Use water...");
    }
    fn clean() {
        use_water();
        use_soap();
        use_water();
    }
    pub fn clean_face() {
        clean();
        println!("Face cleaned...");
    }

    pub fn wash_hands() {
        clean();
        println!("Hands washed!");
    }
}
