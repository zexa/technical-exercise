use tide::log;
use tide::prelude::*;
use tide::Body;
use tide::Request;
use tide::Response;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    log::start();
    app.at("/pokemon/:pokemon_name").get(pokemon_standard);
    app.at("/pokemon/translated/:pokemon_name")
        .get(pokemon_translated);
    // TODO: Make port configurable. Probs via env.
    app.listen("0.0.0.0:5000").await?;

    Ok(())
}

async fn pokemon_standard(req: Request<()>) -> tide::Result {
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
async fn pokemon_translated(req: Request<()>) -> tide::Result {
    let mut pokemon = match get_pokemon(req.param("pokemon_name")?).await {
        Ok(pok) => pok,
        _ => return Ok(Response::new(404)),
    };

    let translation = match pokemon.habitat() {
        "cave" => yoda_translate(pokemon.description()).await,
        _ => shakespeare_translate(pokemon.description()).await,
    };

    match translation {
        Ok(translation) => pokemon.change_description(translation),
        _ => {}
    }

    let mut response = Response::new(200);
    response.set_body(Body::from_json(&pokemon)?);

    Ok(response)
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct Pokemon {
    name: String,
    description: String,
    habitat: String,
    is_legendary: bool,
}

impl Pokemon {
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
struct Language {
    name: String,
}

impl Language {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct PokemonSpeciesFlavorTextEntry {
    flavor_text: String,
    language: Language,
}

impl PokemonSpeciesFlavorTextEntry {
    pub fn language(&self) -> &Language {
        &self.language
    }

    pub fn consume(self) -> String {
        self.flavor_text
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct Habitat {
    name: String,
}

impl Habitat {
    pub fn consume(self) -> String {
        self.name
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct PokemonSpecies {
    is_legendary: bool,
    name: String,
    flavor_text_entries: Vec<PokemonSpeciesFlavorTextEntry>,
    habitat: Option<Habitat>, // API spec said the value can be null.
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
                .replace("\n", " ")
                .replace("\u{c}", " "),
        }
    }
}

// https://pokeapi.co/api/v2/pokemon-species/{id or name}/
async fn get_pokemon(name: &str) -> tide::Result<Pokemon> {
    let url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", name);
    log::info!("Requesting pokemon {}", &url);
    let pokemon: PokemonSpecies = surf::get(url).recv_json().await?;

    Ok(pokemon.into())
}

async fn yoda_translate(text: &str) -> std::io::Result<String> {
    Ok(String::from(text))
}

async fn shakespeare_translate(text: &str) -> std::io::Result<String> {
    Ok(String::from(text))
}

mod test {
    use crate::{Language, Pokemon, PokemonSpecies, PokemonSpeciesFlavorTextEntry};

    fn get_example_pokemon_species() -> PokemonSpecies {
        PokemonSpecies {
            name: String::from("wormadam"),
            is_legendary: false,
            flavor_text_entries: vec![
                PokemonSpeciesFlavorTextEntry {
                    flavor_text: String::from("When the bulb on\nits back grows\nlarge, it appears\u{c}to lose the\nability to stand\non its hind legs."),
                    language: Language {
                        name: String::from("en"),
                    }
                }
            ],
            habitat: None,
        }
    }

    #[test]
    fn pokemon_deserialization() -> std::io::Result<()> {
        let json = r#"
        {
          "id": 413,
          "name": "wormadam",
          "order": 441,
          "gender_rate": 8,
          "capture_rate": 45,
          "base_happiness": 70,
          "is_baby": false,
          "is_legendary": false,
          "is_mythical": false,
          "hatch_counter": 15,
          "has_gender_differences": false,
          "forms_switchable": false,
          "growth_rate": {
            "name": "medium",
            "url": "https://pokeapi.co/api/v2/growth-rate/2/"
          },
          "pokedex_numbers": [
            {
              "entry_number": 45,
              "pokedex": {
                "name": "kalos-central",
                "url": "https://pokeapi.co/api/v2/pokedex/12/"
              }
            }
          ],
          "egg_groups": [
            {
              "name": "bug",
              "url": "https://pokeapi.co/api/v2/egg-group/3/"
            }
          ],
          "color": {
            "name": "gray",
            "url": "https://pokeapi.co/api/v2/pokemon-color/4/"
          },
          "shape": {
            "name": "squiggle",
            "url": "https://pokeapi.co/api/v2/pokemon-shape/2/"
          },
          "evolves_from_species": {
            "name": "burmy",
            "url": "https://pokeapi.co/api/v2/pokemon-species/412/"
          },
          "evolution_chain": {
            "url": "https://pokeapi.co/api/v2/evolution-chain/213/"
          },
          "habitat": null,
          "generation": {
            "name": "generation-iv",
            "url": "https://pokeapi.co/api/v2/generation/4/"
          },
          "names": [
            {
              "name": "Wormadam",
              "language": {
                "name": "en",
                "url": "https://pokeapi.co/api/v2/language/9/"
              }
            }
          ],
          "flavor_text_entries": [
            {
              "flavor_text": "When the bulb on\nits back grows\nlarge, it appears\fto lose the\nability to stand\non its hind legs.",
              "language": {
                "name": "en",
                "url": "https://pokeapi.co/api/v2/language/9/"
              },
              "version": {
                "name": "red",
                "url": "https://pokeapi.co/api/v2/version/1/"
              }
            }
          ],
          "form_descriptions": [
            {
              "description": "Forms have different stats and movepools.  During evolution, Burmy's current cloak becomes Wormadam's form, and can no longer be changed.",
              "language": {
                "name": "en",
                "url": "https://pokeapi.co/api/v2/language/9/"
              }
            }
          ],
          "genera": [
            {
              "genus": "Bagworm",
              "language": {
                "name": "en",
                "url": "https://pokeapi.co/api/v2/language/9/"
              }
            }
          ],
          "varieties": [
            {
              "is_default": true,
              "pokemon": {
                "name": "wormadam-plant",
                "url": "https://pokeapi.co/api/v2/pokemon/413/"
              }
            }
          ]
        }"#;

        Ok(assert_eq!(
            get_example_pokemon_species(),
            serde_json::from_str(json)?
        ))
    }

    #[test]
    fn pokemon_species_to_pokemon() -> std::io::Result<()> {
        let expected = Pokemon {
            name: String::from("wormadam"),
            description: String::from("When the bulb on its back grows large, it appears to lose the ability to stand on its hind legs."),
            habitat: String::from(""),
            is_legendary: false,
        };

        Ok(assert_eq!(expected, get_example_pokemon_species().into()))
    }
}
