use indicatif::{ProgressBar, ProgressStyle, ProgressDrawTarget};

use api::event::{Event, EventListener};

#[derive(Debug)]
pub struct ProgressBarListener {
    progress_bar: ProgressBar,
}

impl ProgressBarListener {
    pub fn new() -> ProgressBarListener {
        ProgressBarListener::with_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:60.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("#>-"))
    }

    pub fn with_style(style: ProgressStyle) -> ProgressBarListener {
        let progress_bar = ProgressBar::hidden();
        progress_bar.set_style(style);

        ProgressBarListener {
            progress_bar,
        }
    }
}

impl EventListener for ProgressBarListener {
    fn on_event(&self, event: &Event) {
        match *event {
            Event::TestRunStarted { num_cukes, .. } => {
                self.progress_bar.set_length(num_cukes as u64);
                self.progress_bar.enable_steady_tick(100);
                self.progress_bar.set_draw_target(ProgressDrawTarget::stderr())
            },
            Event::TestCaseFinished { .. } => {
                self.progress_bar.inc(1);
            },
            Event::TestRunFinished { .. } => {
                self.progress_bar.finish();
            },
            _ => {},
        }
    }
}
