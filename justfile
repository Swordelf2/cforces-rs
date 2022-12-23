# Dir and pattern for backups of old source files
BACKUP_DIR:='src/old'
BACKUP_UNNAMED_DIR:='src/old/unnamed'
BACKUP_FILE_PAT:='main_XXX.rs'
TEXT_TESTS_DIR:='input'
CONTEST_LIB:='contest-lib'

alias r := run
alias re := reinit

# Runs `main.rs` against the given num.in text test file
run num='0': build
	cargo run < {{TEXT_TESTS_DIR}}/{{num}}.in > {{TEXT_TESTS_DIR}}/tmp.out
	@diff -Z {{TEXT_TESTS_DIR}}/tmp.out {{TEXT_TESTS_DIR}}/{{num}}.out && echo "OK!"

# Saves current `main.rs` into BACKUP_DIR and inits a clean `main.rs`
reinit name: (save name) init

# Saves current `main.rs` into BACKUP_DIR. Use reinit
save name:
	@mkdir -p "{{BACKUP_DIR}}"
	mv "src/main.rs" "{{BACKUP_DIR}}/main_{{name}}.rs"

# Resets `main.rs`. Prefer to use 
init: clean_tests
	@mkdir -p "{{BACKUP_UNNAMED_DIR}}"
	[ -f src/main.rs ] && mv src/main.rs `mktemp {{BACKUP_UNNAMED_DIR}}/{{BACKUP_FILE_PAT}}` || true
	cp src/template.rs src/main.rs
	echo "" >> src/main.rs
	cat contest-lib/src/input.rs >> src/main.rs
	@mkdir -p "{{TEXT_TESTS_DIR}}"
	bash -c "touch {{TEXT_TESTS_DIR}}/{0,1}.{in,out} {{TEXT_TESTS_DIR}}/tmp.out"

# Cleans trash files
clean:
	cargo clean
	rm -rf {{BACKUP_UNNAMED_DIR}}

# Cleans trash files and old solutions
clean_old: clean clean_tests
	rm -rf {{BACKUP_DIR}}/*

clean_tests:
	bash -c "rm -f {{TEXT_TESTS_DIR}}/*.{in,out}"

build:
	cargo build

# Updates files `src/template.rs` and `contest-lib/src/input.rs` from the current `main.rs`
update_lib:
	#!/usr/bin/env bash
	set -euo pipefail
	main_rs="src/main.rs"
	template_rs="src/template.rs"
	input_lib_rs={{CONTEST_LIB}}/src/input.rs
	lib_line_num=$(grep -n "/\* Library \*/" $main_rs | cut -d: -f1)
	head --lines=$(($lib_line_num)) $main_rs > $template_rs && echo "Updated $template_rs"
	tail --lines=+$(($lib_line_num + 2)) $main_rs > $input_lib_rs && echo "Updated $input_lib_rs"