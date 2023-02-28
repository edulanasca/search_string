use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::fs::{DirEntry, read_dir};
use structopt::StructOpt;
use rayon::prelude::*;

use crate::Cli;
use crate::utils::{print_error, print_thread_id};
use crate::search_in_file::*;

pub fn parallel_file_search(dir: PathBuf, search_string: String) -> Receiver<(String, Vec<(usize, String)>)> {
    println!("Running in {} mode...", Cli::from_args().mode);
    let (tx, rx) = channel();
    search_files_in_dir(dir, search_string, tx);

    rx
}

fn search_files_in_dir(dir: PathBuf, search_string: String, tx: Sender<(String, Vec<(usize, String)>)>) {
    let entries: Vec<DirEntry> = match read_dir(dir) {
        Ok(entries) => entries.filter_map(|entry| entry.ok()).collect(),
        Err(e) => {
            print_error("Couldn't read dir or file", &e);
            Vec::default()
        }
    };

    entries.par_iter().for_each_with(tx, |sender, entry| {
        let path = entry.path();

        if Cli::from_args().show_thread_id {
            print_thread_id(&thread::current());
        }

        if path.is_dir() {
            let dir = path.clone();
            let search_str = search_string.clone();
            let tx = sender.clone();

            search_files_in_dir(dir, search_str, tx);
        } else {
            if Cli::from_args().mode == "safe" {
                if let Some(result) = search_in_file(&path, search_string.clone()) {
                    //results = (String::from(path.to_str().unwrap()), result);
                    sender.send((String::from(path.to_str().unwrap()), result)).unwrap()
                }
            } else {
                if let Some(result) = unsafe { search_in_file_unsafe(&path, search_string.clone()) } {
                    //results = (String::from(path.to_str().unwrap()), result);
                    sender.send((String::from(path.to_str().unwrap()), result)).unwrap()
                }
            }
        }
    });
}
