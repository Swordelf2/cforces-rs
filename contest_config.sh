#!/usr/bin/env bash

# Usage: ./script.sh CONFIG_FILE configuration
# Where configuration is one of
# * cforces
# * atcoder
# * leetcode
set -euo pipefail

CFORCES_TEMPLATE="src/template.rs"
ATCODER_TEMPLATE="src/template.rs"
LEETCODE_TEMPLATE="src/template_leetcode.rs"

if [ $# -lt 1 ]; then
    echo "error: incorrect number of args passed - $#" 1>&2
    exit 1
fi
if [ ! -e 'src/template.rs' ]; then
    echo "error: could not find `src/template.rs`, CWD could be incorrect -" $(pwd) 1>&2
    exit 1
fi

config_file=$1
touch $config_file
touch rust-toolchain
cur_configuration=$(cat $config_file)

if [ $# == 1 ]; then
    # Just print current configuration
    echo $cur_configuration
    exit
fi

if [ $2 == "--get-template" ]; then
    # Print template file for current configuration
    if [ $cur_configuration == "cforces" ]; then
        echo $CFORCES_TEMPLATE
    elif [ $cur_configuration == "atcoder" ]; then
        echo $ATCODER_TEMPLATE
    elif [ $cur_configuration == "leetcode" ]; then
        echo $LEETCODE_TEMPLATE
    else
        echo "error: current configuration is incorrect - $cur_configuration" 1>&2
        exit 1
    fi
    exit
fi

new_configuration=$2

if [ ! -e rust-toolchain ]; then
    echo "error: could not find rust-toolchain file" 1>&2
fi

if [ $new_configuration == "cforces" ]; then
    echo "stable" > rust-toolchain
elif [ $new_configuration == "atcoder" ]; then
    echo "1.42" > rust-toolchain
elif [ $new_configuration == "leetcode" ]; then
    echo "1.58" > rust-toolchain
else
    echo "error: incorrect configuration passed - $new_configuration" 1>&2
    exit 1
fi

echo $new_configuration > $config_file
    