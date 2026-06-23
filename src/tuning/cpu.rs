use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use tracing::{error, info, warn};

use crate::rollback::{rollback_key, Rollback};
use crate::tuning::modifiers::{read_trimmed, resolve_choice};
use crate::tuning::sysfs::{allowed_sysfs_path, write_raw as write_sysfs_raw};

const ALLOWED_GOVERNORS: &[&str] = &[
    "performance",
    "powersave",
    "ondemand",
    "conservative",
    "schedutil",
    "userspace",
];

const ALLOWED_EPP_VALUES: &[&str] = &[
    "default",
    "performance",
    "balance_performance",
    "balance_power",
    "power",
];

pub fn is_allowed_governor(governor: &str) -> bool {
    ALLOWED_GOVERNORS.contains(&governor)
}

pub fn is_allowed_epp(value: &str) -> bool {
    ALLOWED_EPP_VALUES.contains(&value)
}

pub fn apply_governor(rollback: &Rollback, raw: &str) -> Result<()> {
    let Some(governor) = resolve_choice(raw, is_allowed_governor) else {
        warn!("No supported governor found in '{raw}'");
        return Ok(());
    };
    write_cpu_file(rollback, &governor, scaling_governor_path)
}

pub fn apply_epp(rollback: &Rollback, raw: &str) -> Result<()> {
    let Some(epp) = resolve_choice(raw, is_allowed_epp) else {
        warn!("No supported EPP value found in '{raw}'");
        return Ok(());
    };
    write_cpu_file(rollback, &epp, epp_path)
}

fn scaling_governor_path(entry: &fs::DirEntry) -> PathBuf {
    let file_name = entry.file_name();
    let name = file_name.to_string_lossy();
    if name.starts_with("policy") {
        entry.path().join("scaling_governor")
    } else {
        entry.path().join("cpufreq/scaling_governor")
    }
}

fn epp_path(entry: &fs::DirEntry) -> PathBuf {
    let file_name = entry.file_name();
    let name = file_name.to_string_lossy();
    if name.starts_with("policy") {
        entry.path().join("energy_performance_preference")
    } else {
        entry.path().join("cpufreq/energy_performance_preference")
    }
}

fn write_cpu_file(
    rollback: &Rollback,
    value: &str,
    path_for_entry: fn(&fs::DirEntry) -> PathBuf,
) -> Result<()> {
    let mut updated = 0usize;
    for base in [
        "/sys/devices/system/cpu/cpufreq",
        "/sys/devices/system/cpu",
    ] {
        updated += write_file_dir(rollback, base, value, path_for_entry)?;
    }
    if updated == 0 {
        warn!("No CPU tuning nodes were updated");
    } else {
        info!("Updated CPU settings on {updated} node(s)");
    }
    Ok(())
}

fn write_file_dir(
    rollback: &Rollback,
    base: &str,
    value: &str,
    path_for_entry: fn(&fs::DirEntry) -> PathBuf,
) -> Result<usize> {
    let base_path = Path::new(base);
    let entries = match fs::read_dir(base_path) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(error) => return Err(error.into()),
    };

    let mut updated = 0usize;
    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        let is_policy = name.starts_with("policy")
            && name[6..].chars().all(|c| c.is_ascii_digit());
        let is_cpu = name.starts_with("cpu") && name[3..].chars().all(|c| c.is_ascii_digit());
        if !is_policy && !is_cpu {
            continue;
        }

        let target = path_for_entry(&entry);
        if !target.exists() {
            continue;
        }

        match write_cpu_node(rollback, &target, value) {
            Ok(()) => updated += 1,
            Err(error) => error!("Failed to write {} for {name}: {error}", target.display()),
        }
    }

    Ok(updated)
}

fn write_cpu_node(rollback: &Rollback, target: &Path, value: &str) -> Result<()> {
    validate_cpu_payload(target, value)?;
    let path = allowed_sysfs_path(target)?;
    let original = read_trimmed(&path)?;
    rollback.record_original(&rollback_key("sysfs", &path.to_string_lossy()), &original)?;
    write_sysfs_raw(&path, value)
}

fn validate_cpu_payload(path: &Path, payload: &str) -> Result<()> {
    let leaf = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    match leaf {
        "scaling_governor" if !is_allowed_governor(payload) => {
            bail!("Unknown CPU governor: {payload}")
        }
        "energy_performance_preference" if !is_allowed_epp(payload) => {
            bail!("Unknown energy performance preference: {payload}")
        }
        _ => {}
    }
    Ok(())
}
