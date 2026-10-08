use core::ops::Deref;

use alloc::{rc::Rc, string::String};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LanguageId(pub String);

use alloc::{
    collections::BTreeSet,
    format,
    string::ToString,
    vec::Vec,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Languages(Rc<[Language]>);

impl Languages {
    pub fn new(data: Vec<Language>) -> Self {
        Self(data.into())
    }
}

impl Deref for Languages {
    type Target = [Language];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}