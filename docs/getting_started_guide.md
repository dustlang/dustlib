# Getting Started with Dust Programming Language (DPL) and `dustlib`

Welcome to the Dust Programming Language (DPL)! This guide will help you set up your environment, understand the basic project structure, and start writing your first DPL program using the core standard library (`dustlib`).

## Prerequisites

Before you begin, ensure you have the following tools installed:

*   **Dust Compiler (`dustc`)**: The official compiler for DPL. You can download it from the [Dust release page](https://github.com/dustlang/dust/releases) (hypothetical link) or build it from source (`github.com/dustlang/dust.git`).
*   **Dust Package Manager (`dustpkg`)**: Used for managing dependencies and building projects. This usually comes with the compiler installation.
*   **A Text Editor or IDE**: While specific DPL plugins might not exist yet, a good text editor will suffice. Syntax highlighting for `.ds` files might be available or manually configured based on other similar languages if applicable.

## Setting Up Your First Project

1.  **Create a New Directory**: Create a new directory for your project, e.g., `my_dust_project`.
    ```bash
    mkdir my_dust_project
    cd my_dust_project
    ```

2.  **Initialize the Project**: Currently, DPL project initialization might require manual setup of the directory structure or a command like `dustpkg init`. For now, let's assume manual setup following the standard DPL structure using `sector`s.
    ```
    my_dust_project/
    ├── State.toml      # Project configuration
    ├── sector/         # Directory for DPL code sectors
    │   └── my_program/ # Your main application sector
    │       ├── State.toml
    │       └── src/
    │           └── main.ds # Entry point
    └── ...
    ```

3.  **Create `State.toml`**: In the root `my_dust_project` directory, create the `State.toml` file to define your workspace. This file configures your project and its members (sectors).
    ```toml
    # my_dust_project/State.toml
    [workspace]
    members = [
        "sector/my_program",
        # Add other sectors here if needed
    ]

    [workspace.package]
    version = "0.1.0"
    edition = "2026" # Use the latest stable DPL edition
    ```

4.  **Create Sector Configuration**: Inside `sector/my_program/`, create its `State.toml` file. This configures the specific sector (your application).
    ```toml
    # my_dust_project/sector/my_program/State.toml
    [package]
    name = "my_program"
    version = "0.1.0"
    edition = "2026"

    # Define the main executable
    [[bin]]
    name = "my_program"
    path = "src/main.ds"

    # Declare dependencies
    [dependencies]
    # `dustlib_core` is the standard core library, implicitly linked or available.
    # Other libraries from dustlib (e.g., collections, io) or external ones would go here.
    # dustlib_collections = { path = "../../path/to/collections_if_local" } # Example
    # dustlib_io = { path = "../../path/to/io_if_local" } # Example
    ```

5.  **Write Your Code**: Create the main source file `sector/my_program/src/main.ds`. This is where your program starts executing.
    ```dust
    // sector/my_program/src/main.ds

    // The main function is the entry point for binary sectors (executables).
    // The 'K' annotation indicates this runs in the K-regime (Classical Computation).
    K fn main() {
        // Use items from the core library (dustlib_core), which is usually prelude-imported.
        // If not, you might need to use `use sector::dustlib_core::*;` or specific items.
        let greeting: str = "Hello, Dust!";
        println!("{}", greeting); // `println!` is a macro from the core library

        let number: i32 = 42;
        let maybe_number: Option<i32> = Option::Some(number);
        match maybe_number {
            Some(n) => println!("The number is: {}", n),
            None => println!("No number found."),
        }

        // Example of Result usage
        let calculation: Result<i32, &str> = perform_calculation();
        match calculation {
            Ok(result) => println!("Calculation result: {}", result),
            Err(e) => println!("Calculation failed: {}", e),
        }
    }

    K fn perform_calculation() -> Result<i32, &str> {
        // Simulate a successful calculation
        Ok(100)
        // To simulate an error: Err("Something went wrong")
    }
    ```

## Building and Running Your Project

1.  **Build the Project**: Navigate to your project root (`my_dust_project`) and run the build command using `dustpkg`.
    ```bash
    dustpkg build
    ```
    This command compiles all sectors defined in your workspace.

2.  **Run the Executable**: After a successful build, run your program.
    ```bash
    dustpkg run
    ```
    You should see the output from your `println!` statements.

## Understanding `dustlib`

`dustlib` is the standard library for DPL, designed to be the foundation for all DPL programs. It's structured into several sectors:

*   **`dustlib_core`**: Located at `sector/dustlib_core`, this contains the absolute essentials: basic types (`i32`, `str`, `bool`), core enums (`Result`, `Option`), fundamental traits (`Debug`, `Display`, `Copy`), and essential macros (`println!`, `panic!`, `assert!`). This is implicitly available or easily imported.
*   **`dustlib_collections`**: Located at `sector/dustlib_collections`, this provides common data structures like `Vec` (dynamic array) and `HashMap`. You will need to explicitly depend on this sector in your `State.toml` if you wish to use its types.
*   **`dustlib_io`**: Located at `sector/dustlib_io`, this offers types and functions for input and output operations, primarily intended for the K-regime. Like collections, you need to add it as a dependency.

Items from `dustlib_core` are often automatically brought into scope via a prelude (similar to Rust's `std::prelude`), meaning you can use `Option`, `Result`, `println!`, etc., without explicit `use` statements. For items in `dustlib_collections` or `dustlib_io`, you must declare the dependency in `State.toml` and then use `use` statements in your code to import them.

## Next Steps

*   Explore the API documentation for `dustlib_core`, `dustlib_collections`, and `dustlib_io` once generated (e.g., using `dustdoc`).
*   Look into the K-regime specifics if you are interested in systems programming aspects, as this is the primary focus of DPL v0.2.
*   Refer to the main DPL specification (`github.com/dustlang/dustlang.git`) for detailed language semantics.
*   Check the `dustlib` repository (`github.com/dustlang/dustlib.git`) for examples and more complex usage patterns.