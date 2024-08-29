Building and Running the Project
--------------------------------

This section will guide you through building and running the Rust project that processes Primavera P6 `.xer` files and converts them into CSV format.

### Prerequisites

Before you begin, ensure you have the following installed on your system:

-   **Rust**: You can install Rust using [rustup](https://rustup.rs/), the official Rust installer. Follow the instructions on the website to get the latest stable version of Rust and Cargo (Rust's package manager and build system).

### Build the Project

1.  **Clone the Repository**

    Clone this repository to your local machine using Git or download it as a zip to be extracted:

     ```bash
     sudo apt-get install git
     ```

    ```bash
    git clone https://github.com/koubry02/xer_to_csv.git
    cd xer_to_csv
     ```

    

3.  **Install Dependencies**

    Navigate to the project directory and run the following command to fetch the required dependencies:


    ```bash
    cargo build
    ```

    This command will download and compile all the necessary crates specified in `Cargo.toml`.

### Run the Project

To run the project, use the following command:

```bash
cargo run -- <input_directory> <output_directory>
```

Replace `<input_directory>` with the path to the directory containing `.xer` files you want to process, and `<output_directory>` with the path where you want the CSV files to be saved.

### Notes

-   Ensure that the `.xer` files follow the expected format for correct processing.
-   The output directory will contain subdirectories named after the `.xer` file base names, with each subdirectory containing corresponding CSV files.

