# CaveStory Save Editor

[![dependency status](https://deps.rs/repo/github/poly000/doukutsu-save-editor-rs/status.svg)](https://deps.rs/repo/github/poly000/doukutsu-save-editor-rs)

Another save editor for `doukutsu-rs` in rust.

Supports Windows, Linux/BSD and MacOS.

## Install

### Arch(-based)

```
paru -S doukutsu-save-editor
```

### Other

```
cargo install --path .
```

This will place the binary to `~/.cargo/bin`. If you cannot execute `doukutsu-save-editor`, please check your `PATH`.

## Build Dependencies

We use [GTK3 backend](https://docs.rs/rfd/latest/rfd/#linux--bsd-backends) on GNU/Linux and *BSD for `rfd`.

## Updating egui

As of 2022, egui is in active development with frequent releases with breaking changes. [eframe_template](https://github.com/emilk/eframe_template/) will be updated in lock-step to always use the latest version of egui.

When updating `egui` and `eframe` it is recommended you do so one version at the time, and read about the changes in [the egui changelog](https://github.com/emilk/egui/blob/master/CHANGELOG.md) and [eframe changelog](https://github.com/emilk/egui/blob/master/crates/eframe/CHANGELOG.md).
