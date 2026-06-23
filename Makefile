PREFIX ?= /usr
BINDIR ?= $(PREFIX)/sbin
SYSTEMDUNITDIR ?= /usr/lib/systemd/system
DBUSCONFDIR ?= /usr/share/dbus-1/system.d
DBUSSERVICEDIR ?= /usr/share/dbus-1/system-services
POLKITDIR ?= /usr/share/polkit-1/actions
DOCDIR ?= /usr/share/doc/tuned-rs

.PHONY: all build test check install install-ppd

all: build

build:
	cargo build --release

check:
	cargo check
	cargo clippy -- -D warnings

test:
	cargo test

install: build
	install -D -m 0755 target/release/tuned-rs $(DESTDIR)$(BINDIR)/tuned-rs
	install -D -m 0644 packaging/tuned-rs.service $(DESTDIR)$(SYSTEMDUNITDIR)/tuned-rs.service
	install -D -m 0644 packaging/com.redhat.tuned.conf $(DESTDIR)$(DBUSCONFDIR)/com.redhat.tuned.conf
	install -D -m 0644 packaging/com.redhat.tuned.policy $(DESTDIR)$(POLKITDIR)/com.redhat.tuned.policy
	install -D -m 0644 selinux/tuned-rs.fc $(DESTDIR)$(DOCDIR)/selinux/tuned-rs.fc
	install -D -m 0644 selinux/tuned-rs.te $(DESTDIR)$(DOCDIR)/selinux/tuned-rs.te
	install -D -m 0644 README.md $(DESTDIR)$(DOCDIR)/README.md
	$(MAKE) install-ppd

install-ppd: build
	install -D -m 0755 target/release/tuned-rs-ppd $(DESTDIR)$(BINDIR)/tuned-rs-ppd
	install -D -m 0644 packaging/tuned-rs-ppd.service $(DESTDIR)$(SYSTEMDUNITDIR)/tuned-rs-ppd.service
	install -D -m 0644 packaging/org.freedesktop.UPower.PowerProfiles.conf $(DESTDIR)$(DBUSCONFDIR)/org.freedesktop.UPower.PowerProfiles.conf
	install -D -m 0644 packaging/org.freedesktop.UPower.PowerProfiles.service $(DESTDIR)$(DBUSSERVICEDIR)/org.freedesktop.UPower.PowerProfiles.service
	install -D -m 0644 packaging/net.hadess.PowerProfiles.service $(DESTDIR)$(DBUSSERVICEDIR)/net.hadess.PowerProfiles.service
	install -D -m 0644 packaging/org.freedesktop.UPower.PowerProfiles.policy $(DESTDIR)$(POLKITDIR)/org.freedesktop.UPower.PowerProfiles.policy
	install -D -m 0644 packaging/net.hadess.PowerProfiles.policy $(DESTDIR)$(POLKITDIR)/net.hadess.PowerProfiles.policy
