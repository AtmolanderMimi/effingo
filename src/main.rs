use clap::Parser;
pub use effingo;
use effingo::config::Config;
use effingo::copy_manager::CopyManager;
use effingo::waiting_animation;

fn main() {
    let config = Config::parse();

    waiting_animation::make_animation_thread();

    let copy_manager = CopyManager::new(config);
    match copy_manager.run() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}
