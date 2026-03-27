//! Debugging utilities for rendering values as human-readable strings.
//!
//! This module provides the [`Dumpable`] trait, a lightweight debugging interface for types that
//! can produce a string representation of themselves. Unlike [`std::fmt::Debug`], `Dumpable` is
//! designed for interactive debugging workflows: printing to the console, pausing execution to
//! inspect state, or writing snapshots to files.
//!
//! # Example
//!
//! ```rust,no_run
//! use zhc_utils::Dumpable;
//!
//! struct Point { x: i32, y: i32 }
//!
//! impl Dumpable for Point {
//!     fn dump_to_string(&self) -> String {
//!         format!("({}, {})", self.x, self.y)
//!     }
//! }
//!
//! let p = Point { x: 3, y: 4 };
//! p.dump();                         // prints "(3, 4)"
//! p.dump_to_file("point.txt");      // writes "(3, 4)" to file
//!
//! let points = [Point { x: 0, y: 0 }, Point { x: 1, y: 1 }];
//! points.dump();                    // prints "[(0, 0), (1, 1)]"
//! ```

use std::{collections::VecDeque, path::Path};

/// A type that can render itself as a human-readable string for debugging.
///
/// Implement [`dump_to_string`](Dumpable::dump_to_string) to define the string representation;
/// all other methods have default implementations that build on it. The trait is intentionally
/// minimal — it provides just enough functionality for quick inspection during development without
/// the ceremony of formatter arguments or write targets.
///
/// A blanket implementation is provided for slices `[E]` where `E: Dumpable`, rendering elements
/// in bracketed, comma-separated form.
pub trait Dumpable {
    /// Produces the string representation of this value.
    ///
    /// This is the only required method. Implementations should return a complete, self-contained
    /// string suitable for printing or writing to a file. The format is entirely up to the
    /// implementor — there are no constraints on length, structure, or style.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_utils::Dumpable;
    /// # struct Registers { pc: u64, sp: u64 }
    /// # impl Dumpable for Registers {
    /// #     fn dump_to_string(&self) -> String {
    /// #         format!("pc=0x{:016x} sp=0x{:016x}", self.pc, self.sp)
    /// #     }
    /// # }
    /// let regs = Registers { pc: 0x1000, sp: 0xFFFF };
    /// assert_eq!(regs.dump_to_string(), "pc=0x0000000000001000 sp=0x000000000000ffff");
    /// ```
    fn dump_to_string(&self) -> String;

    /// Prints the string representation to standard output.
    ///
    /// Equivalent to `println!("{}", self.dump_to_string())`. Use this for quick inspection when
    /// you don't need to capture the output or control the destination.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_utils::Dumpable;
    /// # struct Config { debug: bool }
    /// # impl Dumpable for Config { fn dump_to_string(&self) -> String { format!("debug={}", self.debug) } }
    /// let cfg = Config { debug: true };
    /// cfg.dump();  // prints "debug=true" followed by a newline
    /// ```
    fn dump(&self) {
        println!("{}", self.dump_to_string());
    }

    /// Prints the string representation and pauses until the user presses Enter.
    ///
    /// This method is useful for breakpoint-style debugging: dump the current state, then wait
    /// for manual confirmation before continuing. The prompt `"Hit enter to resume execution >>>"
    /// is displayed after the dump output.
    ///
    /// # Panics
    ///
    /// Panics if reading from standard input fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_utils::Dumpable;
    /// # struct State { step: u32 }
    /// # impl Dumpable for State { fn dump_to_string(&self) -> String { format!("step {}", self.step) } }
    /// let state = State { step: 42 };
    /// state.dump_and_wait();  // prints state, waits for Enter, then continues
    /// ```
    fn dump_and_wait(&self) {
        self.dump();
        println!("Hit enter to resume execution >>>");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }

    /// Prints the string representation and immediately panics.
    ///
    /// Use this as a "fail-fast" debugging tool: dump the problematic state, then abort execution.
    /// The panic message is empty — all diagnostic information comes from the dump output that
    /// precedes it.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_utils::Dumpable;
    /// # struct Invariant { broken: bool }
    /// # impl Dumpable for Invariant { fn dump_to_string(&self) -> String { format!("broken={}", self.broken) } }
    /// # fn check() -> bool { false }
    /// let inv = Invariant { broken: true };
    /// if !check() {
    ///     inv.dump_and_panic();  // prints state, then panics
    /// }
    /// ```
    fn dump_and_panic(&self) -> ! {
        self.dump();
        panic!();
    }

    /// Writes the string representation to a file.
    ///
    /// Creates the file if it does not exist, or truncates it if it does. The `path` argument
    /// accepts anything convertible to a [`Path`] reference — strings, `PathBuf`, etc.
    ///
    /// # Panics
    ///
    /// Panics if the file cannot be written (e.g., invalid path, permission denied).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_utils::Dumpable;
    /// # struct Snapshot { data: Vec<u8> }
    /// # impl Dumpable for Snapshot { fn dump_to_string(&self) -> String { format!("{:?}", self.data) } }
    /// let snap = Snapshot { data: vec![1, 2, 3] };
    /// snap.dump_to_file("snapshot.txt");
    /// snap.dump_to_file(std::path::PathBuf::from("/tmp/debug.txt"));
    /// ```
    fn dump_to_file<P: AsRef<Path>>(&self, path: P) {
        std::fs::write(path, self.dump_to_string()).expect("Failed to write to file");
    }
}

impl<E: Dumpable> Dumpable for [E] {
    fn dump_to_string(&self) -> String {
        let elements: Vec<String> = self.iter().map(|e| e.dump_to_string()).collect();
        format!("[{}]", elements.join(", "))
    }
}

impl<E: Dumpable> Dumpable for VecDeque<E> {
    fn dump_to_string(&self) -> String {
        let elements: Vec<String> = self.iter().map(|e| e.dump_to_string()).collect();
        format!("[{}]", elements.join(", "))
    }
}

impl<A: Dumpable, B: Dumpable> Dumpable for (A, B) {
    fn dump_to_string(&self) -> String {
        format!("({}, {})", self.0.dump_to_string(), self.1.dump_to_string())
    }
}

macro_rules! impl_dumpable_via_display {
    ($($t:ty),* $(,)?) => {
        $(
            impl $crate::Dumpable for $t {
                fn dump_to_string(&self) -> String {
                    format!("{}", self)
                }
            }
        )*
    };
}

macro_rules! impl_dumpable_via_debug {
    ($($t:ty),* $(,)?) => {
        $(
            impl $crate::Dumpable for $t {
                fn dump_to_string(&self) -> String {
                    format!("{:?}", self)
                }
            }
        )*
    };
}

impl_dumpable_via_debug!(());

impl_dumpable_via_display!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    std::backtrace::Backtrace
);
