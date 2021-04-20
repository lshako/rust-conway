use structopt::StructOpt;

#[derive(StructOpt, Debug, Copy, Clone)]
pub struct Opt {
    #[structopt(short, long, default_value = "30")]
    pub size: usize,

    #[structopt(short, long, default_value = "30")]
    pub concentration: i32,

    #[structopt(short, long, default_value = "10")]
    pub fps: u32,

    #[structopt(subcommand)]
    pub game_type: GameType,
}

#[derive(StructOpt, Debug, Copy, Clone)]
pub enum GameType {
    Classic,
    Immigration {
        #[structopt(long, short, default_value = "50")]
        type1_concentration: i32,
    },
}
