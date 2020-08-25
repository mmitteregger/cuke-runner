use indicatif::{ProgressBar, ProgressDrawTarget};
pub use indicatif::ProgressStyle;

use cuke_runner::api::event::{Event, EventListener};

/// Shows the cuke-runner tests progress with a progress bar.
///
/// The `progress_bar` feature needs to be enabled to use this event listener.
///
/// This listener implements `Sync` and thus can be used in parallel execution modes.
///
/// # Examples
///
/// ```rust
/// use cuke_runner_listener::{ProgressBarListener, ProgressStyle};
///
/// let event_listeners = &[
///     &ProgressBarListener::with_style(ProgressStyle::default_bar()
///         .template("[{elapsed}] [{bar:60.cyan/blue}] {pos}/{len}")
///         .progress_chars("=> ")),
/// ];
/// ```
///
/// <script src="https://asciinema.org/a/gP63IBMKo9D9kBU9wKHV9Qam5.js" id="asciicast-gP63IBMKo9D9kBU9wKHV9Qam5" async data-rows="20" data-cols="100"></script>
#[derive(Debug)]
pub struct ProgressBarListener {
    progress_bar: ProgressBar,
}

impl Default for ProgressBarListener {
    fn default() -> ProgressBarListener {
        ProgressBarListener::with_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:60.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("#>-"))
    }
}

impl ProgressBarListener {
    pub fn new() -> ProgressBarListener {
        ProgressBarListener::default()
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
    fn on_event(&self, event: &Event<'_, '_>) {
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

#[cfg(test)]
mod tests {
    use super::ProgressBarListener;

    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}

    #[test]
    fn test_send_sync() {
        assert_send::<ProgressBarListener>();
        assert_sync::<ProgressBarListener>();
    }
}
