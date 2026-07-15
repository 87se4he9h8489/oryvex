# Oryvex

### اینترنت آزاد برای همه :))
**[راهنمای فارسی](README.fa.md)** · **[English Guide](Docs/GUIDE.en.md)** · **[راهنمای کامل فارسی](Docs/GUIDE.fa.md)**

Telegram: https://t.me/CluvexStudio

Oryvex is a censorship circumvention client designed for heavily restricted networks. It automatically discovers reachable routes, establishes an encrypted tunnel, and exposes a local SOCKS5 proxy for your applications.

Unlike traditional VPN clients, Oryvex is built for environments where Deep Packet Inspection (DPI), protocol fingerprinting, UDP throttling, and endpoint blocking are common.

## Features

- Automatic endpoint discovery
- MASQUE (HTTP/3 & HTTP/2)
- WireGuard support
- Nested WireGuard mode (`gool`)
- Traffic obfuscation
- Automatic reconnection
- Local SOCKS5 proxy
- Linux, Windows, macOS and Android (Termux)

## Download

Prebuilt binaries are available on the Releases page for:

- Linux
- Windows
- macOS
- Android (Termux)

### Termux (Android) — one-line install

```bash
curl -fsSL https://raw.githubusercontent.com/CluvexStudio/oryvex/main/oryvex.sh -o oryvex.sh && chmod +x oryvex.sh && ./oryvex.sh install