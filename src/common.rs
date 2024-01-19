use std::{
    fs::{metadata, File},
    io::Read,
    os::unix::fs::MetadataExt,
};

use colored::Colorize;
use encoding_rs::{Encoding, UTF_8};
use regex::Regex;

fn can_read(path: String) -> (bool, String) {
    let mut result: (bool, String) = (false, String::new());
    let max_size = 1024 * 1024;
    let metadata = metadata(&path);

    if let Ok(meta) = metadata {
        if meta.size() < max_size {
            if let Ok(mut file) = File::open(path) {
                let mut buffer = Vec::new();

                match file.read_to_end(&mut buffer) {
                    Ok(_) => match try_decode(&buffer, UTF_8) {
                        Some(decoded_text) => result = (true, decoded_text.to_owned()),
                        None => result = (false, String::new()),
                    },
                    Err(_) => result = (false, String::new()),
                }
            } else {
                result = (false, String::new())
            }
        }
    };

    result
}

fn try_decode(buffer: &[u8], encoding: &'static Encoding) -> Option<String> {
    let (decoded, _, had_errors) = encoding.decode(buffer);
    if had_errors {
        None
    } else {
        Some(decoded.into_owned())
    }
}

pub fn matcher(item: String, query: &str) -> Vec<String> {
    let re = Regex::new(&query.to_lowercase()).unwrap();

    let mut ret: Vec<String> = Vec::new();
    let mut line_numb = 1;
    let reader = can_read(item).to_owned();

    if reader.0 {
        for line in reader.1.lines() {
            if re.is_match(&line.to_lowercase()) {
                ret.push(format!(
                    "[{}] {}",
                    line_numb.to_string().red(),
                    color_finding(line.to_string(), query.to_string())
                ));
            }
            line_numb += 1;
        }
    }

    ret
}

fn color_finding(line: String, word: String) -> String {
    if let Some(start) = line.find(&word) {
        let end = start + word.len();
        let before_word = &line[..start];
        let after_word = &line[end..];

        format!("{}{}{}", before_word, word.yellow(), after_word)
    } else {
        line
    }
}
