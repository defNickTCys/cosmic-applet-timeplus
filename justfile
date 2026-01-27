# cosmic-applet-timeplus justfile
# Build system following System76 standards

# Installation directories (can be overridden)
rootdir := env('DESTDIR', '')
prefix := env('PREFIX', '/usr')
debug := env('DEBUG', '0')

# Derived paths
target := if debug == '1' { 'debug' } else { 'release' }
debug_args := if debug == '1' { '' } else { '--release' }

targetdir := env('CARGO_TARGET_DIR', 'target')
bindir := rootdir + prefix / 'bin'
sharedir := rootdir + prefix / 'share'
applicationsdir := sharedir / 'applications'
iconsdir := sharedir / 'icons/hicolor/scalable/apps'
soundsdir := sharedir / 'cosmic-applet-timeplus/sounds'

# Binary and desktop file names
applet_name := 'cosmic-applet-timeplus'
desktop_id := 'com.system76.CosmicAppletTimePlus'

# Default target
default: build-release

# Compile with debug profile
build-debug *args:
    cargo build {{args}}

# Compile with release profile  
build-release *args: (build-debug '--release' args)

# Run the applet in development mode
run *args:
    COSMIC_APPLET_TIMEPLUS_DATA={{justfile_directory()}}/assets cargo run {{args}}

# Run tests
test:
    cargo test

# Install all files
install: install-bin install-desktop install-icon install-sounds

# Install binary to $PREFIX/bin
install-bin:
    install -Dm755 {{targetdir}}/{{target}}/{{applet_name}} {{bindir}}/{{applet_name}}

# Install .desktop file
install-desktop:
    install -Dm644 data/{{desktop_id}}.desktop {{applicationsdir}}/{{desktop_id}}.desktop

# Install icon
install-icon:
    install -Dm644 data/{{desktop_id}}.svg {{iconsdir}}/{{desktop_id}}.svg

# Install audio assets (if they exist)
install-sounds:
    @if [ -d "assets/sounds" ]; then \
        install -dm755 {{soundsdir}}; \
        find assets/sounds -type f -name "*.wav" -exec \
            install -Dm644 {} {{soundsdir}}/{} \;; \
    else \
        echo "⚠️  No sounds/ directory found, skipping audio installation"; \
    fi

# Install to user directory (~/.local)
install-user:
    PREFIX=$HOME/.local just install

# Uninstall from system
uninstall:
    rm -f {{bindir}}/{{applet_name}}
    rm -f {{applicationsdir}}/{{desktop_id}}.desktop
    rm -f {{iconsdir}}/{{desktop_id}}.svg
    rm -rf {{soundsdir}}

# Uninstall from user directory
uninstall-user:
    PREFIX=$HOME/.local just uninstall

# Clean build artifacts
clean:
    cargo clean

# Check code without building
check:
    cargo check

# Run clippy linter
clippy:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Full CI pipeline
ci: check clippy test build-release
