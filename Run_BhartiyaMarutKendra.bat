@echo off
title Bhartiya Marut Kendra (भारतीय मारुत केंद्र)
cd /d "%~dp0"

echo ========================================================
echo   Bhartiya Marut Kendra (भारतीय मारुत केंद्र)
echo   Starting High-Performance Server...
echo ========================================================

start "" "%~dp0bin\rust_server.exe"
timeout /t 1 /nobreak >nul
start http://localhost:8080/

echo.
echo [OK] Bhartiya Marut Kendra live on http://localhost:8080/
echo You can close this command window anytime.
