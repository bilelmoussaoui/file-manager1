#![deny(missing_docs)]

//! Zbus wrapper for the [file manager DBus interface](https://www.freedesktop.org/wiki/Specifications/file-manager-interface/)
mod proxy;

pub use proxy::FileManager1;
