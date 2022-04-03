use zbus::Result;

pub struct FileManager1Proxy<'a>(zbus::Proxy<'a>);

impl<'a> FileManager1Proxy<'a> {
    pub async fn new(connection: &zbus::Connection) -> Result<FileManager1Proxy<'a>> {
        let proxy = zbus::ProxyBuilder::new_bare(connection)
            .interface("org.freedesktop.FileManager1")?
            .path("/org/freedesktop/FileManager1")?
            .destination("org.freedesktop.FileManager1")?
            .build()
            .await?;
        Ok(Self(proxy))
    }

    pub fn inner(&self) -> &zbus::Proxy<'_> {
        &self.0
    }

    #[doc(alias = "ShowFolders")]
    pub async fn show_folders(&self, uris: &[&str], startup_id: &str) -> Result<()> {
        self.inner()
            .call_method("ShowFolders", &(uris, startup_id))
            .await?;
        Ok(())
    }

    #[doc(alias = "ShowItems")]
    pub async fn show_items(&self, uris: &[&str], startup_id: &str) -> Result<()> {
        self.inner()
            .call_method("ShowItems", &(uris, startup_id))
            .await?;
        Ok(())
    }

    #[doc(alias = "ShowItemProperties")]
    pub async fn show_item_properties(&self, uris: &[&str], startup_id: &str) -> Result<()> {
        self.inner()
            .call_method("ShowItemProperties", &(uris, startup_id))
            .await?;
        Ok(())
    }
}
