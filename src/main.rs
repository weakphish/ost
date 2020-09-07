use std::fs;
use std::env;

fn main() {
    // get directory argument from the input - if none provided, current dir
    let mut dir = env::args().nth(1);
    let dir_string: String;
    match &dir {
        Some(d) => dir_string = dir.unwrap(),
        None => dir_string = ".".to_string()
    }
    // collect the directory entries as ReadDir objects
    let dir_entries = fs::read_dir(dir_string).expect("Could not read directory.");

    // test
    default_display(dir_entries, false);
}

/// The default display that displays each entry of the directory line by line non-recursively.
fn default_display(dir: fs::ReadDir, show_hidden: bool) {
    for entry in dir {
        // unwrap entry to a result
        let entry_result = entry.expect("Cannot read entry.");
        let file_str = entry_result.file_name().to_str().unwrap().to_string();

        // determine if it should be shown if a dotfile
        if show_hidden {
            println!("{:?}", &file_str);
        } else {
            if !&file_str.starts_with('.') {
                println!("{:?}", file_str);
            }
        }
    }
}