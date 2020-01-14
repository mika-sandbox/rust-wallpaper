use std::env;
use std::fs::canonicalize;
use std::path::PathBuf;

mod platform;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => eprintln!(
            "No arguments is passed. Please pass the file path that you want to set to the wallpaper."
        ),
        2 => {
            let path = PathBuf::from(&args[1]);
            if !path.exists() || !path.is_file() {
                eprintln!("No such file or could not open the file");
                return;
            }

            // convert to absolute path
            let path = canonicalize(&path).unwrap();

            platform::apply_change(&path.to_str().unwrap()).unwrap();
        }
        _ => eprintln!(
            "More than 1 argument. Please pass the only 1 argument that you want to set to the wallpaper."
        ),
    }
}
