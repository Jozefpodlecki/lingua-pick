use chrono::{Local, Utc};
use lingua_pick_store::{DatabaseManager, LanguageRepository, User, UserRepository};
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

    let user  = user_repo.get_by_username("test").unwrap();

    dbg!(user);
}