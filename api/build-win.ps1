# Ensure we are in the correct directory for the project
Write-Host "Running cargo build..."
# Run cargo build
cargo build

if ($LASTEXITCODE -ne 0) {
    Write-Host "cargo build failed, aborting script."
    exit $LASTEXITCODE
}

Write-Host "Running maturin build --debug..."
# Run maturin build
maturin build

if ($LASTEXITCODE -ne 0) {
    Write-Host "maturin build failed."
    exit $LASTEXITCODE
}

Write-Host "Build process completed successfully."

# The generated .whl file or package will be in the target directory, usually in target/wheels
Write-Host "Installing the package using pip..."

# Find the .whl file generated by maturin build
$wheel_file = Get-ChildItem -Path "./target/wheels" -Filter "*.whl" | Sort-Object LastWriteTime -Descending | Select-Object -First 1

if ($wheel_file) {
    # Install the package using pip with a prefix
    Write-Host "Found wheel file: $($wheel_file.FullName)"
    pip install $wheel_file.FullName --prefix beatrice_v_api --force-reinstall
} else {
    Write-Host "No wheel file found, skipping pip install."
    exit 1
}

Write-Host "Installation completed successfully."
