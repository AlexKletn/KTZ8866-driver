# KTZ8866-driver

[Русская версия](README.ru.md)

[Contributing](CONTRIBUTING.md) · [GPL-2.0-only](LICENSE)

Linux driver workspace for the Kinetic KTZ8866 backlight and LCD bias controller. The workspace contains a reusable library and a standalone kernel module.

The `no_std` library programs LED channel settings, brightness, and LCD bias through a caller-provided register bus. Register settings and timing come from the board profile.

The `kernel/` adapter provides register writes through a bound Linux I²C client and sleep-based delays. It is intended to be linked into the consuming kernel module.

## Standalone kernel module

Build `ktz8866_rust.ko` against a configured and built kernel with Rust, I²C and backlight support.

```sh
make -C "$KERNEL_BUILD" M="$PWD/crates/ktz8866-module" modules
```

The module binds to `kinetic,ktz8866` and registers a Linux backlight device for one chip. Select it as the driver for that I²C client. It consumes the board's `vddpos` and `vddneg` regulators. The coordinated two-chip panel sequence belongs to the consuming panel driver.
