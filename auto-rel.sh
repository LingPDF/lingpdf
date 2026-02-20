#!/bin/bash

set -e

# Get version from argument or use default
VERSION=${1:-"0.0.1"}
TAG_NAME="v${VERSION}"

echo "======================================"
echo "  lingpdf Release Script"
echo "======================================"
echo ""
echo "Version: $VERSION"
echo "Tag: $TAG_NAME"
echo ""

# Function to update version in files
update_version() {
    local version=$1
    echo "Step 0: Updating version numbers..."
    
    # Update Cargo.toml
    if [ -f "Cargo.toml" ]; then
        sed -i.bak "s/^version = \"[^\"]*\"/version = \"${version}\"/" Cargo.toml
        rm -f Cargo.toml.bak
        echo "  ✓ Updated Cargo.toml"
    fi
    
    # Update version in build.sh
    if [ -f "build.sh" ]; then
        sed -i.bak "s/<string>0\.1\.0<\/string>/<string>${version}<\/string>/g" build.sh
        rm -f build.sh.bak
        echo "  ✓ Updated build.sh"
    fi
    
    # Update version in package.sh
    if [ -f "package.sh" ]; then
        sed -i.bak "s/<string>0\.1\.0<\/string>/<string>${version}<\/string>/g" package.sh
        rm -f package.sh.bak
        echo "  ✓ Updated package.sh"
    fi
    
    echo ""
}

# Function to check if working directory is clean
check_git_status() {
    if [ -n "$(git status --porcelain)" ]; then
        echo "Warning: You have uncommitted changes."
        read -p "Do you want to commit them? (y/n): " answer
        if [ "$answer" = "y" ]; then
            read -p "Enter commit message: " msg
            git add -A
            git commit -m "$msg"
            echo "  ✓ Changes committed"
        else
            echo "Please commit your changes before creating a release."
            exit 1
        fi
    fi
}

echo "Step 1: Checking git status..."
check_git_status
echo "  ✓ Git status clean"
echo ""

echo "Step 2: Updating version numbers..."
update_version "$VERSION"
echo ""

echo "Step 3: Checking for existing tags..."

# Delete local tag if exists
if git tag -l | grep -q "^${TAG_NAME}$"; then
    echo "  Found local tag: ${TAG_NAME}"
    git tag -d "${TAG_NAME}"
    echo "  ✓ Deleted local tag: ${TAG_NAME}"
else
    echo "  No local tag found: ${TAG_NAME}"
fi

# Delete remote tag if exists
if git ls-remote --tags origin | grep -q "refs/tags/${TAG_NAME}$"; then
    echo "  Found remote tag: ${TAG_NAME}"
    git push origin --delete "${TAG_NAME}"
    echo "  ✓ Deleted remote tag: ${TAG_NAME}"
else
    echo "  No remote tag found: ${TAG_NAME}"
fi

echo ""
echo "Step 4: Committing version update..."

# Commit version update if there are changes
if [ -n "$(git status --porcelain)" ]; then
    git add -A
    git commit -m "Bump version to ${VERSION}"
    echo "  ✓ Version update committed"
else
    echo "  No version changes to commit"
fi

echo ""
echo "Step 5: Pushing commits..."
git push origin HEAD
echo "  ✓ Pushed commits to remote"

echo ""
echo "Step 6: Creating new tag..."

# Create new tag
git tag -a "${TAG_NAME}" -m "Release ${TAG_NAME}"
echo "  ✓ Created tag: ${TAG_NAME}"

echo ""
echo "Step 7: Pushing tag to remote..."

# Push tag to remote
git push origin "${TAG_NAME}"
echo "  ✓ Pushed tag to remote: ${TAG_NAME}"

echo ""
echo "======================================"
echo "  Done!"
echo "======================================"
echo ""
echo "GitHub Actions will automatically build and create a release."
echo "Watch the progress at: https://github.com/LingPDF/lingpdf/actions"
echo ""
echo "Release ${TAG_NAME} has been triggered!"
echo ""
