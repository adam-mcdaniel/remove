use clap::{clap_app, crate_version, crate_authors, AppSettings};
// use std::fs::{rename, remove_dir_all, create_dir_all};
use std::path::PathBuf;
use std::process::exit;

/// The name of the trash folder
const TRASH: &str = ".trash";


/// Returns the absolute path to the trash folder
fn trash_dir() -> PathBuf {
    match dirs::home_dir() {
        Some(path) => path.join(TRASH),
        None => {
            eprintln!("Oh no, I could not discern your home directory!");
            exit(1);
        }
    }
}

/// Makes a directory and its parents
/// Exits with error code 1 on failure
fn mkdir(dir: PathBuf) {
    match std::fs::create_dir_all(dir.clone()) {
        Err(_) => {
            println!("Failed to create folder {:?}", dir);
            exit(1);
        },
        _ => {}
    };
}

/// Removes a directory and its contents
/// Exits with error code 1 on failure
fn rmdir(dir: PathBuf) {
    match std::fs::remove_dir_all(dir.clone()) {
        Err(_) => {
            println!("Failed to remove folder {:?}", dir);
            exit(1);
        },
        _ => {}
    };
}

/// Makes a directory and its parents
/// Exits with error code 1 on failure
fn rename(from: PathBuf, to: PathBuf) {
    match std::fs::rename(from.clone(), to.clone()) {
        Err(_) => {
            println!("Failed to move {:?} to {:?}", from, to);
            exit(1);
        },
        _ => {}
    };
}


/// Represents a file on disk
enum File {
    File(PathBuf),
    Directory(PathBuf),
    Nothing(PathBuf),
}


trait Remove {
    fn remove(&self) -> Result<(), String>;
}

impl Remove for File {
    fn remove(&self) -> Result<(), String> {
        match self {
            File::File(f) => {
                let name = f.file_name().unwrap().to_str().unwrap();
                rename(f.clone(), trash_dir().join(name));
                Ok(())
            },
            File::Directory(dir) => {
                let name = dir.file_name().unwrap().to_str().unwrap();
                rename(dir.clone(), trash_dir().join(name));
                Ok(())
            },
            File::Nothing(f) => Err(format!("Could not find file {:?}", f)),
        }
    }
}

/// Convert a ToString impl to a File instance
impl<T: ToString> From<T> for File {
    fn from(file: T) -> File {
        let path = PathBuf::from(&file.to_string());
        if path.is_dir() {
            File::Directory(path)
        } else if path.is_file() {
            File::File(path)
        } else {
            File::Nothing(path)
        }
    }
}



fn main() {
    // The absolute path to the trash folder
    let trash = trash_dir();
    // Make trash folder
    mkdir(trash.clone());

    let matches = clap_app!(rusty_ci =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "A cross platform, safe alternative to rm")
        // Accept multiple files as input
        (@arg FILES: +takes_value ...)
        // Empty the trash when done
        (@arg EMPTY: -e --empty "Empty trash")
    ).setting(AppSettings::ArgRequiredElseHelp).get_matches();


    if let Some(paths) = matches.values_of("FILES") {
        for path in paths {
            match File::from(path).remove() {
                Ok(_) => println!("Removed \"{}\"", path),
                Err(e) => eprintln!("Error: {}", e)
            };
        }
    }
    
    if matches.is_present("EMPTY") {
        rmdir(trash.clone());
        mkdir(trash);
        println!("Emptied trash");
    }
}