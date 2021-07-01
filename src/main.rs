use tide::prelude::*;
use tide::Request;
use uwuifier::uwuify_str_sse;

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
    // TODO: If we had very long strings we could probably spawn a thread for this.
    // though uwuify is extremely fast so maybe unnecessary.
    Ok(uwuify_str_sse("hello world").into())
}

