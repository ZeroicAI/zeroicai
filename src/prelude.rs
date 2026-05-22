//! Unified prelude for the zeroicai framework.
//!
//! Import everything you need with a single use statement:
//!
//! ```rust
//! use zeroicai::prelude::*;
//! ```

// Core primitives
pub use crate::agent_core::prelude::*;

// Messaging
pub use crate::messaging::prelude::*;

// Cognition (BDI)
pub use crate::cognition::prelude::*;

// Organizational patterns
pub use crate::patterns::prelude::*;

// Runtime (just use the prelude to avoid conflicts)
pub use crate::runtime::prelude::*;
