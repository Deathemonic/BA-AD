use baad_shared::Downloads;
use crate::errors::CatalogError;
use baad_utils::info;

#[allow(async_fn_in_trait)]
pub trait Catalog {
    type Resources;

    async fn fetch_resources(&self) -> Result<(String, Self::Resources, bool), CatalogError>;

    fn build_downloads(&self, resources: Self::Resources, base_or_url: &str) -> Downloads;

    async fn prepare_downloads(&self) -> Result<Downloads, CatalogError> {
        let (base_or_url, resources, up_to_date) = self.fetch_resources().await?;

        if up_to_date {
            info!("Catalog up to date");
        } else {
            info!(success = true, "Catalog fetched successfully");
        }

        Ok(self.build_downloads(resources, &base_or_url))
    }
}
