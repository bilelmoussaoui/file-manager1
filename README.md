# file-manager1

Rust wrapper for the [File Manager DBus interface](https://www.freedesktop.org/wiki/Specifications/file-manager-interface/) using [zbus](https://lib.rs/zbus).

 # Examples

 ```rust
 use file_manager1::{FileManager1, zbus};

 async fn run() -> zbus::Result<()> {
     let cnx = zbus::Connection::session().await?;
     FileManager1::new(&cnx).await?
         .show_folders(&["file://home/bilelmoussaoui/Downloads"], "")
         .await?;
     Ok(())
 }
 ```