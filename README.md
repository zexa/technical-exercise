# pokemon-exercise
Rust programming exercise for a really cool company :)

## What would I have done differently in prod
* Optimized more
  * See TODO comments
* Separated tests into a separate tests folder
* Implement caching
  * Less API calls to funtranslate and pokeapi
  * Faster
* Add CI/CD magic that would not allow PR's if
  * Code has warnings (clippy)
  * Code is not formatted correctly (rustfmt)
  * Code does not pass security audit (cargo-audit)
    
## TODO:
* Refactor application into two separate modules
* Add test that makes sure that yoda translation gets called.
* Add test that checks PokemonSpecies habitat deserialization.
* Set up docker
* Add uwuify for fun c:
* Moar logs
