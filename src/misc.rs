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

use serde::{Deserialize, Serialize};
use serde_json;
use std::{env, fmt};
use sys_info;
const PROJ: &'static str = env!("CARGO_PKG_NAME");
const NAME: &'static str = env!("CARGO_PKG_DESCRIPTION");
const VER: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const LICENSE: &'static str = "Apache License 2.0";

pub fn print_info() {
    eprintln!("{}(v {}): {}", PROJ, VER, NAME);
    eprintln!("Released under {}", LICENSE);
    eprintln!("Copyright 2020 {}", AUTHORS);
}

pub fn print_usage() {
    print_info();
    println!();
    println!("Usage:");
    println!("        cargo run -- [configuration_file]");
    println!();
}

pub fn say_hello() {
    print_info();
    eprintln!("\n[arch_sim] running on {}", sys_info::hostname().unwrap());
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigInfo {
    num_cpus: usize,
}

impl fmt::Display for ConfigInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "num_cores: {}\n", self.num_cpus)
    }
}

impl ConfigInfo {
    pub fn get_num_cpus(&self) -> usize {
        self.num_cpus
    }
}

/// process the configuration file, and extract information we need, and pass
/// those informations to the simulator
pub fn process_config_file(file_name: &String) -> std::io::Result<ConfigInfo> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    let file = File::open(&file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let config: ConfigInfo = serde_json::from_str(&contents)?;

    Ok(config)
}
