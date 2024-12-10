use std::{env, collections::HashMap};

mod ws;
mod hypr;

fn main() {
    // get WM
    let wm = env::var("XDG_CURRENT_DESKTOP").expect("Failed to detect window manager");
    // get output
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("no output provided");
    }
    let output: u32 = args[1].parse().expect("Output is not a valid number");

    let names = HashMap::from([
        (1, "".to_string()),
        (2, "󰄛".to_string()),
        (3, "󰴈".to_string()),
        (4, "♪".to_string()),
        (5, "".to_string()),
        (6, "".to_string()),
        (7, "󰑴".to_string()),
        (8, "".to_string()),
    ]);

    match wm.as_str() {
        "sway" => {},
        "Hyprland" => hypr::start(output, names),
        _ => eprintln!("Unknown window manager {}", wm)
    };
}

