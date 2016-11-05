enum AddonType {
    NativeExtension,
    WebExtension,
    Theme,
    Plugin,
    Service,
}

enum InstallState  {
    Available,
    Downloading,
    Downloaded,
    DownloadFailed,
    Verifying,
    Verified,
    VerifyFailed,
    Postponing,
    Postponed,
    PostponedFailed,
    Resumed,
    Resuming,
    ResumeFailed,
    Staging,
    Staged,
    StagingFailed,
    Installing,
    Installed,
    InstallFailed,
    Cancelling,
    Cancelled,
    CancelFailed,
    Uninstalling,
    Uninstalled,
    UninstallFailed,
}

/// An Manifest describes an Addon.
struct Manifest {
    id: String,
    name: String,
    version: String,
    addon_type: AddonType,
    url: String,
}

impl Manifest {
    fn new(id: String, name: String, version: String, addon_type: AddonType, url: String) -> Self {
        Manifest {
            id: id,
            name: name,
            version: version,
            addon_type: addon_type,
            url: url,
        }
    }
/*
        // TODO verify ID
        self.id = id;

        // TODO verify name
        self.name = name;

        self.addon_type = match addon_type {
            addon_type::NativeExtension => {},
            addon_type::WebExtension => {},
            addon_type::Theme => {},
            addon_type::Plugin => {},
            addon_type::Service => {},
            _ => panic!("Invalid addon type"),
        };

        // TODO validate URL
        self.url = url;
*/
}

/// An Addon represents an individual addon.
struct Addon {
    id: String,
    name: String,
    version: String,
    install_url: String,
    install_location: InstallLocation,
}

impl Addon {
    //fn new(manifest: Vec<u8>, type, install_location: install_location) -> Addon {
    fn new(manifest: Manifest, install_location: InstallLocation) -> Self {
        Addon {
            id: manifest.id,
            name: manifest.name,
            version: manifest.version,
            install_url: manifest.url,
            install_location: install_location,
        }
        // FIXME
        //self.id = manifest.get("id");
        //self.name = manifest.get("name");
        //self.version = manifest.get("version");
        //self.install_location = install_location;
        //self.install_url = Manifest::url;
    }
}

/// Install downloads, verifies, and installs an Addon.
struct Install {
    state: InstallState,
    addon: Addon,
}

impl Install {
  fn new(addon: Addon) -> Self {
      Install {
          state: InstallState::Available,
          addon: addon,
      }
  }

  fn download(&mut self) {
      self.state = match self.state {
          InstallState::Available => InstallState::Downloading,
          _ => panic!("Invalid state transition"),
      };
      println!("Downloading {}...", self.addon.name);
      // TODO Actually download to `self.install_location:downloadDirectory`.
      // TODO Set to DownloadFailed if failed.
      println!("Finished downloading {}", self.addon.name);
      self.state = match self.state {
          InstallState::Downloading => InstallState::Downloaded,
          _ => panic!("Invalid state transition"),
      };
      self.verify();
  }

  fn verify(&mut self) {
      self.state = match self.state {
          InstallState::Downloaded => InstallState::Verifying,
          _ => panic!("Invalid state transition"),
      };
      println!("Verifying {}...", self.addon.name);
      // TODO Actually verify from `self.install_location:downloadDirectory`.
      // TODO Set to VerifyFailed if failed.
      println!("Finished verifying {}", self.addon.name);
      self.state = match self.state {
          InstallState::Verifying => InstallState::Verified,
          _ => panic!("Invalid state transition"),
      };
      self.stage();
  }

  fn stage(&mut self) {
      self.state = match self.state {
          InstallState::Verified => InstallState::Staging,
          _ => panic!("Invalid state transition"),
      };
      println!("Staging {}...", self.addon.name);
      // TODO Actually copy from `self.install_location:downloadDirectory` to
      //      `self.install_location:stageDirectory`.
      // TODO Set to StagingFailed if failed.
      println!("Finished staging {}", self.addon.name);
      self.state = match self.state {
          InstallState::Staging => InstallState::Staged,
          _ => panic!("Invalid state transition"),
      };
      self.install();
  }

