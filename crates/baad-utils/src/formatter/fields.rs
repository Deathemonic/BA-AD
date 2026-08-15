use std::borrow::Cow;
use std::fmt;

use itoa::Buffer;
use smallvec::SmallVec;
use tracing::field::{Field, Visit};

pub struct FieldCollector {
    pub fields: SmallVec<[(&'static str, Cow<'static, str>); 4]>
}

impl FieldCollector {
    #[inline]
    pub fn new() -> Self { Self { fields: SmallVec::new() } }

    #[inline]
    pub fn has_success_field(&self) -> bool {
        self.fields.iter().any(|(name, value)| *name == "success" && value == "true")
    }

    #[inline]
    pub fn is_simple_message(&self) -> bool {
        self.fields.len() == 1 && self.fields.first().is_some_and(|(name, _)| *name == "message")
    }
}

impl Visit for FieldCollector {
    fn record_i64(&mut self, field: &Field, value: i64) {
        let mut buf = Buffer::new();
        self.fields.push((field.name(), Cow::Owned(buf.format(value).to_owned())));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        let mut buf = Buffer::new();
        self.fields.push((field.name(), Cow::Owned(buf.format(value).to_owned())));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.fields.push((field.name(), Cow::Borrowed(if value { "true" } else { "false" })));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields.push((field.name(), Cow::Owned(value.to_owned())));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.fields.push((field.name(), Cow::Owned(format!("{value:?}"))));
    }
}
