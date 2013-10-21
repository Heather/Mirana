@echo off
rustc --version
cd src
rustc Mirana.rs -O -o ../mirana.exe

::handle error messages
pause