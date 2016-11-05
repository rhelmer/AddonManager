enum AddonTypes {
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

/// An Addon represents an individual addon.
struct Addon {
    id: <u8,
    name: <u8>,
    version: <u8>,
}

impl Addon() {
    fn new(manifest, type, installLocation) -> Self {
        self.type = match type {
            AddonTypes:NativeExtension,
            AddonTypes:WebExtension,
            AddonTypes:Theme,
            AddonTypes:Plugin,
            AddonTypes:Service,
            _ => panic!("Invalid addon type"),
        }

        self.id = manifest.id;
        self.name = manifest.name;
        self.version = manifest.version;
        self.installLocation = installLocation;
    }
}

/// AddonInstall downloads, verifies, and installs an Addon.
struct AddonInstall { state: InstallState }

impl AddonInstall() {
  fn new(Addon: addon) -> Self {
      AddonInstall {
          state: InstallState.Available
      }

      self.addon = addon;
  }

  fn download(&mut self) {
      self.state = match self.state {
          InstallState::Available => InstallState::Downloading,
          _ => panic!("Invalid state transition"),
      }
      println!("Downloading {}...", self.addon.name);
      // TODO Actually download to `self.installLocation:downloadDirectory`.
      // TODO Set to DownloadFailed if failed.
      println!("Finished downloading {}", self.addon.name);
      self.verify();
  }

  fn verify(&mut self) {
      self.state = match self.state {
          InstallState::Downloading => InstallState::Verifying,
          _ => panic!("Invalid state transition"),
      }
      println!("Verifying {}...", self.addon.name);
      // TODO Actually verify from `self.installLocation:downloadDirectory`.
      // TODO Set to VerifyFailed if failed.
      println!("Finished verifying {}", self.addon.name);
      self.stage();
  }

  fn stage(&mut self) {
      self.state = match self.state {
          InstallState::Verified => InstallState::Installing,
          _ => panic!("Invalid state transition"),
      }
      println("Staging {}...", self.addon.name);
      // TODO Actually copy from `self.installLocation:downloadDirectory` to
      //      `self.installLocation:stageDirectory`.
      // TODO Set to StagingFailed if failed.
      println("Finished staging {}", self.addon.name);
      self.install();
  }

  fn install(&mut self) {
      self.state = match self.state {
          InstallState::Staged => InstallState::Installing,
          _ => panic!("Invalid state transition"),
      }
      println!("Installing {}...", self.addon.name);
      // TODO Actually install.
      // TODO set to InstallFailed if failed.
      println!("Finished verifying {}", self.addon.name);
      self.state = InstallState:Installed;
  }

  fn cancel(&mut self) {
      self.state = match self.state {
          InstallState:Downloading | InstallState:Downloaded => {
              println!("Stopping download...");
              println!("Remove downloaded files...");
          },
          InstallState:Verifying | InstallState:Verified => {
              println!("Stopping verification...");
          },
          InstallState:Postponing | InstallState:Postponed => {
              println!("Removing postponed install...");
          },
          InstallState:Resuming | InstallState:Resumed => {
              println!("Stop resuming install...");
              println!("Uninstall resumed install...");
          },
          InstallState:Staging | InstallState:Staged => {
              println!("Stopping staging...");
              println!("Remove staged files...")
          },
          InstallState:Installing | InstallState:Installed => {
              println!("Stopping install...");
              println!("Uninstalling installed addon...")
          },
          _ => panic!("Invalid state transition"),
      }
  }

  fn postpone(&mut self) {
      self.state = match self.state {
          InstallState::Verified => InstallState:Postponing,
          _ => panic!("Invalid state transition"),
      }
      println("Postponing addon...");
      // TODO set to PostponeFailed if failed.
      self.state = InstallState:Postponed;
  }

  fn resume(&mut self) {
      self.state = match self.state {
          InstallState::Postponed => InstallState:Resuming,
          _ => panic!("Invalid state transition"),
      }
      println("Resuming addon...");
      // TODO set to ResumeFailed if failed.
      self.state = InstallState:Resumed;
  }
}


struct InstallLocation {
    name: <u8>,
    baseDirectory: <u8>, // FIXME use real file type
    stageDirectory: <u8>, // FIXME use real file type
    downloadDirectory: <u8>, // FIXME use real file type
}

impl InstallLocation {
    fn new(name: <u8>, baseDirectory: <u8>) -> Self {
        println!("Initialized install location {} in {}", name, baseDirectory);
    }

    fn getStagingDirectory() {
        println!("Creating staging directory for install location {}", self.name);
        self.stageDirectory = self.baseDirectory.append("staging");
        // TODO wrap in a lock and release it when references drop.
        //      also remove directory when all references have dropped.
        return self.stageDirectory;
    }

    fn getDownloadDirectory() {
        println!("Creating download directory for install location {}", self.name);
        self.downloadDirectory = self.baseDirectory.append("download");
        // TODO wrap in a lock and release it when references drop.
        //      also remove directory when all references have dropped.
        return self.downloadDirectory;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn install_addon() {
        let manifest = {
            id: "@test-addon",
            name: "Test Addon",
            type: AddonTypes:WebExtension,
        }

        let installLocation = InstallLocation:new("profile", "c:\\Addons")
        let addon = Addon:new(manifest, installLocation);
        let addonInstall = AddonInstall:new(addon);

        // Start the install process from scratch.
        addonInstall.download();
    }
}
