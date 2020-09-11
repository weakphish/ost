use std::fs;
use fs::DirEntry;
use structopt::StructOpt;

fn main() {
    // get args
    let args = ArgumentData::from_args();
    let path_string = args.path.as_path().display().to_string();

    let entries = collect_dir(path_string);
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
    
}
