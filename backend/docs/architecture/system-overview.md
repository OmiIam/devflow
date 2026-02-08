# DevFlow System Overview

## 1. Architecture Pattern
DevFlow employs a classic **Layered (or N-Tier) Architecture**. This pattern promotes separation of concerns, making the application easier to develop, test, and maintain.

The layers are organized as follows:
1.  **HTTP Layer (Routes & Handlers):** The outermost layer, responsible for receiving HTTP requests and sending responses. It handles request parsing, validation, and serialization. No business logic resides here.
    -   *Components:* `axum::Router`, `routes`, `handlers`, `dto` (Data Transfer Objects).
2.  **Business Logic Layer (Services):** This is the core of the application. It contains the business rules, orchestrates data access, and performs the primary logic for each use case. It is completely decoupled from the HTTP layer.
    -   *Components:* `services`.
3.  **Data Access Layer (Repositories):** This layer abstracts the database interactions. It provides a clean, typed API for querying and manipulating data, hiding the underlying SQL and database-specific details.
    -   *Components:* `repositories`, `sqlx`.
4.  **Database Layer (PostgreSQL):** The physical database that persists the application's state.

## 2. Request Flow
A typical request flows through the system in a clear, unidirectional path:

`Client` → `HTTP Request` → `Axum Router` → `Middleware (Auth, Logging)` → `Handler` → `Service` → `Repository` → `Database`

The response then flows back up the chain:

`Database` → `Repository` → `Service` → `Handler` → `Axum (Serialization)` → `HTTP Response` → `Client`

This strict flow ensures that concerns are properly separated. For example, a `Service` never directly interacts with an HTTP `Request`, and a `Handler` never directly queries the `Database`.

## 3. Data Flow & Models
-   **DTOs (Data Transfer Objects):** Used to shape data for incoming requests (`RegisterRequest`) and outgoing responses (`AuthResponse`). They are the contract between the API and its clients. DTOs often include validation rules.
-   **Models:** These are Rust structs that represent the database entities (e.g., `User`, `Task`). They are used primarily within the `Repository` and `Service` layers. A strict separation is maintained between `Models` (internal representation) and `DTOs` (external representation).

## 4. Security Layers
Security is integrated at multiple levels:
1.  **Transport Layer:** HTTPS will be enforced in production to encrypt all data in transit.
2.  **Network Layer:** CORS (Cross-Origin Resource Sharing) middleware is configured to only allow requests from trusted frontend origins. Rate limiting will be applied to prevent abuse.
3.  **Authentication Layer:** JWT (JSON Web Tokens) are used to secure endpoints. The `auth` middleware validates the token on protected routes, ensuring only authenticated users can proceed.
4.  **Application Layer:**
    -   **Password Hashing:** User passwords are never stored in plaintext. They are hashed using the strong `Argon2` algorithm.
    -   **Input Validation:** All incoming data from clients (via DTOs) is rigorously validated to prevent malformed data and potential injection attacks.
5.  **Data Access Layer:** All database queries are executed using `sqlx`'s parameterized queries, which completely mitigates the risk of SQL injection.

## 5. Concurrency & State
-   The application is fully asynchronous, built on the `Tokio` runtime.
-   Shared state, such as the database connection pool (`PgPool`), is managed using thread-safe smart pointers like `Arc` (Atomic Reference Counting) to ensure safe access across concurrent requests.
