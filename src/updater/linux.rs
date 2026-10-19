mod download;
mod install;
mod manifest;
mod package;

pub(super) use install::install_linux;
pub(super) use manifest::check_linux;
#[cfg(test)]
pub(super) use manifest::select_linux_update;
#[cfg(test)]
pub(super) use package::installation_command;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PackageFormat {
    Deb,
    Rpm,
}

impl PackageFormat {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Deb => "deb",
            Self::Rpm => "rpm",
        }
    }

    pub(super) fn manager(self) -> &'static str {
        match self {
            Self::Deb => "/usr/bin/apt-get",
            Self::Rpm => "/usr/bin/dnf",
        }
    }
}

#[derive(Clone, Deserialize)]
pub(super) struct Package {
    pub(super) url: String,
    pub(super) signature: String,
}

#[derive(Deserialize)]
pub(super) struct LinuxManifest {
    pub(super) version: String,
    pub(super) packages: HashMap<String, HashMap<String, Package>>,
}

#[derive(Clone)]
pub struct LinuxUpdate {
    pub(super) version: String,
    pub(super) package: Package,
    pub(super) format: PackageFormat,
}
