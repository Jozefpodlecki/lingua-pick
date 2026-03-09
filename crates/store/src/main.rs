use chrono::{Local, NaiveDate, Utc};
use lingua_pick_store::*;
use uuid::Uuid;

fn main() {
    let now = Local::now();
    // let db_path = format!("test_{}.duckdb", now.format("%M%S"));
    let manager = DatabaseManager::memory().unwrap();
    manager.ensure_created().unwrap();

    let lang_repo = LanguageRepository::new(manager.pool().clone());
    let langs = lang_repo.get_all().unwrap();

    let user_repo = UserRepository::new(manager.pool().clone());

    let user = User {
        id: Uuid::now_v7(),
        created_on: Utc::now(),
        user_name: "test".into(),
        password_hash: vec![1, 2, 3],
    };

    let user = user_repo.create(user).unwrap();
    let user  = user_repo.get_by_username("test").unwrap().unwrap();

    let profile_repo = UserProfileRepository::new(manager.pool().clone());
    let source_lang = lang_repo.get_by_iso639_1("en").unwrap().unwrap();
    let target_lang = lang_repo.get_by_iso639_1("zh").unwrap().unwrap();

    let profile = UserProfile {
        id: Uuid::now_v7(),
        user_id: Uuid::now_v7(),
        created_on: Utc::now(),
        updated_on: Utc::now(),
        is_active: true,
        source_language_id: source_lang.id,
        target_language_id: target_lang.id,
    };

    let profile = profile_repo.create(profile).unwrap();

    let session = Session {
        id: Uuid::now_v7(),
        user_profile_id: profile.id,
        created_on: Utc::now(),
        updated_on: Utc::now(),
        completed_on: None,
        exercise_count: 0,
        total_exercise_count: 5,
        current_exercise_id: None
    };

    let session_repo = SessionRepository::new(manager.pool().clone());

    let session = session_repo.create(session).unwrap();
    let session = session_repo.get_by_id(session.id).unwrap().unwrap();

    let exercise_type_repo = ExerciseTypeRepository::new(manager.pool().clone());

    let exercise_types = exercise_type_repo.get_by_language_id(target_lang.id).unwrap();
    let exercise_type = exercise_types.into_vec().into_iter().next().unwrap();

    let exercise_repo = ExerciseRepository::new(manager.pool().clone());

    let mut exercise = Exercise {
        id: 0,
        type_id: exercise_type.id,
        session_id: session.id,
        created_on: Utc::now(),
        completed_on: None,
        data: "".into(),
        result: None,
        verdict: None,
    };

    exercise.id = exercise_repo.create(&exercise).unwrap();
    let exercise = exercise_repo.get_by_id(exercise.id).unwrap().unwrap();

    dbg!(&exercise);
    session_repo.link_to_user(session.id, user.id).unwrap();
    session_repo.set_exercise(session.id, exercise.id).unwrap();

    exercise_repo.set_result(exercise.id, "").unwrap();
    exercise_repo.set_verdict(exercise.id, "").unwrap();
    exercise_repo.complete(exercise.id, Utc::now()).unwrap();

    let lexeme_repo = LexemeRepository::new(manager.pool().clone());

    let lexemes = lexeme_repo.get_by_language_id(target_lang.id).unwrap();

    let first = lexemes.into_vec().into_iter().next().unwrap();

    let wotd_repo = WordOfTheDayRepository::new(manager.pool().clone());

    let wotd = WordOfTheDay {
        id: Utc::now().date_naive(),
        lexeme_id: first.id,
        language_id: target_lang.id,
    };

    let wotd = wotd_repo.create(wotd).unwrap();

    let wotd = wotd_repo.get_by_id(wotd.id).unwrap();

    dbg!(wotd);
}