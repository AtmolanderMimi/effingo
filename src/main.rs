pub use effingo;
use effingo::{config::Config, copy_manager::CopyManager};

use std::env;

fn main() {
    let config = Config::build_from_args(&env::args().collect::<Vec<String>>())
        .expect("Missing arguements");

    let copy_manager = CopyManager::new(config);
    match copy_manager.run() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}
