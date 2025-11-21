//! Progress indicators for long-running operations

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Progress tracker for CLI operations
pub struct ProgressTracker {
    bar: ProgressBar,
}

impl ProgressTracker {
    /// Create a new progress tracker with a known total
    pub fn new(total: u64, message: &str) -> Self {
        let bar = ProgressBar::new(total);
        bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        bar.set_message(message.to_string());
        Self { bar }
    }

    /// Create a spinner for operations without known progress
    pub fn spinner(message: &str) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        bar.set_message(message.to_string());
        bar.enable_steady_tick(Duration::from_millis(100));
        Self { bar }
    }

    /// Increment progress
    pub fn inc(&self, delta: u64) {
        self.bar.inc(delta);
    }

    /// Set progress to specific value
    pub fn set_position(&self, pos: u64) {
        self.bar.set_position(pos);
    }

    /// Update message
    pub fn set_message(&self, message: &str) {
        self.bar.set_message(message.to_string());
    }

    /// Finish with success message
    pub fn finish_success(&self, message: &str) {
        self.bar.finish_with_message(format!("✓ {}", message));
    }

    /// Finish with error message
    pub fn finish_error(&self, message: &str) {
        self.bar.finish_with_message(format!("✗ {}", message));
    }

    /// Abandon the progress bar (for errors)
    pub fn abandon(&self) {
        self.bar.abandon();
    }
}

impl Drop for ProgressTracker {
    fn drop(&mut self) {
        if !self.bar.is_finished() {
            self.bar.finish_and_clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_tracker_creation() {
        let tracker = ProgressTracker::new(100, "Testing");
        tracker.inc(10);
        tracker.finish_success("Complete");
    }

    #[test]
    fn test_spinner_creation() {
        let spinner = ProgressTracker::spinner("Processing...");
        spinner.finish_success("Done");
    }
}
