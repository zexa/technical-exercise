pub mod health_check;
pub mod pokemon;
pub mod translate;

use crate::health_check::health_check;
use crate::pokemon::{pokemon_standard, pokemon_translated};
use tide::Server;

// TODO: We could avoid all those clones by initializing translations_api_key with lazy_static
// and just passing references to the translation api.
#[derive(Clone)]
pub struct ApplicationState {
    translations_api_key: Option<String>,
}

impl ApplicationState {
    pub fn new(translations_api_key: Option<String>) -> Self {
        Self {
            translations_api_key,
        }
    }

    pub fn translations_api_key(&self) -> &Option<String> {
        &self.translations_api_key
    }
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            translations_api_key: None,
        }
    }
}

pub fn build_app(initial_state: ApplicationState) -> Server<ApplicationState> {
    let mut app = tide::with_state(initial_state);
    app.at("/pokemon/:pokemon_name").get(pokemon_standard);
    app.at("/pokemon/translated/:pokemon_name")
        .get(pokemon_translated);
    app.at("/health_check").get(health_check);

    app
}
