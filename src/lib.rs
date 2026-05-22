//! A comprehensive Rust framework for building intelligent, autonomous multi-agent systems.
#![warn(missing_docs)]
#![warn(clippy::all)]

pub use z_core as agent_core;
pub use z_messaging as messaging;
pub use z_cognition as cognition;
pub use z_patterns as patterns;
pub use z_runtime as runtime;

/// Unified prelude for convenient imports
pub mod prelude;

// Re-export commonly used types at crate root for convenience
pub use agent_core::{Agent, AgentId};
pub use messaging::{Message, Performative, Router, Mailbox};
pub use runtime::{Runtime, RuntimeConfig};