//! Stellar event indexer.
//!
//! Pipeline:
//!   Stellar RPC (getEvents)
//!     → listener  (poll / cursor management)
//!     → decoder   (XDR → typed SettleEvent)
//!     → processor (SettleEvent → DB projections)

pub mod decoder;
pub mod listener;
pub mod processor;
