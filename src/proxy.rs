use zbus::Result;

/// A wrapper of the `org.freedesktop.FileManager1` DBus interface.
pub struct FileManager1<'a>(zbus::Proxy<'a>);

impl<'a> FileManager1<'a> {
    /// Create a new instance of FileManager1.
    pub async fn new(connection: &zbus::Connection) -> Result<FileManager1<'a>> {
        let proxy = zbus::ProxyBuilder::new_bare(connection)
            .interface("org.freedesktop.FileManager1")?
            .path("/org/freedesktop/FileManager1")?
            .destination("org.freedesktop.FileManager1")?
            .build()
            .await?;
        Ok(Self(proxy))
    }

    /// Get a reference to the underlying Proxy.
    pub fn inner(&self) -> &zbus::Proxy<'_> {
        &self.0
    }

    #[doc(alias = "ShowFolders")]
    /// Show a or multiple folders
    pub async fn show_folders(&self, uris: &[&str], startup_id: &str) -> Result<()> {
        self.inner()
            .call_method("ShowFolders", &(uris, startup_id))
            .await?;
        Ok(())
    }

    #[doc(alias = "ShowItems")]
    /// Select a or multiple files/folders.
    pub async fn show_items(&self, uris: &[&str], startup_id: &str) -> Result<()> {
        self.inner()
            .call_method("ShowItems", &(uris, startup_id))
            .await?;
        Ok(())
    }

    #[doc(alias = "ShowItemProperties")]
    /// Show the file/folder properties
    pub async fn show_item_properties(&self, uris: &[&str], startup_id: &str) -> Result<()> {
        self.inner()
            .call_method("ShowItemProperties", &(uris, startup_id))
            .await?;
        Ok(())
    }

    #[doc(alias = "OpenLocations")]
    /// Returns a list of opened URIs
    pub async fn open_locations(&self) -> Result<Vec<String>> {
        self.inner().get_property("OpenLocations").await
    }

    #[doc(alias = "OpenWindowsWithLocations")]
    /// Returns a list of StartupID and the corresponding URIs
    pub async fn open_windows_with_locations(&self) -> Result<Vec<(String, Vec<String>)>> {
        self.inner().get_property("OpenWindowsWithLocations").await
    }
}
