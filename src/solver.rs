use crate::items::{Item, ItemId, ItemManager, Production};
use clap::ValueEnum;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, ValueEnum, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    R0,
    P1,
    P2,
    P3,
    P4,
}

impl Tier {
    pub fn expected_cycle_count(&self) -> usize {
        use Tier::*;

        match self {
            R0 | P1 => 1,
            P2 => 2,
            P3 => 3,
            P4 => 4,
        }
    }
}

#[derive(Default, Debug)]
pub struct Builder {
    max_tier: Option<Tier>,
}

impl Builder {
    pub fn max_tier(mut self, tier: Option<Tier>) -> Self {
        self.max_tier = tier;
        self
    }

    pub fn build(self) -> Solver {
        Solver {
            max_tier: self.max_tier.unwrap_or(Tier::P4),
        }
    }
}

#[derive(Debug)]
pub struct Solver {
    max_tier: Tier,
}

impl Solver {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn solve<'a>(&self, planet: &Planet, item_manager: &'a ItemManager) -> Vec<Cycle<'a>> {
        let resource_ids: Vec<ItemId> = planet
            .resources
            .iter()
            .map(|res| res.item_id.clone())
            .collect();

        let mut available_inputs: HashSet<ItemId> = HashSet::new();
        available_inputs.extend(resource_ids.iter().cloned());

        let first_cycle = self.solve_items(&resource_ids, item_manager, &available_inputs);

        Self::add_available_inputs(&mut available_inputs, &first_cycle);

        let mut cycles: Vec<Cycle<'a>> = Vec::with_capacity(self.max_tier.expected_cycle_count());
        cycles.push(first_cycle);

        if self.max_tier > Tier::P1 {
            loop {
                // Unwrap should be safe here since we know we just computed the previous step. The
                // vec can never be empty.
                let input_ids: Vec<&ItemId> = cycles
                    .last()
                    .unwrap()
                    .steps
                    .iter()
                    .map(|step| &step.output.id)
                    .collect();

                if input_ids.is_empty() {
                    break;
                }

                let cycle = self.solve_items(&input_ids, item_manager, &available_inputs);

                if cycle.steps.is_empty() {
                    break;
                }

                Self::add_available_inputs(&mut available_inputs, &cycle);

                let tier = cycle.get_tier();

                cycles.push(cycle);

                if tier.is_none() || tier.is_some_and(|v| v >= self.max_tier) {
                    break;
                }
            }
        }

        cycles
    }

    fn solve_items<'a, Id>(
        &self,
        input_ids: &[Id],
        item_manager: &'a ItemManager,
        available_inputs: &HashSet<ItemId>,
    ) -> Cycle<'a>
        where
            Id: AsRef<str>,
    {
        let mut result = Cycle::default();
        let mut added: HashSet<&ItemId> = HashSet::new();

        for input_id in input_ids {
            let input_id = input_id.as_ref();
            let used_in = item_manager.find_products(input_id);

            for output in used_in {
                if added.contains(&output.id) {
                    continue;
                }

                let Some(production) = &output.production else {
                    continue;
                };

                // If we haven't encountered an input used for this good, we aren't ready to
                // examine it yet, or it doesn't exist. Either way, we need to skip it for now.
                let missing_inputs: Vec<_> = production
                    .inputs
                    .keys()
                    .filter(|id| !available_inputs.contains(*id))
                    .collect();

                if !missing_inputs.is_empty() {
                    continue;
                }

                added.insert(&output.id);
                result.steps.push(Step { output, production });
            }
        }

        result
    }

    fn add_available_inputs(inputs: &mut HashSet<ItemId>, cycle: &Cycle) {
        inputs.extend(cycle.steps.iter().map(|step| step.output.id.clone()))
    }
}

#[derive(Debug, Default)]
pub struct Cycle<'a> {
    pub steps: Vec<Step<'a>>,
}

impl Cycle<'_> {
    pub fn get_tier(&self) -> Option<Tier> {
        self.steps.first().map(|first| first.output.tier)
    }
}

#[derive(Debug)]
pub struct Step<'a> {
    pub output: &'a Item,
    pub production: &'a Production,
}

impl Step<'_> {
    pub fn get_combined_input_label(&self, item_manager: &ItemManager) -> String {
        self.production
            .inputs
            .keys()
            .enumerate()
            .fold(String::new(), |mut output, (index, key)| {
                // unwrap should be safe here since items are checked during solving
                let item = item_manager.get(key).unwrap();

                if index > 0 {
                    output.push_str(" + ");
                }

                output.push_str(&item.label);
                output
            })
    }
}

pub type ResourceList = [Resource; 5];

#[derive(Debug, Default)]
pub struct System {
    planets: Vec<Planet>,
}

#[derive(Debug)]
pub struct Planet {
    pub label: String,
    pub resources: ResourceList,
}

impl Planet {
    pub fn new<Label>(label: Label, resources: ResourceList) -> Self
        where
            Label: ToString,
    {
        Self {
            label: label.to_string(),
            resources,
        }
    }
}

#[derive(Debug)]
pub struct Resource {
    pub item_id: ItemId,
    pub ratio: f32,
}

impl Resource {
    pub fn new<Id>(item_id: Id, ratio: f32) -> Self
        where
            Id: ToString,
    {
        Self {
            item_id: item_id.to_string(),
            ratio,
        }
    }
}
