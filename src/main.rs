use std::env;

mod platform;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => eprintln!(
            "No arguments is passed. Please pass the file path that you want to set to the wallpaper."
        ),
        2 => {
            platform::apply_change(&args[1]).unwrap();
        }
        _ => eprintln!(
            "More than 1 argument. Please pass the only 1 argument that you want to set to the wallpaper."
        ),
    }
}
