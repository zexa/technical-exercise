# pokemon-exercise
Rust programming exercise for a really cool company :)

## What would I have done differently in prod
* Optimized more
  * See TODO comments
* Implement caching
  * Less API calls to funtranslate and pokeapi
  * Faster
* Add CI/CD magic that would not allow PR's if
  * Code has warnings (clippy)
  * Code is not formatted correctly (rustfmt)
  * Code does not pass security audit (cargo-audit)
* Decouple framework & http client from the implementation itself (wrap it in my own methods)