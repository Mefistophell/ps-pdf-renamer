use std::error::Error;
use pdf::file::File;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

mod file_object;
mod config;
mod primitive_parser;

pub use crate::file_object::FileObject;
pub use crate::primitive_parser::{PrimitiveParser};
pub use crate::config::{DIR_NAME, FILE_EXTENSION};

fn main() {
    println!("reading directory: {}", &DIR_NAME);
    let dirs = fs::read_dir(Path::new(&DIR_NAME)).unwrap();

    for dir in dirs {
        if let Some(file_name) = dir.unwrap().file_name().to_str() {
            let file_path = FileObject::new(&DIR_NAME, file_name, None)
                .get_file_path();

            if let Some(file_extension) = file_path.extension() {
                if file_extension != FILE_EXTENSION { continue; }
            }

            println!("opening file: {:?}", &file_path);

            match get_new_file_path(&file_path) {
                Ok(new_file_path) => {
                    rename(&file_path, &new_file_path);
                }
                Err(_) => {
                    println!("skipping the file: {:?}", &file_path);
                }
            }
        }
    }
}

fn get_new_file_path(file_path: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let file: File<Vec<u8>> = File::open(&file_path)?;
    if let Some(new_file_name) = get_new_file_name(&file) {
        let new_file_path = FileObject::new(DIR_NAME, &new_file_name, Some(&FILE_EXTENSION))
            .get_file_path();
        Ok(new_file_path)
    } else {
        Err("Unsatisfied file type".into())
    }
}

fn rename(old_file_path: &PathBuf, new_file_path: &PathBuf) {
    fs::rename(old_file_path, new_file_path).unwrap();
    println!("File {:?} has been renamed to {:?}", old_file_path, new_file_path);
}

fn get_new_file_name(file: &File<Vec<u8>>) -> Option<String> {
    for page in file.pages() {
        let page = page.unwrap();

        if let Some(content) = &page.contents {
            let string_iter = PrimitiveParser::parse(&content);

            for string in string_iter {
                let reg_ex = Regex::new(r"@boozt.com").unwrap();
                if reg_ex.is_match(&string) {
                    return Some(string);
                }
            }
        }
    }
    None
}
