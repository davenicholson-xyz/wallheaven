:: This needs to be in the same directory as setwallaper.ps1 (on system PATH ideally)

:: & setwallpaper.bat (& .\wallheaven.exe -t)

@echo off
powershell -ExecutionPolicy Bypass -File "%~dp0setwallpaper.ps1" -path %1
