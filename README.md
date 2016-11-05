AddonManager
============

This is a replacement for the Firefox AddonManager, written in Rust.

Add-ons are components that can be hot-loaded into Firefox, such as:

* WebExtensions
* Native Firefox Extensions
* Themes
* Services

AddonManager is responsible for:

* parsing manifests
* installing add-ons (download, verify, stage, install)

The API is easy to use:

```rust
use addonmanager::{Manifest, InstallLocation, Addon, Install}

fn main() {
  // This could come from a remote server or a local file.
  let manifest = Manifest::new(...);

  // The place to install add-ons.
  let install_location = InstallLocation::new(...);

  // Represents the addon itself.
  let addon = Addon::new(manifest, install_location);

  // The state machine that downloads, verifies, installs
  // the Addon to the InstallLocation.
  let mut install = Install::new(addon);
  
  // Start the install process.
  install.download();
}
```

Running
-------

```bash
$ cargo run
```

Testing
-------

```bash
$ cargo test
````
