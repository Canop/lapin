
use {
    anyhow::Result,
    crate::{
        fromage::NewCampaignCommand,
        persist::{
            self,
            Bag,
            SerdeFormat,
        },
    },
};

mod campaign;
mod loaded_campaign;

pub use campaign::Campaign;
pub use loaded_campaign::LoadedCampaign;

pub fn create(ncc: &NewCampaignCommand) -> Result<()> {
    let mut c = Campaign::default();
    c.name = "Unnamed Campaign".to_string();
    let bag = Bag::from(c);
    let format = ncc.output_format.as_ref()
        .and_then(|key| SerdeFormat::from_key(&key))
        .unwrap_or(SerdeFormat::Json);
    persist::write_file(
        &bag,
        &ncc.path,
        Some(format),
        true,
    )
}
