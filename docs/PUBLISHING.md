# Automated Publishing Pipelines

Snap purged. Automated pipelines for Flatpak (Flathub), Ubuntu (Launchpad PPA), and AUR.

## Flatpak → Flathub
- Build: `.github/workflows/publish-flatpak.yml` uses `flatpak/flatpak-github-actions` + `flatpak-builder` with `packaging/flatpak/dpino.json`.
- Deploy: Use `flatpak/flatpak-github-actions/flat-manager` to open/update PR against `flathub/io.abdou.dpino`. Requires `FLATHUB_GITHUB_TOKEN` secret.
- Source: `packaging/flatpak/dpino.json` (manifest) + `scripts/build_flatpak.sh`.

## Ubuntu → Launchpad PPA
- Build: `.github/workflows/publish-ppa.yml`. Uses `debuild -S` with `packaging/deb/DEBIAN/` + GPG key (`LAUNCHPAD_GPG_PRIVATE_KEY`).
- Upload: `dput` to PPA (e.g., `ppa:abdou-da0wew/dpino`). Requires `LAUNCHPAD_PPA` secret.
- Target series: `noble` (24.04), `jammy` (22.04). Build matrix per series.
- Note: Launchpad builds source only; uploads need `dput` via FTP or SFTP.

## AUR → Arch Linux
- Build: `.github/workflows/publish-aur.yml` uses `KSXGitHub/github-actions-deploy-aur@v4.1.1`.
- Updates `packaging/aur/PKGBUILD` + `.SRCINFO` automatically from release tag (`v*`).
- Requires `AUR_SSH_PRIVATE_KEY` secret registered on AUR profile.
- Action downloads tarball, updates `pkgver`/checksums, pushes to `aur.archlinux.org`.

## Secrets Required
| Pipeline | Secrets |
|---|---|
| Flatpak | `FLATHUB_GITHUB_TOKEN` |
| Ubuntu PPA | `LAUNCHPAD_GPG_PRIVATE_KEY`, `LAUNCHPAD_PPA` |
| AUR | `AUR_SSH_PRIVATE_KEY` |
