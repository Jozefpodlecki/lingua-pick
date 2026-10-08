use std::fmt;

use chrono::Utc;
use serde_json::Value;
use tracing::{
    Event, Subscriber,
    field::{Field, Visit},
};
use tracing_subscriber::{
    fmt::{FmtContext, FormatEvent, FormatFields, format::Writer},
    registry::LookupSpan,
};

pub(super) struct Readable;

#[derive(Default)]
struct EventFields {
    message: Option<String>,
    values: Vec<(String, String)>,
}

impl Visit for EventFields {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.record(field.name(), value.to_owned());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.record(field.name(), format!("{value:?}"));
    }
}

impl EventFields {
    fn record(&mut self, name: &str, value: String) {
        if name == "message" {
            self.message = Some(value);
        } else {
            self.values.push((name.to_owned(), value));
        }
    }
}

impl<S, N> FormatEvent<S, N> for Readable
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    N: for<'writer> FormatFields<'writer> + 'static,
{
    fn format_event(
        &self,
        _context: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let mut fields = EventFields::default();
        event.record(&mut fields);

        let metadata = event.metadata();
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");

        write!(
            writer,
            "{timestamp} UTC {:5} [{}]",
            metadata.level(),
            metadata.target()
        )?;

        if let Some(message) = fields.message {
            if !message.is_empty() {
                write!(writer, " {message}")?;
            }
        }

        writeln!(writer)?;

        for (name, value) in fields.values {
            write_field(&mut writer, &name, &value, 1)?;
        }

        Ok(())
    }
}

fn write_field(
    writer: &mut Writer<'_>,
    name: &str,
    text: &str,
    depth: usize,
) -> fmt::Result {
    match serde_json::from_str::<Value>(text) {
        Ok(Value::Object(map)) if !map.is_empty() => {
            write_json(writer, name, &Value::Object(map), depth)
        }
        Ok(Value::Array(items)) if !items.is_empty() => {
            write_json(writer, name, &Value::Array(items), depth)
        }
        _ => write_text(writer, name, text, depth),
    }
}

fn write_json(
    writer: &mut Writer<'_>,
    name: &str,
    value: &Value,
    depth: usize,
) -> fmt::Result {
    match value {
        Value::Object(map) if !map.is_empty() => {
            write_label(writer, name, depth)?;

            for (key, value) in map {
                write_json(writer, key, value, depth + 1)?;
            }
        }

        Value::Array(items) if !items.is_empty() => {
            write_label(writer, name, depth)?;

            for (index, value) in items.iter().enumerate() {
                write_json(
                    writer,
                    &format!("[{index}]"),
                    value,
                    depth + 1,
                )?;
            }
        }

        Value::String(text) => {
            write_text(writer, name, text, depth)?;
        }

        _ => {
            write_text(writer, name, &value.to_string(), depth)?;
        }
    }

    Ok(())
}

fn write_label(
    writer: &mut Writer<'_>,
    name: &str,
    depth: usize,
) -> fmt::Result {
    write_indent(writer, depth)?;
    writeln!(writer, "{name}:")
}

fn write_text(
    writer: &mut Writer<'_>,
    name: &str,
    text: &str,
    depth: usize,
) -> fmt::Result {
    let text = text.trim_end_matches(['\r', '\n']);

    write_indent(writer, depth)?;

    if text.contains('\n') {
        writeln!(writer, "{name}:")?;

        for line in text.lines() {
            write_indent(writer, depth + 1)?;
            writeln!(writer, "{}", line.trim_end_matches('\r'))?;
        }
    } else {
        writeln!(writer, "{name}: {text}")?;
    }

    Ok(())
}

fn write_indent(writer: &mut Writer<'_>, depth: usize) -> fmt::Result {
    for _ in 0..depth {
        writer.write_str("  ")?;
    }

    Ok(())
}