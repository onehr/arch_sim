//     Copyright 2019 Haoran Wang
//
//     Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
//     You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
//     distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//     See the License for the specific language governing permissions and
//     limitations under the License.

//! src/misc.rs:
//! this is file for utility functions that are needed by the simulator entry
//! function.

use sys_info;
const PROJ: &'static str = env!("CARGO_PKG_NAME");
const NAME: &'static str = env!("CARGO_PKG_DESCRIPTION");
const VER: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const LICENSE: &'static str = "Apache License 2.0";

pub fn print_info() {
    eprintln!("{}(v {}): {}", PROJ, VER, NAME);
    eprintln!("Released under {}", LICENSE);
    eprintln!("Copyright 2019 {}", AUTHORS);
}

pub fn print_usage() {
    print_info();
    println!();
    println!("Usage:");
    println!("        cargo run -q -- [configuration_file]");
    println!();
}

pub fn say_hello() {
    print_info();
    eprintln!("\n[arch_sim] running on {}", sys_info::hostname().unwrap());
}

fn remove_comment(input: String) -> std::io::Result<String> {
    fn in_comment(single: bool) -> bool {
        return single;
    }
    let mut res = String::new();

    let mut idx = 0;
    let mut single_line_in_comment = false;
    while idx < input.len() {
        if idx == input.len() - 1 {
            if single_line_in_comment {
                break;
            } else {
                res.push(char::from(input.as_bytes()[idx]));
                break;
            }
        }
        let b1 = char::from(input.as_bytes()[idx]);
        let b2 = char::from(input.as_bytes()[idx + 1]);

        let mut combine = String::new();
        combine.push(b1);
        combine.push(b2);
        match combine.as_ref() {
            "//" => {
                if !in_comment(single_line_in_comment) {
                    single_line_in_comment = true;
                }
                idx = idx + 2;
            }
            _ => {
                if b1 == '\n' && single_line_in_comment {
                    single_line_in_comment = false;
                    idx = idx + 1;
                } else {
                    if in_comment(single_line_in_comment) {
                        idx = idx + 1;
                    } else {
                        res.push(b1);
                        idx = idx + 1;
                    }
                }
            }
        }
    }
    return Ok(res);
}

/// process the configuration file, and extract information we need, and pass
/// those informations to the simulator
pub fn process_config_file(file_name: &String) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    let file = File::open(&file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let contents = remove_comment(contents)?;

    // iterate through every line of the configuration file, and extract the
    // information
    eprintln!("[configuration]:");
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        let line_arg: Vec<&str> = line.split_whitespace().collect();
        let var = line_arg.get(0).unwrap();
        let val = line_arg.get(1).unwrap();
        eprintln!("{}: {}", var, val);
    }
    Ok(())
}
