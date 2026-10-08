use lingua_ai::LlmClient;
use serde_json::json;

use crate::{api::*, generator::*, simulator::*, seed::SeedResult, types::*};

extern crate alloc;

mod types;
mod utils;
mod seed;
mod migration;
mod store;
mod prompt;
mod generator;
mod evaluator;
mod simulator;
mod api;

#[tokio::main]
async fn main() {
    let SeedResult {
        user_id,
        session_id,
        lang_store,
        user_store,
        user_stats_store,
        exercise_store,
        exercise_definition_store,
        session_store,
    } = seed::generate("test-user", "en-GB", "pt-BR").unwrap();

    let generator = ExerciseGenerator::new("meta/muse-glimmer");
    let evaluator = ExerciseEvaluator::new("meta/muse-glimmer");
    let simulator = ExerciseSimulator::new("meta/muse-glimmer");

    let user = user_store.get_by_id(user_id).unwrap().unwrap();
    let mut session = session_store.get_by_id(session_id).unwrap().unwrap();

    while session.exercise_count < session.max_exercise_count {
        // 1. Load the learner's current state for the target language.
        //
        // This includes aggregate user stats and, eventually, UserConcept
        // mastery/confidence values. This state changes after every exercise,
        // so it should be loaded again on every iteration.
        let user_stats = user_stats_store
            .get(user.id, &session.target_language_id)
            .unwrap()
            .unwrap();

        // 2. Find the exercise definitions available for the target language.
        //
        // These describe the exercise mechanics the generator is allowed to
        // choose from: fill_blank, conjugate_verb, dialogue, etc.
        let available_exercises = exercise_definition_store
            .get_for_language(&session.target_language_id)
            .unwrap();

        if let Some(last_exercise) = exercise_store
            .get_last_for_session(session.id)
            .unwrap()
        {
            available_exercises.retain(|definition| {
                definition.id != last_exercise.definition_id
            });
        }

        // 3. Build the generation request.
        //
        // Eventually this should also contain relevant UserConcept records,
        // weak concepts, recent exercise history, and possibly topic selection.
        let request = ExerciseRequest {
            source_language: user.source_language_id.clone(),
            target_language: session.target_language_id.clone(),
            user_stats,
            available_exercises,
        };

        // 4. Ask the AI to choose an appropriate exercise definition and
        // generate exercise content from the learner's current state.
        //
        // GeneratedExercise is transient AI output. It is not yet our
        // persisted Exercise database entity.
        let generated = generator
            .generate(&request)
            .await
            .unwrap();

        // 5. Convert the generated exercise into a persisted Exercise.
        //
        // Assign application-owned data here:
        // - exercise UUID
        // - session UUID
        // - definition UUID
        // - timestamps
        // - status
        //
        // The AI should not be responsible for generating database identity
        // or session state.
        let mut exercise = Exercise::from_generated(
            session.id,
            generated,
        );

        // 6. Store the exercise before presenting it to the learner.
        //
        // This gives the exercise a durable identity and allows interactive
        // exercises such as dialogues to append turns later.
        exercise_store.insert(&exercise).unwrap();

        // 7. Simulate the learner answering the exercise.
        //
        // This exists only for testing. In the real application this step
        // will be performed by the UI/user.
        let answer = simulator
            .answer(&exercise)
            .await
            .unwrap();

        // 8. Persist the learner's answer.
        //
        // For single-turn exercises this is exercise.answer.
        // Interactive exercises will instead accumulate exercise_turn rows
        // until the exercise is complete.
        exercise_store
            .set_answer(exercise.id, &answer)
            .unwrap();

        // 9. Evaluate the answer.
        //
        // The evaluator receives the exercise definition, generated payload,
        // learner answer and targeted concepts. It returns a structured
        // verdict rather than only correct/incorrect.
        let verdict = evaluator
            .evaluate(&exercise, &answer)
            .await
            .unwrap();

        // 10. Persist the verdict and mark the exercise completed.
        //
        // The completed Exercise becomes the historical record of exactly
        // what was asked, what the learner answered, and how it was evaluated.
        exercise_store
            .complete(exercise.id, &answer, &verdict)
            .unwrap();

        // 11. Update UserConcept mastery.
        //
        // Each concept evaluated by the verdict updates:
        // - attempts
        // - correct
        // - mastery
        // - confidence
        // - last_seen
        //
        // This is the important feedback loop: the result of this exercise
        // changes what the generator should choose next.
        //
        // user_concept_store.apply_verdict(
        //     user.id,
        //     &session.target_language_id,
        //     &verdict,
        // ).unwrap();

        // 12. Update aggregate user statistics if we decide to keep them.
        //
        // UserConcept should contain detailed learning state. UserStats should
        // only contain useful aggregate/session-independent information rather
        // than duplicating concept mastery.
        //
        // user_stats_store.apply_verdict(...).unwrap();

        // 13. Advance the session.
        //
        // Increment exercise_count after the exercise has been completed and
        // learner state has been successfully updated.
        session_store
            .increment_exercise_count(session.id)
            .unwrap();

        // 14. Reload the session and return to step 1.
        //
        // Because UserConcept/UserStats changed, the next generation request
        // sees the result of the previous exercise and can adapt accordingly.
        session = session_store
            .get_by_id(session.id)
            .unwrap()
            .unwrap();
    }

    // 15. Session complete.
    //
    // At this point we could produce a session summary, identify newly
    // discovered weak concepts, and report progress to the learner.
}