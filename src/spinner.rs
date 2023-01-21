use std::{
    sync::mpsc::{channel, Sender, TryRecvError},
    thread,
    time::{Duration, Instant},
};

use crate::{stream::Stream, variants};

#[derive(Debug)]
pub struct Spinner {
    sender: Sender<(Instant, Option<String>)>,
    join: Option<thread::JoinHandle<()>>,
    stream: Stream,
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if self.join.is_some() {
            self.sender.send((Instant::now(), None)).unwrap();
            self.join.take().unwrap().join().unwrap();
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpinnerBuilder {
    spinner: variants::SpinnerVariant,
    message: String,
    start_time: Option<Instant>,
    stream: Option<Stream>,
}

impl SpinnerBuilder {
    /// Create a new spinner
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use atomic_spinners::{Stream, SpinnerBuilder};
    ///
    /// let sp = SpinnerBuilder::new()
    ///     .variant("dots9")
    ///     .message("Loading things into memory...")
    ///     .stream(Stream::Stderr)
    ///     .timer()
    ///     .build();
    /// ```
    ///
    /// No Message:
    ///
    /// ```
    /// use atomic_spinners::SpinnerBuilder;
    ///
    /// let sp = SpinnerBuilder::new().build();
    /// ```
    pub fn new() -> Self {
        Self {
            spinner: variants::Dots.into(),
            ..Default::default()
        }
    }

    /// Specify a spinner variant
    ///
    /// By enabling a specific spinner variant as a feature in your cargo.toml file,
    /// you can import the variant from `variants` and use it as your spinner.
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_spinners::{SpinnerBuilder, variants::Dots9};
    ///
    /// let sp = SpinnerBuilder::new()
    ///     .variant(Dots9)
    ///     .build();
    /// ```
    pub fn variant(
        &mut self,
        variant: impl Into<variants::SpinnerVariant>,
    ) -> &mut Self {
        self.spinner = variant.into();
        self
    }

    /// Specify a message to be displayed along with the spinner
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_spinners::SpinnerBuilder;
    ///
    /// let sp = SpinnerBuilder::new()
    ///     .message("Loading things into memory...")
    ///     .build();
    /// ```
    pub fn message(&mut self, message: impl Into<String>) -> &mut Self {
        self.message = message.into();
        self
    }

    /// Log the time since a spinner was built
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_spinners::SpinnerBuilder;
    ///
    /// let sp = SpinnerBuilder::new()
    ///     .message("Loading things into memory...")
    ///     .timer()
    ///     .build();
    /// ```
    pub fn timer(&mut self) -> &mut Self {
        self.start_time = Some(Instant::now());
        self
    }

    /// Specify an output stream
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_spinners::{SpinnerBuilder, Stream};
    ///
    /// let sp = SpinnerBuilder::new()
    ///     .stream(Stream::Stderr)
    ///     .build();
    /// ```
    pub fn stream(&mut self, stream: Stream) -> &mut Self {
        self.stream = Some(stream);
        self
    }

    /// Build a spinner
    pub fn build(&self) -> Spinner {
        Spinner::new(
            self.spinner.clone(),
            self.message.clone(),
            self.start_time,
            self.stream,
        )
    }
}

impl Spinner {
    pub fn new(
        spinner: impl Into<variants::SpinnerVariant>,
        message: impl Into<String>,
        start_time: Option<Instant>,
        stream: Option<Stream>,
    ) -> Self {
        let spinner: variants::SpinnerVariant = spinner.into();
        let stream = stream.unwrap_or_default();

        let (sender, recv) = channel::<(Instant, Option<String>)>();

        let message = message.into();
        let join = thread::spawn(move || 'outer: loop {
            for frame in spinner.frames.iter() {
                let (do_stop, stop_time, stop_symbol) = match recv.try_recv() {
                    Ok((stop_time, stop_symbol)) => {
                        (true, Some(stop_time), stop_symbol)
                    }
                    Err(TryRecvError::Disconnected) => (true, None, None),
                    Err(TryRecvError::Empty) => (false, None, None),
                };

                let frame = stop_symbol.unwrap_or_else(|| frame.to_string());

                stream
                    .write(&frame, &message, start_time, stop_time)
                    .expect("IO Error");

                if do_stop {
                    break 'outer;
                }

                thread::sleep(Duration::from_millis(spinner.interval as u64));
            }
        });

        Self {
            sender,
            join: Some(join),
            stream,
        }
    }

    /// Stops the spinner
    ///
    /// Stops the spinner that was created with the [`Spinner::new`] function.
    ///
    /// Optionally call [`stop_with_newline`] to print a newline after the spinner is stopped,
    /// or the [`stop_with_message`] function to print a message after the spinner is stopped.
    ///
    /// [`Spinner::new`]: struct.Spinner.html#method.new
    /// [`stop_with_newline`]: struct.Spinner.html#method.stop_with_newline
    /// [`stop_with_message`]: struct.Spinner.html#method.stop_with_message
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use atomic_spinners::SpinnerBuilder;
    ///
    /// let mut sp = SpinnerBuilder::new().build();
    ///
    /// sp.stop();
    /// ```
    pub fn stop(&mut self) {
        self.stop_inner(Instant::now(), None);
    }

    /// Stop with a symbol that replaces the spinner
    ///
    /// The symbol is a String rather than a Char to allow for more flexibility, such as using ANSI color codes.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use atomic_spinners::SpinnerBuilder;
    ///
    /// let mut sp = SpinnerBuilder::new().build();
    ///
    /// sp.stop_with_symbol("🗸");
    /// ```
    ///
    /// ANSI colors (green check mark):
    ///
    /// ```
    /// use atomic_spinners::SpinnerBuilder;
    ///
    /// let mut sp = SpinnerBuilder::new().build();
    ///
    /// sp.stop_with_symbol("\x1b[32m🗸\x1b[0m");
    /// ```
    pub fn stop_with_symbol(&mut self, symbol: &str) {
        self.stop_inner(Instant::now(), Some(symbol.to_owned()));
        self.stream.stop(None, Some(symbol)).expect("IO error");
    }

    /// Stops the spinner and prints a new line
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_spinners::SpinnerBuilder;
    ///
    /// let mut sp = SpinnerBuilder::new().build();
    ///
    /// sp.stop_with_newline();
    /// ```
    pub fn stop_with_newline(&mut self) {
        self.stop();
        self.stream.stop(None, None).expect("IO error");
    }

    /// Stops the spinner and prints the provided message
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_spinners::SpinnerBuilder;
    ///
    /// let mut sp = SpinnerBuilder::new().build();
    ///
    /// sp.stop_with_message("Finished loading things into memory!".into());
    /// ```
    pub fn stop_with_message(&mut self, msg: String) {
        self.stop();
        self.stream.stop(Some(&msg), None).expect("IO Error");
    }

    /// Stops the spinner with a provided symbol and message
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_spinners::SpinnerBuilder;
    ///
    /// let mut sp = SpinnerBuilder::new().build();
    ///
    /// sp.stop_and_persist("✔", "Finished loading things into memory!".into());
    /// ```
    pub fn stop_and_persist(&mut self, symbol: &str, msg: String) {
        self.stop();
        self.stream
            .stop(Some(&msg), Some(symbol))
            .expect("IO Error");
    }

    fn stop_inner(&mut self, stop_time: Instant, stop_symbol: Option<String>) {
        self.sender
            .send((stop_time, stop_symbol))
            .expect("Could not stop spinner thread.");
        self.join.take().unwrap().join().unwrap();
    }
}
