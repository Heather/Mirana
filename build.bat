@echo off
cd src
rust build Rylai.rs -O -o ../Rylai.exe

::handle error messages
pause