# Project README

This project is a Rust-based application that demonstrates some simple usages 
of PyO3.

## Prerequisites

To run this project locally, ensure you have the following installed:

- **Rust**: Install from [rust-lang.org](https://www.rust-lang.org/).
- **Python**: Python 3.x from [python.org](https://www.python.org/).
- **Virtualenv**: For creating an isolated Python environment.

## Setup Instructions

1. **Clone the Repository**

   ```bash
   git clone git@github.com:jameynakama/pyo3-examples.git
   cd pyo3-examples
   ```

2. **Set Up Python Virtual Environment**

   Create and activate a virtual environment to manage dependencies:

   ```bash
   python3 -m venv venv
   source venv/bin/activate  # On Windows use `venv\Scripts\activate`
   ```

3. **Install Python Dependencies**

   With the virtual environment activated, install the required Python
   packages listed in `requirements.txt`:

   ```bash
   pip install -r requirements.txt
   ```

4. **Set Environment Variables**

   Create a `.env` file in the project root directory. You can use the example
   provided in the `.env.example` file as a reference to specify the path to
   your Python site-packages. Update the path as necessary:

   ```
   PYTHON_SITE_PACKAGES=/path/to/your/venv/site-packages
   ```

5. **Build the Rust Project**

   Make sure Rust and Cargo are installed. Build the project using Cargo:

   ```bash
   cargo build
   ```

6. **Run the Application**

   Finally, run the application using Cargo:

   ```bash
   cargo run
   ```

## Features

- Calls Python functions from Rust using PyO3.
- Loads Python packages from a virtual environment.
