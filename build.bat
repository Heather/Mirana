@echo off
rustc --version

::clean
rm -rf bin
mkdir bin

::compile
cd src/Mirana

:: Release ->
::rustc main.rs -O -o ../../bin/Mirana.exe
:: Debig ->
rustc main.rs -Z debug-info -o ../../bin/Mirana.exe

::wait / handle errors
pause