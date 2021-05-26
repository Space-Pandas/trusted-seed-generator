use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "seed-generator")]
pub struct Opt {
    #[structopt(short, long, default_value = "1")]
    pub chain_id: u32,

    #[structopt(short, long)]
    pub mint_id: u32,
}
