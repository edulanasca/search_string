use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::utils::print_thread_id;

pub fn parallel_file_search(dir: PathBuf, search_string: String) -> Receiver<Option<PathBuf>> {
    let (tx, rx) = channel();

    thread::spawn(move || {
        print_thread_id(&thread::current());
        search_files_in_dir(dir, search_string, tx);
    });

    rx
}

fn search_files_in_dir(dir: PathBuf, search_string: String, tx: Sender<Option<PathBuf>>) {
    let mut results = Vec::new();

    match read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_dir() {
                        let dir = path.clone();
                        let search_str = search_string.clone();
                        let tx = tx.clone();
                        thread::spawn(move || {
                            print_thread_id(&thread::current());
                            search_files_in_dir(dir, search_str, tx)
                        });
                    } else {
                        results.push(search_file(path, search_string.clone()))
                    }
                }
            }
        }
        Err(e) => println!("Couldn't read dir: {}", e)
    }

    for result in results {
        tx.send(result).unwrap();
    }
}

fn search_file(path: PathBuf, search_string: String) -> Option<PathBuf> {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            println!("Couldn't open file: {}", e);
            None
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.to_lowercase().contains(&search_string) {
                return Some(path);
            }
        }
    }

    None
}