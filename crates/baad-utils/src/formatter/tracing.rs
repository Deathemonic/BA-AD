use std::fmt;

use better_default::Default;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;

use crate::formatter::fields::FieldCollector;
use crate::formatter::line::LineFormatter;

#[derive(Clone, Default)]
#[default(line_formatter: LineFormatter::new())]
pub struct ConsoleFormatter {
    line_formatter: LineFormatter
}

impl ConsoleFormatter {
    pub fn new() -> Self {
        Self {
            line_formatter: LineFormatter::new()
        }
    }

    pub const fn with_timestamps(mut self, include_timestamps: bool) -> Self {
        self.line_formatter = self.line_formatter.with_timestamps(include_timestamps);
        self
    }
}

impl<S, N> FormatEvent<S, N> for ConsoleFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>
    ) -> fmt::Result {
        let level = event.metadata().level();
        let mut visitor = FieldCollector::new();
        event.record(&mut visitor);

        let is_success = level == &Level::INFO && visitor.has_success_field();

        if visitor.is_simple_message() && !self.line_formatter.includes_timestamps() {
            if let Some((_, message)) = visitor.fields.first() {
                self.line_formatter.write_simple_message(
                    &mut writer,
                    level,
                    is_success,
                    message
                )?;
            }
            return writeln!(writer);
        }

        let fields: Vec<(&str, &str)> = visitor
            .fields
            .iter()
            .filter(|(name, _)| *name != "message")
            .map(|(name, value)| (*name, value.as_ref()))
            .collect();

        let message = visitor
            .fields
            .iter()
            .find(|(name, _)| *name == "message")
            .map_or("", |(_, value)| value.as_ref());

        self.line_formatter.write_line(&mut writer, level, is_success, message, &fields)
    }
}
