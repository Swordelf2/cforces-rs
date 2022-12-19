alias r := run

run num='0': build
	cargo run < {{num}}.in

build:
	cargo build

test num='0':
	cargo run < {{num}}.in | tee tmp.out
	diff tmp.out {{num}}.out

