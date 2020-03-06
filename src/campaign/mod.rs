
use {
    anyhow::Result,
    crate::{
        app::{
            NewCampaignCommand,
            PackCampaignCommand,
        },
        persist::{
            self,
            Bag,
            SerdeFormat,
        },
    },
};

mod campaign;
mod loaded_campaign;

pub use {
    campaign::Campaign,
    loaded_campaign::*,
};

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

/// write a bag containing a campaign and its levels
/// (which are found preferably in external files and
/// in the initial bag if not found externally)
pub fn pack(pcc: &PackCampaignCommand) -> Result<()> {
    let mut in_bag: Bag = persist::read_file(&pcc.unpacked_path)?;
    if !in_bag.is_campaign() {
        Err(anyhow!("no campaign found in bag"))?
    }
    let loaded_campaign = LoadedCampaign::load(
        &pcc.unpacked_path,
        in_bag,
        LoadOrigin::External,
    )?;
    let mut out_bag = Bag::from(loaded_campaign.campaign.clone());
    let len = loaded_campaign.campaign.levels.len();
    for i in 0..len {
        out_bag.levels.insert(
            loaded_campaign.campaign.levels[i].to_string(),
            loaded_campaign.levels[i].level.clone(),
        );
    }
    persist::write_file(
        &out_bag,
        &pcc.packed_path,
        None,
        false,
    )
}
