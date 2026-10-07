use alloc::{
    collections::BTreeSet,
    format,
    string::{String, ToString},
    vec::Vec,
};
use serde::Deserialize;

pub use lingua_core::LanguageId;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Language {
    pub id: LanguageId,
    pub name: String,
    pub region: String,
    pub native_name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
}

impl Language {
    pub fn matches(&self, query: &str) -> bool {
        self.search_rank(query).is_some()
    }

    pub fn search_rank(&self, query: &str) -> Option<u8> {
        let query = normalize_search(query);
        let fields: Vec<_> = [&self.name, &self.region, &self.native_name, &self.id.0]
            .into_iter()
            .chain(self.aliases.iter())
            .map(|value| normalize_search(value))
            .collect();
        let combined = fields.join(" ");
        if !query
            .split_whitespace()
            .all(|token| combined.contains(token))
        {
            return None;
        }
        Some(if fields.iter().any(|value| value == &query) {
            0
        } else if fields.iter().any(|value| value.starts_with(&query)) {
            1
        } else {
            2
        })
    }
}

fn normalize_search(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .chars()
        .filter_map(|character| {
            Some(match character {
                '\u{0300}'..='\u{036f}' => return None,
                'á' | 'à' | 'â' | 'ä' | 'ã' | 'å' => 'a',
                'é' | 'è' | 'ê' | 'ë' => 'e',
                'í' | 'ì' | 'î' | 'ï' => 'i',
                'ó' | 'ò' | 'ô' | 'ö' | 'õ' | 'ø' => 'o',
                'ú' | 'ù' | 'û' | 'ü' => 'u',
                'ç' | 'č' | 'ć' => 'c',
                'ñ' | 'ň' => 'n',
                'š' | 'ś' => 's',
                'ž' | 'ź' | 'ż' => 'z',
                'ý' | 'ÿ' => 'y',
                'ł' => 'l',
                other => other,
            })
        })
        .collect()
}

pub fn load_catalogue(json: &str) -> Result<Vec<Language>, String> {
    let languages: Vec<Language> = serde_json::from_str(json).map_err(|error| error.to_string())?;
    if languages.is_empty() {
        return Err(String::from("Language catalogue must not be empty."));
    }
    let mut ids = BTreeSet::new();
    for language in &languages {
        if [
            &language.id.0,
            &language.name,
            &language.region,
            &language.native_name,
        ]
        .into_iter()
        .any(|value| value.trim().is_empty())
        {
            return Err(String::from("Language fields must not be blank."));
        }
        if language.id.0.trim() != language.id.0 {
            return Err(format!(
                "Language identifier contains surrounding whitespace: {}",
                language.id.0
            ));
        }
        if !ids.insert(&language.id) {
            return Err(format!("Duplicate language identifier: {}", language.id.0));
        }
    }
    Ok(languages)
}

#[cfg(test)]
mod tests {
    use super::*;
    const CATALOGUE: &str = include_str!("../../../assets/languages.json");

    #[test]
    fn varieties_search_and_storage_remain_distinct() {
        let catalogue = load_catalogue(CATALOGUE).unwrap();
        assert_eq!(catalogue.len(), 100);
        let brazilian = catalogue
            .iter()
            .find(|language| language.id.0 == "pt-BR")
            .unwrap();
        let european = catalogue
            .iter()
            .find(|language| language.id.0 == "pt-PT")
            .unwrap();
        assert_ne!(brazilian.id, european.id);
        assert!(brazilian.matches(" BRAZIL "));
        assert!(european.matches("portugues"));
        assert!(european.matches("portuguese portugal"));
        assert!(!european.matches("Brazil"));
        let id: LanguageId = serde_json::from_str(r#""pt-BR""#).unwrap();
        assert_eq!(serde_json::to_string(&id).unwrap(), r#""pt-BR""#);
    }

    #[test]
    fn invalid_catalogues_are_rejected() {
        assert!(load_catalogue("[]").is_err());
        assert!(load_catalogue("not json").is_err());
        let entry =
            r#"{"id":"pt-BR","name":"Portuguese","region":"Brazil","native_name":"Português"}"#;
        assert!(load_catalogue(&format!("[{entry},{entry}]")).is_err());
        assert!(
            load_catalogue(
                r#"[{"id":"pt-BR","name":" ","region":"Brazil","native_name":"Português"}]"#
            )
            .is_err()
        );
    }
}
