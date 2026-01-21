# Dir and pattern for backups of old source files
BACKUP_DIR:='src/old'
BACKUP_UNNAMED_DIR:='src/old/unnamed'
BACKUP_FILE_PAT:='main_XXX.rs'
TEXT_TESTS_DIR:='input'
CONFIG_FILE:='contest_config'
CONFIG_SCRIPT:='contest_config.sh'
GET_TEMPLATE:="bash" + " " + CONFIG_SCRIPT + " " + CONFIG_FILE + " " + "--get-template"

alias r := run
alias i := irun
alias re := reinit
alias c := clippy

_default: run

# Runs `main.cpp` against the given num.in text test file
cp num='0': 
	g++ -Wall -Wextra -g -std=gnu++17 src/main.cpp -o main
	./main < {{TEXT_TESTS_DIR}}/{{num}}.in > {{TEXT_TESTS_DIR}}/tmp.out
	@diff -Z --strip-trailing-cr {{TEXT_TESTS_DIR}}/tmp.out {{TEXT_TESTS_DIR}}/{{num}}.out && echo "OK!"

# Runs `main.rs` against the given num.in text test file
run num='0': build
	cargo run < {{TEXT_TESTS_DIR}}/{{num}}.in > {{TEXT_TESTS_DIR}}/tmp.out
	@diff -Z {{TEXT_TESTS_DIR}}/tmp.out {{TEXT_TESTS_DIR}}/{{num}}.out && echo "OK!"

# Run in interactive mode
irun: build
	cargo run | tee {{TEXT_TESTS_DIR}}/tmp.out

# Saves current `main.rs` into BACKUP_DIR and inits a clean `main.rs`
reinit name: (save name) init

# Saves current `main.rs` into BACKUP_DIR. Use reinit
save name:
	@mkdir -p "{{BACKUP_DIR}}"
	mv "src/main.rs" "{{BACKUP_DIR}}/main_{{name}}.rs"

# Configure project to compete on the given site. Supported values: [cforces, atcoder, leetcode]
config configuration='':
	bash {{CONFIG_SCRIPT}} {{CONFIG_FILE}} {{configuration}}

# Restores `main.rs` from
restore name: save_unnamed
	mv "{{BACKUP_DIR}}/main_{{name}}.rs" "src/main.rs"

# Resets `main.rs` and input/output tests. Prefer to use `reinit`
init: save_unnamed clean_tests
	cp $({{GET_TEMPLATE}}) "src/main.rs"
	@mkdir -p "{{TEXT_TESTS_DIR}}"
	bash -c "touch {{TEXT_TESTS_DIR}}/{0,1,2,3}.{in,out} {{TEXT_TESTS_DIR}}/tmp.out"

# Saves `main.rs` into a directory for old unnamed solutions
save_unnamed:
	@mkdir -p "{{BACKUP_UNNAMED_DIR}}"
	[ -f src/main.rs ] && mv "src/main.rs" $(mktemp {{BACKUP_UNNAMED_DIR}}/{{BACKUP_FILE_PAT}}) || true


# Cleans trash files
clean: clean_tests
	cargo clean
	rm -rf {{BACKUP_UNNAMED_DIR}}

clean_tests:
	bash -c "rm -f {{TEXT_TESTS_DIR}}/*.{in,out}"

# Cleans trash files and old solutions
clean_old: clean clean_tests
	rm -rf {{BACKUP_DIR}}/*

build:
	cargo build

update_lib:
	cp "src/main.rs" $({{GET_TEMPLATE}})

# # Updates files `src/template.rs` and `contest-lib/src/input.rs` from the current `main.rs`
# NOTE: OUT OF DATE
# update_lib:
# 	#!/usr/bin/env bash
# 	set -euo pipefail
# 	main_rs="src/main.rs"
# 	template_rs=$({{GET_TEMPLATE}})
# 	input_lib_rs={{CONTEST_LIB}}/src/input.rs
# 	lib_line_num=$(grep -n "/\* Library \*/" $main_rs | cut -d: -f1)
# 	head --lines=$(($lib_line_num)) $main_rs > $template_rs && echo "Updated $template_rs"
# 	tail --lines=+$(($lib_line_num + 2)) $main_rs > $input_lib_rs && echo "Updated $input_lib_rs"

clippy:
	cargo clippy

clippy_lib:
	cargo clippy --lib

libtest: clippy
	cargo test

py num='0':
	python3 src/main.py < input/{{num}}.in

ipy:
	python3 -i src/main.py

gen num='2':
	python3 src/gen.py > input/{{num}}.in
