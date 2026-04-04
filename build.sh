#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")" && pwd)"
BINARY_NAME="gmd"
INSTALL_DIR="/usr/local/bin"
GITHUB_REPO="leaf-kit/g.md"
HOMEBREW_TAP_REPO="leaf-kit/homebrew-gmd"

cd "$PROJECT_ROOT"

get_version() {
    grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/'
}

show_menu() {
    echo "=================================="
    echo "  gmd — Build & Distribution"
    echo "=================================="
    echo ""
    echo "  1) Build (debug, clean)"
    echo "  2) Build (release)"
    echo "  3) Build & Install locally"
    echo "  4) Run tests"
    echo "  5) Run clippy (lint)"
    echo "  6) Clean build artifacts"
    echo "  7) Package for Homebrew (manual)"
    echo "  8) Create release tarball"
    echo "  9) Deploy to Homebrew (full pipeline)"
    echo "  0) Exit"
    echo ""
    echo -n "  Select: "
}

build_debug() {
    echo ">> Cleaning previous build..."
    cargo clean
    echo ">> Building debug (clean)..."
    cargo build
    echo ">> Done: target/debug/$BINARY_NAME"
}

require_tests() {
    echo ">> Running tests (required before release)..."
    if ! cargo test; then
        echo "!! Tests failed. Aborting."
        return 1
    fi
    echo ">> Running clippy..."
    if ! cargo clippy -- -D warnings; then
        echo "!! Clippy found issues. Aborting."
        return 1
    fi
    echo ">> All checks passed."
}

build_release() {
    require_tests || return 1
    echo ">> Building release..."
    cargo build --release
    echo ">> Done: target/release/$BINARY_NAME"
}

install_local() {
    build_release || return 1
    echo ">> Installing to $INSTALL_DIR/$BINARY_NAME"
    if [ -w "$INSTALL_DIR" ]; then
        cp "target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    else
        echo ">> Requires sudo for $INSTALL_DIR"
        sudo cp "target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    fi
    echo ">> Installed. Run: $BINARY_NAME --help"
}

run_tests() {
    echo ">> Running tests..."
    cargo test
    echo ">> Tests complete."
}

run_clippy() {
    echo ">> Running clippy..."
    cargo clippy -- -W clippy::all
    echo ">> Clippy complete."
}

clean() {
    echo ">> Cleaning cargo build artifacts..."
    cargo clean
    echo ">> Clearing Homebrew cache for $BINARY_NAME..."
    brew cleanup -s "$BINARY_NAME" 2>/dev/null || true
    brew cleanup -s 2>/dev/null || true
    echo ">> Clean complete."
}

package_homebrew() {
    build_release || return 1

    VERSION=$(get_version)
    TARBALL="$BINARY_NAME-$(uname -m)-apple-darwin.tar.gz"

    echo ">> Creating tarball: $TARBALL"
    tar -czf "$TARBALL" -C target/release "$BINARY_NAME"

    SHA256=$(shasum -a 256 "$TARBALL" | awk '{print $1}')

    brew cleanup -s "$BINARY_NAME" 2>/dev/null || true

    echo ""
    echo "=================================="
    echo "  Homebrew Distribution Info"
    echo "=================================="
    echo "  Version:  $VERSION"
    echo "  Tarball:  $TARBALL"
    echo "  SHA256:   $SHA256"
    echo ""
    echo "  Use option 9 for automated deploy."
    echo "=================================="
}

create_tarball() {
    build_release || return 1

    VERSION=$(get_version)
    ARCH="$(uname -m)"
    OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
    TARBALL="$BINARY_NAME-$VERSION-$ARCH-$OS.tar.gz"

    echo ">> Creating release tarball: $TARBALL"

    STAGING=$(mktemp -d)
    cp "target/release/$BINARY_NAME" "$STAGING/"
    cp README.md LICENSE "$STAGING/" 2>/dev/null || true

    tar -czf "$TARBALL" -C "$STAGING" .
    rm -rf "$STAGING"

    echo ">> Done: $TARBALL"
    echo ">> SHA256: $(shasum -a 256 "$TARBALL" | awk '{print $1}')"
}

# ══════════════════════════════════════════════════════════════
#  Option 9: Full Homebrew Deploy Pipeline
#
#  Steps:
#    1. Clean build (x86_64 + aarch64)
#    2. Create tarballs
#    3. Delete old GitHub release (if exists)
#    4. Upload tarballs to new GitHub release
#    5. Wait for CDN propagation
#    6. Download from CDN and compute ACTUAL SHA256
#    7. Update Formula with CDN SHA256
#    8. Commit & push Formula to g.md repo
#    9. Push Formula to homebrew-gmd tap
#   10. Verify: untap → tap → install → version check
# ══════════════════════════════════════════════════════════════

