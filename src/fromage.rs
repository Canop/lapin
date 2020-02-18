use {
    argh::FromArgs,
    std::{
        path::PathBuf,
    },
};

#[derive(FromArgs, PartialEq, Debug)]
/// A game I make under supervision of my kids
pub struct Fromage {
    #[argh(subcommand)]
    pub sub: Option<SubCommand>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommand {
    Play(PlaySubCommand),
    Edit(EditSubCommand),
    Test(TestSubCommand),
}

impl Fromage {
    pub fn is_test(&self) -> bool {
        match self.sub {
            Some(SubCommand::Test(_)) => true,
            _ => false,
        }
    }
}

#[derive(FromArgs, PartialEq, Debug, Default)]
/// play a game
#[argh(subcommand, name = "play")]
pub struct PlaySubCommand {
    #[argh(positional)]
    /// optional path to a level file
    pub path: Option<PathBuf>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// edit a level
#[argh(subcommand, name = "edit")]
pub struct EditSubCommand {
    #[argh(positional)]
    /// path to the level to create or modify
    pub path: PathBuf,
}

#[derive(FromArgs, PartialEq, Debug)]
/// does a special test for the dev
#[argh(subcommand, name = "test")]
pub struct TestSubCommand {
}

