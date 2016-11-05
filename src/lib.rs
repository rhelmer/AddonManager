extern crate url;

pub mod addon_manager {
    use url::Url;

    pub enum AddonType {
        NativeExtension,
        WebExtension,
        Theme,
        Plugin,
        Service,
    }

    pub enum InstallState  {
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
    pub struct Manifest {
        pub id: String,
        pub name: String,
        pub version: String,
        pub addon_type: AddonType,
        pub url: Url,
    }

    impl Manifest {
        pub fn new(id: String, name: String, version: String, addon_type: AddonType,
                   url: Url) -> Self {
            Manifest {
                id: id,
                name: name,
                version: version,
                addon_type: addon_type,
                url: url,
            }
        }
    }

    pub struct InstallLocation {
        pub name: String,
        pub base_directory: String, // FIXME use real file type
    }

    impl InstallLocation {
        pub fn new(name: String, base_directory: String) -> Self {
            println!("Initialized install location {} in {}", name, base_directory);
            InstallLocation {
                name: name,
                base_directory: base_directory,
            }
        }

        pub fn get_download_directory(&mut self) -> String {
            println!("Creating download directory for install location {}", self.name);
            //self.downloadDirectory = self.base_directory.append("download");
            // TODO wrap in a lock and release it when references drop.
            //      also remove directory when all references have dropped.
            return String::from("downloaddir");
        }

        pub fn get_staging_directory(&mut self) -> String {
            println!("Creating staging directory for install location {}", self.name);
            //self.stageDirectory = self.base_directory.append("staging");
            // TODO wrap in a lock and release it when references drop.
            //      also remove directory when all references have dropped.
            return String::from("stagedir");
        }
    }

    /// An Addon represents an individual addon.
    pub struct Addon {
        pub id: String,
        pub name: String,
        pub version: String,
        pub install_url: Url,
        pub install_location: InstallLocation,
        pub source_uri: String,
    }

    impl Addon {
        //fn new(manifest: Vec<u8>, type, install_location: install_location) -> Addon {
        pub fn new(manifest: Manifest, install_location: InstallLocation) -> Self {
            Addon {
                id: manifest.id,
                name: manifest.name,
                version: manifest.version,
                install_url: manifest.url,
                install_location: install_location,
                source_uri: String::from(""),
            }
        }
    }

    /// Install downloads, verifies, and installs an Addon.
    pub struct Install {
        pub state: InstallState,
        pub addon: Addon,
    }

    impl Install {
      pub fn new(addon: Addon) -> Self {
          Install {
              state: InstallState::Available,
              addon: addon,
          }
      }

      pub fn download(&mut self) {
          self.state = match self.state {
              InstallState::Available => InstallState::Downloading,
              _ => panic!("Invalid state transition"),
          };
          println!("Downloading {}...", self.addon.name);
          self.addon.source_uri = self.addon.install_location.get_download_directory();
          // TODO Actually download to `self.addon.download_directory`.
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
          // TODO Actually verify from `sefl.addon.download_irectory`.
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
          // TODO Actually copy from `downloadDirectory` to `stageDirectory`.
          self.addon.source_uri = self.addon.install_location.get_staging_directory();
          // TODO Set to StagingFailed if failed.
          println!("Finished staging {}", self.addon.name);
          self.state = match self.state {
              InstallState::Staging => InstallState::Staged,
              _ => panic!("Invalid state transition"),
          };
      }

      pub fn install(&mut self) {
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

      pub fn cancel(&mut self) {
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

      pub fn postpone(&mut self) {
          self.state = match self.state {
              InstallState::Verified => InstallState::Postponing,
              _ => panic!("Invalid state transition"),
          };
          println!("Postponing addon...");
          // TODO set to PostponeFailed if failed.
          self.state = InstallState::Postponed;
      }

      pub fn resume(&mut self) {
          self.state = match self.state {
              InstallState::Postponed => InstallState::Resuming,
              _ => panic!("Invalid state transition"),
          };
          println!("Resuming addon...");
          // TODO set to ResumeFailed if failed.
          self.state = InstallState::Resumed;
      }
    }

    #[cfg(test)]
    mod tests {
        use super::Manifest;
        use super::Addon;
        use super::AddonType;
        use super::InstallLocation;
        use super::Install;
        use url::Url;

        #[test]
        fn install_addon() {
            let id = String::from("@test123");
            let name = String::from("Test Addon");
            let version = String::from("0.1");
            let addon_type = AddonType::WebExtension;
            let url = Url::parse("data:text/plain,Hello?World#").unwrap();
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
}
