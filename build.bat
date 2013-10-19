@echo off
rustc --version
cd src
rustc Rylai.rs -O -o ../Rylai.exe

::handle error messages
pause