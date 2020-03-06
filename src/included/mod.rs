
use {
    anyhow::Result,
    crate::{
        campaign::LoadedCampaign,
        persist::*,
    },
};

/// provide the default campaign included with the bag
pub fn loaded_campaign() -> Result<LoadedCampaign> {
    LoadedCampaign::from_packed_bag(
        read_bytes(
            include_bytes!("../../std/discovery-campaign.mpack"),
            SerdeFormat::MessagePack,
        )?
    )
}
