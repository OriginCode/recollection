<p align="center">
    <img src="misc/logo.png" width="128" alt="Logo of Recollection">
</p>

<div align="center"> 

# Recollection

Notify you for daily, weekly, monthly and yearly events.

</div>

## Usage

Use `recollectctl` to manage the events, and run `recollectd` daemon to monitor the
events and send notifications.

Before running `recollectd`, please execute `recollectctl init` to initialize the
data file or `recollectctl new` to create an event, otherwise the daemon will fail.

### Systemd

A sample systemd service file can be found in `misc/recollectd.service`. For
per-user use, you may install the file to `$HOME/.config/systemd/user/`. For
system-wide use, install the file to `/etc/systemd/system/` instead. Edit the file
and specify the path to your installed `recollectd` binary before starting the
service.

```unit file (systemd)
[Service]
...
ExecStart=/your/path/to/recollectd
```

## Install

You can install from <crates.io> using `cargo`

```bash
cargo install --locked recollectctl recollectd
```

Or, build from source

```bash
cargo install --git=https://factoria.origincode.me/OriginCode/recollection.git
```

## Requirements

### Build

- `rustc` 1.58 or higher
- Cargo

### Runtime

- See <https://github.com/hoodie/notify-rust#readme> for more information.

## Structure

- librecollect: Library for parsing events and utilities to control the storage
- recollectd: Daemon to monitor the events and send notifications
- recollectctl: Command line interface for managing the events