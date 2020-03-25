pub enum ShowerTemperature {
    Hot,
    Cold,
    Mild,
}

pub fn take_shower(temp: ShowerTemperature) {
    match temp {
        ShowerTemperature::Cold => println!("Brrrr.... Cold water ❄"),
        ShowerTemperature::Hot => println!("Nice sauna bath 🛀"),
        ShowerTemperature::Mild => println!("Not too shabby"),
    }
}
