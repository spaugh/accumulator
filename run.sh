#!/bin/sh

if ! command -v cargo &> /dev/null
then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    exit 1
fi

RED='\033[0;31m'
GREEN='\033[0;32m'
DEFAULT='\033[0m'

cargo build

pushd ./target/debug
URI=http://localhost:3000

./web &
PID=$!


function add {
    printf '%s' "Trying to add $1 to accumulator......."
    ./cli --uri $URI add $1 &>/dev/null && echo "${GREEN}Success${DEFAULT}" || echo "${RED}Failure${DEFAULT}" 
}

function verify {
    printf '%s' "Checking that $1 is at index $2........"
    ./cli --uri $URI verify $1 $2 &>/dev/null && echo "${GREEN}Success${DEFAULT}" || echo "${RED}Failure${DEFAULT}" 
}

function verify_false {
    printf '%s' "Checking that $1 is not at index $2...."
    ! ./cli --uri $URI verify $1 $2 &>/dev/null && echo "${GREEN}Success${DEFAULT}" || echo "${RED}Failure${DEFAULT}" 
}

add foo
add bar
add baz
add qux

verify "foo" 0
verify_false "baz" 0
verify_false "foo" 1

verify "baz" 2
verify_false "baz" 0
verify_false "baz" 1
verify_false "foo" 2
verify_false "baz" 3

kill $PID