use alloc::{format, string::String, vec::Vec};
use gloo::net::http::Request;
use lingua_api::Runtime;
use lingua_core::{Exercise, LanguageId, Session};

// pub fn sample_path(language: &LanguageId) -> Result<String, String> {
//     if language.0.is_empty()
//         || !language
//             .0
//             .bytes()
//             .all(|value| value.is_ascii_alphanumeric() || value == b'-')
//     {
//         return Err(String::from("Invalid sample language identifier."));
//     }

//     Ok(format!("samples/sample-{}.json", language.0))
// }

// pub fn parse_samples(json: &str, language: &LanguageId) -> Result<Session, String> {
//     let exercises: Vec<Exercise> =
//         serde_json::from_str(json).map_err(|error| format!("Invalid sample exercises: {error}"))?;

//     Session::new(language.clone(), exercises)
//         .map_err(|error| format!("Invalid sample session: {error}"))
// }

// pub async fn load_session(runtime: Runtime, language: &LanguageId) -> Result<Session, String> {
//     // if !runtime.uses_samples() {
//     //     return Err(String::from(
//     //         "Desktop exercise generation is not connected yet.",
//     //     ));
//     // }

//     let path = sample_path(language)?;
//     let response = Request::get(&path)
//         .send()
//         .await
//         .map_err(|_| String::from("Could not load sample exercises."))?;

//     if response.status() == 404 {
//         return Err(String::from(
//             "Sample exercises are not available for this language yet.",
//         ));
//     }

//     if !response.ok() {
//         return Err(format!(
//             "Could not load sample exercises (HTTP {}).",
//             response.status()
//         ));
//     }

//     let json = response
//         .text()
//         .await
//         .map_err(|_| String::from("Could not read sample exercises."))?;

//     parse_samples(&json, language)
// }
