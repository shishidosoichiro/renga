# Generate CHANGELOG.md for a new release.
# Usage: just changelog v0.14.0
changelog tag:
    git cliff v0.13.0..HEAD --tag {{tag}} -o CHANGELOG.md
    cat .changelog/legacy.md >> CHANGELOG.md
