use lingua_core::{
    Choice, CurriculumLevel, Exercise, ExerciseContent, ExerciseError, LanguageId,
    SESSION_EXERCISE_COUNT, Session, SessionError, SingleChoice, Text,
};

fn text(value: &str) -> Text {
    Text {
        value: value.into(),
        script: None,
        reading: None,
    }
}

fn choices() -> Vec<Choice> {
    ["cat", "dog", "horse"]
        .into_iter()
        .map(|id| Choice {
            id: id.into(),
            text: text(id),
        })
        .collect()
}

fn exercise(id: &str) -> Exercise {
    Exercise {
        id: id.into(),
        language: LanguageId("zh-Hans-CN".into()),
        instruction: "Choose the meaning".into(),
        prompt: Text {
            value: "猫".into(),
            script: Some("Hans".into()),
            reading: Some("māo".into()),
        },
        level: Some(CurriculumLevel {
            framework: "HSK".into(),
            edition: None,
            level: "1".into(),
        }),
        content: ExerciseContent::SingleChoice(SingleChoice::new(choices(), "cat".into()).unwrap()),
    }
}

fn session_exercises() -> Vec<Exercise> {
    (1..=SESSION_EXERCISE_COUNT)
        .map(|index| exercise(&format!("exercise-{index}")))
        .collect()
}

#[test]
fn invalid_answers_do_not_advance_and_valid_answers_complete_session() {
    let mut session = Session::new(LanguageId("zh-Hans-CN".into()), session_exercises()).unwrap();
    assert_eq!(
        session.answer("missing"),
        Err(SessionError::InvalidAnswer(ExerciseError::UnknownChoice))
    );
    assert_eq!(session.current().unwrap().id, "exercise-1");
    assert!(!session.answer("dog").unwrap().correct);

    for index in 2..=SESSION_EXERCISE_COUNT {
        assert_eq!(session.current().unwrap().id, format!("exercise-{index}"));
        assert!(session.answer("cat").unwrap().correct);
    }

    assert!(session.is_complete());
    assert_eq!(session.correct_count(), SESSION_EXERCISE_COUNT - 1);
    assert_eq!(session.answer("cat"), Err(SessionError::Complete));
}

#[test]
fn invalid_choice_sets_are_rejected() {
    assert_eq!(
        SingleChoice::new(vec![], "cat".into()),
        Err(ExerciseError::TooFewChoices)
    );
    assert_eq!(
        SingleChoice::new(choices(), "missing".into()),
        Err(ExerciseError::MissingCorrectChoice)
    );
    let mut duplicate = choices();
    duplicate[1].id = duplicate[0].id.clone();
    assert_eq!(
        SingleChoice::new(duplicate, "cat".into()),
        Err(ExerciseError::DuplicateChoiceId)
    );
}

#[test]
fn serialized_choices_round_trip_and_invalid_data_cannot_bypass_validation() {
    let choice = SingleChoice::new(choices(), "cat".into()).unwrap();
    let json = serde_json::to_string(&choice).unwrap();
    assert_eq!(serde_json::from_str::<SingleChoice>(&json).unwrap(), choice);
    assert!(
        serde_json::from_str::<SingleChoice>(r#"{"choices":[],"correct_choice_id":"cat"}"#)
            .is_err()
    );
}

#[test]
fn sessions_reject_mixed_languages_and_duplicate_exercises() {
    assert_eq!(
        Session::new(LanguageId("pt-BR".into()), session_exercises()),
        Err(SessionError::LanguageMismatch)
    );

    let mut duplicate = session_exercises();
    duplicate[1].id = duplicate[0].id.clone();

    assert_eq!(
        Session::new(LanguageId("zh-Hans-CN".into()), duplicate),
        Err(SessionError::DuplicateExerciseId)
    );
    assert_eq!(
        Session::new(LanguageId("zh-Hans-CN".into()), vec![]),
        Err(SessionError::ExerciseCount {
            expected: SESSION_EXERCISE_COUNT,
            actual: 0,
        })
    );
    assert_eq!(
        Session::new(LanguageId("zh-Hans-CN".into()), vec![exercise("only")]),
        Err(SessionError::ExerciseCount {
            expected: SESSION_EXERCISE_COUNT,
            actual: 1,
        })
    );
}
