# Release Process

This document outlines the process for creating new releases of the gyst CLI tool.

## Prerequisites

- You must have Rust and Cargo installed
- You must have GitHub CLI (`gh`) installed
- You must have push access to the repository

## Steps to Release

1. **Update Version**
   ```bash
   # Update version in Cargo.toml
   sed -i '' 's/^version = ".*"/version = "X.Y.Z"/' Cargo.toml
   
   # Commit the version change
   git add Cargo.toml
   git commit -m "chore: bump version to X.Y.Z"
   git push
   ```

2. **Build Release Binary**
   ```bash
   # Build optimized release binary
   cargo build --release
   
   # Create binary with architecture suffix
   cp target/release/gyst gyst-darwin-$(uname -m)
   
   # Generate SHA256 checksum
   shasum -a 256 gyst-darwin-$(uname -m) > gyst-darwin-$(uname -m).sha256
   ```

3. **Create GitHub Release**
   ```bash
   # Create a new tag
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   git push origin vX.Y.Z
   
   # Create GitHub release with binary and checksum
   gh release create vX.Y.Z gyst-darwin-* \
     --title "Release vX.Y.Z" \
     --notes "Release notes for version X.Y.Z"
   ```

4. **Verify Installation**
   ```bash
   # Test the installation script
   curl -fsSL https://raw.githubusercontent.com/created-by-varun/gyst/main/install.sh | bash
   
   # Verify the installed version
   gyst --version
   ```

## Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Commit version change
- [ ] Build release binary
- [ ] Generate checksum
- [ ] Create Git tag
- [ ] Create GitHub release
- [ ] Test installation script
- [ ] Verify installed version

## Troubleshooting

### Common Issues

1. **Build Failures**
   - Ensure all dependencies are up to date
   - Check for any breaking changes in dependencies

2. **Release Creation Failures**
   - Verify you have the correct permissions
   - Ensure the tag doesn't already exist
   - Make sure the binary and checksum files exist

3. **Installation Script Issues**
   - Verify the release assets are properly uploaded
   - Check the URLs in the installation script
   - Ensure the checksums match

### Getting Help

If you encounter any issues during the release process:

1. Check the [GitHub Actions logs](https://github.com/created-by-varun/gyst/actions) if using CI/CD
2. Open an issue in the repository
3. Contact the maintainers

## Notes

- The release process currently only supports macOS (both Intel and Apple Silicon)
- Future improvements might include:
  - Automated version bumping
  - Cross-compilation for other platforms
  - Homebrew formula updates
