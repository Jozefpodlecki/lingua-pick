
use alloc::{collections::BTreeSet, string::String, vec::Vec};
use core::{error::Error, fmt, future::Future};
use lingua_core::{Exercise, LanguageId, Session, SessionError};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const DEFAULT_MAX_TOKENS: u32 = 8_192;
pub const MAX_PROMPT_BYTES: usize = 64 * 1_024;
pub const MAX_SYSTEM_PROMPT_BYTES: usize = 32 * 1_024;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetLanguage {
    pub id: LanguageId,
    pub name: String,
    pub native_name: String,
    pub region: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WordCategory {
    pub id: String,
    pub description: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseKind {
    SingleChoice,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PromptRequest {
    pub model: String,
    pub system_prompt: String,
    pub user_prompt: String,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub response_format: Option<Value>,
}

const fn default_max_tokens() -> u32 {
    DEFAULT_MAX_TOKENS
}

impl PromptRequest {
    pub fn validate(&self) -> Result<(), PromptRequestError> {
        if self.model.trim().is_empty() {
            return Err(PromptRequestError::BlankModel);
        }

        if self.system_prompt.trim().is_empty() {
            return Err(PromptRequestError::BlankSystemPrompt);
        }

        if self.user_prompt.trim().is_empty() {
            return Err(PromptRequestError::BlankUserPrompt);
        }

        if self.system_prompt.len() > MAX_SYSTEM_PROMPT_BYTES {
            return Err(PromptRequestError::SystemPromptTooLarge);
        }

        if self.user_prompt.len() > MAX_PROMPT_BYTES {
            return Err(PromptRequestError::UserPromptTooLarge);
        }

        if self.max_tokens == 0 || self.max_tokens > 32_768 {
            return Err(PromptRequestError::InvalidMaxTokens);
        }

        if self.temperature.is_some_and(|temperature| {
            !temperature.is_finite() || !(0.0..=2.0).contains(&temperature)
        }) {
            return Err(PromptRequestError::InvalidTemperature);
        }

        if self
            .response_format
            .as_ref()
            .is_some_and(|format| !format.is_object())
        {
            return Err(PromptRequestError::InvalidResponseFormat);
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PromptRequestError {
    BlankModel,
    BlankSystemPrompt,
    BlankUserPrompt,
    SystemPromptTooLarge,
    UserPromptTooLarge,
    InvalidMaxTokens,
    InvalidTemperature,
    InvalidResponseFormat,
}

impl fmt::Display for PromptRequestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::BlankModel => "The model identifier must not be blank.",
            Self::BlankSystemPrompt => "The system prompt must not be blank.",
            Self::BlankUserPrompt => "The user prompt must not be blank.",
            Self::SystemPromptTooLarge => "The system prompt is too large.",
            Self::UserPromptTooLarge => "The user prompt is too large.",
            Self::InvalidMaxTokens => "The maximum token count must be between 1 and 32768.",
            Self::InvalidTemperature => "Temperature must be between 0 and 2.",
            Self::InvalidResponseFormat => "The response format must be a JSON object.",
        })
    }
}

impl Error for PromptRequestError {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptResponse {
    pub model: String,
    pub content: String,
    pub finish_reason: Option<String>,
    pub usage: Option<PromptUsage>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptErrorCode {
    InvalidRequest,
    Client,
    Unavailable,
    Http,
    ResponseTooLarge,
    InvalidResponse,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptError {
    pub code: PromptErrorCode,
    pub message: String,
    pub status: Option<u16>,
}

impl PromptError {
    pub fn new(code: PromptErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            status: None,
        }
    }

    pub fn with_status(code: PromptErrorCode, status: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            status: Some(status),
        }
    }
}

impl fmt::Display for PromptError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for PromptError {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub target_language: TargetLanguage,
    pub available_categories: Vec<WordCategory>,
    pub selected_categories: Vec<String>,
    pub exercise_kind: ExerciseKind,
}

impl GenerationRequest {
    pub fn validate(&self) -> Result<(), GenerationRequestError> {
        let target = &self.target_language;

        if [
            &target.id.0,
            &target.name,
            &target.native_name,
            &target.region,
        ]
        .into_iter()
        .any(|value| value.trim().is_empty())
        {
            return Err(GenerationRequestError::BlankTarget);
        }

        if self.available_categories.is_empty() {
            return Err(GenerationRequestError::NoAvailableCategories);
        }

        let mut available = BTreeSet::new();

        for category in &self.available_categories {
            if category.id.trim().is_empty() || category.description.trim().is_empty() {
                return Err(GenerationRequestError::BlankCategory);
            }

            if !available.insert(category.id.as_str()) {
                return Err(GenerationRequestError::DuplicateCategory);
            }
        }

        if self.selected_categories.is_empty() {
            return Err(GenerationRequestError::NoSelectedCategories);
        }

        let mut selected = BTreeSet::new();

        for category in &self.selected_categories {
            if !selected.insert(category.as_str()) {
                return Err(GenerationRequestError::DuplicateSelection);
            }

            if !available.contains(category.as_str()) {
                return Err(GenerationRequestError::UnknownSelection);
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenerationRequestError {
    BlankTarget,
    NoAvailableCategories,
    BlankCategory,
    DuplicateCategory,
    NoSelectedCategories,
    DuplicateSelection,
    UnknownSelection,
}

impl fmt::Display for GenerationRequestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::BlankTarget => "Target language fields must not be blank.",
            Self::NoAvailableCategories => "At least one word category must be available.",
            Self::BlankCategory => "Word category fields must not be blank.",
            Self::DuplicateCategory => "Available word category identifiers must be unique.",
            Self::NoSelectedCategories => "At least one word category must be selected.",
            Self::DuplicateSelection => "Selected word category identifiers must be unique.",
            Self::UnknownSelection => "Selected word categories must be available.",
        })
    }
}

impl Error for GenerationRequestError {}

pub trait ExerciseProvider {
    type Error;

    fn generate(
        &self,
        request: &GenerationRequest,
    ) -> impl Future<Output = Result<Vec<Exercise>, Self::Error>>;
}

pub struct AiFacade<P> {
    provider: P,
}

impl<P> AiFacade<P> {
    pub const fn new(provider: P) -> Self {
        Self { provider }
    }

    pub fn provider(&self) -> &P {
        &self.provider
    }
}

impl<P> AiFacade<P>
where
    P: ExerciseProvider,
{
    pub async fn start_session(
        &self,
        request: GenerationRequest,
    ) -> Result<Session, AiFacadeError<P::Error>> {
        request.validate().map_err(AiFacadeError::InvalidRequest)?;

        let language = request.target_language.id.clone();
        let exercises = self
            .provider
            .generate(&request)
            .await
            .map_err(AiFacadeError::Provider)?;

        Session::new(language, exercises).map_err(AiFacadeError::InvalidSession)
    }
}

#[derive(Debug)]
pub enum AiFacadeError<E> {
    InvalidRequest(GenerationRequestError),
    Provider(E),
    InvalidSession(SessionError),
}

impl<E> fmt::Display for AiFacadeError<E>
where
    E: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest(error) => write!(formatter, "Invalid generation request: {error}"),
            Self::Provider(error) => write!(formatter, "Exercise provider failed: {error}"),
            Self::InvalidSession(error) => {
                write!(formatter, "Provider returned an invalid session: {error}")
            }
        }
    }
}

