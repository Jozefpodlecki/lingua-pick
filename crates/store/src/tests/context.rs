
use std::collections::{HashMap, HashSet};

use crate::{tests::utils::*, *};

use super::*;
use chrono::Utc;
use uuid::Uuid;
use anyhow::Result;

pub struct DbContext {
    pub manager: DatabaseManager,
    pub lang_repo: LanguageRepository,
    pub lang_feature_repo: LanguageFeatureRepository,
    pub lang_feature_lang_repo: LanguageFeatureLanguageRepository,
    pub user_repo: UserRepository,
    pub profile_repo: UserProfileRepository,
    pub session_repo: SessionRepository,
    pub exercise_type_repo: ExerciseTypeRepository,
    pub exercise_repo: ExerciseRepository,
    pub lexeme_repo: LexemeRepository,
    pub wotd_repo: WordOfTheDayRepository
}

impl DbContext {
    pub fn new() -> Result<Self> {
        let manager = DatabaseManager::memory()?;
        manager.ensure_created()?;

        let pool = manager.pool().clone();
        Ok(Self {
            manager,
            lang_repo: LanguageRepository::new(pool.clone()),
            lang_feature_repo: LanguageFeatureRepository::new(pool.clone()),
            lang_feature_lang_repo: LanguageFeatureLanguageRepository::new(pool.clone()),
            user_repo: UserRepository::new(pool.clone()),
            profile_repo: UserProfileRepository::new(pool.clone()),
            session_repo: SessionRepository::new(pool.clone()),
            exercise_type_repo: ExerciseTypeRepository::new(pool.clone()),
            exercise_repo: ExerciseRepository::new(pool.clone()),
            lexeme_repo: LexemeRepository::new(pool.clone()),
            wotd_repo: WordOfTheDayRepository::new(pool.clone())
        })
    }

    pub fn basic_checks(&self) -> Result<()> {
        let langs = self.lang_repo.get_all().unwrap();
        assert!(!langs.is_empty(), "Languages table should be seeded");

        let first_lang = langs.first().unwrap();

        let lang_features = self.lang_feature_repo.get_all()?;
        assert!(!lang_features.is_empty(), "Language features table should be seeded");

        Ok(())
    }

    pub fn create_test_user(&self) -> Result<User> {
        let user_id = Uuid::now_v7();
        let user = User {
            id: user_id,
            created_on: Utc::now(),
            user_name: "test_user".into(),
            password_hash: vec![0],
        };

        let user = self.user_repo.create(user)?;
        let fetched = self.user_repo
            .get_by_username("test_user")?
            .with_context(|| "User should exist after creation")?;

        Ok(fetched)
    }

    pub fn create_profile(&self, user_id: Uuid, source_language_id: u32, target_language_id: u32) -> Result<UserProfile> {
        let profile = UserProfile {
            id: Uuid::now_v7(),
            user_id,
            created_on: Utc::now(),
            updated_on: Utc::now(),
            is_active: false,
            source_language_id,
            target_language_id
        };

        let profile = self.profile_repo.create(profile)?;
        Ok(profile)
    }

    pub fn set_active_profile(&self, id: Uuid) -> Result<()> {
        self.profile_repo.set_active(id, true)
    }

    pub fn create_test_session(&self, user_profile_id: Uuid) -> Result<Session> {
        let session = Session {
            id: Uuid::now_v7(),
            user_profile_id,
            created_on: Utc::now(),
            updated_on: Utc::now(),
            completed_on: None,
            exercise_count: 0,
            total_exercise_count: 5,
            current_exercise_id: None
        };

        let profile = self.profile_repo.get_by_id(user_profile_id)?.with_context(|| "Could not find profile")?;
        let session = self.session_repo.create(session)?;
        self.session_repo.link_to_user(session.id, profile.user_id)?;
        self.session_repo.get_by_id(session.id)?.with_context(|| "Could not find session")?;

        Ok(session)
    }

    pub fn get_features(&self, language_id: u32) -> Result<Vec<FeatureNode>> {
        let lang_features = self.lang_feature_repo.get_all()?;
        let items = self.lang_feature_lang_repo.get_by_language_id(language_id)?;
        println!("{:?}", items);

        let all_features_map: HashMap<u32, LanguageFeature> =
            lang_features.iter().map(|f| (f.id, f.clone())).collect();

        let mut collected: HashMap<u32, LanguageFeature> = HashMap::new();

        for item in &items {
            if let Some(feature) = all_features_map.get(&item.feature_id) {
                collect_with_ancestors(feature, &all_features_map, &mut collected);
            }
        }

        let filtered_features: Vec<LanguageFeature> = collected.values().cloned().collect();

        Ok(build_feature_tree(&filtered_features))
    }

    pub fn create_test_exercise(&self, session_id: Uuid, exercise_type_id: u8) -> Result<Exercise> {
        let mut exercise = Exercise {
            id: 0,
            type_id: exercise_type_id,
            session_id: session_id,
            created_on: Utc::now(),
            completed_on: None,
            data: "".into(),
            result: None,
            verdict: None,
        };

        exercise.id = self.exercise_repo.create(&exercise)?;
        self.exercise_repo.set_result(exercise.id, "")?;
        self.exercise_repo.set_verdict(exercise.id, "")?;
        self.exercise_repo.complete(exercise.id, Utc::now())?;

        Ok(exercise)
    }

    pub fn create_wotd(&self, language_id: u32) -> Result<WordOfTheDay> {
        let lexemes = self.lexeme_repo.get_by_language_id(language_id)?;
        let lexeme = lexemes.into_vec().into_iter().next().with_context(|| "Could not find lexeme")?;

        let wotd = WordOfTheDay {
            id: Utc::now().date_naive(),
            lexeme_id: lexeme.id,
            language_id,
        };

        let wotd = self.wotd_repo.create(wotd)?;
        let wotd = self.wotd_repo.get_by_id(wotd.id)?.with_context(|| "Could not find word of the day")?;

        Ok(wotd)
    }
}
