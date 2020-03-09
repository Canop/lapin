//! Definition of the launch arguments of the lapin program
//!
use {
    argh::FromArgs,
    std::{
        path::PathBuf,
    },
};

#[derive(FromArgs, PartialEq, Debug, Clone)]
/// A game I make under supervision of my kids
pub struct Fromage {

    #[argh(subcommand)]
    pub command: Option<Command>, // TODO rename as "command" / "Command" ?

}

/// One of the root commands of Lapin
#[derive(FromArgs, PartialEq, Debug, Clone)]
#[argh(subcommand)]
pub enum Command {
    Play(PlayCommand),
    Edit(EditCommand),
    Campaign(CampaignCommand),
}

#[derive(FromArgs, PartialEq, Debug, Default, Clone)]
/// play a campaign or level
#[argh(subcommand, name = "play")]
pub struct PlayCommand {
    #[argh(positional)]
    /// optional path to a level file
    pub path: Option<PathBuf>,
}

#[derive(FromArgs, PartialEq, Eq, Debug, Clone)]
/// create/edit a level
#[argh(subcommand, name = "edit")]
pub struct EditCommand {

    /// format of the written level file (same as input if not precised)
    #[argh(option, short='f')]
    pub output_format: Option<String>, // argh doesn't support enum as values :(

    #[argh(positional)]
    /// path to the level to create or modify
    pub path: PathBuf,
}

#[derive(FromArgs, PartialEq, Eq, Debug, Clone)]
/// campaign building/packing tool
#[argh(subcommand, name = "campaign")]
pub struct CampaignCommand {

    #[argh(subcommand)]
    pub sub: CampaignSubCommand,

}

/// various operations done on campaigns
#[derive(FromArgs, PartialEq, Eq, Debug, Clone)]
#[argh(subcommand)]
pub enum CampaignSubCommand {
    New(NewCampaignCommand),
    Pack(PackCampaignCommand),
}

#[derive(FromArgs, PartialEq, Eq, Debug, Clone)]
/// create a campaign file
#[argh(subcommand, name = "new")]
pub struct NewCampaignCommand {

    /// format of the written level file (same as input if not precised)
    #[argh(option, short='f')]
    pub output_format: Option<String>, // argh doesn't support enum as values :(

    #[argh(positional)]
    /// path to the file to create
    pub path: PathBuf,

}

#[derive(FromArgs, PartialEq, Eq, Debug, Clone)]
/// pack all levels into a campaign file
#[argh(subcommand, name = "pack")]
pub struct PackCampaignCommand {

    #[argh(positional)]
    /// path to the unpacked file
    pub unpacked_path: PathBuf,

    #[argh(positional)]
    /// path to the packed file to create
    pub packed_path: PathBuf,

}
