use super::PackageFormat;
use anyhow::{Context, Result, bail};
use std::{path::Path, process::Command};

pub(super) fn installed_package_format(executable: &Path) -> Option<PackageFormat> {
    if let Ok(output) = Command::new("/usr/bin/dpkg-query")
        .arg("-S")
        .arg(executable)
        .output()
        && output.status.success()
        && String::from_utf8_lossy(&output.stdout)
            .lines()
            .any(|line| line.starts_with("techdoro:") || line.starts_with("techdoro:amd64:"))
    {
        return Some(PackageFormat::Deb);
    }
    if let Ok(output) = Command::new("/usr/bin/rpm")
        .args(["-qf", "--qf", "%{NAME}"])
        .arg(executable)
        .output()
        && output.status.success()
        && String::from_utf8_lossy(&output.stdout).trim() == "techdoro"
    {
        return Some(PackageFormat::Rpm);
    }
    None
}

pub(in crate::updater) fn installation_command(format: PackageFormat, path: &Path) -> Command {
    let mut command = Command::new("/usr/bin/pkexec");
    command
        .arg(format.manager())
        .args(["install", "-y"])
        .arg(path);
    command
}

pub(super) fn run_package_manager(format: PackageFormat, path: &Path) -> Result<()> {
    let output = installation_command(format, path)
        .output()
        .context("Could not start the package manager")?;
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        bail!("Installation canceled or failed: {}", error.trim());
    }
    Ok(())
}
