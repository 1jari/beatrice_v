# Ensure we are in the correct directory for the project
Write-Host "Running cargo build..."
# Run cargo build
cargo build

if ($LASTEXITCODE -ne 0) {
    Write-Host "cargo build failed, aborting script."
    exit $LASTEXITCODE
}

./target/debug/beatrice_v