//! A simple animated terminal spinner for indicating ongoing work.

use crate::symbols::frames::DOTS;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// An animated spinner for displaying indeterminate progress in the terminal.
///
/// Spinners run on a background thread once started via [`Spinner::start`],
/// and can be stopped at any time with [`Spinner::stop`].
///
/// # Examples
///
/// ```no_run
/// use blazer::spinner::Spinner;
/// use std::time::Duration;
///
/// let mut spinner = Spinner::new("Loading...");
/// spinner.start();
/// std::thread::sleep(Duration::from_secs(2));
/// spinner.stop("Done!");
/// ```
pub struct Spinner {
    message: String,
    frames: &'static [&'static str],
    interval: Duration,
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Spinner {
    /// Creates a new spinner with the default frame set and a message.
    pub fn new(message: impl Into<String>) -> Self {
        Spinner {
            message: message.into(),
            frames: DOTS,
            interval: Duration::from_millis(80),
            running: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    /// Sets a custom frame set for the spinner animation.
    ///
    /// See [`crate::symbols::frames`] for built-in options.
    pub fn with_frames(mut self, frames: &'static [&'static str]) -> Self {
        self.frames = frames;
        self
    }

    /// Sets the delay between animation frames.
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Starts the spinner animation on a background thread.
    ///
    /// Calling this while the spinner is already running is a no-op.
    pub fn start(&mut self) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }
        self.running.store(true, Ordering::SeqCst);

        let running = Arc::clone(&self.running);
        let frames = self.frames;
        let message = self.message.clone();
        let interval = self.interval;

        let handle = thread::spawn(move || {
            let mut i = 0usize;
            while running.load(Ordering::SeqCst) {
                let frame = frames[i % frames.len()];
                print!("\r{} {}", frame, message);
                let _ = io::stdout().flush();
                thread::sleep(interval);
                i = i.wrapping_add(1);
            }
        });

        self.handle = Some(handle);
    }

    /// Updates the message displayed next to the spinner while it is running.
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    /// Stops the spinner, clears its line, and prints a final message.
    pub fn stop(&mut self, final_message: impl AsRef<str>) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        print!("\r\x1b[2K");
        println!("{}", final_message.as_ref());
        let _ = io::stdout().flush();
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}