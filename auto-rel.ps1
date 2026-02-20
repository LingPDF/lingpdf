param(
    [string]$Version = "0.0.1"
)

$ErrorActionPreference = "Stop"

$TAG_NAME = "v${Version}"

Write-Host "======================================" -ForegroundColor Cyan
Write-Host "  lingpdf Release Script (Windows)" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Version: $Version" -ForegroundColor Yellow
Write-Host "Tag: $TAG_NAME" -ForegroundColor Yellow
Write-Host ""

function Update-Version {
    param([string]$version)

    Write-Host "Step 0: Updating version numbers..." -ForegroundColor Green

    if (Test-Path "Cargo.toml") {
        $content = Get-Content "Cargo.toml" -Raw
        $content = $content -replace '(?m)^(\s*)version = "[0-9]+\.[0-9]+\.[0-9]+"', "`$1version = `"$version`""
        Set-Content -Path "Cargo.toml" -Value $content -NoNewline
        Write-Host "  [OK] Updated Cargo.toml" -ForegroundColor Green
    }

    if (Test-Path "package.sh") {
        $content = Get-Content "package.sh" -Raw
        $content = $content -replace '<string>0\.[0-9]+\.[0-9]+</string>', "<string>$version</string>"
        Set-Content -Path "package.sh" -Value $content -NoNewline
        Write-Host "  [OK] Updated package.sh" -ForegroundColor Green
    }

    if (Test-Path "winres\version.json") {
        $content = Get-Content "winres\version.json" -Raw
        $json = $content | ConvertFrom-Json
        $json.version = $version
        $json | ConvertTo-Json -Depth 10 | Set-Content "winres\version.json" -NoNewline
        Write-Host "  [OK] Updated winres\version.json" -ForegroundColor Green
    }

    Write-Host ""
}

function Check-GitStatus {
    $status = git status --porcelain
    if ($status) {
        Write-Host "Warning: You have uncommitted changes." -ForegroundColor Yellow
        $answer = Read-Host "Do you want to commit them? (y/n)"
        if ($answer -eq "y") {
            $msg = Read-Host "Enter commit message"
            git add -A
            git commit -m $msg
            Write-Host "  [OK] Changes committed" -ForegroundColor Green
        } else {
            Write-Host "Please commit your changes before creating a release." -ForegroundColor Red
            exit 1
        }
    }
}

Write-Host "Step 1: Checking git status..." -ForegroundColor Green
Check-GitStatus
Write-Host "  [OK] Git status clean" -ForegroundColor Green
Write-Host ""

Write-Host "Step 2: Updating version numbers..." -ForegroundColor Green
Update-Version -version $Version
Write-Host ""

Write-Host "Step 3: Checking for existing tags..." -ForegroundColor Green

$localTag = git tag -l | Where-Object { $_ -eq $TAG_NAME }
if ($localTag) {
    Write-Host "  Found local tag: $TAG_NAME" -ForegroundColor Yellow
    git tag -d $TAG_NAME
    Write-Host "  [OK] Deleted local tag: $TAG_NAME" -ForegroundColor Green
} else {
    Write-Host "  No local tag found: $TAG_NAME" -ForegroundColor Gray
}

$remoteTag = git ls-remote --tags origin "refs/tags/$TAG_NAME"
if ($remoteTag) {
    Write-Host "  Found remote tag: $TAG_NAME" -ForegroundColor Yellow
    git push origin --delete $TAG_NAME
    Write-Host "  [OK] Deleted remote tag: $TAG_NAME" -ForegroundColor Green
} else {
    Write-Host "  No remote tag found: $TAG_NAME" -ForegroundColor Gray
}

Write-Host ""
Write-Host "Step 4: Committing version update..." -ForegroundColor Green

$status = git status --porcelain
if ($status) {
    git add -A
    git commit -m "Bump version to $Version"
    Write-Host "  [OK] Version update committed" -ForegroundColor Green
} else {
    Write-Host "  No version changes to commit" -ForegroundColor Gray
}

Write-Host ""
Write-Host "Step 5: Pushing commits..." -ForegroundColor Green
git push origin HEAD
Write-Host "  [OK] Pushed commits to remote" -ForegroundColor Green

Write-Host ""
Write-Host "Step 6: Creating new tag..." -ForegroundColor Green

git tag -a $TAG_NAME -m "Release $TAG_NAME"
Write-Host "  [OK] Created tag: $TAG_NAME" -ForegroundColor Green

Write-Host ""
Write-Host "Step 7: Pushing tag to remote..." -ForegroundColor Green

git push origin $TAG_NAME
Write-Host "  [OK] Pushed tag to remote: $TAG_NAME" -ForegroundColor Green

Write-Host ""
Write-Host "======================================" -ForegroundColor Cyan
Write-Host "  Done!" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "GitHub Actions will automatically build and create a release." -ForegroundColor Yellow
Write-Host "Watch the progress at: https://github.com/LingPDF/lingpdf/actions" -ForegroundColor Yellow
Write-Host ""
Write-Host "Release $TAG_NAME has been triggered!" -ForegroundColor Green
Write-Host ""
