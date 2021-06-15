# Build document
cargo doc

# Build & open document
cargo doc --open

# Test examples in document
cargo test

# Publish
cargo publish

# Yank
cargo yank --vers 1.0.1

# Undo yank
cargo yank --vers 1.0.1 --undo
