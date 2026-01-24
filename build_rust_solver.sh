#!/bin/sh
set -e

echo "[Rust] Building cube_solver..."

cd cube_solver

/Users/jorgelarach/.cargo/bin/cargo build \
--target thumbv7em-none-eabihf \
--release

echo "[Rust] Copying static library..."

cp target/thumbv7em-none-eabihf/release/libcube_solver.a \
../Middlewares/cube_solver/libcube_solver.a

echo "[Rust] Done."