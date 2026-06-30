use baad_shared::{CatalogError, Downloads};

#[allow(async_fn_in_trait)]
pub trait Catalog {
    type Resources;

    async fn fetch_resources(&self) -> Result<(String, Self::Resources), CatalogError>;

    fn build_downloads(&self, resources: Self::Resources, base_or_url: &str) -> Downloads;

    async fn prepare_downloads(&self) -> Result<Downloads, CatalogError> {
        let (base_or_url, resources) = self.fetch_resources().await?;
        Ok(self.build_downloads(resources, &base_or_url))
    }
}
