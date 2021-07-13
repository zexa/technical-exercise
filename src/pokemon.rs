use crate::translate::{shakespeare_translate, yoda_translate};
use crate::ApplicationState;
use serde::Deserialize;
use serde::Serialize;
use tide::log;
use tide::{Body, Request, Response};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Pokemon {
    name: String,
    description: String,
    habitat: String,
    is_legendary: bool,
}

impl Pokemon {
    pub fn new(name: String, description: String, habitat: String, is_legendary: bool) -> Self {
        Self {
            name,
            description,
            habitat,
            is_legendary,
        }
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn change_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn habitat(&self) -> &str {
        &self.habitat
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Language {
    name: String,
}

impl Language {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct PokemonSpeciesFlavorTextEntry {
    flavor_text: String,
    language: Language,
}

impl PokemonSpeciesFlavorTextEntry {
    pub fn new(flavor_text: String, language: Language) -> Self {
        Self {
            flavor_text,
            language,
        }
    }

    pub fn language(&self) -> &Language {
        &self.language
    }

    pub fn consume(self) -> String {
        self.flavor_text
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Habitat {
    name: String,
}

impl Habitat {
    pub fn consume(self) -> String {
        self.name
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct PokemonSpecies {
    is_legendary: bool,
    name: String,
    flavor_text_entries: Vec<PokemonSpeciesFlavorTextEntry>,
    habitat: Option<Habitat>, // API spec said the value can be null.
}

impl PokemonSpecies {
    pub fn new(
        is_legendary: bool,
        name: String,
        flavor_text_entries: Vec<PokemonSpeciesFlavorTextEntry>,
        habitat: Option<Habitat>,
    ) -> Self {
        Self {
            is_legendary,
            name,
            flavor_text_entries,
            habitat,
        }
    }
}

impl Into<Pokemon> for PokemonSpecies {
    fn into(self) -> Pokemon {
        Pokemon {
            name: self.name,
            is_legendary: self.is_legendary,
            habitat: self
                .habitat
                .unwrap_or(Habitat {
                    name: String::from(""),
                })
                .consume(),
            description: self
                .flavor_text_entries
                .into_iter()
                .filter(|f| f.language().name() == "en")
                .last()
                .unwrap_or(PokemonSpeciesFlavorTextEntry {
                    flavor_text: String::from(""),
                    language: Language {
                        name: String::from("en"),
                    },
                })
                .consume()
                // TODO: At production we could probably optimize these two replacements into one.
                // https://users.rust-lang.org/t/string-replace-performance/7478/5 has some nice
                // suggestions.
                .replace("\n", " ") //
                .replace("\u{c}", " "),
        }
    }
}

async fn get_pokemon(name: &str) -> tide::Result<Pokemon> {
    let url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", name);
    log::info!("Requesting pokemon {}", &url);
    let pokemon: PokemonSpecies = surf::get(url).recv_json().await?;

    Ok(pokemon.into())
}

pub async fn pokemon_standard(req: Request<ApplicationState>) -> tide::Result {
    let pokemon_name = req.param("pokemon_name")?;
    log::info!("Searching for {}", pokemon_name);
    match &get_pokemon(&pokemon_name).await {
        Ok(pok) => {
            log::info!("Found {}", &pokemon_name);
            let mut response = Response::new(200);
            response.set_body(Body::from_json(pok)?);

            Ok(response)
        }
        Err(err) => {
            log::info!("Could not find {}. Reason: {}", &pokemon_name, err);

            Ok(Response::new(404))
        }
    }
}

// HTTP/GET /pokemon/translated/:pokemon_name
// http://localhost:5000/pokemon/translated/mewtwo
pub async fn pokemon_translated(req: Request<ApplicationState>) -> tide::Result {
    let mut pokemon = match get_pokemon(req.param("pokemon_name")?).await {
        Ok(pok) => pok,
        _ => return Ok(Response::new(404)),
    };

    let translation_api = req.state().translations_api_key();

    let translation = match pokemon.habitat() {
        "cave" => yoda_translate(pokemon.description(), translation_api).await,
        _ => shakespeare_translate(pokemon.description(), translation_api).await,
    };

    match translation {
        Ok(translation) => pokemon.change_description(translation),
        _ => {}
    }

    let mut response = Response::new(200);
    response.set_body(Body::from_json(&pokemon)?);

    Ok(response)
}
