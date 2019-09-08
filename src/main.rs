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
fn mkdir(dir: PathBuf) -> Result<(), String> {
    if let Err(_) = std::fs::create_dir_all(dir.clone()) {
        Err(format!("Failed to create folder {:?}", dir))
    } else {
        Ok(())
    }
}

/// Removes a directory and its contents
/// Exits with error code 1 on failure
fn rmdir(dir: PathBuf) -> Result<(), String> {
    if let Err(_) = std::fs::remove_dir_all(dir.clone()) {
        Err(format!("Failed to remove folder {:?}", dir))
    } else {
        Ok(())
    }
}

/// Makes a directory and its parents
/// Exits with error code 1 on failure
fn rename(from: PathBuf, to: PathBuf) -> Result<(), String> {
    if let Err(_) = std::fs::rename(from.clone(), to.clone()) {
        Err(format!("Failed to move {:?} to {:?}", from, to))
    } else {
        Ok(())
    }
}


/// Represents a file on disk
enum File {
    File(PathBuf),      // A singular file
    Directory(PathBuf), // A directory on disk
    Nothing(PathBuf)    // Basically an invalid path
}


trait Remove {
    fn remove(&self) -> Result<(), String>;
}

impl Remove for File {
    fn remove(&self) -> Result<(), String> {
        match self {
            File::File(f) => {
                // The path to the hypothetical trashed file: $HOME/.trash/{f}
                let trashed_file = trash_dir().join(f.file_name().unwrap().to_str().unwrap());

                // Remove file/directory that this file would replace anyways
                match rmdir(trashed_file.clone()) { _ => {} };

                // Move to trash
                rename(f.clone(), trashed_file)?;

                Ok(())
            },
            File::Directory(dir) => {
                // The path to the hypothetical trashed directory: $HOME/.trash/{dir}
                let trashed_dir = trash_dir().join(dir.file_name().unwrap().to_str().unwrap());

                // Remove file/directory that this file would replace anyways
                match rmdir(trashed_dir.clone()) { _ => {} };

                // Move dir to trash
                rename(dir.clone(), trashed_dir)?;
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
    if let Err(e) = mkdir(trash.clone()) {
        println!("{}", e);
        exit(1);
    }

    let matches = clap_app!(remove =>
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
        if let Err(e) = rmdir(trash.clone()) {
            println!("{}", e);
            exit(1);
        }

        if let Err(e) = mkdir(trash) {
            println!("{}", e);
            exit(1);
        }

        println!("Emptied trash");
    }
}
