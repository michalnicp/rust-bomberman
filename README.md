# Rust Bomberman

## Dependencies

### Arch Linux

```bash
pacman -S alsa-lib libx11 openssl
```

## Running

```bash
RUST_BACKTRACE=1 RUST_LOG=rust_bomberman=info cargo +nightly run --features nightly
```
