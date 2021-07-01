use tide::prelude::*;
use tide::Request;
use tide::Body;
use tide::Response;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/pokemon/:pokemon_name").get(pokemon_standard);
    app.at("/pokemon/translated/:pokemon_name").get(pokemon_translated);
    // TODO: Make port configurable. Probs via env.
    app.listen("0.0.0.0:5000").await?;
    Ok(())
}

async fn pokemon_standard(req: Request<()>) -> tide::Result {
    let mut response = Response::new(200);
    response.set_body(Body::from_json(&get_pokemon(req.param("pokemon_name")?).await?)?);

    Ok(response)
}

// HTTP/GET /pokemon/translated/:pokemon_name
// http://localhost:5000/pokemon/translated/mewtwo
async fn pokemon_translated(req: Request<()>) -> tide::Result {
    let mut pokemon = get_pokemon(req.param("pokemon_name")?).await?;

    let translation = match pokemon.habitat() {
        "cave" => yoda_translate(pokemon.description()).await,
        _ => shakespeare_translate(pokemon.description()).await,
    };

    match translation {
        Ok(translation) => pokemon.change_description(translation),
        _ => {},
    }

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

    pub fn habitat(&self) -> &str {
        &self.habitat
    }
}

async fn get_pokemon(_name: &str) -> std::io::Result<Pokemon> {
    Ok(Pokemon {
        name: "Charizard".to_string(),
        description: "Best collectible".to_string(),
        habitat: "Caves, I guess".to_string(),
        is_legendary: false,
    })
}

async fn yoda_translate(text: &str) -> std::io::Result<String> {
    Ok(String::from("huh"))
}

async fn shakespeare_translate(text: &str) -> std::io::Result<String> {
    Ok(String::from("ugh"))
}

