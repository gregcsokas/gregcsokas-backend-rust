# Rust Backend for gregcsokas.hu
Gregcsokas Backend in Rust

This is a modular Rust backend project leveraging **Axum** for routing and **Tokio** for async runtime support. The project is structured as a Rust workspace for increased modularity and scalability.

---

## Features

- **Axum-based routing**: Simple and clean hierarchy of routes.
- Modular project structure with reusable components.
- Basic `newsletter` scaffold endpoint added as an example:
    - `GET /newsletter` - Displays a welcome message for the newsletter service.
    - `POST /newsletter/subscribe` - Endpoint for handling newsletter subscriptions (future implementation).
- Ready for future extensions (e.g., user management, admin panels, analytics, etc.).

---

## Project Structure

```text
.
├── Cargo.toml                # Workspace configuration
├── README.md
├── main/                     # Main application
│   ├── Cargo.toml            # Main app dependencies
│   └── src/
│       └── main.rs           # Application entry point (Axum server)
├── newsletter/               # Newsletter module
│   ├── Cargo.toml            # Newsletter-specific dependencies
│   └── src/
│       └── lib.rs            # Newsletter routing and logic
└── src/
    └── main.rs               # (Optional, currently unused in this project)
```

- **`main/`**: The primary entry point of the application. It starts the Axum server and integrates the `newsletter` routes under the path `/newsletter`.
- **`newsletter/`**: A reusable Rust module for newsletter-related functionality, containing endpoints and logic for subscription management.

```text
URL structure:
  - GET  /newsletter
  - POST /newsletter/subscribe
```

---

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your_username/gregcsokas-backend.git && cd gregcsokas-backend
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the project:
   ```bash
   cargo run -p main
   ```

The server will start at `http://127.0.0.1:8080`.

---

## Usage

### Available Routes

| HTTP Method | Endpoint                  | Description                          |
|-------------|---------------------------|--------------------------------------|
| `GET`       | `/`                       | Root endpoint – returns a welcome message. |
| `GET`       | `/newsletter`             | Displays a basic newsletter homepage message. |
| `POST`      | `/newsletter/subscribe`   | Endpoint for subscribing to the newsletter (currently a placeholder). |

---

## Testing

Unit and integration tests can be found in the corresponding modules (e.g., `newsletter/src/lib.rs`). To run the tests:
```bash
cargo test -p newsletter
```

---

## Future Development

- [ ] Implement subscription logic with data validation.
- [ ] Add database integration for storing user subscriptions.
- [ ] Authentication and middleware for secured endpoints.
- [ ] Extend the modular structure for other features (e.g., user management, admin panels).

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
## 