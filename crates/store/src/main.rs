use chrono::{Local, Utc};
use lingua_pick_store::{DatabaseManager, LanguageRepository, Session, SessionRepository, User, UserProfile, UserProfileRepository, UserRepository};
use uuid::Uuid;

fn main() {
    let now = Local::now();
    let db_path = format!("test_{}.duckdb", now.format("%M%S"));
    let manager = DatabaseManager::new(db_path.into()).unwrap();
    manager.ensure_created().unwrap();

    let languages = LanguageRepository::new(manager.pool().clone());

    let langs = languages.get_all().unwrap();

    // dbg!(langs);

    let user_repo = UserRepository::new(manager.pool().clone());

    // let user = User {
    //     id: Uuid::now_v7(),
    //     created_on: Utc::now(),
    //     username: "test".into(),
    //     password_hash: vec![1, 2, 3],
    // };

    // user_repo.create(user).unwrap();

    let user  = user_repo.get_by_username("test").unwrap().unwrap();

    let profile_repo = UserProfileRepository::new(manager.pool().clone());

    let profile = UserProfile {
        id: Uuid::now_v7(),
        user_id: Uuid::now_v7(),
        created_on: Utc::now(),
        updated_on: Utc::now(),
        is_active: true,
        source_language_id: todo!(),
        target_language_id: todo!(),
    };

    profile_repo.create(profile).unwrap();

    let session = Session {
        id: Uuid::now_v7(),
        user_profile_id: Uuid::now_v7(),
        created_on: Utc::now(),
        updated_on: Utc::now(),
        completed_on: None,
        exercise_count: 0,
        total_exercise_count: 5,
        current_exercise_id: None
    };

    let session_repo = SessionRepository::new(manager.pool().clone());

    session_repo.create(session).unwrap();

    dbg!(user);
}