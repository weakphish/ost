use std::fs;
use std::env;

fn main() {
    // get directory argument from the input
    let dir = env::args().nth(1).expect("Missing directory argument");
    // collect the directory entries as ReadDir objects
    let dir_entries = fs::read_dir(dir).expect("Could not read directory.");

    // test
    default_display(dir_entries);
}

/// The default display that displays each entry of the directory line by line non-recursively.
fn default_display(dir: fs::ReadDir) {
    for entry in dir {
        println!("{:?}", entry);
    }
}