@echo off
echo Building OwoWork contract...
echo ============================

echo 1. Building contract for WASM target...
call cargo build --target wasm32-unknown-unknown --release

if %errorlevel% equ 0 (
    echo ✓ Build successful!
    echo ✓ WASM file created: target\wasm32-unknown-unknown\release\owowork_contract.wasm
    for %%F in ("target\wasm32-unknown-unknown\release\owowork_contract.wasm") do (
        echo ✓ File size: %%~zF bytes
    )
) else (
    echo ✗ Build failed!
    exit /b 1
)

echo.
echo 2. Running tests...
call cargo test

if %errorlevel% equ 0 (
    echo ✓ All tests passed!
) else (
    echo ✗ Tests failed!
    exit /b 1
)

echo.
echo 3. Checking for warnings...
call cargo check 2>&1 | findstr /i "warning" >nul
if %errorlevel% equ 0 (
    echo ⚠ Warnings found, check with: cargo check
) else (
    echo ✓ No warnings found
)

echo.
echo Build completed successfully!
pause