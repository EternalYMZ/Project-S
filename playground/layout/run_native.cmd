@echo off
setlocal
cd /d "%~dp0"
if not defined VCVARS64 set "VCVARS64=C:\Program Files\Microsoft Visual Studio\18\Community\VC\Auxiliary\Build\vcvars64.bat"
call "%VCVARS64%" >nul
if errorlevel 1 exit /b 1
set "SCRATCH=%TEMP%\subsorting_matching_%RANDOM%_%RANDOM%"
mkdir "%SCRATCH%"
if errorlevel 1 exit /b 1
cl.exe /nologo /std:c++17 /O2 /EHsc /W4 /Fo"%SCRATCH%\matching_bench.obj" /Fe"%SCRATCH%\matching_bench.exe" matching_bench.cpp
if errorlevel 1 exit /b 1
if /i "%~1"=="--hierarchy-only" goto hierarchy
"%SCRATCH%\matching_bench.exe" > timings.csv 2> host.txt
if errorlevel 1 exit /b 1
:hierarchy
"%SCRATCH%\matching_bench.exe" --hierarchy > hierarchy.csv 2> hierarchy_host.txt
exit /b %ERRORLEVEL%
