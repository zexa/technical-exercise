use async_std::task::spawn;
use tide::{log, Server};

// TODO: We could avoid all those clones by initializing translations_api_key with lazy_static
// and just passing references to the translation api.
#[derive(Clone)]
pub struct ApplicationState {
    translations_api_key: Option<String>,
}

impl ApplicationState {
    pub fn translations_api_key(&self) -> &Option<String> {
        &self.translations_api_key
    }
}

pub fn run(
    address: &str,
    initial_state: ApplicationState,
) -> std::io::Result<Server<ApplicationState>> {
    log::start();
    let mut app = tide::with_state(initial_state);
    app.at("/pokemon/:pokemon_name").get(pokemon_standard);
    app.at("/pokemon/translated/:pokemon_name")
        .get(pokemon_translated);
    app.listen(address);

    Ok(app)
}
