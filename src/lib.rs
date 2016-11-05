extern crate hyper;
extern crate semver;

pub mod addon_manager {
    use std::fs;
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::PathBuf;

    use hyper::client::Client;
    use hyper::Url;
    use semver::Version;

    /// Possible types of add-ons.
    pub enum AddonType {
        NativeExtension,
        WebExtension,
        Theme,
        Plugin,
        Service,
    }

    /// Possible install states.
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
        Resuming,
        Resumed,
        ResumeFailed,
        Staging,
        Staged,
        StagingFailed,
        Installing,
        Installed,
        InstallFailed,
        Canceling,
        Canceled,
        CancelFailed,
        Uninstalling,
        Uninstalled,
        UninstallFailed,
    }

    /// Possible names for InstallLocations.
    pub enum InstallLocationName {
        Profile,
        Application,
        System,
    }

    /// A Manifest describes an available Add-on.
    pub struct Manifest {
        pub id: String,
        pub name: String,
        pub version: Version,
        pub addon_type: AddonType,
        pub url: Url,
    }

    impl Manifest {
        pub fn new(id: String, name: String, version: Version, addon_type: AddonType,
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

    /// An InstallLocation points to the area on the filesystem to store installed add-ons,
    /// and controls access to staging and download areas.
    pub struct InstallLocation {
        pub name: InstallLocationName,
        pub base_directory: String, // FIXME use real file type
        download_directory: String, // FIXME use real file type
        staging_directory: String, // FIXME use real file type
    }

    impl InstallLocation {
        pub fn new(name: InstallLocationName, base_directory: String) -> Self {
            InstallLocation {
                name: name,
                base_directory: base_directory,
                download_directory: String::from("download"),
                staging_directory: String::from("staging"),
            }
        }

        pub fn get_download_directory(&mut self) -> String {
            // FIXME make `InstallLocationName`s printable
            //println!("Creating download directory for install location {}", self.name);
            // TODO wrap in a lock and release it when references drop.
            //      also remove directory when all references have drop
            return self.base_directory.clone() + "/" + self.download_directory.as_str();
        }

        pub fn get_staging_directory(&mut self) -> String {
            // FIXME make `InstallLocationName`s printable
            //println!("Creating staging directory for install location {}", self.name);
            // TODO wrap in a lock and release it when references drop.
            //      also remove directory when all references have dropped.
            return self.base_directory.clone() + "/" + self.staging_directory.as_str();
        }
    }

    /// An Addon represents an individual addon.
    pub struct Addon {
        pub id: String,
        pub name: String,
        pub version: Version,
        pub install_url: Url,
        pub install_location: InstallLocation,
        pub source_uri: PathBuf,
    }

    impl Addon {
        pub fn new(manifest: Manifest, install_location: InstallLocation) -> Self {
            Addon {
                id: manifest.id,
                name: manifest.name,
                version: manifest.version,
                install_url: manifest.url,
                install_location: install_location,
                source_uri: PathBuf::new(),
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

          let client = Client::new();
          let url = self.addon.install_url.clone();
          let mut response = match client.get(url).send() {
              Ok(response) => response,
              Err(err) => {
                  self.state = InstallState::DownloadFailed;
                  return;
              },
          };

          let mut buf = String::new();
          match response.read_to_string(&mut buf) {
              Ok(_) => (),
              Err(_) => self.state = InstallState::DownloadFailed,
          };

          let download_directory = self.addon.install_location.get_download_directory();
          let mut path = PathBuf::from(download_directory);

          println!("About to create download dir {:?}", path);
          match fs::create_dir_all(&path) {
              Ok(_) => (),
              Err(err) => {
                  self.state = InstallState::DownloadFailed;
              },
          }

          let filename = self.addon.id.clone() + ".xpi";
          path.push(filename);

          let mut file = match File::create(&path) {
              Ok(f) => (f),
              Err(err) => {
                self.state = InstallState::DownloadFailed;
                return;
              }
          };

          match file.write_all(buf.as_bytes()) {
              Ok(_) => (),
              Err(err) => self.state = InstallState::DownloadFailed,
          };

          println!("Finished downloading {}", self.addon.name);
          self.state = match self.state {
              InstallState::Downloading => InstallState::Downloaded,
              _ => panic!("Invalid state transition"),
          };

          self.addon.source_uri = path;

          self.verify();
      }

      fn verify(&mut self) {
          self.state = match self.state {
              InstallState::Downloaded => InstallState::Verifying,
              _ => panic!("Invalid state transition"),
          };
          println!("Verifying {}...", self.addon.name);

          // TODO actually verify
          println!("File: {:?}", self.addon.source_uri);

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
          let file = self.addon.source_uri.clone();
          let staging_directory = self.addon.install_location.get_staging_directory();

          let mut path = PathBuf::from(staging_directory);

          println!("About to create staging dir {:?}", path);
          match fs::create_dir_all(&path) {
              Ok(_) => (),
              Err(err) => {
                  self.state = InstallState::StagingFailed;
                  return;
              },
          }

          let filename = self.addon.id.clone() + ".xpi";
          path.push(filename);

          println!("About to copy {:?} to {:?}", file, path.to_str());
          fs::copy(file, path).unwrap();

          // TODO Set to StagingFailed if failed.
          println!("Finished staging {}", self.addon.name);
          self.state = match self.state {
              InstallState::Staging => InstallState::Staged,
              _ => panic!("Invalid state transition"),
          };
          self.install();
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
                  InstallState::Canceled
              },
              InstallState::Verifying | InstallState::Verified => {
                  println!("Stopping verification...");
                  InstallState::Canceled
              },
              InstallState::Postponing | InstallState::Postponed => {
                  println!("Removing postponed install...");
                  InstallState::Canceled
              },
              InstallState::Resuming | InstallState::Resumed => {
                  println!("Stop resuming install...");
                  println!("Uninstall resumed install...");
                  InstallState::Canceled
              },
              InstallState::Staging | InstallState::Staged => {
                  println!("Stopping staging...");
                  println!("Remove staged files...");
                  InstallState::Canceled
              },
              InstallState::Installing | InstallState::Installed => {
                  println!("Stopping install...");
                  println!("Uninstalling installed addon...");
                  InstallState::Canceled
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
        use hyper::Url;
        use semver::Version;

        use super::Addon;
        use super::AddonType;
        use super::InstallLocation;
        use super::InstallLocationName;
        use super::Install;
        use super::InstallState;
        use super::Manifest;

        #[test]
        fn install_addon() {
            let id = String::from("@test123");
            let name = String::from("Test Addon");
            let version = Version::parse("0.0.1").unwrap();
            let addon_type = AddonType::WebExtension;
            // FIXME need to mock hyper
            let url = Url::parse("http://localhost:8080").unwrap();
            let manifest = Manifest::new(id, name, version, addon_type, url);

            let name = InstallLocationName::Profile;
            // TODO use std::fs::DirEntry instead
            // TODO need to figure out how to mock it...
            let base_directory = String::from("./test/tmp");
            let install_location = InstallLocation::new(name, base_directory);

            let addon = Addon::new(manifest, install_location);

            let mut install = Install::new(addon);

            // Start the install process from scratch.
            install.download();

            match install.state {
                InstallState::Installed => {},
                _ => panic!("Unexpected install state"),
            }
        }
    }
}
