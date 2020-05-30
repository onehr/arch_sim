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
use std::thread;
mod misc;
use rvemu::bus::DRAM_BASE;
use rvemu::emulator::Emulator;

/// This function should be the entry point for the simulator.
fn main() -> std::io::Result<()> {

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        misc::print_usage();
        return Ok(());
    }

    misc::say_hello();
    let start = Instant::now();

    // now I plan to load the configuration from file, and configuration file should
    // be as simple as possible, to reduce the code to parse the configuration

    // read configuration file and set-up the simulator
    let config_file_name = args.get(1).unwrap();
    let config = misc::process_config_file(&config_file_name)?;
    let mut children = vec![];
    
    // create cpu cores based on the config file
    // TODO: but now they would just run individually
    for _thread_idx in 0..config.get_num_cpus() {
        // Spin up another thread
        children.push(thread::spawn(move || {

            // Create a dummy binary data.
            let data = vec![
                0x93, 0x0f, 0xa0, 0x02, // addi x31, x0, 42
            ];

            // Create an emulator object.
            let mut core = Emulator::new();
            // Place the binary data in the beginning of DRAM.
            core.set_dram(data);
            // Set the program counter to 0x8000_0000, which is the address DRAM starts.
            core.set_pc(DRAM_BASE);
            // Start the emulator.
            core.start();

            // `IllegalInstruction` is raised for now because of the termination condition of the emulator,
            // but the register is successfully updated.
            assert_eq!(42, core.cpu.xregs.read(31));

        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }    

    let duration = start.elapsed();
    println!("\n[arch_sim] shutdown after execution time: {:?}", duration);

    Ok(())
}