impl<E> Error for AiFacadeError<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidRequest(error) => Some(error),
            Self::Provider(error) => Some(error),
            Self::InvalidSession(error) => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{format, vec};
    use core::convert::Infallible;
    use futures::executor::block_on;
    use lingua_core::{Choice, ExerciseContent, SESSION_EXERCISE_COUNT, SingleChoice, Text};
    use serde_json::json;

    struct StubProvider {
        exercises: Vec<Exercise>,
    }

    impl ExerciseProvider for StubProvider {
        type Error = Infallible;

        async fn generate(
            &self,
            _request: &GenerationRequest,
        ) -> Result<Vec<Exercise>, Self::Error> {
            Ok(self.exercises.clone())
        }
    }

    fn request() -> GenerationRequest {
        GenerationRequest {
            target_language: TargetLanguage {
                id: LanguageId("pt-BR".into()),
                name: "Brazilian Portuguese".into(),
                native_name: "Português brasileiro".into(),
                region: "Brazil".into(),
            },
            available_categories: vec![WordCategory {
                id: "animals".into(),
                description: "Common animal names".into(),
            }],
            selected_categories: vec!["animals".into()],
            exercise_kind: ExerciseKind::SingleChoice,
        }
    }

    fn exercises(count: usize) -> Vec<Exercise> {
        (1..=count)
            .map(|index| Exercise {
                id: format!("exercise-{index}"),
                language: LanguageId("pt-BR".into()),
                instruction: "Choose the meaning".into(),
                prompt: Text {
                    value: "gato".into(),
                    script: Some("Latn".into()),
                    reading: None,
                },
                level: None,
                content: ExerciseContent::SingleChoice(
                    SingleChoice::new(
                        vec![
                            Choice {
                                id: "cat".into(),
                                text: Text {
                                    value: "cat".into(),
                                    script: Some("Latn".into()),
                                    reading: None,
                                },
                            },
                            Choice {
                                id: "dog".into(),
                                text: Text {
                                    value: "dog".into(),
                                    script: Some("Latn".into()),
                                    reading: None,
                                },
                            },
                        ],
                        "cat".into(),
                    )
                    .unwrap(),
                ),
            })
            .collect()
    }

    fn prompt_request() -> PromptRequest {
        PromptRequest {
            model: "local-model".into(),
            system_prompt: "Return JSON.".into(),
            user_prompt: "Create exercises.".into(),
            max_tokens: DEFAULT_MAX_TOKENS,
            temperature: Some(0.2),
            response_format: Some(json!({
                "type": "json_schema",
                "json_schema": {"name": "exercise_session"}
            })),
        }
    }

    #[test]
    fn facade_builds_a_valid_ten_exercise_session() {
        let facade = AiFacade::new(StubProvider {
            exercises: exercises(SESSION_EXERCISE_COUNT),
        });
        let session = block_on(facade.start_session(request())).unwrap();

        assert_eq!(session.total(), SESSION_EXERCISE_COUNT);
        assert_eq!(session.language(), &LanguageId("pt-BR".into()));
    }

    #[test]
    fn facade_rejects_unknown_categories_and_partial_batches() {
        let mut invalid_request = request();
        invalid_request.selected_categories = vec!["travel".into()];
        let facade = AiFacade::new(StubProvider {
            exercises: exercises(SESSION_EXERCISE_COUNT),
        });

        assert!(matches!(
            block_on(facade.start_session(invalid_request)),
            Err(AiFacadeError::InvalidRequest(
                GenerationRequestError::UnknownSelection
            ))
        ));

        let facade = AiFacade::new(StubProvider {
            exercises: exercises(1),
        });

        assert!(matches!(
            block_on(facade.start_session(request())),
            Err(AiFacadeError::InvalidSession(SessionError::ExerciseCount {
                expected: SESSION_EXERCISE_COUNT,
                actual: 1,
            }))
        ));
    }

    #[test]
    fn prompt_request_enforces_transport_limits() {
        assert_eq!(prompt_request().validate(), Ok(()));

        let mut request = prompt_request();
        request.user_prompt = " ".into();
        assert_eq!(request.validate(), Err(PromptRequestError::BlankUserPrompt));

        let mut request = prompt_request();
        request.max_tokens = 0;
        assert_eq!(
            request.validate(),
            Err(PromptRequestError::InvalidMaxTokens)
        );

        let mut request = prompt_request();
        request.temperature = Some(2.1);
        assert_eq!(
            request.validate(),
            Err(PromptRequestError::InvalidTemperature)
        );

        let mut request = prompt_request();
        request.response_format = Some(json!(["json_schema"]));
        assert_eq!(
            request.validate(),
            Err(PromptRequestError::InvalidResponseFormat)
        );
    }
}
