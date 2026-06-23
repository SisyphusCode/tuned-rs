# tuned-rs

Rust rewrite of the TuneD system tuning daemon.

## Features

- Drop-in D-Bus API compatibility with `com.redhat.tuned` / `com.redhat.tuned.control`
- Loads existing profiles from `/usr/lib/tuned` and `/etc/tuned`
- Plugin coverage:
  - **cpu** — governor, energy_performance_preference
  - **sysctl** — assignment operators (`>`, `>=`, `=>`, `<`, `<=`, `=<`)
  - **vm** — dirty bytes/ratios (including `%`), transparent hugepages
  - **disk** — readahead (with `=>` floor semantics), elevator, optional device list
  - **acpi** — platform_profile with `|` fallbacks
- Rollback of original values on profile switch and shutdown (`rollback=auto` in `tuned-main.conf`)
- PolicyKit authorization matching TuneD (`com.redhat.tuned.<method>` with root fallback)
- SELinux-friendly allowlisted sysfs/proc writes

## Build

```bash
cargo build --release
make check
make test
```

## Install

Install conflicts with the Python `tuned` package because both services claim
`com.redhat.tuned` on the system bus.

```bash
sudo systemctl stop tuned
sudo make install
sudo restorecon -v /usr/sbin/tuned-rs
sudo systemctl enable --now tuned-rs
```

## Verify

```bash
busctl call com.redhat.tuned /Tuned com.redhat.tuned.control profiles
busctl call com.redhat.tuned /Tuned com.redhat.tuned.control active_profile
tuned-adm active
tuned-adm profile balanced
```

## Configuration

- `TUNED_RS_PROFILE_DIRS` — comma-separated profile search path
- `TUNED_RS_ROOT` — chroot-style prefix for config/state paths (testing)
- `RUST_LOG` — logging filter, e.g. `RUST_LOG=tuned_rs=debug`
- `/etc/tuned/tuned-main.conf` — honors `rollback = auto|not_on_exit`

Rollback state is persisted to `/var/lib/tuned-rs/rollback.json` and recovered after crashes.

## SELinux

Label the production binary with `tuned_exec_t` (see `selinux/tuned-rs.fc`) so system
` tuned_t` policy applies:

```bash
sudo restorecon -v /usr/sbin/tuned-rs
ps -eZ | grep tuned-rs
```

## Still unsupported vs full TuneD

Dynamic tuning, disk APM/spindown, network/audio/video/plugins, instance APIs, and full
profile verification semantics.
