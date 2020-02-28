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
    pub sub: Option<SubCommand>,

}

#[derive(FromArgs, PartialEq, Debug, Clone)]
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
    pub fn output_format(&self) -> Option<String> {
        match &self.sub {
            Some(SubCommand::Edit(sub)) => sub.output_format.clone(),
            Some(SubCommand::Test(sub)) => sub.output_format.clone(),
            _ => None,
        }
    }
}

#[derive(FromArgs, PartialEq, Debug, Default, Clone)]
/// play a game
#[argh(subcommand, name = "play")]
pub struct PlaySubCommand {
    #[argh(positional)]
    /// optional path to a level file
    pub path: Option<PathBuf>,
}

#[derive(FromArgs, PartialEq, Eq, Debug, Clone)]
/// edit a level
#[argh(subcommand, name = "edit")]
pub struct EditSubCommand {

    /// format of the written level file (same as input if not precised)
    #[argh(option, short='f')]
    pub output_format: Option<String>, // argh doesn't support enum as values :(

    #[argh(positional)]
    /// path to the level to create or modify
    pub path: PathBuf,
}

#[derive(FromArgs, PartialEq, Eq, Debug, Clone)]
/// does a special test for the dev
#[argh(subcommand, name = "test")]
pub struct TestSubCommand {

    /// format of the written level file (same as input if not precised)
    #[argh(option, short='f')]
    pub output_format: Option<String>, // argh doesn't support enum as values :(

}



