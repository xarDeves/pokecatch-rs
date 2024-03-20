//TODO: implement correct --help command
//TODO: remove commands that have '--' or make all of them work with '--'

/* ------------------------------ commands ------------------------------ */
//pokecatch throw <ball name ie pokeball>
//pokecatch pc show --caught (with sprites or not, percentage and amount of caught)
//pokecatch pc show --uncaught (with sprites or not, percentage and amount of uncaught)
//pokecatch bag show
//pokecatch bag use item

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Whether to hide the pokemon's name which appears above it
    #[arg(long, default_value_t = false)]
    pub hide_name: bool,

    /// The form of the pokemon
    #[arg(short, long, default_value = "")]
    pub form: String,

    /// Display the pokemon as it's mega form
    #[arg(short, long, default_value_t = false)]
    pub mega: bool,

    /// Display the pokemon as it's mega X form
    #[arg(long, default_value_t = false)]
    pub mega_x: bool,

    /// Display the pokemon as it's mega Y form
    #[arg(long, default_value_t = false)]
    pub mega_y: bool,

    /// Display the pokemon as shiny
    #[arg(short, long, default_value_t = false)]
    pub shiny: bool,

    /// Display the alolan variant of the pokemon
    #[arg(short, long, default_value_t = false)]
    pub alolan: bool,

    /// Display the gigantamax variant of the pokemon
    #[arg(short, long, default_value_t = false)]
    pub gmax: bool,

    /// Display the hisui variant of the pokemon
    #[arg(long, default_value_t = false)]
    pub hisui: bool,

    /// Display the noble variant of the pokemon, this option often times only works in tandom with --hisui.
    #[arg(short, long, default_value_t = false)]
    pub noble: bool,

    /// Display the galarian variant of the pokemon
    #[arg(long, default_value_t = false)]
    pub galar: bool,

    /// Display the female variant of the pokemon if it exists. This doesn't apply to nidoran, for some reason.
    #[arg(long, default_value_t = false)]
    pub female: bool,

    #[clap(subcommand)]
    pub subcommand: Option<SubCommand>,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    Throw {
        #[clap()]
        ball: String,
    },

    Pc {
        #[clap(subcommand)]
        pc_subcommand: Option<PcSubCommand>,
    },

    Bag {
        #[clap(subcommand)]
        bag_subcommand: Option<BagSubCommand>,
    },
}

#[derive(Parser, Debug)]
pub enum PcSubCommand {
    #[clap(name = "show")]
    Show(PcShowArgs),
}

#[derive(Parser, Debug)]
pub struct PcShowArgs {
    /// Show all pokemon (caught and uncaught)
    #[clap(short, long, name = "all")]
    pub all: bool,

    /// Show caught pokemon
    #[clap(short, long, name = "caught")]
    pub caught: bool,

    /// Show uncaught pokemon
    #[clap(short, long, name = "uncaught")]
    pub uncaught: bool,
}

#[derive(Parser, Debug)]
pub enum BagSubCommand {
    #[clap(name = "show")]
    Show,

    #[clap(name = "use")]
    Use(BagUseArgs),

    #[clap(name = "pokeballs")]
    Pokeballs,
}

#[derive(Parser, Debug)]
pub struct BagUseArgs {
    /// Index or item name
    #[clap()]
    pub items: Vec<String>,
}