use std::fmt;

use chrono::Utc;
use tracing::{
    Event, Subscriber,
    field::{Field, Visit},
};
use tracing_subscriber::{
    fmt::{FmtContext, FormatEvent, FormatFields, FormattedFields, format::Writer},
    registry::LookupSpan,
};

pub(super) struct Readable;

#[derive(Default)]
struct Fields(Vec<(String, String)>);

impl Visit for Fields {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.push((field.name().to_string(), value.to_string()));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.0
            .push((field.name().to_string(), format!("{value:?}")));
    }
}

impl<S, N> FormatEvent<S, N> for Readable
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    N: for<'writer> FormatFields<'writer> + 'static,
{
    fn format_event(
        &self,
        context: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let mut fields = Fields::default();
        event.record(&mut fields);
        let message = fields
            .0
            .iter()
            .find(|(name, _)| name == "message")
            .map(|(_, value)| value.as_str())
            .unwrap_or("");
        writeln!(
            writer,
            "\n{} {:5} {}",
            Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC"),
            event.metadata().level(),
            message
        )?;
        writeln!(writer, "  module: {}", event.metadata().target())?;
        if let Some(scope) = context.event_scope() {
            write!(writer, "  context: ")?;
            let mut first = true;
            for span in scope.from_root() {
                if !first {
                    write!(writer, " > ")?;
                }
                first = false;
                write!(writer, "{}", span.metadata().name())?;
                let extensions = span.extensions();
                if let Some(fields) = extensions.get::<FormattedFields<N>>() {
                    if !fields.is_empty() {
                        write!(writer, " {{{fields}}}")?;
                    }
                }
            }
            writeln!(writer)?;
        }
        for (name, value) in fields.0 {
            if name == "message" {
                continue;
            }
            let formatted = serde_json::from_str::<serde_json::Value>(&value)
                .ok()
                .filter(|value| value.is_object() || value.is_array())
                .and_then(|value| serde_json::to_string_pretty(&value).ok())
                .unwrap_or(value);
            if formatted.contains('\n') {
                writeln!(writer, "  {name}:")?;
                for line in formatted.split('\n') {
                    writeln!(writer, "    {line}")?;
                }
            } else {
                writeln!(writer, "  {name}: {formatted}")?;
            }
        }
        Ok(())
    }
}
