//! Core types and traits for spatial narratives.
//!
//! This module provides the fundamental building blocks:
//! - [`Location`] - Geographic coordinates (WGS84)
//! - [`Timestamp`] - Temporal information with precision
//! - [`Event`] - Something that happened at a place and time
//! - [`Narrative`] - A collection of related events
//! - [`SourceRef`] - Reference to source material

mod location;
mod timestamp;
mod event;
mod narrative;
mod source;
mod bounds;
mod traits;

pub use location::{Location, LocationBuilder};
pub use timestamp::{Timestamp, TemporalPrecision};
pub use event::{Event, EventBuilder, EventId};
pub use narrative::{Narrative, NarrativeBuilder, NarrativeId, NarrativeMetadata};
pub use source::{SourceRef, SourceType};
pub use bounds::{GeoBounds, TimeRange};
pub use traits::{SpatialEntity, TemporalEntity};
