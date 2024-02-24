use crate::solver::Tier;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub from_file: Option<PathBuf>,

    #[arg(short, long, value_enum, value_name = "TIER")]
    pub max_tier: Option<Tier>,

    #[arg(long, value_name = "FILE")]
    pub items_file: Option<PathBuf>,
}
