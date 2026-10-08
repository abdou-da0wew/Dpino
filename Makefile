.PHONY: build test clean package deb snap flatpak appimage rpm all-packages help

VERSION ?= 0.1.0
ARCH ?= amd64

help:
	@echo "dpino Build System"
	@echo ""
	@echo "Targets:"
	@echo "  build          - Build release binary"
	@echo "  test           - Run tests"
	@echo "  clean          - Clean build artifacts"
	@echo "  deb            - Build Debian package"
	@echo "  snap           - Build Snap package"
	@echo "  flatpak        - Build Flatpak package"
	@echo "  appimage       - Build AppImage"
	@echo "  rpm            - Build RPM package"
	@echo "  all-packages   - Build all packages"
	@echo ""
	@echo "Usage:"
	@echo "  make build VERSION=0.1.0"
	@echo "  make deb VERSION=0.1.0 ARCH=amd64"

build:
	cargo build --release

test:
	cargo test --verbose

clean:
	cargo clean
	rm -rf build/

deb:
	@./scripts/build_deb.sh $(VERSION) $(ARCH)

snap:
	@./scripts/build_snap.sh $(VERSION)

flatpak:
	@./scripts/build_flatpak.sh $(VERSION)

appimage:
	@./scripts/build_appimage.sh $(VERSION)

rpm:
	@echo "Building RPM package..."
	@VERSION=$(VERSION) && \
	tar -czf dpino-$$VERSION.tar.gz --exclude='.git' --exclude='target' --exclude='build' . && \
	mkdir -p ~/rpmbuild/{SOURCES,SPECS,RPMS} && \
	cp dpino-$$VERSION.tar.gz ~/rpmbuild/SOURCES/ && \
	cp packaging/rpm/dpino.spec ~/rpmbuild/SPECS/ && \
	sed -i "s/Version:.*/Version: $$VERSION/" ~/rpmbuild/SPECS/dpino.spec && \
	rpmbuild -bb ~/rpmbuild/SPECS/dpino.spec && \
	echo "RPM built: ~/rpmbuild/RPMS/x86_64/"

all-packages:
	@./scripts/build_all.sh $(VERSION)

