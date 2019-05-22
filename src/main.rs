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
//! src/main.rs:
//! This is file contains the entry function for simulator.
//! The first thing we need to do is try to build the high level abstraction for
//! simulation. so the basic operation for the simulator is to setup the
//! callbackQ and iterate through the callbackq, and let it startup

use std::time::Instant;
mod misc;

/// This function should be the entry point for the simulator.
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        misc::print_usage();
        return Ok(());
    }

    let start = Instant::now();
    misc::say_hello();

    // now I plan to load the configuration from file, and configuration file should
    // be as simple as possible, to reduce the code to parse the configuration

    // read configuration file and set-up the simulator
    let config_file_name = args.get(1).unwrap();
    misc::process_config_file(&config_file_name)?;

    // start the simulator
    eprintln!("Entering event queue.  Starting simulation...");

    let duration = start.elapsed();
    println!("\n[arch_sim] shutdown after execution time: {:?}", duration);
    return Ok(());
}
