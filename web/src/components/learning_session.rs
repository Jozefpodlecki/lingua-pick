use alloc::{rc::Rc, string::String};
use core::cell::Cell;
use lingua_api::Runtime;
use lingua_core::{Exercise, ExerciseContent, LanguageId, Session};
use lingua_web_exercise::{AnswerCard, SessionProgress};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{client::ApiClient, routes::Route};

#[function_component(LearningSession)]
pub fn learning_session() -> Html {
    let loaded = use_state(|| None::<Result<Session, String>>);
    let selected = use_state(|| None::<String>);
    let feedback = use_state(|| None::<(Exercise, bool)>);
    let answer_error = use_state(|| None::<String>);
    let client = use_context::<ApiClient>();

    // {
    //     let loaded = loaded.clone();

    //     use_effect_with(
    //         (props.language.clone(), props.runtime),
    //         move |(language, runtime)| {
    //             let cancelled = Rc::new(Cell::new(false));
    //             let request_cancelled = cancelled.clone();
    //             let language = language.clone();
    //             let runtime = *runtime;

    //             spawn_local(async move {
    //                 let result = load_session(runtime, &language).await;

    //                 if !request_cancelled.get() {
    //                     loaded.set(Some(result));
    //                 }
    //             });

    //             move || cancelled.set(true)
    //         },
    //     );
    // }

    let onselect = {
        let selected = selected.clone();
        let answer_error = answer_error.clone();

        Callback::from(move |id: String| {
            selected.set(Some(id));
            answer_error.set(None);
        })
    };

    let oncheck = {
        let loaded = loaded.clone();
        let selected = selected.clone();
        let feedback = feedback.clone();
        let answer_error = answer_error.clone();

        Callback::from(move |_| {
            if feedback.is_some() {
                return;
            }

            let (Some(Ok(session)), Some(choice)) = (loaded.as_ref(), selected.as_ref()) else {
                return;
            };

            let Some(exercise) = session.current().cloned() else {
                return;
            };

            let mut session = session.clone();

            match session.answer(choice) {
                Ok(outcome) => {
                    feedback.set(Some((exercise, outcome.correct)));
                    loaded.set(Some(Ok(session)));
                }
                Err(error) => answer_error.set(Some(alloc::format!("{error}"))),
            }
        })
    };

    let oncontinue = {
        let feedback = feedback.clone();
        let selected = selected.clone();

        Callback::from(move |_| {
            selected.set(None);
            feedback.set(None);
        })
    };

    let (progress, content) = match loaded.as_ref() {
        None => (
            Html::default(),
            html! { <p role="status" class="text-gray-300">{"Loading…"}</p> },
        ),
        Some(Err(error)) => (
            Html::default(),
            html! {
                <section data-state="error">
                    <p role="alert" class="mb-6 text-amber-200">{error}</p>
                    <Link<Route> to={Route::Learn} classes="text-teal-300 underline">{"Back to dashboard"}</Link<Route>>
                </section>
            },
        ),
        Some(Ok(session)) => {
            let current = session.answers().len()
                + usize::from(feedback.is_none() && session.current().is_some());
            let progress = html! {
                <SessionProgress {current} total={session.total()} />
            };
            let content = if let Some((exercise, correct)) = feedback.as_ref() {
                let result = if *correct { "Correct" } else { "Incorrect" };

                html! {
                    <section data-state="feedback">
                        <h2 class="mb-4 text-3xl">{&exercise.prompt.value}</h2>
                        <p role="status" class="mb-4">{result}</p>
                        <button type="button" onclick={oncontinue} class="rounded-lg bg-teal-700 px-5 py-3 focus-visible:outline-2 focus-visible:outline-teal-400">{"Continue"}</button>
                    </section>
                }
            } else if let Some(exercise) = session.current() {
                let ExerciseContent::SingleChoice(single_choice) = &exercise.content;
                let cards = single_choice.choices().iter().map(|choice| {
                    let is_selected = selected.as_ref() == Some(&choice.id);

                    html! {
                        <AnswerCard key={choice.id.clone()} choice={choice.clone()} selected={is_selected} onselect={onselect.clone()} />
                    }
                }).collect::<Html>();
                let disabled = selected.is_none();
                let error_content = answer_error
                    .as_ref()
                    .map(|error| {
                        html! {
                            <p role="alert">{error}</p>
                        }
                    })
                    .unwrap_or_default();

                html! {
                    <section data-state="answering">
                        <p class="mb-4 text-gray-300">{&exercise.instruction}</p>
                        <h2 class="mb-6 text-3xl">{&exercise.prompt.value}</h2>
                        <div class="mb-6 grid gap-4 sm:grid-cols-3">{cards}</div>
                        {error_content}
                        <button type="button" onclick={oncheck} {disabled} class="rounded-lg bg-teal-700 px-5 py-3 disabled:opacity-40 focus-visible:outline-2 focus-visible:outline-teal-400">{"Check"}</button>
                    </section>
                }
            } else {
                let score = alloc::format!("{} / {}", session.correct_count(), session.total());

                html! {
                    <section data-state="complete">
                        <h2 class="mb-4 text-2xl">{"Session complete"}</h2>
                        <p class="mb-6 text-3xl font-semibold">{score}</p>
                        <Link<Route> to={Route::Learn} classes="text-teal-300 underline">{"Back to dashboard"}</Link<Route>>
                    </section>
                }
            };

            (progress, content)
        }
    };

    html! {
        <div class="min-h-screen bg-gray-950 text-gray-100" data-screen="session">
            {progress}
            <main class="mx-auto w-full max-w-4xl px-4 py-10 sm:px-6 sm:py-14 lg:px-8">
                {content}
            </main>
        </div>
    }
}
