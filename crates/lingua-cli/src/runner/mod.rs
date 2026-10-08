use tracing::Instrument;
use uuid::Uuid;

use crate::{
    evaluator::ExerciseEvaluator,
    generator::ExerciseGenerator,
    seed::{self, SeedResult},
    simulator::ExerciseSimulator,
};

pub use config::RunnerConfig;
pub use error::RunnerError;

mod config;
mod context;
mod error;
mod exercise;

pub struct Runner {
    config: RunnerConfig,
    stores: SeedResult,
    generator: ExerciseGenerator,
    simulator: ExerciseSimulator,
    evaluator: ExerciseEvaluator,
}

impl Runner {
    pub fn open(config: RunnerConfig) -> Result<Self, RunnerError> {
        tracing::info!(database = %config.database_path.display(), model = %config.model,
            source = %config.source_language, target = %config.target_language, "Opening runner");
        let stores = seed::open(
            &config.database_path,
            &config.user_name,
            &config.source_language,
            &config.target_language,
        )?;
        let generator = ExerciseGenerator::new(&config.model);
        let evaluator = ExerciseEvaluator::new(&config.model);
        let simulator = ExerciseSimulator::new(&config.model, config.mistake_distribution.clone());
        Ok(Self {
            config,
            stores,
            generator,
            simulator,
            evaluator,
        })
    }

    pub async fn run(&self, run_id: Uuid) -> Result<(), RunnerError> {
        self.run_session()
            .instrument(
                tracing::info_span!("run", %run_id, user_id = %self.stores.user_id,
                session_id = %self.stores.session_id, target = %self.config.target_language),
            )
            .await
    }

    async fn run_session(&self) -> Result<(), RunnerError> {
        let user = self
            .stores
            .user_store
            .get_by_id(self.stores.user_id)?
            .ok_or(RunnerError::InvalidState("saved user is missing"))?;
        let mut session = self
            .stores
            .session_store
            .get_by_id(self.stores.session_id)?
            .ok_or(RunnerError::InvalidState("saved session is missing"))?;
        tracing::info!(
            completed = session.exercise_count,
            total = session.max_exercise_count,
            "Session started"
        );
        while session.exercise_count < session.max_exercise_count {
            self.run_exercise(&user, &session)
                .instrument(tracing::info_span!(
                    "exercise",
                    number = session.exercise_count + 1
                ))
                .await?;
            session = self.stores.session_store.get_by_id(session.id)?.ok_or(
                RunnerError::InvalidState("session disappeared after completion"),
            )?;
            tracing::info!(
                completed = session.exercise_count,
                total = session.max_exercise_count,
                "Session progress"
            );
        }
        tracing::info!(completed = session.exercise_count, "Session completed");
        Ok(())
    }
}
