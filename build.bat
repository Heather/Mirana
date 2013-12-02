@echo off
rustc --version

::clean
rm -rf bin
mkdir bin

::compile
cd src/Mirana
rustc main.rs -O -o ../../bin/Mirana.exe

::wait / handle errors
pause