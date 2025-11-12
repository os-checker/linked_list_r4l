#!/bin/bash

set -eux

miri() {
  cargo miri test "$1" 2>&1 | tee tests/miri-snapshots/sb-"$1".txt
  MIRIFLAGS=-Zmiri-tree-borrows cargo miri test "$1" 2>&1 | tee tests/miri-snapshots/tb-"$1".txt
}

testcases=("pop_front_unsoundness" "cursor_mut_unsoundness" "mutable_arc" "test_push_back")

for testcase in "${testcases[@]}"; do
  miri "$testcase"
done
