use super::{
    linux::{LinuxManifest, PackageFormat, installation_command, select_linux_update},
    signature::{public_key, verify_package},
};
use cargo_packager_updater::semver::Version;
use std::path::Path;

fn manifest(version: &str) -> LinuxManifest {
    serde_json::from_value(serde_json::json!({ "version": version, "packages": {
        "linux-x86_64": {
            "deb": {"url":"https://example.com/techdoro.deb", "signature":"signed-deb"},
            "rpm": {"url":"https://example.com/techdoro.rpm", "signature":"signed-rpm"}
        }
    }}))
    .unwrap()
}

#[test]
fn selects_only_the_installed_format_and_a_newer_version() {
    let current = Version::parse("4.0.0").unwrap();
    let update = select_linux_update(manifest("v4.1.0"), PackageFormat::Rpm, &current, "x86_64")
        .unwrap()
        .unwrap();
    assert_eq!(update.package.signature, "signed-rpm");
    assert_eq!(update.format, PackageFormat::Rpm);
    assert!(
        select_linux_update(manifest("4.0.0"), PackageFormat::Deb, &current, "x86_64")
            .unwrap()
            .is_none()
    );
    assert!(
        select_linux_update(manifest("3.9.0"), PackageFormat::Deb, &current, "x86_64")
            .unwrap()
            .is_none()
    );
    assert!(
        select_linux_update(manifest("4.1.0"), PackageFormat::Deb, &current, "aarch64").is_err()
    );
}

#[test]
fn rejects_insecure_urls_and_mismatched_package_types() {
    let current = Version::parse("4.0.0").unwrap();
    let mut value = manifest("4.1.0");
    value
        .packages
        .get_mut("linux-x86_64")
        .unwrap()
        .get_mut("deb")
        .unwrap()
        .url = "http://example.com/techdoro.deb".into();
    assert!(select_linux_update(value, PackageFormat::Deb, &current, "x86_64").is_err());
    let mut value = manifest("4.1.0");
    value
        .packages
        .get_mut("linux-x86_64")
        .unwrap()
        .get_mut("deb")
        .unwrap()
        .url = "https://example.com/techdoro.rpm".into();
    assert!(select_linux_update(value, PackageFormat::Deb, &current, "x86_64").is_err());
}

#[test]
fn native_manifest_is_understood_by_the_windows_updater() {
    let manifest = serde_json::json!({ "version": "4.1.0", "platforms": {
            "windows-x86_64": {"url":"https://example.com/Techdoro.exe", "signature":"test", "format":"nsis"}
        }, "packages": {"linux-x86_64": {}} });
    let release: cargo_packager_updater::RemoteRelease = serde_json::from_value(manifest).unwrap();
    assert_eq!(
        release.format("windows-x86_64").unwrap().to_string(),
        "nsis"
    );
    assert!(release.format("linux-x86_64").is_err());
}

#[test]
fn invalid_signatures_fail_before_installation() {
    assert!(verify_package(b"untrusted", "invalid", public_key()).is_err());
}

#[test]
fn accepts_a_signed_package_and_rejects_a_modified_download() {
    let bytes = include_bytes!("../../.keys/update.txt");
    let signature = include_str!("../../.keys/update.txt.sig");
    let key = include_str!("../../.keys/update.pub");
    verify_package(bytes, signature, key).unwrap();
    let mut modified = bytes.to_vec();
    modified[0] ^= 1;
    assert!(verify_package(&modified, signature, key).is_err());
    assert!(verify_package(bytes, signature, public_key()).is_err());
}

#[test]
fn installation_uses_fixed_managers_and_passes_the_package_as_one_argument() {
    let path = Path::new("/tmp/package with spaces;echo.deb");
    let command = installation_command(PackageFormat::Deb, path);
    assert_eq!(command.get_program(), "/usr/bin/pkexec");
    assert_eq!(
        command.get_args().collect::<Vec<_>>(),
        ["/usr/bin/apt-get", "install", "-y", path.to_str().unwrap()]
    );
    let command = installation_command(PackageFormat::Rpm, Path::new("/tmp/techdoro.rpm"));
    assert_eq!(command.get_args().next().unwrap(), "/usr/bin/dnf");
}
