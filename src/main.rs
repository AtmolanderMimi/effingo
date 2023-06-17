use effingo::config::Config;
use effingo::copy_directory;

use std::env;

fn main() {
    let config = Config::build_from_args(&env::args().collect::<Vec<String>>())
        .expect("Missing arguements");

    copy_directory(&config.directory_to_copy.to_str().unwrap(), &config.target_directory.to_str().unwrap())
        .unwrap();
}
