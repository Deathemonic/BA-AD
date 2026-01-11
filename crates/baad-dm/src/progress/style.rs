use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Clone)]
pub struct StyleOptions {
    pub main: ProgressBarOpts,
    pub child: ProgressBarOpts
}

impl Default for StyleOptions {
    fn default() -> Self {
        Self {
            main: ProgressBarOpts::main_default(),
            child: ProgressBarOpts::child_default()
        }
    }
}

impl StyleOptions {
    pub fn new(main: ProgressBarOpts, child: ProgressBarOpts) -> Self { Self { main, child } }

    pub fn hidden() -> Self {
        Self {
            main: ProgressBarOpts::hidden(),
            child: ProgressBarOpts::hidden()
        }
    }

    pub fn is_enabled(&self) -> bool { self.main.enabled || self.child.enabled }
}

#[derive(Debug, Clone)]
pub struct ProgressBarOpts {
    pub template: Option<String>,
    pub progress_chars: Option<String>,
    pub enabled: bool,
    pub clear: bool
}

impl Default for ProgressBarOpts {
    fn default() -> Self {
        Self {
            template: None,
            progress_chars: None,
            enabled: true,
            clear: true
        }
    }
}

impl ProgressBarOpts {
    pub const CHARS_FINE: &'static str = "█▉▊▋▌▍▎▏  ";
    pub const CHARS_LINE: &'static str = "━╾╴─";
    pub const TEMPLATE_CHILD: &'static str = "{bar:40.green/black} {bytes:>11.green}/{total_bytes:<11.green} {bytes_per_sec:>13.red} eta {eta:.blue}";
    pub const TEMPLATE_MAIN: &'static str =
        "{bar:40.blue} {pos:>}/{len} ({percent}%) eta {eta_precise:.blue}";

    pub fn main_default() -> Self {
        Self {
            template: Some(Self::TEMPLATE_MAIN.into()),
            progress_chars: Some(Self::CHARS_FINE.into()),
            enabled: true,
            clear: false
        }
    }

    pub fn child_default() -> Self {
        Self {
            template: Some(Self::TEMPLATE_CHILD.into()),
            progress_chars: Some(Self::CHARS_LINE.into()),
            enabled: true,
            clear: true
        }
    }

    pub fn hidden() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }

    pub fn to_style(&self) -> ProgressStyle {
        let style = match &self.template {
            Some(template) => ProgressStyle::default_bar()
                .template(template)
                .unwrap_or_else(|_| ProgressStyle::default_bar()),
            None => ProgressStyle::default_bar()
        };

        match &self.progress_chars {
            Some(chars) => style.progress_chars(chars),
            None => style
        }
    }

    pub fn to_progress_bar(&self, len: u64) -> ProgressBar {
        if !self.enabled {
            return ProgressBar::hidden();
        }
        ProgressBar::new(len).with_style(self.to_style())
    }
}
