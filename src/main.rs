use std::fs;
use fs::DirEntry;
use structopt::StructOpt;

fn main() {
    // get args
    let args = ArgumentData::from_args();
    let path_string = args.path.as_path().display().to_string();

    // collect entries into a DirEntry vector and sort if applicable
    let mut entries = collect_dir(path_string);
    if !args.no_sort {
        entries.sort_by(|a, b| a.path().cmp(&b.path()));
    }

    // test
    default_display(entries, args.hidden)
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
    
    /// Show extra information
    #[structopt(short="l")]
    long: bool,

    /// No sort
    #[structopt(short="f")]
    no_sort: bool
}

/// Collect entries from a directory indicated by a string and return a vector of DirEntry
fn collect_dir(path: String) -> Vec<DirEntry> {
    // collect the directory entries as ReadDir objects and then as DirEntry into a vec
    let reader = fs::read_dir(path).expect("Could not read directory.");
    let mut dir_entries: Vec<DirEntry> = Vec::new();
    for entry in reader {
        // unwrap entry to a result
        let entry_result = entry.expect("Cannot read entry.");
        dir_entries.push(entry_result);
    }

    return dir_entries;
}

/// The default display that displays each entry of the directory line by line non-recursively.
fn default_display(entries: Vec<DirEntry>, show_hidden: bool) {
    for entry in entries {
        let entry_string: String = entry.path().file_name().unwrap().to_string_lossy().into_owned();
        
        if show_hidden {
            println!("{}", entry_string);
        } else {
            if !entry_string.starts_with(".") {
                println!("{}", entry_string);
            }
        }
    }
}
