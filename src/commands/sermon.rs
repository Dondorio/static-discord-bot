use rand::{Rng, SeedableRng, rng, rngs::SmallRng};
use serde::Deserialize;
use std::{collections::HashMap, fs};
use strfmt::{Formatter, strfmt_map};

use crate::commands::*;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
struct Completions {
    pronoun_pos: Vec<String>,
    pronoun_nom: Vec<String>,
    strength: Vec<String>,
    interaction: Vec<String>,
    static_verb: Vec<String>,

    sentence_object: Vec<String>,
    sentence_object_good: Vec<String>,
    sentence_object_bad: Vec<String>,

    static_feeling_good: Vec<String>,
    static_feeling_bad: Vec<String>,
    donation: Vec<String>,

    foo: String,
}

fn generate_sermon() -> Result<String, Error> {
    let len = rand::random_range(5..13);
    // TODO move phrases to toml file
    const PHRASES: [&str; 9] = [
        "The Static is {pronoun_pos} {strength}, {strength} and {strength}. ",
        "The Static is {pronoun_pos} {strength} and {strength}. ",
        "The Static is {pronoun_pos} {strength}. ",
        "In The Static {pronoun_nom} take {interaction}. ",
        "The Static {static_verb} {sentence_object}. ",
        "The Static {static_feeling_good} {sentence_object_good}. ",
        "The Static {static_feeling_good} {sentence_object_good} and {sentence_object_good}. ",
        "The Static {static_feeling_bad} {sentence_object_bad}. ",
        "{donation} be to The Static. ",
    ];

    let file = fs::read_to_string("src/commands/sermon.toml")?;
    let cmp: HashMap<String, Vec<String>> = toml::from_str(file.as_str()).unwrap();

    let select_random_element = |mut fmt: Formatter| {
        const DEFAULT: &Vec<String> = &Vec::<String>::new();

        let possible_elements = &cmp.get(fmt.key).unwrap_or(DEFAULT);
        let element = &possible_elements
            [SmallRng::from_rng(&mut rng()).random_range(0..possible_elements.len())];

        fmt.str(element)
    };

    let fmt = strfmt_map(
        &(0..len)
            .map(|_| {
                PHRASES[SmallRng::from_rng(&mut rng()).random_range(0..PHRASES.len() - 1)]
                    .to_owned()
                    + "\n"
            })
            .collect::<String>(),
        select_random_element,
    )?;

    Ok(fmt)
}

#[poise::command(slash_command, prefix_command)]
pub async fn sermon(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(generate_sermon().unwrap_or("Failed to generate sermon D:".to_string()))
        .await?;
    Ok(())
}
