use crate::cli::Cli;
use crate::items::ItemManager;
use crate::solver::{Planet, Resource, Solver};
use clap::Parser;
use log::debug;
use std::path::PathBuf;

mod cli;
mod data;
mod items;
mod solver;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("item.{0}")]
    Item(#[from] items::ItemError),
}

fn main() -> Result<(), Error> {
    env_logger::init();

    let cli = Cli::parse();
    debug!("{:?}", cli);

    let items_file = cli
        .items_file
        .unwrap_or_else(|| PathBuf::from("./items.yaml"));

    let item_manager = ItemManager::new(&items_file)?;
    let solver = Solver::builder().build();

    let planet = Planet::new(
        "J103326 IV",
        [
            Resource::new("aqueous_liquids", 0.39),
            Resource::new("base_metals", 0.63),
            Resource::new("carbon_compounds", 0.68),
            Resource::new("microorganisms", 0.67),
            Resource::new("noble_metals", 0.42),
        ],
    );

    let cycles = solver.solve(&planet, &item_manager);

    for (index, cycle) in cycles.into_iter().enumerate() {
        println!("Cycle #{}", index + 1);

        for step in cycle.steps {
            let input_labels = step.get_combined_input_label(&item_manager);
            println!("\t{input_labels} > {}", step.output.label);
        }

        println!();
    }

    Ok(())
}
