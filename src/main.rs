use fs::DirEntry;
use std::error::Error;
use std::fs;
use std::os::unix::fs::PermissionsExt;

use structopt::StructOpt;

fn main() {
    // get args
    let args = ArgumentData::from_args();
    let path_string = args.path.as_path().display().to_string();

    // collect entries into a DirEntry vector and sort if applicable
    let mut entries = match collect_dir(path_string) {
        Err(e) => vec![], // FIXME just make it an empty list if the directory isn't found
        Ok(entries) => entries,
    };
    if !args.no_sort {
        entries.sort_by(|a, b| a.path().cmp(&b.path()));
    }

    // Display output
    match display_directory(entries, args.hidden, args.long) {
        Err(e) => println!("{:?}", e.to_string()),
        Ok(()) => (),
    }
}

// Command line argument structs
#[derive(StructOpt, Debug)]
#[structopt(name = "ost")]
struct ArgumentData {
    /// Files to process
    #[structopt(parse(from_os_str), default_value = ".")]
    path: std::path::PathBuf,

    /// Show hidden (dot) files
    #[structopt(short = "a")]
    hidden: bool,

    /// Show extra information
    #[structopt(short = "l")]
    long: bool,

    /// Don't sort the output of the search
    #[structopt(short = "f")]
    no_sort: bool,
}

/// Collect entries from a directory indicated by a string and return a vector of DirEntry
fn collect_dir(path: String) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    // collect the directory entries as ReadDir objects and then as DirEntry into a vec
    let reader = fs::read_dir(path)?;
    let mut dir_entries: Vec<DirEntry> = Vec::new();
    for entry in reader {
        // unwrap entry to a result
        dir_entries.push(entry?);
    }
    Ok(dir_entries)
}

/// The default display that displays each entry of the directory line by line non-recursively.
fn display_directory(
    entries: Vec<DirEntry>,
    show_hidden: bool,
    show_detailed: bool,
) -> Result<(), Box<dyn Error>> {
    for entry in entries {
        let entry_string: String = entry.path().file_name().unwrap().to_string_lossy().into_owned();
        let entry_metadata = entry.metadata()?;

        if show_detailed {
            // TODO: figure out how to pretty print read permissions
            println!("{:o}", entry_metadata.permissions().mode());
        } else {
            // do things
        }
    }
    Ok(())
}
