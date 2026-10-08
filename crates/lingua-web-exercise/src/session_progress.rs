use alloc::string::ToString;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ProgressStage {
    Starting,
    Advancing,
    Finishing,
}

impl ProgressStage {
    fn from_position(current: usize, total: usize) -> Self {
        let percentage = current.saturating_mul(100) / total.max(1);

        match percentage {
            0..=33 => Self::Starting,
            34..=66 => Self::Advancing,
            _ => Self::Finishing,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Starting => "starting",
            Self::Advancing => "advancing",
            Self::Finishing => "finishing",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SessionProgressProps {
    pub current: usize,
    pub total: usize,
}

#[function_component(SessionProgress)]
pub fn session_progress(props: &SessionProgressProps) -> Html {
    let current_value = props.current.min(props.total);
    let percentage = current_value.saturating_mul(100) / props.total.max(1);
    let label = alloc::format!("Exercise {} of {}", current_value, props.total);
    let current = current_value.to_string();
    let total = props.total.to_string();
    let stage = ProgressStage::from_position(current_value, props.total).name();
    let width = alloc::format!("width: {percentage}%");

    html! {
        <header class="border-b border-gray-800 bg-gray-950" data-progress-stage={stage}>
            <div class="mx-auto w-full max-w-6xl px-4 py-5 sm:px-6 lg:px-8">
                <div class="mb-2 flex items-center justify-between text-sm text-gray-300">
                    <span>{label.clone()}</span>
                    <span>{alloc::format!("{percentage}%")}</span>
                </div>
                <div role="progressbar" aria-label={label} aria-valuemin="0" aria-valuemax={total}
                    aria-valuenow={current} class="h-2 overflow-hidden rounded-full bg-gray-800">
                    <div data-stage={stage} style={width}
                        class="h-full rounded-full bg-rose-500 transition-[width,background-color] duration-300 data-[stage=advancing]:bg-amber-400 data-[stage=finishing]:bg-emerald-500">
                    </div>
                </div>
            </div>
        </header>
    }
}
