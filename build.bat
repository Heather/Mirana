@echo off
rustc --version
cd src

rustc Mirana.rs -O -o ../Mirana.exe

:: trying to understand compilation process -------
:: rustc Mirana.rs -O --emit-llvm -o ../Mirana.bc
:: llc Mirana.bc -O3 -o mirana.s
:: gcc mirana.s -O3 -o mirana.exe
:: ------------------------------------------------

::handle error messages
pause