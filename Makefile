
# change it smaller if you want
# cause I use 12 cores cpu, so I set it to my core number 12.
build_jobs=12

build:
	cargo build --jobs $(build_jobs)
run:
	cargo run -q -- arch_config.template
