//! Analytical tools and metrics for spatial narratives.
//!
//! This module provides functions for analyzing narratives,
//! including spatial metrics, temporal metrics, clustering,
//! and movement analysis.
//!
//! # Features
//!
//! - **Spatial Metrics** - Geographic extent, distance, dispersion ([`SpatialMetrics`])
//! - **Temporal Metrics** - Duration, event rate, gaps, bursts ([`TemporalMetrics`])
//! - **Movement** - Trajectory extraction and analysis ([`Trajectory`], [`detect_stops`])
//! - **Clustering** - DBSCAN, k-means clustering ([`DBSCAN`], [`KMeans`])
//! - **Comparison** - Narrative similarity and comparison ([`compare_narratives`])
//!
//! # Examples
//!
//! ## Spatial Metrics
//!
//! ```
//! use spatial_narrative::core::{Event, Location, Timestamp};
//! use spatial_narrative::analysis::SpatialMetrics;
//!
//! let events = vec![
//!     Event::new(Location::new(40.7128, -74.0060), Timestamp::now(), "NYC"),
//!     Event::new(Location::new(34.0522, -118.2437), Timestamp::now(), "LA"),
//! ];
//!
//! let metrics = SpatialMetrics::from_events(&events);
//! println!("Total distance: {} meters", metrics.total_distance);
//! ```
//!
//! ## DBSCAN Clustering
//!
//! ```
//! use spatial_narrative::core::{Event, Location, Timestamp};
//! use spatial_narrative::analysis::DBSCAN;
//!
//! let events = vec![
//!     Event::new(Location::new(40.0, -74.0), Timestamp::now(), "A"),
//!     Event::new(Location::new(40.001, -74.001), Timestamp::now(), "B"),
//!     Event::new(Location::new(50.0, -80.0), Timestamp::now(), "Far"),
//! ];
//!
//! let dbscan = DBSCAN::new(1000.0, 2); // 1km radius, min 2 points
//! let result = dbscan.cluster(&events);
//! println!("Found {} clusters", result.num_clusters());
//! ```

mod clustering;
mod comparison;
mod movement;
mod spatial_metrics;
mod temporal_metrics;

// Re-export main types
pub use clustering::{Cluster, ClusteringResult, KMeans, DBSCAN};
pub use comparison::{
    common_locations, compare_narratives, spatial_intersection, spatial_similarity, spatial_union,
    temporal_similarity, thematic_similarity, ComparisonConfig, NarrativeSimilarity,
};
pub use movement::{detect_stops, MovementAnalyzer, Stop, StopThreshold, Trajectory};
pub use spatial_metrics::{
    bearing, density_map, destination_point, haversine_distance, DensityCell, SpatialMetrics,
};
pub use temporal_metrics::{
    detect_bursts, detect_gaps, event_rate, TemporalMetrics, TimeBin, TimeBinCount,
};
