use technical_exercise::pokemon::{
    Language, Pokemon, PokemonSpecies, PokemonSpeciesFlavorTextEntry,
};

fn get_wormodam_pokemon_species() -> PokemonSpecies {
    PokemonSpecies::new(
        false,
        String::from("wormadam"),
        vec![
            PokemonSpeciesFlavorTextEntry::new(
                String::from("When the bulb on\nits back grows\nlarge, it appears\u{c}to lose the\nability to stand\non its hind legs."),
                Language::new(String::from("en"))
            )
        ],
        None,
    )
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
          ]
        }"#;

    Ok(assert_eq!(
        get_wormodam_pokemon_species(),
        serde_json::from_str(json)?
    ))
}

#[test]
fn pokemon_species_to_pokemon() -> std::io::Result<()> {
    Ok(assert_eq!(Pokemon::new(
        String::from("wormadam"),
        String::from("When the bulb on its back grows large, it appears to lose the ability to stand on its hind legs."),
        String::from(""),
        false,
    ), get_wormodam_pokemon_species().into()))
}
