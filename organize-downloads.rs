use std::env;
use std::fs;
use std::path::{Path};

fn main() {
    // Get the download folder path
    let download_folder = match env::var("HOME") {
        Ok(home_dir) => Path::new(&home_dir).join("Downloads"),
        Err(_) => {
            println!("Unable to get the download folder path.");
            return;
        }
    };

    // Iterate through the files in the download folder
    match fs::read_dir(&download_folder) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        // Get the file extension
                        let extension = match path.extension() {
                            Some(ext) => ext.to_string_lossy().to_string(),
                            None => continue, // Skip files without extensions
                        };

                        // Create the destination folder path
                        let dest_folder = download_folder.join(&extension);

                        // Create the destination folder if it doesn't exist
                        if !dest_folder.exists() {
                            fs::create_dir(&dest_folder).unwrap_or_else(|_| {
                                println!("Unable to create destination folder: {:?}", &dest_folder);
                                return;
                            });
                        }

                        // Move the file to the destination folder
                        let dest_path = dest_folder.join(path.file_name().unwrap());
                        match fs::rename(&path, &dest_path) {
                            Ok(_) => {
                                println!("Moved {:?} to {:?}", &path, &dest_path);
                            }
                            Err(_) => {
                                println!("Unable to move {:?} to {:?}", &path, &dest_path);
                            }
                        }
                    }
                }
            }
        }
        Err(_) => {
            println!("Unable to read the download folder.");
            return;
        }
    }
}
