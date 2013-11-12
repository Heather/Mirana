@echo off
::setlocal
rustc --version
cd src/Mirana

rustc main.rs -O -o ../../Mirana.exe

:: <--------------- rustpkg is buggy
::mkdir .tmp
::set TMPDIR=.tmp
::rustpkg build Mirana

::handle error messages
::endlocal
pause