deploy_homebrew() {
    VERSION=$(get_version)
    TAG="v${VERSION}"
    echo ""
    echo "══════════════════════════════════════"
    echo "  Deploying gmd ${VERSION} to Homebrew"
    echo "══════════════════════════════════════"
    echo ""

    # ── Step 1: Clean build ──
    echo "[1/10] Clean release build (x86_64)..."
    cargo clean
    require_tests || return 1
    cargo build --release
    echo "        Done."

    echo "[1/10] Clean release build (aarch64)..."
    if rustup target list --installed | grep -q aarch64-apple-darwin; then
        cargo build --release --target aarch64-apple-darwin
        echo "        Done."
    else
        echo "        !! aarch64 target not installed. Skipping."
    fi

    # ── Step 2: Create tarballs ──
    echo "[2/10] Creating tarballs..."
    rm -f gmd-x86_64-apple-darwin.tar.gz gmd-aarch64-apple-darwin.tar.gz

    tar -czf gmd-x86_64-apple-darwin.tar.gz -C target/release "$BINARY_NAME"

    if [ -f "target/aarch64-apple-darwin/release/$BINARY_NAME" ]; then
        tar -czf gmd-aarch64-apple-darwin.tar.gz -C target/aarch64-apple-darwin/release "$BINARY_NAME"
    fi
    echo "        Done."

    # ── Step 3: Delete old release ──
    echo "[3/10] Deleting old release ${TAG} (if exists)..."
    gh release delete "$TAG" --yes --cleanup-tag 2>/dev/null || true
    git tag -d "$TAG" 2>/dev/null || true
    echo "        Done."

    # ── Step 4: Create new release ──
    echo "[4/10] Creating GitHub release ${TAG}..."
    git tag "$TAG"
    git push origin "$TAG" 2>/dev/null

    ASSETS="gmd-x86_64-apple-darwin.tar.gz"
    if [ -f gmd-aarch64-apple-darwin.tar.gz ]; then
        ASSETS="$ASSETS gmd-aarch64-apple-darwin.tar.gz"
    fi
    # shellcheck disable=SC2086
    gh release create "$TAG" $ASSETS --title "$TAG" --notes "gmd $VERSION"
    echo "        Done."

    # ── Step 5: Wait for CDN propagation ──
    echo "[5/10] Waiting for CDN propagation (10s)..."
    sleep 10

    # ── Step 6: Download from CDN and compute ACTUAL SHA256 ──
    echo "[6/10] Downloading from CDN to verify SHA256..."
    VERIFY_DIR=$(mktemp -d)

    CDN_X86_SHA=""
    CDN_ARM_SHA=""

    # Retry up to 3 times for stable SHA
    for attempt in 1 2 3; do
        curl -sL "https://github.com/${GITHUB_REPO}/releases/download/${TAG}/gmd-x86_64-apple-darwin.tar.gz" \
            -o "$VERIFY_DIR/x86.tar.gz"
        CDN_X86_SHA=$(shasum -a 256 "$VERIFY_DIR/x86.tar.gz" | awk '{print $1}')

        if [ -f gmd-aarch64-apple-darwin.tar.gz ]; then
            curl -sL "https://github.com/${GITHUB_REPO}/releases/download/${TAG}/gmd-aarch64-apple-darwin.tar.gz" \
                -o "$VERIFY_DIR/arm.tar.gz"
            CDN_ARM_SHA=$(shasum -a 256 "$VERIFY_DIR/arm.tar.gz" | awk '{print $1}')
        fi

        # Verify the downloaded binary works
        tar xzf "$VERIFY_DIR/x86.tar.gz" -C "$VERIFY_DIR" 2>/dev/null || true
        if [ -f "$VERIFY_DIR/gmd" ]; then
            DL_VERSION=$("$VERIFY_DIR/gmd" --version 2>/dev/null || echo "unknown")
            if echo "$DL_VERSION" | grep -q "$VERSION"; then
                echo "        Attempt $attempt: CDN binary verified (${DL_VERSION})"
                echo "        x86_64 SHA: ${CDN_X86_SHA}"
                [ -n "$CDN_ARM_SHA" ] && echo "        arm64  SHA: ${CDN_ARM_SHA}"
                break
            else
                echo "        Attempt $attempt: CDN returned wrong version ($DL_VERSION), retrying in 5s..."
                sleep 5
            fi
        else
            echo "        Attempt $attempt: CDN download incomplete, retrying in 5s..."
            sleep 5
        fi
    done
    rm -rf "$VERIFY_DIR"

    if [ -z "$CDN_X86_SHA" ]; then
        echo "!! Failed to get CDN SHA. Aborting."
        return 1
    fi

    # ── Step 7: Update Formula with CDN SHA256 ──
    echo "[7/10] Updating Formula/gmd.rb with CDN SHA256..."

    mkdir -p Formula

    ARM_BLOCK=""
    if [ -n "$CDN_ARM_SHA" ]; then
        ARM_BLOCK="    if Hardware::CPU.arm?
      url \"https://github.com/${GITHUB_REPO}/releases/download/${TAG}/gmd-aarch64-apple-darwin.tar.gz\"
      sha256 \"${CDN_ARM_SHA}\"
    else
      url \"https://github.com/${GITHUB_REPO}/releases/download/${TAG}/gmd-x86_64-apple-darwin.tar.gz\"
      sha256 \"${CDN_X86_SHA}\"
    end"
    else
        ARM_BLOCK="    url \"https://github.com/${GITHUB_REPO}/releases/download/${TAG}/gmd-x86_64-apple-darwin.tar.gz\"
    sha256 \"${CDN_X86_SHA}\""
    fi

    cat > Formula/gmd.rb << FORMULA
class Gmd < Formula
  desc "gmd — Grep Markdown, fast CLI search & analysis tool"
  homepage "https://github.com/${GITHUB_REPO}"
  version "${VERSION}"

  on_macos do
${ARM_BLOCK}
  end

  on_linux do
    url "https://github.com/${GITHUB_REPO}/releases/download/${TAG}/gmd-x86_64-linux.tar.gz"
    sha256 "TODO"
  end

  def install
    bin.install "gmd"
  end

  test do
    assert_match "gmd", shell_output("#{bin}/gmd --version")
  end
end
FORMULA
    echo "        Done."

    # ── Step 8: Commit & push to g.md ──
    echo "[8/10] Committing Formula to g.md repo..."
    git add Formula/gmd.rb
    if git diff --cached --quiet; then
        echo "        Formula unchanged, skipping commit."
    else
        git commit -m "Formula: update to ${TAG} with CDN-verified SHA256"
        git push origin main
    fi
    echo "        Done."

    # ── Step 9: Push to homebrew-gmd tap ──
    echo "[9/10] Pushing Formula to homebrew-gmd tap..."
    TAP_DIR=$(mktemp -d)
    gh repo clone "$HOMEBREW_TAP_REPO" "$TAP_DIR/hb" 2>/dev/null
    cp Formula/gmd.rb "$TAP_DIR/hb/Formula/gmd.rb"
    (
        cd "$TAP_DIR/hb"
        git add Formula/gmd.rb
        if git diff --cached --quiet; then
            echo "        Tap formula unchanged."
        else
            git commit -m "Update gmd to ${TAG}"
            git push origin main
        fi
    )
    rm -rf "$TAP_DIR"
    echo "        Done."

    # ── Step 10: Verify installation ──
    echo "[10/10] Verifying brew installation..."
    brew uninstall "$BINARY_NAME" 2>/dev/null || true
    brew untap leaf-kit/gmd 2>/dev/null || true
    brew cleanup -s 2>/dev/null || true
    brew tap leaf-kit/gmd

    if brew install "$BINARY_NAME"; then
        INSTALLED_VERSION=$("$BINARY_NAME" --version 2>/dev/null || echo "unknown")
        if echo "$INSTALLED_VERSION" | grep -q "$VERSION"; then
            echo ""
            echo "══════════════════════════════════════"
            echo "  Deploy successful!"
            echo "  Version: ${INSTALLED_VERSION}"
            echo "  brew install gmd -> OK"
            echo "══════════════════════════════════════"
        else
            echo ""
            echo "!! WARNING: Installed version mismatch!"
            echo "   Expected: ${VERSION}"
            echo "   Got:      ${INSTALLED_VERSION}"
            echo "   Manual verification required."
        fi
    else
        echo ""
        echo "!! brew install failed. Check Formula SHA256."
        echo "   Run: brew untap leaf-kit/gmd && brew tap leaf-kit/gmd && brew install gmd"
    fi
}

# Main loop
while true; do
    show_menu
    read -r choice
    echo ""

    case $choice in
        1) build_debug ;;
        2) build_release ;;
        3) install_local ;;
        4) run_tests ;;
        5) run_clippy ;;
        6) clean ;;
        7) package_homebrew ;;
        8) create_tarball ;;
        9) deploy_homebrew ;;
        0) echo "Bye."; exit 0 ;;
        *) echo "Invalid selection." ;;
    esac

    echo ""
done
