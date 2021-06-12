# Get help
## Get help for cargo test
cargo test --help
## Get help for binary test file
cargo test -- --help

## Single thread test
cargo test -- --test-threads=1

## Show stdout from passed tests
cargo test -- --nocapture

## Test specific testcase
cargo test add_for_100

## Test part of testcases
cargo test add

## Test only ignored
cargo test -- --ignored

## A long param demo
cargo test \
  --color=always \
  --package run-tests \
  --lib \
  tests::add_for_100 \
  --no-fail-fast \
  -- \
  --format=json\
   --exact \
   -Z unstable-options \
   --show-output
