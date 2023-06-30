use std::thread;
use std::time::Duration;
use std::io::Write;

pub fn make_animation_thread() {
    thread::spawn(animation_thread);
}

fn animation_thread() {
    let message = "Please wait, this might take a while".to_string();
    println!("\n");
    loop {
        for i in 0..4 {
            let points_str = (0..4).map(|j| {
                    if j <= i {
                        '.'
                    } else {
                        ' '
                    }
                }).collect::<String>();

            print!("\r");
            print!("{}{}", message, points_str);
            let _ = std::io::stdout().flush();
            
            thread::sleep(Duration::from_millis(800));
        }
    }
}