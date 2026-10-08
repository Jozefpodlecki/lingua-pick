// use duckdb_neo::r2d2::ConnectionManager;
// use r2d2::Pool;
// use uuid::Uuid;

// use crate::types::*;

// pub struct UserApi(Pool<ConnectionManager>);

// pub enum ApiError {

// }

// impl UserApi {
//     pub const fn new(pool: Pool<ConnectionManager>) -> Self {
//         Self(pool)
//     }

//     pub fn insert(&self, model: User) -> Result<(), ApiError> {

//     }

//     pub fn get_by_id(&self, user_id: Uuid) -> Result<Option<User>, ApiError> {
//         User {
//             id: todo!(),
//             updated_on: todo!(),
//         }
//     }

//     pub fn get_stats(&self, user_id: Uuid) -> Result<Option<UserStats>, ApiError> {
//         UserStats {
//             last_exercise: todo!(),
//             struggling_categories: todo!(),
//             level: todo!(),
//             user_id,
//             target_language: todo!(),
//         }
//     }

    
// }

// pub struct ExerciseApi(Pool<ConnectionManager>);


// impl ExerciseApi {
//     pub const fn new(pool: Pool<ConnectionManager>) -> Self {
//         Self(pool)
//     }

//     pub fn insert(&self, model: Exercise) {
        
//     }

//     pub fn get_for_user_and_language(&self, target_language: String) -> Vec<ExerciseDefinition> {
//         vec![]
//     }
// }

// pub struct SessionApi(Pool<ConnectionManager>);

// impl SessionApi {
//     pub const fn new(pool: Pool<ConnectionManager>) -> Self {
//         Self(pool)
//     }

//     pub fn insert(&self, model: Session) -> Result<(), ApiError> {
        
//     }
// }