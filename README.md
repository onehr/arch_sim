# arch_sim
Computer architecture simulator.

# why new one?
* Using architecture simulator is pretty painful (based on my prior experience).
* No legacy code, easier to read and contribute.
* No data race, means it's eaiser to make the simulator runs in parallel. (architecture simulator
is always too slow to simulate multicore architecture, because lack of parallelism in their code-base)
* No need to compile your cross-compiler every-time (implement the workloads in rust and add target for it)

# Goal
Full system simulator. Easy to interact, easy to define your own computer architecture, 
able to run kernel and workloads.

- [ ] risc-v

# Workflow
## Getting the sources
This repository uses submodules. Clone with --recursive option.

``` sh
$ git clone --recursive https://github.com/onehr/arch_sim.git
```

## Run

``` sh
# currently using json file as the configuration format
$ cargo run -- arch_config.json
```

# Copyright
Copyright (c) 2020, Haoran Wang
