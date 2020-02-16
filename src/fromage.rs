
use argh::FromArgs;

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

#[derive(FromArgs, PartialEq, Debug)]
/// play a game
#[argh(subcommand, name = "play")]
pub struct PlaySubCommand {
}

#[derive(FromArgs, PartialEq, Debug)]
/// edit a level
#[argh(subcommand, name = "edit")]
pub struct EditSubCommand {
}

#[derive(FromArgs, PartialEq, Debug)]
/// does a special test for the dev
#[argh(subcommand, name = "test")]
pub struct TestSubCommand {
}

