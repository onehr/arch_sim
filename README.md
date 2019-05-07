# arch_sim
Computer architecture simulator.

# why new one?
* Using architecture simulator is pretty painful (based on my prior experience).
* No legacy code, easier to read and contribute.
* No data race, means it's eaiser to make the simulator runs in parallel. (architecture simulator is always too slow to 
simulate multicore architecture, because lack of parallelism in their code-base)
* No need to compile your cross-compiler every-time (implement the workloads in rust and add target for it)

# Goal
Full system simulator. Easy to interact, easy to define your own computer architecture, able to run kernel and workloads.

# Copyright
Copyright (c) 2019, Haoran Wang
