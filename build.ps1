$ErrorActionPreference = "Stop"

Set-Location $PSScriptRoot

function Assert-CleanGitTree {
    param([string]$Name)

    git diff --quiet

    if ($LASTEXITCODE -ne 0) {
        Write-Host "$Name working tree not clean"
        exit 1
    }
}

function Assert-Success {
    param([string]$Message)

    if ($LASTEXITCODE -ne 0) {
        Write-Host $Message
        exit 1
    }
}

Assert-CleanGitTree "RMSC"

git switch main
Assert-Success "Failed to switch main"

python bumpver.py
Assert-Success "bumpver.py failed"

cargo build --release
Assert-Success "Cargo build failed"

cargo install --path ./rmsc-cli
Assert-Success "Cargo install failed"

wsl --cd "$($PWD.Path)" bash -lc "cargo build --release"
Assert-Success "WSL build failed"

git add .
git commit -m "Bumpver"
Assert-Success "Git commit failed"

$tag = Read-Host "RMSC Enter release tag (e.g. v1.2.3-alpha)"
if ([string]::IsNullOrWhiteSpace($tag)) {
    Write-Host "No tag provided, aborting"
    exit 1
}

if (git tag -l $tag) {
    Write-Host "Tag already exists"
    exit 1
}

git tag -a $tag -m "Release $tag"

git push
Assert-Success "Git push failed"
git push origin $tag
Assert-Success "Git tag push failed"

explorer "target\release"
Start-Process "https://github.com/Divy1211/rms-check/releases/"
