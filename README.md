# `virtualbox_rs`
is a Rust library for interacting with VirtualBox through its SDK.

 This library provides a comprehensive set of bindings to manage VirtualBox VMs, including
creating, deleting, and modifying virtual machines, handling snapshots, and configuring
various VM settings such as network adapters, audio settings, and encryption.

 # Features

 - Create and manage VirtualBox VMs
 - Handle VM snapshots
 - Configure VM settings (network, audio, encryption, etc.)
 - Support for different VirtualBox API versions (6.1 and 7.0)
!
 # Example

```rust
use virtualbox_rs::{Session, VirtualBox};
use virtualbox_rs::enums::SessionType;

fn main() {
    let vbox = VirtualBox::init().unwrap();
    let mut session = Session::init().unwrap();
    let machine = vbox.find_machines("Freebsd_14").unwrap();
    machine.lock_machine(&mut session, SessionType::Shared).unwrap();

    let machine_mut = session.get_machine().unwrap();
    let progress = machine_mut.delete_snapshot_and_all_children("278ef54a-2e75-4aba-b212-551af4c69725").unwrap();
    progress.wait_for_completion(-1).unwrap();
}
 ```

 # Supported Platforms

This library has been tested on:
 - FreeBSD 14.0
 - Ubuntu 24.04

 # Supported VirtualBox Versions

 This library supports VirtualBox versions 6.1, 7.0, and 7.1. By default, the library is built for version 7.1.
 To change the target version, enable the corresponding feature flag: `v7_1`, `v7_0`, or `v6_1`.
 If multiple versions are specified, the library will be built for the highest version available.

 **Important:** Running the library with an incompatible VirtualBox version is likely to result in a `Segmentation fault`.
 Before using the library, ensure compatibility by invoking `VirtualBoxClient::check_version().unwrap();`.
 If the versions do not match, the program should terminate immediately.

 # Note

 Not everything has been implemented yet. The library emphasizes providing examples for each method, which should simplify its usage.
