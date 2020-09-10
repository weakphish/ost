use std::fs;
use structopt::StructOpt;

fn main() {
    // get args
    let args = ArgumentData::from_args();
    let path_string = args.path.as_path().display().to_string();

    // collect the directory entries as ReadDir objects
    let dir_entries = fs::read_dir(path_string).expect("Could not read directory.");

    // test
    default_display(dir_entries, false);
}

// Command line argument structs
#[derive(StructOpt, Debug)]
#[structopt(name = "ost")]
struct ArgumentData {
    /// Files to process
    #[structopt(parse(from_os_str), default_value=".")]
    path: std::path::PathBuf,

    /// Show hidden (dot) files
    #[structopt(short="a")]
    hidden: bool,    
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
