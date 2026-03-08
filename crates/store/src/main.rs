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

    session_repo.create(session).unwrap();

    // dbg!(user);

    let wotd_repo = WordOfTheDayRepository::new(manager.pool().clone());

    let wotd = WordOfTheDay {
        id: Utc::now().date_naive(),
        lexeme_id: 1,
        language_id: target_lang.id,
    };

    let wotd = wotd_repo.create(wotd).unwrap();

    let wotd = wotd_repo.get_by_id(wotd.id).unwrap();

    dbg!(wotd);
}