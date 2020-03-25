/**
 * This is an example project. Its purpose is to show how to include
 * "resources" (mod, enums, functions, structs) to the project and how
 * to use them when it has a complex directory structure.
 *
 * This isn't supposed to show you good coding practices or how are you
 * supposed to structure your project.
 *
 * See that if we want to include the hourse/bathroom/shower.rs file
 * we need to "barrel" it from the bottom up. And we only type
 * mod house and then construct the path from there.
 *
 * Note: Rust doesn't comprehend the concept of "files" per se.
 * It sees all the files as modules, and all the files that are inside
 * directories are seen as submodules.
 *
 * For submodules to be seen, you need to export them (This concept is called)
 * barrelling in which you make them publicly visible to the parent.
 *
 * I STRONGLY RECOMMEND TO Watch this YouTube video from TensorProgramming for an excellent and
 * easy explanation on how Rust works with the file include system.
 *
 * https://www.youtube.com/watch?v=U8uhW9bFm-8
 */
mod house;
// Please, do not use hyphens as this is proven to have an
// inconsistent behavior.
// https://github.com/rust-lang/book/issues/1709
// https://rust-lang.github.io/rfcs/0940-hyphens-considered-harmful.html
// https://stackoverflow.com/a/57535534/1057052
#[path = "./welcome-home.rs"]
mod welcome_home;
// Includes the function directly
// When using crate, you use the root directory to include everything.
use crate::house::diner;
use house::bathroom::sink::wash_face;
use house::kitchen::prepare::food_preparation::prepare_food;

fn main() {
    let user = "Jose";
    building::lobby::arrive_lobby();
    welcome_home::run(user);
    house::bathroom::shower::take_shower(house::bathroom::shower::ShowerTemperature::Cold);
    wash_face();
    house::bathroom::toilet::use_toilet();
    prepare_food();
    diner::eat();
}

// https://doc.rust-lang.org/reference/visibility-and-privacy.html
/**
 * From the docs (Link above)
 *
 * By default, everything in Rust is private, with two exceptions:
 * Associated items in a pub Trait are public by default; Enum
 * variants in a pub enum are also public by default. When an
 * item is declared as pub, it can be thought of as being accessible
 * to the outside world.
 */
mod building {
    pub mod lobby {
        pub(in crate) fn arrive_lobby() {
            println!("You have arrived to the lobby");
        }
    }
}
