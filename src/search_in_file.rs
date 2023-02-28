use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::fd::AsRawFd;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use libc::size_t;

use crate::utils::{calculate_duration, print_error};

/// Search the given ```search_string``` in the file specified by a ```path``` using Rust's [`BufReader`].
///
/// Returns an [`Option`] with the vector containing the line number and paragraph where the
/// ```search_string``` was found
pub fn search_in_file(path: &PathBuf, search_string: String) -> Option<Vec<(usize, String)>> {
    let start_date = SystemTime::now();
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Couldn't open file: {}", e);
            return None;
        }
    };

    let reader = BufReader::new(file);
    let mut results = Vec::new();
    let mut line_number = 1;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.to_lowercase().trim().contains(&search_string) {
                    results.push((line_number as usize, line));
                }
            }
            Err(e) => {
                print_error("Error while reading line", &e);
                line_number += 1;
                continue;
            }
        }

        line_number += 1;
    }
    let end_date = SystemTime::now();

    print_time_elapsed(&results, path, calculate_duration(start_date, end_date));

    Some(results)
}

/// Search the given ```search_string``` in the file specified by a ```path```
///
/// **⚠️Warning:** This implementation uses `unsafe` rust with [`libc::mmap`].
///
/// Returns an [`Option`] with the vector containing the line number and paragraph where the
/// ```search_string``` was found
pub unsafe fn search_in_file_unsafe(path: &PathBuf, search_string: String) -> Option<Vec<(usize, String)>> {
    let start_date = SystemTime::now();

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            print_error("Couldn't open file", &e);
            return None;
        }
    };

    let size;

    match file.metadata() {
        Ok(metadata) => {
            size = metadata.len() as size_t;
        }
        Err(e) => {
            print_error("Couldn't read file metadata {}", &e);
            return None;
        }
    }

    // Memory map the file for faster access https://man7.org/linux/man-pages/man2/mmap.2.html
    let ptr = libc::mmap(
        std::ptr::null_mut(),
        size,
        libc::PROT_READ, // describes the desired memory protection of the mapping - Pages may be read
        libc::MAP_PRIVATE, // Updates to the mapping are not visible to other processes mapping the same file
        file.as_raw_fd(),
        0,
    );

    if ptr == libc::MAP_FAILED {
        println!("Failed to map the file into memory");
        return None;
    }

    let mut results = Vec::new();
    let mut line_number = 1;
    let mut line_start = ptr as *const u8;

    for i in 0..size {
        if *(ptr.offset(i as isize) as *const u8) == b'\n' {
            let line_end: usize = ptr.offset(i as isize) as usize - line_start as usize;
            let line: &[u8] = unsafe { std::slice::from_raw_parts(line_start, line_end) };
            let line_str = String::from_utf8_lossy(line).to_string();

            if line_str.to_lowercase().trim().contains(&search_string) {
                results.push((line_number, line_str));
            }

            line_number += 1;
            line_start = ptr.offset(i as isize + 1) as *const u8;
        }
    }

    libc::munmap(ptr, size);
    let end_date = SystemTime::now();

    print_time_elapsed(&results, path, calculate_duration(start_date, end_date));

    Some(results)
}

pub fn print_time_elapsed(results: &Vec<(usize, String)>, path: &PathBuf, duration: Duration) {
    if results.len() > 0 {
        println!("File: {:?} Time elapsed: {:?}", path.file_name().unwrap(), duration);
    }
}