# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is `webex-rust`, an asynchronous Rust library providing a minimal interface to Webex Teams APIs.

## Commands

### Build and Test
- `cargo build` - Build the library
- `cargo test` - Run unit tests  
- `cargo clippy` - Run linter (note: very strict clippy rules enabled)
- `cargo fmt` - Format code
- `cargo doc` - Generate documentation

### Examples
- `cargo run --example hello-world` - Basic message sending example
- `cargo run --example auto-reply` - Bot that automatically replies to messages
- `cargo run --example adaptivecard` - Demonstrates AdaptiveCard usage
- `cargo run --example device-authentication` - Shows device authentication flow
- `cargo run --example list-spaces` - Lists all Webex spaces (rooms) you are a member of

### Development
- `cargo test --lib` - Run library tests only
- `cargo clippy --all-targets --all-features` - Full clippy check
- `cargo build --all-targets` - Build everything including examples

## Architecture

### Core Components

- **`Webex` struct** (`src/lib.rs:93`) - Main API client with token-based authentication
- **`WebexEventStream`** (`src/lib.rs:104`) - WebSocket event stream handler for real-time events
- **`RestClient`** (`src/lib.rs:252`) - Low-level HTTP client wrapper
- **Types module** (`src/types.rs`) - All API data structures and serialization including:
  - `Room` - Enhanced with `is_announcement_only`, `is_read_only`, `is_public`, `description` fields
  - `Membership` - Enhanced with `is_room_hidden` and `room_type` fields
  - `Destination` - Type-safe enum for message routing
  - `MessageOut` - Builder pattern for creating messages
- **AdaptiveCard module** (`src/adaptive_card.rs`) - Support for interactive cards
- **Auth module** (`src/auth.rs`) - Device authentication flows
- **Error module** (`src/error.rs`) - Comprehensive error handling using `thiserror` 2.0 with transparent error forwarding

### Key Patterns

- **Generic API methods**: `get<T>()`, `list<T>()`, `delete<T>()` work with any `Gettable` type
- **Device registration**: Automatic device setup and caching for WebSocket connections
- **Type-safe message routing**: Uses `Destination` enum (RoomId, ToPersonId, ToPersonEmail) for message destinations
- **Builder pattern for messages**: Chainable methods like `MessageOut::to_room(id).text(msg).markdown(md)`
- **Message handling**: Supports both direct messages and room messages with threading
- **Event streaming**: WebSocket-based real-time event processing with automatic reconnection

### Authentication Flow

1. Token-based authentication for REST API calls
2. Device registration with Webex for WebSocket connections  
3. Mercury URL discovery for optimal WebSocket endpoint
4. Automatic device cleanup and recreation as needed

## Important Notes

- Uses Rust 1.91 toolchain with Edition 2024 (see `rust-toolchain.toml`)
- Very strict clippy configuration with pedantic and nursery lints enabled
- All public APIs must have documentation (`#![deny(missing_docs)]`)
- WebSocket connections require device registration and token authentication
- Mercury URL caching reduces API calls for device discovery

## Recent Changes

- **Rust Edition 2024**: Upgraded from Edition 2021 to 2024
- **Error handling**: Upgraded `thiserror` from 1.0 to 2.0.17 with transparent error forwarding for better error chains
- **Dependency updates**: Updated to latest stable versions:
  - `futures` and `futures-util`: 0.3.31
  - `tungstenite`: 0.28.0
  - `tokio-tungstenite`: 0.28.0
  - `serde_with`: 3.15.1
  - `serde_html_form`: 0.2.8
- **Type-safe messaging**: Introduced `Destination` enum replacing separate room_id/to_person_id/to_person_email fields
- **Enhanced API types**: Added missing fields to `Room` and `Membership` structs for complete API coverage
