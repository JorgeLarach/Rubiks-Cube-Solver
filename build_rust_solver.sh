#!/bin/sh
set -e

echo "[Rust] Building cube_solver..."

cd cube_solver

/Users/jorgelarach/.cargo/bin/cargo rustc \
--target thumbv7em-none-eabihf \
--release \
--no-default-features \
--crate-type staticlib

echo "[Rust] Copying static library..."

cp target/thumbv7em-none-eabihf/release/libcube_solver.a \
../Middlewares/cube_solver/libcube_solver.a

echo "[Rust] Cleaning Cargo..."

/Users/jorgelarach/.cargo/bin/cargo clean

echo "[Rust] Done."

