use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
struct TranslationContents {
    translated: String,
}

impl TranslationContents {
    pub fn consume(self) -> String {
        self.translated
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct Translation {
    contents: TranslationContents,
}

impl Translation {
    pub fn consume(self) -> String {
        self.contents.consume()
    }
}

pub async fn yoda_translate(text: &str, api_key: &Option<String>) -> tide::Result<String> {
    let request = match api_key {
        Some(key) => surf::post(format!(
            "https://api.funtranslations.com/translate/yoda.json?text={}",
            text
        ))
        .header("X-Funtranslations-Api-Secret", key),
        _ => surf::post(format!(
            "https://api.funtranslations.com/translate/yoda.json?text={}",
            text
        )),
    };

    let translation: Translation = request.recv_json().await?;

    Ok(translation.consume())
}

pub async fn shakespeare_translate(text: &str, api_key: &Option<String>) -> tide::Result<String> {
    let request = match api_key {
        Some(key) => surf::post(format!(
            "https://api.funtranslations.com/translate/shakespeare.json?text={}",
            text
        ))
        .header("X-Funtranslations-Api-Secret", key),
        _ => surf::post(format!(
            "https://api.funtranslations.com/translate/shakespeare.json?text={}",
            text
        )),
    };

    let translation: Translation = request.recv_json().await?;

    Ok(translation.consume())
}