  fn install(&mut self) {
      self.state = match self.state {
          InstallState::Staged => InstallState::Installing,
          _ => panic!("Invalid state transition"),
      };
      println!("Installing {}...", self.addon.name);
      // TODO Actually install.
      // TODO set to InstallFailed if failed.
      println!("Finished verifying {}", self.addon.name);
      self.state = InstallState::Installed;
  }

  fn cancel(&mut self) {
      self.state = match self.state {
          InstallState::Downloading | InstallState::Downloaded => {
              println!("Stopping download...");
              println!("Remove downloaded files...");
              InstallState::Cancelled
          },
          InstallState::Verifying | InstallState::Verified => {
              println!("Stopping verification...");
              InstallState::Cancelled
          },
          InstallState::Postponing | InstallState::Postponed => {
              println!("Removing postponed install...");
              InstallState::Cancelled
          },
          InstallState::Resuming | InstallState::Resumed => {
              println!("Stop resuming install...");
              println!("Uninstall resumed install...");
              InstallState::Cancelled
          },
          InstallState::Staging | InstallState::Staged => {
              println!("Stopping staging...");
              println!("Remove staged files...");
              InstallState::Cancelled
          },
          InstallState::Installing | InstallState::Installed => {
              println!("Stopping install...");
              println!("Uninstalling installed addon...");
              InstallState::Cancelled
          },
          _ => panic!("Invalid state transition"),
      };
  }

  fn postpone(&mut self) {
      self.state = match self.state {
          InstallState::Verified => InstallState::Postponing,
          _ => panic!("Invalid state transition"),
      };
      println!("Postponing addon...");
      // TODO set to PostponeFailed if failed.
      self.state = InstallState::Postponed;
  }

  fn resume(&mut self) {
      self.state = match self.state {
          InstallState::Postponed => InstallState::Resuming,
          _ => panic!("Invalid state transition"),
      };
      println!("Resuming addon...");
      // TODO set to ResumeFailed if failed.
      self.state = InstallState::Resumed;
  }
}


struct InstallLocation {
    name: String,
    base_directory: String, // FIXME use real file type
}

impl InstallLocation {
    fn new(name: String, base_directory: String) -> Self {
        println!("Initialized install location {} in {}", name, base_directory);
        InstallLocation {
            name: name,
            base_directory: base_directory,
        }
    }

    fn get_download_directory(&mut self) -> String {
        println!("Creating download directory for install location {}", self.name);
        //self.downloadDirectory = self.base_directory.append("download");
        // TODO wrap in a lock and release it when references drop.
        //      also remove directory when all references have dropped.
        return String::from("downloaddir");
    }

    fn get_staging_directory(&mut self) -> String {
        println!("Creating staging directory for install location {}", self.name);
        //self.stageDirectory = self.base_directory.append("staging");
        // TODO wrap in a lock and release it when references drop.
        //      also remove directory when all references have dropped.
        return String::from("stagedir");
    }
}

#[cfg(test)]
mod tests {
    use super::Manifest;
    use super::Addon;
    use super::AddonType;
    use super::InstallLocation;
    use super::Install;

    #[test]
    fn install_addon() {
        let id = String::from("@test123");
        let name = String::from("Test Addon");
        let version = String::from("0.1");
        let addon_type = AddonType::WebExtension;
        let url = String::from("http://.../");
        let manifest = Manifest::new(id, name, version, addon_type, url);

        let name = String::from("profile");
        let base_directory = String::from("c:\\Addons");
        let install_location = InstallLocation::new(name, base_directory);

        let addon = Addon::new(manifest, install_location);

        let mut install = Install::new(addon);

        // Start the install process from scratch.
        install.download();

        // TODO check that addon is actually installed now.
    }
}
