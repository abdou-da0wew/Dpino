Name:           dpino
Version:        0.1.0
Release:        1%{?dist}
Summary:        Rust-powered package metadata extractor + desktop entry installer + browser
License:        MIT OR Apache-2.0
URL:            https://github.com/abdou-da0wew/dpino
Source0:        %{name}-%{version}.tar.gz
BuildArch:      x86_64
Requires:       glibc >= 2.31

%description
dpino scans directories for Debian packages (.deb), AppImages (.AppImage), and
AppDir packages. It can extract metadata, generate Freedesktop-compliant desktop
entries, install them to XDG directories, and provide an interactive TUI browser
for package management.

Features:
- Scan directories for packages
- Extract package metadata
- Generate and install desktop entries
- Interactive TUI browser
- Support for .deb, .AppImage, and AppDir formats

%prep
%setup -q

%build
cargo build --release --locked

%install
install -Dm755 target/release/dpino %{buildroot}%{_bindir}/dpino
install -Dm644 final/dpino/usr/share/applications/dpino.desktop \
    %{buildroot}%{_datadir}/applications/dpino.desktop
install -Dm644 final/dpino/usr/share/man/man1/dpino.1 \
    %{buildroot}%{_mandir}/man1/dpino.1
# Install all icons
for size in 16x16 22x22 24x24 32x32 48x48 64x64 128x128 256x256 512x512; do
    if [ -f "final/dpino/usr/share/icons/hicolor/${size}/apps/dpino.png" ]; then
        install -Dm644 "final/dpino/usr/share/icons/hicolor/${size}/apps/dpino.png" \
            %{buildroot}%{_datadir}/icons/hicolor/${size}/apps/dpino.png
    fi
done

%files
%{_bindir}/dpino
%{_datadir}/applications/dpino.desktop
%{_mandir}/man1/dpino.1
%{_datadir}/icons/hicolor/16x16/apps/dpino.png
%{_datadir}/icons/hicolor/22x22/apps/dpino.png
%{_datadir}/icons/hicolor/24x24/apps/dpino.png
%{_datadir}/icons/hicolor/32x32/apps/dpino.png
%{_datadir}/icons/hicolor/48x48/apps/dpino.png
%{_datadir}/icons/hicolor/64x64/apps/dpino.png
%{_datadir}/icons/hicolor/128x128/apps/dpino.png
%{_datadir}/icons/hicolor/256x256/apps/dpino.png
%{_datadir}/icons/hicolor/512x512/apps/dpino.png

%changelog
* Mon Dec 01 2024 Abdou <aabdou911aydev@gmail.com> - 0.1.0-1
- Initial release

