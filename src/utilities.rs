//! Provide utilities for the rest of the crate, such as mathematical functions and enhanced functionality.
pub mod maths;

/// Generate detailed output if compiled with `feature = uPPPP_debug`
///
/// Macro leads to debug possibility without overhead when disabled.
/// Same functionality as println!
#[macro_export]
macro_rules! print_debug{
    ($($arg:tt)*) =>{
        //#[cfg(feature= "uPPPP_debug",)]
        //print!("Debug: ");
        #[cfg(feature= "uPPPP_debug",)]
        println!($($arg)*)
    }
}

/// Generate detailed Warnings if compiled with `feature = uPPPP_warning`
///
/// Macro leads to debug possibility without overhead when disabled.
/// Same functionality as println!
#[macro_export]
macro_rules! print_warning{
    ($($arg:tt)*) =>{
        #[cfg(feature= "uPPPP_warning",)]
        print!("WARNING");
        #[cfg(feature= "uPPPP_warning",)]
        println!($($arg)*);
    }
}

/// Check kill- signals set by ctrl-c in the command line
#[macro_export]
macro_rules! check_signals {
    () => {
        #[cfg(feature = "python")]
        unsafe {
            let sig = pyo3::ffi::PyErr_CheckSignals();
            if sig == -1 {
                panic!("KeyboardInterrupt: Rust caught a ctrl-c signal and exits!")
            }
        }
    };
}

#[macro_export]
macro_rules! setup_bar {
    ($name:expr,$len:expr) => {{
        // import types to satisfy the compiler
        use indicatif::{ProgressBar, ProgressStyle, ProgressState};
        use std::fmt::Write;
        let bar = ProgressBar::new($len as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "{}{}{}",
                    "{spinner:.green} ",
                    $name,
                    " [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% {per_sec} ({eta})"
                )).unwrap()
                .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                    write!(w, "Time left: {:.1}s", state.eta().as_secs_f64()).unwrap()
                })
                .with_key("per_sec", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1} steps/s", state.per_sec()).unwrap())
                .progress_chars("#>-"),
        );
        bar
    }};
}

