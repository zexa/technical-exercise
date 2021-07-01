use tide::prelude::*;
use tide::Request;
use uwuifier::uwuify_str_sse;
use tide::Body;
use tide::Response;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/pokemon/translated/:pokemon_name").get(pokemon_translated);
    // TODO: Make port configurable. Probs via env.
    app.listen("0.0.0.0:5000").await?;
    Ok(())
}

// HTTP/GET /pokemon/translated/:pokemon_name
// http://localhost:5000/pokemon/translated/mewtwo
async fn pokemon_translated(mut req: Request<()>) -> tide::Result {
    let mut pokemon = get_pokemon(req.param("pokemon_name")?).await?;
    pokemon.change_description(uwuify_str_sse(pokemon.description()));

    let mut response = Response::new(200);
    response.set_body(Body::from_json(&pokemon)?);

    Ok(response)
}

#[derive(Serialize)]
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
}

async fn get_pokemon(name: &str) -> std::io::Result<Pokemon> {
    Ok(Pokemon {
        name: "Charizard".to_string(),
        description: "Best collectible".to_string(),
        habitat: "Caves, I guess".to_string(),
        is_legendary: false,
    })
}

