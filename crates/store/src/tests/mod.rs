use crate::tests::context::DbContext;
use anyhow::{Context, Result};
mod context;
mod utils;

#[test]
fn it_should_migrate_database_and_perform_checks() -> Result<()> {
    let context = DbContext::new()?;

    context.basic_checks()?;

    let user = context.create_test_user()?;

    let source_lang = context.lang_repo.get_by_iso639_1("en")?.with_context(|| "Could not find language")?;
    let target_lang = context.lang_repo.get_by_iso639_1("aa")?.with_context(|| "Could not find language")?;
    let profile = context.create_profile(user.id, source_lang.id, target_lang.id)?;

    let source_lang = context.lang_repo.get_by_iso639_1("en")?.with_context(|| "Could not find language")?;
    let target_lang = context.lang_repo.get_by_iso639_1("zh")?.with_context(|| "Could not find language")?;
    let profile = context.create_profile(user.id, source_lang.id, target_lang.id)?;
    
    let feature_tree = context.get_features(target_lang.id)?;
    dbg!(feature_tree);

    context.set_active_profile(profile.id)?;

    let session = context.create_test_session(profile.id)?;
    let exercise_types = context.exercise_type_repo.get_by_name("draw-hanzi")?.with_context(|| "Could not find exercise type")?;
    let exercise = context.create_test_exercise(session.id, exercise_types.id)?;
    let wotd = context.create_wotd(profile.target_language_id)?;

    Ok(())
}
