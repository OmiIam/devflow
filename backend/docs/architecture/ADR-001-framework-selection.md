# ADR 001: Framework Selection

## Status
Accepted

## Context
We need to select a web framework for the Rust backend API of DevFlow. The choice will significantly impact development speed, performance, and maintainability. Key considerations are type safety, asynchronous performance, middleware support, and community ecosystem.

## Decision
We will use **Axum (version 0.7)**, built on top of Tokio and Tower.

## Rationale
- **Type-Safe & Ergonomic:** Axum's use of extractors provides a highly ergonomic and type-safe way to handle requests. It allows developers to define handlers as simple functions that take request components (like `Json`, `Path`, `State`) as arguments, with validation handled automatically. This catches many potential bugs at compile time.
- **Performance:** Built on Hyper, Tokio, and Tower, Axum is one of the fastest web frameworks available in any language. Its asynchronous, non-blocking architecture is ideal for I/O-bound applications like a web API.
- **Middleware Ecosystem:** Axum leverages the powerful `tower` and `tower-http` ecosystem for middleware. This provides a rich set of pre-built, battle-tested components for common needs like CORS, tracing, compression, rate limiting, and timeouts.
- **Community & Stability:** Axum is developed by the Tokio team, ensuring high-quality code and long-term maintenance. It has a rapidly growing community and is considered production-ready, famously used by companies like Discord for their backend services.
- **Learning Curve:** While async Rust has a learning curve, Axum's design is relatively straightforward for those familiar with modern web frameworks. Its clear error messages and strong typing help guide new Rust developers.

## Consequences
- **Initial Learning:** The team will need to be comfortable with Rust's ownership model, lifetimes, and async/await syntax.
- **Compile Times:** As a compiled language, Rust will have longer build times compared to interpreted languages like Python or Node.js. However, this is a trade-off for the immense benefits of compile-time checks and runtime performance.
- **Ecosystem Maturity:** While the Rust web ecosystem is mature, it may not have a library for every conceivable niche, unlike more established ecosystems. This is a minor risk, as the core components for this project are well-supported.
