## Prerequisites

1. Install Rust and Cargo:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Verify the installation:
   ```bash
   rustc --version
   cargo --version
   ```

## Running the Application

1. Clone the repository:
   ```bash
   git clone [repository-url]
   cd data-table-app
   ```

2. Build and run the application:
   ```bash
   cargo run
   ```

3. The application will start on `http://127.0.0.1:8080`

4. Log in with the following credentials:
   - Username: `admin`
   - Password: `password`


# Data Table Application

A Rust web application built with Rocket that displays a dynamic data table with nested rows and authentication.

## Features

- User authentication (login/logout)
- Dynamic data table with 1000 rows
- Nested child rows for each main row
- REST API endpoint for retrieving data
- Responsive table layout with CSS styling

## Technical Stack

- Rust
- Rocket web framework
- Handlebars templating
- Serde for serialization
- Random data generation

## Project Structure

- `src/main.rs` - Main application logic and routes
- `src/models.rs` - Data models and structs
- `templates/` - Handlebars template files
  - `login.html.hbs` - Login page template
  - `table.html.hbs` - Data table template

## Running the Application

1. Make sure you have Rust and Cargo installed
2. Clone the repository
3. Run `cargo run` in the project directory
4. Access the application at `http://127.0.0.1:8080`

## Authentication

Default credentials:
- Username: admin
- Password: password

## API Endpoints

- `GET /` - Main page (requires authentication)
- `POST /login` - Login endpoint
- `GET /logout` - Logout endpoint
- `GET /api/data` - REST API endpoint for raw data

## Data Structure

The application generates random data with the following structure:
