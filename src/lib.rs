#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

pub use zbus;

mod proxy;
pub use proxy::FileManager1;
