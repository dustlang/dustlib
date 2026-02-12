// macros/mod.ds: Module declaration and definition of core macros.
// This file contains essential macros provided by the core library, similar to `std::prelude` in Rust.

// Note: The actual syntax and mechanism for defining macros in DPL v0.2
// are not explicitly detailed in the v0.1 specification or the provided plan PDF.
// The following definitions use a hypothetical syntax based on common macro patterns
// and the needs expressed (e.g., panic, assert). The exact implementation details
// will depend on the finalized DPL macro system.

// --- Core Macros ---

// The `panic!` macro causes the current thread to abort with a specified error message.
// It likely takes a format string and arguments, expands to a call that triggers a panic.
// Example usage: panic!("Something went wrong: {}", error_code);
macro_rules! panic {
    ($msg:expr) => {
        // Implementation would involve calling a core runtime function
        // to print the message and terminate execution.
        // This is a placeholder for the actual mechanism.
        // core_panic($msg)
    };
    ($fmt:expr, $($arg:expr),+) => {
        // Implementation would format the string using the fmt system
        // and then trigger the panic.
        // core_panic(format!($fmt, $($arg),+))
    };
}

// The `assert!` macro checks that a condition is true at runtime.
// If the condition is false, it panics.
// Example usage: assert!(x > 0, "x must be positive");
macro_rules! assert {
    ($cond:expr) => {
        if !($cond) {
            panic!("Assertion failed: {}", stringify!($cond));
        }
    };
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            panic!($msg);
        }
    };
    ($cond:expr, $fmt:expr, $($arg:expr),+) => {
        if !($cond) {
            panic!($fmt, $($arg),+);
        }
    };
}

// The `assert_eq!` macro checks that two expressions are equal.
// If they are not, it panics with a message showing the values.
// Example usage: assert_eq!(a, b);
macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        // Obtain values, potentially using a temporary scope to avoid double evaluation
        // if the expressions have side effects (though discouraged in assertions).
        let left_val = $left;
        let right_val = $right;
        if left_val != right_val { // Assumes PartialEq trait is available and implemented for T
             // A more detailed panic message showing both values would be ideal.
             panic!("assertion failed: `(left == right)`\n  left: `{:?}`,\n right: `{:?}`", left_val, right_val);
        }
    };
    ($left:expr, $right:expr, $fmt:expr, $($arg:expr),+) => {
        let left_val = $left;
        let right_val = $right;
        if left_val != right_val {
             panic!($fmt, $($arg),+); // Use custom message if values are not equal
        }
    };
}

// The `println!` macro prints a formatted string to the standard output, followed by a newline.
// Example usage: println!("Hello, {}!", name);
macro_rules! println {
    () => {
        // Print just a newline
        print("\n");
    };
    ($fmt:expr) => {
        // Print the format string literally if no args
        print($fmt);
        print("\n");
    };
    ($fmt:expr, $($arg:expr),+) => {
        // Format the string using the fmt system and print it followed by a newline.
        print(format!($fmt, $($arg),+));
        print("\n"); // Or use a dedicated print_newline function if available
    };
}

// The `format!` macro creates a String (or str?) using interpolation of runtime expressions.
// Example usage: let s = format!("Value is {}", value);
// Note: This likely requires a String type and a working fmt system.
macro_rules! format {
    ($fmt:expr, $($arg:expr),+) => {
        // Implementation would use the formatting system (Formatter, Display/Debug traits)
        // to create a string based on the format string and arguments.
        // This is a placeholder assuming a core formatting function exists.
        // core_format($fmt, $($arg),+)
    };
    // Handle the case with no arguments
    ($fmt:expr) => {
        // Return the format string itself if no arguments are provided.
        $fmt
    };
}

// The `todo!` macro indicates that a section of code is not yet implemented.
// Calling it causes a panic with a "not yet implemented" message.
macro_rules! todo {
    () => {
        panic!("not yet implemented")
    };
    ($msg:expr) => {
        panic!("not yet implemented: {}", $msg)
    };
}

// The `unreachable!` macro indicates code that should not be reachable.
// If executed, it panics.
macro_rules! unreachable {
    () => {
        panic!("internal error: entered unreachable code")
    };
    ($msg:expr) => {
        panic!("internal error: entered unreachable code: {}", $msg)
    };
    ($fmt:expr, $($arg:expr),+) => {
        panic!("internal error: entered unreachable code: {}", format!($fmt, $($arg),+))
    };
}

// Add other essential macros here as needed for the core library.
// e.g., debug_assert!, dbg!, etc.

// End of macros module definition.