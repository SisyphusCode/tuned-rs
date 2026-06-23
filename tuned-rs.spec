Name:           tuned-rs
Version:        0.1.0
Release:        1%{?dist}
Summary:        Blazing fast, pure Rust drop-in replacement for TuneD and PPD

License:        GPLv3
URL:            https://github.com/SisyphusCode/tuned-rs
Source0:        %{url}/archive/main.tar.gz

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  systemd-rpm-macros
BuildRequires:  make

Provides:       tuned = %{version}
Provides:       power-profiles-daemon = %{version}
Conflicts:      tuned
Conflicts:      power-profiles-daemon

%description
A memory-safe, asynchronous daemon written in pure Rust that serves as a 1:1 drop-in replacement for the Red Hat TuneD daemon and the GNOME Power Profiles Daemon (PPD). Bypasses the middleman to directly manipulate ACPI states and CPU scaling governors.

%prep
%autosetup -n tuned-rs-main

%build
cargo build --release

%install
%make_install

%post
%systemd_post tuned-rs.service tuned-rs-ppd.service

%preun
%systemd_preun tuned-rs.service tuned-rs-ppd.service

%postun
%systemd_postun_with_restart tuned-rs.service tuned-rs-ppd.service

%files
/usr/bin/tuned-rs
/usr/bin/tuned-rs-ppd
/usr/lib/systemd/system/tuned-rs.service
/usr/lib/systemd/system/tuned-rs-ppd.service
/usr/share/dbus-1/system-services/org.freedesktop.UPower.PowerProfiles.service
/etc/dbus-1/system.d/com.redhat.tuned.conf
/usr/share/polkit-1/actions/net.hadess.PowerProfiles.policy
/usr/share/doc/tuned-rs/README.md
/usr/share/doc/tuned-rs/selinux/tuned-rs.te
/usr/share/doc/tuned-rs/selinux/tuned-rs.fc

%changelog
* Tue Jun 23 2026 Kenneth Glowner <klglownerjr@usmarinecorps.vet> - 0.1.0-1
- Initial release of the pure Rust TuneD/PPD replacement.
