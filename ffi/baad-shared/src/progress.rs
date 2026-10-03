#[diplomat::bridge]
pub mod ffi {
    pub enum BaadSharedProgressEventKind {}
    pub enum BaadSharedProgressStatusKind {}
    pub enum BaadSharedProgressUnit {}
    #[diplomat::opaque]
    pub struct BaadSharedProgressEvent(pub baad_shared::ProgressEvent);

    impl BaadSharedProgressEvent {
        pub fn kind(&self) -> BaadSharedProgressEventKind { (&self.0).into() }

        pub fn id<'a>(&'a self) -> &'a str {
            match &self.0 {
                baad_shared::ProgressEvent::Started { id, .. }
                | baad_shared::ProgressEvent::Advance { id, .. }
                | baad_shared::ProgressEvent::Completed { id, .. } => id
            }
        }

        pub fn label<'a>(&'a self) -> &'a str {
            match &self.0 {
                baad_shared::ProgressEvent::Started { label, .. } => label,
                _ => ""
            }
        }

        pub const fn current(&self) -> u64 {
            match self.0 {
                baad_shared::ProgressEvent::Advance { current, .. } => current,
                _ => 0
            }
        }

        pub const fn total(&self) -> u64 {
            match self.0 {
                baad_shared::ProgressEvent::Started { total, .. }
                | baad_shared::ProgressEvent::Advance { total, .. } => total,
                _ => 0
            }
        }

        pub fn unit(&self) -> BaadSharedProgressUnit {
            match self.0 {
                baad_shared::ProgressEvent::Started { unit, .. } => unit.into(),
                _ => BaadSharedProgressUnit::Bytes
            }
        }

        pub fn status(&self) -> BaadSharedProgressStatusKind {
            match &self.0 {
                baad_shared::ProgressEvent::Completed { status, .. } => status.into(),
                _ => BaadSharedProgressStatusKind::NotStarted
            }
        }

        pub fn reason<'a>(&'a self) -> &'a str {
            match &self.0 {
                baad_shared::ProgressEvent::Completed { status, .. } => {
                    crate::progress::status_reason(status)
                }
                _ => ""
            }
        }
    }

    #[diplomat::opaque]
    pub struct BaadSharedProgress(pub Option<baad_shared::Progress>);

    impl BaadSharedProgress {
        pub fn start(id: &str, label: &str, count_unit: bool, total: u64) -> Box<Self> {
            Box::new(Self(Some(baad_shared::Progress::start(
                id,
                label,
                if count_unit {
                    baad_shared::ProgressUnit::Count
                } else {
                    baad_shared::ProgressUnit::Bytes
                },
                total
            ))))
        }

        pub fn advance(&self, delta: u64) {
            if let Some(progress) = &self.0 {
                progress.advance(delta);
            }
        }

        pub fn set_total(&self, total: u64) {
            if let Some(progress) = &self.0 {
                progress.set_total(total);
            }
        }

        pub fn finish(&mut self) {
            if let Some(progress) = self.0.take() {
                progress.finish();
            }
        }

        pub fn fail(&mut self, reason: &str) {
            if let Some(progress) = self.0.take() {
                progress.fail(reason.to_owned());
            }
        }
    }
}
