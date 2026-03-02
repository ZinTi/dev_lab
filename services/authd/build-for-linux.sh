#!/usr/bin/env bash
# 1. Build dependencies
# cmake -S . -B "target/cmake_build" -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=Release -DCMAKE_C_COMPILER=gcc -DCMAKE_CXX_COMPILER=g++
# cmake --build "target/cmake_build" --parallel ${NUMBER_OF_PROCESSORS}
# rm -f test1-v*/bin/*

# 2. Build project
cargo build --release --verbose

# 3. Move executable to target directory
mv target/release/authd authd-v*/bin/
mv target/release/*.rlib authd-v*/bin/
ls -al .
cp -R config/ authd-v*/      # configuration examples
cp -R docs/ authd-v*/
