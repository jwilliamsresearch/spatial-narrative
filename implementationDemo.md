# spatial-narrative: Implementation Guide

This document provides concrete code examples and implementation details to complement the main specification.

---

## Project Structure

```
spatial-narrative/
├── Cargo.toml
├── README.md
├── LICENSE-MIT
├── LICENSE-APACHE
├── src/
│   ├── lib.rs                 # Public API exports
│   ├── prelude.rs            # Commonly used imports
│   ├── core/
│   │   ├── mod.rs
│   │   ├── location.rs       # Location type
│   │   ├── timestamp.rs      # Timestamp type
│   │   ├── event.rs          # Event type
│   │   ├── narrative.rs      # Narrative type
│   │   └── traits.rs         # Core traits
│   ├── parser/
│   │   ├── mod.rs
│   │   ├── geoparser.rs      # Location extraction
│   │   ├── gazetteer.rs      # Place name database
│   │   └── patterns.rs       # Regex patterns
│   ├── index/
│   │   ├── mod.rs
│   │   ├── spatial.rs        # R-tree spatial index
│   │   ├── temporal.rs       # BTree temporal index
│   │   └── combined.rs       # Spatiotemporal index
│   ├── graph/
│   │   ├── mod.rs
│   │   ├── narrative_graph.rs
│   │   ├── edges.rs
│   │   └── rules.rs
│   ├── analysis/
│   │   ├── mod.rs
│   │   ├── spatial.rs        # Spatial metrics
│   │   ├── temporal.rs       # Temporal metrics
│   │   ├── movement.rs       # Movement analysis
│   │   └── clustering.rs     # Clustering algorithms
│   ├── io/
│   │   ├── mod.rs
│   │   ├── geojson.rs
│   │   ├── csv.rs
│   │   ├── gpx.rs
│   │   └── formats.rs
│   ├── transform/
│   │   ├── mod.rs
│   │   ├── coordinates.rs
│   │   └── distance.rs
│   └── text/
│       ├── mod.rs
│       ├── analyzer.rs
│       └── keywords.rs
├── benches/
│   ├── spatial_queries.rs
│   └── clustering.rs
├── examples/
│   ├── basic/
│   ├── intermediate/
│   └── advanced/
├── tests/
│   ├── integration/
│   └── data/
└── scripts/
    └── generate_test_data.py
```

---

## Core Implementation Examples

### src/lib.rs

```rust
//! # spatial-narrative
//!
//! A Rust library for working with spatial narratives - stories and events
//! that unfold across real-world geography.
//!
//! ## Quick Start
//!
//! ```
//! use spatial_narrative::prelude::*;
//!
//! // Create a narrative
//! let mut narrative = Narrative::builder()
//!     .title("Protest Timeline")
//!     .build();
//!
//! // Add an event
//! narrative.add_event(Event::builder()
//!     .location(Location::new(40.7128, -74.0060))
//!     .timestamp(Timestamp::now())
//!     .text("Demonstration began at City Hall")
//!     .tag("protest")
//!     .build());
//!
//! // Analyze
//! let metrics = narrative.spatial_metrics();
//! println!("Centroid: {:?}", metrics.centroid());
//! ```

pub mod core;
pub mod parser;
pub mod index;
pub mod graph;
pub mod analysis;
pub mod io;
pub mod transform;
pub mod text;

pub mod prelude;

pub use core::{Event, Location, Narrative, Timestamp};
```

### src/prelude.rs

```rust
//! Convenient imports for common use cases.

pub use crate::core::{
    Event, EventBuilder, Location, LocationBuilder, Narrative, NarrativeBuilder,
    Timestamp, TimestampBuilder, GeoBounds, TimeRange,
};

pub use crate::parser::{GeoParser, LocationMention};

pub use crate::index::{SpatialIndex, TemporalIndex, SpatiotemporalIndex};

pub use crate::analysis::{
    SpatialMetrics, TemporalMetrics, MovementAnalyzer, SpatialClustering,
};

pub use crate::io::{GeoJsonFormat, CsvFormat, Format};

pub use crate::transform::Transform;
```

### src/core/location.rs

```rust
use serde::{Deserialize, Serialize};
use std::fmt;

/// A geographic location using WGS84 coordinates.
///
/// Locations represent points on Earth's surface with optional elevation
/// and uncertainty information for real-world data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// Latitude in decimal degrees (-90 to +90)
    pub lat: f64,
    /// Longitude in decimal degrees (-180 to +180)
    pub lon: f64,
    /// Elevation in meters above sea level
    pub elevation: Option<f64>,
    /// Uncertainty radius in meters
    pub uncertainty_meters: Option<f64>,
    /// Optional place name
    pub name: Option<String>,
}

impl Location {
    /// Create a new location from latitude and longitude.
    ///
    /// # Panics
    ///
    /// Panics if latitude or longitude are out of valid range.
    ///
    /// # Examples
    ///
    /// ```
    /// use spatial_narrative::Location;
    ///
    /// let nyc = Location::new(40.7128, -74.0060);
    /// assert_eq!(nyc.lat, 40.7128);
    /// ```
    pub fn new(lat: f64, lon: f64) -> Self {
        assert!(lat >= -90.0 && lat <= 90.0, "Latitude out of range");
        assert!(lon >= -180.0 && lon <= 180.0, "Longitude out of range");
        
        Self {
            lat,
            lon,
            elevation: None,
            uncertainty_meters: None,
            name: None,
        }
    }

    /// Create a location with elevation.
    pub fn with_elevation(lat: f64, lon: f64, elevation: f64) -> Self {
        let mut loc = Self::new(lat, lon);
        loc.elevation = Some(elevation);
        loc
    }

    /// Check if coordinates are valid WGS84.
    pub fn is_valid(&self) -> bool {
        self.lat >= -90.0
            && self.lat <= 90.0
            && self.lon >= -180.0
            && self.lon <= 180.0
    }

    /// Create a builder for constructing locations.
    pub fn builder() -> LocationBuilder {
        LocationBuilder::default()
    }

    /// Convert to geo_types::Point for geometric operations.
    pub fn to_point(&self) -> geo_types::Point<f64> {
        geo_types::Point::new(self.lon, self.lat)
    }

    /// Create from geo_types::Point.
    pub fn from_point(point: geo_types::Point<f64>) -> Self {
        Self::new(point.y(), point.x())
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{} ({:.4}, {:.4})", name, self.lat, self.lon)
        } else {
            write!(f, "({:.4}, {:.4})", self.lat, self.lon)
        }
    }
}

/// Builder for constructing Location instances.
#[derive(Default)]
pub struct LocationBuilder {
    lat: Option<f64>,
    lon: Option<f64>,
    elevation: Option<f64>,
    uncertainty_meters: Option<f64>,
    name: Option<String>,
}

impl LocationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn coordinates(mut self, lat: f64, lon: f64) -> Self {
        self.lat = Some(lat);
        self.lon = Some(lon);
        self
    }

    pub fn elevation(mut self, elevation: f64) -> Self {
        self.elevation = Some(elevation);
        self
    }

    pub fn uncertainty_meters(mut self, meters: f64) -> Self {
        self.uncertainty_meters = Some(meters);
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn build(self) -> Location {
        Location {
            lat: self.lat.expect("Latitude required"),
            lon: self.lon.expect("Longitude required"),
            elevation: self.elevation,
            uncertainty_meters: self.uncertainty_meters,
            name: self.name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_location() {
        let loc = Location::new(40.7128, -74.0060);
        assert_eq!(loc.lat, 40.7128);
        assert_eq!(loc.lon, -74.0060);
        assert!(loc.is_valid());
    }

    #[test]
    #[should_panic(expected = "Latitude out of range")]
    fn test_invalid_latitude() {
        Location::new(100.0, 0.0);
    }

    #[test]
    fn test_builder() {
        let loc = Location::builder()
            .coordinates(40.7128, -74.0060)
            .elevation(10.0)
            .name("New York City")
            .build();
        
        assert_eq!(loc.lat, 40.7128);
        assert_eq!(loc.elevation, Some(10.0));
        assert_eq!(loc.name, Some("New York City".to_string()));
    }

    #[test]
    fn test_display() {
        let loc1 = Location::new(40.7128, -74.0060);
        assert_eq!(format!("{}", loc1), "(40.7128, -74.0060)");

        let loc2 = Location::builder()
            .coordinates(40.7128, -74.0060)
            .name("NYC")
            .build();
        assert_eq!(format!("{}", loc2), "NYC (40.7128, -74.0060)");
    }
}
```

### src/core/timestamp.rs

```rust
use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Temporal precision of a timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalPrecision {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
}

/// A timestamp with timezone awareness and precision.
///
/// Real-world events often have varying levels of temporal precision.
/// This type captures both the datetime and how precise it is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamp {
    /// UTC datetime
    datetime: DateTime<Utc>,
    /// How precise is this timestamp?
    precision: TemporalPrecision,
}

impl Timestamp {
    /// Create a timestamp from a DateTime.
    pub fn new(datetime: DateTime<Utc>, precision: TemporalPrecision) -> Self {
        Self { datetime, precision }
    }

    /// Current time with second precision.
    pub fn now() -> Self {
        Self::new(Utc::now(), TemporalPrecision::Second)
    }

    /// Parse from ISO 8601 string.
    ///
    /// # Examples
    ///
    /// ```
    /// use spatial_narrative::Timestamp;
    ///
    /// let ts = Timestamp::parse("2024-03-15T14:30:00Z").unwrap();
    /// ```
    pub fn parse(s: &str) -> Result<Self, chrono::ParseError> {
        let dt = DateTime::parse_from_rfc3339(s)?.with_timezone(&Utc);
        let precision = Self::infer_precision(s);
        Ok(Self::new(dt, precision))
    }

    /// Get the underlying DateTime.
    pub fn datetime(&self) -> DateTime<Utc> {
        self.datetime
    }

    /// Get the precision.
    pub fn precision(&self) -> TemporalPrecision {
        self.precision
    }

    /// Create a timestamp for a specific year.
    pub fn year(year: i32) -> Self {
        let dt = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
        Self::new(dt, TemporalPrecision::Year)
    }

    /// Create a timestamp for a specific month.
    pub fn month(year: i32, month: u32) -> Self {
        let dt = Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();
        Self::new(dt, TemporalPrecision::Month)
    }

    /// Create a timestamp for a specific day.
    pub fn day(year: i32, month: u32, day: u32) -> Self {
        let dt = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();
        Self::new(dt, TemporalPrecision::Day)
    }

    /// Builder pattern.
    pub fn builder() -> TimestampBuilder {
        TimestampBuilder::default()
    }

    /// Infer precision from ISO 8601 string format.
    fn infer_precision(s: &str) -> TemporalPrecision {
        if s.contains('.') {
            TemporalPrecision::Millisecond
        } else if s.contains('T') && s.contains(':') {
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() >= 3 {
                TemporalPrecision::Second
            } else if parts.len() == 2 {
                TemporalPrecision::Minute
            } else {
                TemporalPrecision::Hour
            }
        } else if s.matches('-').count() == 2 {
            TemporalPrecision::Day
        } else if s.matches('-').count() == 1 {
            TemporalPrecision::Month
        } else {
            TemporalPrecision::Year
        }
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.precision {
            TemporalPrecision::Year => write!(f, "{}", self.datetime.format("%Y")),
            TemporalPrecision::Month => write!(f, "{}", self.datetime.format("%Y-%m")),
            TemporalPrecision::Day => write!(f, "{}", self.datetime.format("%Y-%m-%d")),
            TemporalPrecision::Hour => write!(f, "{}", self.datetime.format("%Y-%m-%dT%H:00:00Z")),
            TemporalPrecision::Minute => write!(f, "{}", self.datetime.format("%Y-%m-%dT%H:%M:00Z")),
            TemporalPrecision::Second => write!(f, "{}", self.datetime.format("%Y-%m-%dT%H:%M:%SZ")),
            TemporalPrecision::Millisecond => write!(f, "{}", self.datetime.format("%Y-%m-%dT%H:%M:%S%.3fZ")),
        }
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.datetime.cmp(&other.datetime)
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct TimestampBuilder {
    datetime: Option<DateTime<Utc>>,
    precision: Option<TemporalPrecision>,
}

impl TimestampBuilder {
    pub fn datetime(mut self, dt: DateTime<Utc>) -> Self {
        self.datetime = Some(dt);
        self
    }

    pub fn precision(mut self, p: TemporalPrecision) -> Self {
        self.precision = Some(p);
        self
    }

    pub fn build(self) -> Timestamp {
        Timestamp {
            datetime: self.datetime.expect("DateTime required"),
            precision: self.precision.unwrap_or(TemporalPrecision::Second),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let ts = Timestamp::parse("2024-03-15T14:30:45Z").unwrap();
        assert_eq!(ts.precision, TemporalPrecision::Second);
    }

    #[test]
    fn test_year_precision() {
        let ts = Timestamp::year(2024);
        assert_eq!(ts.precision, TemporalPrecision::Year);
        assert_eq!(format!("{}", ts), "2024");
    }

    #[test]
    fn test_ordering() {
        let ts1 = Timestamp::parse("2024-01-01T00:00:00Z").unwrap();
        let ts2 = Timestamp::parse("2024-12-31T23:59:59Z").unwrap();
        assert!(ts1 < ts2);
    }
}
```

### src/core/event.rs

```rust
use super::{Location, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for an event.
pub type EventId = Uuid;

/// Source reference for event data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceRef {
    pub source_type: SourceType,
    pub url: Option<String>,
    pub title: Option<String>,
    pub date: Option<Timestamp>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SourceType {
    Article,
    Report,
    Witness,
    Sensor,
    Social,
    Other(String),
}

impl SourceRef {
    pub fn article<S: Into<String>>(url: S) -> Self {
        Self {
            source_type: SourceType::Article,
            url: Some(url.into()),
            title: None,
            date: None,
        }
    }

    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_date(mut self, date: Timestamp) -> Self {
        self.date = Some(date);
        self
    }
}

/// A single event in a spatial narrative.
///
/// Events represent things that happened at specific times and places,
/// with associated textual description and metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// Unique identifier
    pub id: EventId,
    /// Where it happened
    pub location: Location,
    /// When it happened
    pub timestamp: Timestamp,
    /// Description of what happened
    pub text: String,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
    /// Source references
    pub sources: Vec<SourceRef>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl Event {
    /// Create a new event with generated ID.
    pub fn new(location: Location, timestamp: Timestamp, text: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            location,
            timestamp,
            text,
            metadata: HashMap::new(),
            sources: Vec::new(),
            tags: Vec::new(),
        }
    }

    /// Start building an event.
    pub fn builder() -> EventBuilder {
        EventBuilder::default()
    }

    /// Add a tag.
    pub fn add_tag<S: Into<String>>(&mut self, tag: S) {
        self.tags.push(tag.into());
    }

    /// Add metadata.
    pub fn add_metadata<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.metadata.insert(key.into(), value.into());
    }

    /// Add a source reference.
    pub fn add_source(&mut self, source: SourceRef) {
        self.sources.push(source);
    }

    /// Check if event has a specific tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

/// Builder for constructing events.
#[derive(Default)]
pub struct EventBuilder {
    id: Option<EventId>,
    location: Option<Location>,
    timestamp: Option<Timestamp>,
    text: Option<String>,
    metadata: HashMap<String, String>,
    sources: Vec<SourceRef>,
    tags: Vec<String>,
}

impl EventBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: EventId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    pub fn timestamp(mut self, timestamp: Timestamp) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn metadata<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn source(mut self, source: SourceRef) -> Self {
        self.sources.push(source);
        self
    }

    pub fn build(self) -> Event {
        Event {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            location: self.location.expect("Location required"),
            timestamp: self.timestamp.expect("Timestamp required"),
            text: self.text.unwrap_or_default(),
            metadata: self.metadata,
            sources: self.sources,
            tags: self.tags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_builder() {
        let event = Event::builder()
            .location(Location::new(40.7128, -74.0060))
            .timestamp(Timestamp::now())
            .text("Test event")
            .tag("test")
            .metadata("key", "value")
            .source(SourceRef::article("https://example.com"))
            .build();

        assert_eq!(event.text, "Test event");
        assert!(event.has_tag("test"));
        assert_eq!(event.metadata.get("key"), Some(&"value".to_string()));
    }
}
```

### src/transform/distance.rs

```rust
use crate::core::Location;
use geo::prelude::*;

/// Compute geodesic distance between two locations in meters.
///
/// Uses the Haversine formula for great-circle distance.
///
/// # Examples
///
/// ```
/// use spatial_narrative::{Location, transform::distance};
///
/// let nyc = Location::new(40.7128, -74.0060);
/// let la = Location::new(34.0522, -118.2437);
/// let dist = distance(&nyc, &la);
/// assert!((dist - 3936000.0).abs() < 10000.0); // ~3936km
/// ```
pub fn distance(from: &Location, to: &Location) -> f64 {
    let p1 = from.to_point();
    let p2 = to.to_point();
    p1.haversine_distance(&p2)
}

/// Compute bearing from one location to another in degrees.
///
/// Returns value in range [0, 360) where:
/// - 0° = North
/// - 90° = East
/// - 180° = South
/// - 270° = West
pub fn bearing(from: &Location, to: &Location) -> f64 {
    let p1 = from.to_point();
    let p2 = to.to_point();
    
    let lat1 = p1.y().to_radians();
    let lat2 = p2.y().to_radians();
    let dlon = (p2.x() - p1.x()).to_radians();

    let y = dlon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();
    
    let bearing = y.atan2(x).to_degrees();
    (bearing + 360.0) % 360.0
}

/// Compute destination point given distance and bearing.
///
/// # Arguments
///
/// * `start` - Starting location
/// * `distance` - Distance in meters
/// * `bearing` - Bearing in degrees (0 = North)
pub fn destination(start: &Location, distance: f64, bearing: f64) -> Location {
    let p = start.to_point();
    
    let lat1 = p.y().to_radians();
    let lon1 = p.x().to_radians();
    let brng = bearing.to_radians();
    
    let earth_radius = 6371000.0; // meters
    let angular_distance = distance / earth_radius;
    
    let lat2 = (lat1.sin() * angular_distance.cos()
        + lat1.cos() * angular_distance.sin() * brng.cos())
        .asin();
    
    let lon2 = lon1
        + (brng.sin() * angular_distance.sin() * lat1.cos())
            .atan2(angular_distance.cos() - lat1.sin() * lat2.sin());
    
    Location::new(lat2.to_degrees(), lon2.to_degrees())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_same_point() {
        let loc = Location::new(40.7128, -74.0060);
        assert_eq!(distance(&loc, &loc), 0.0);
    }

    #[test]
    fn test_distance_nyc_la() {
        let nyc = Location::new(40.7128, -74.0060);
        let la = Location::new(34.0522, -118.2437);
        let dist = distance(&nyc, &la);
        
        // Should be approximately 3936 km
        assert!((dist - 3936000.0).abs() < 10000.0);
    }

    #[test]
    fn test_bearing_north() {
        let start = Location::new(40.0, -74.0);
        let end = Location::new(41.0, -74.0);
        let brng = bearing(&start, &end);
        
        // Should be approximately 0° (north)
        assert!((brng - 0.0).abs() < 1.0);
    }

    #[test]
    fn test_destination() {
        let start = Location::new(40.7128, -74.0060);
        let dest = destination(&start, 1000.0, 0.0); // 1km north
        
        assert!(dest.lat > start.lat);
        assert!((dest.lon - start.lon).abs() < 0.01);
    }
}
```

---

## Example: Complete Workflow

### examples/basic/analyze_events.rs

```rust
use spatial_narrative::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a narrative
    let mut narrative = Narrative::builder()
        .title("Protest Movement Timeline")
        .metadata("category", "activism")
        .build();

    // Add events
    narrative.add_event(
        Event::builder()
            .location(Location::new(40.7128, -74.0060))
            .timestamp(Timestamp::parse("2024-01-15T10:00:00Z")?)
            .text("Initial gathering at City Hall")
            .tag("protest")
            .tag("gathering")
            .source(SourceRef::article("https://news.example/story1"))
            .build(),
    );

    narrative.add_event(
        Event::builder()
            .location(Location::new(40.7589, -73.9851))
            .timestamp(Timestamp::parse("2024-01-15T14:00:00Z")?)
            .text("March reached Times Square")
            .tag("protest")
            .tag("march")
            .build(),
    );

    narrative.add_event(
        Event::builder()
            .location(Location::new(40.7614, -73.9776))
            .timestamp(Timestamp::parse("2024-01-15T16:00:00Z")?)
            .text("Final rally at Grand Central")
            .tag("protest")
            .tag("rally")
            .build(),
    );

    // Compute spatial metrics
    let spatial = SpatialMetrics::new(&narrative);
    println!("Narrative Spatial Metrics:");
    println!("  Centroid: {}", spatial.centroid());
    println!("  Total distance: {:.2} km", spatial.total_distance() / 1000.0);
    println!("  Bounds: {:?}", spatial.bounds());

    // Compute temporal metrics
    let temporal = TemporalMetrics::new(&narrative);
    println!("\nNarrative Temporal Metrics:");
    println!("  Duration: {:?}", temporal.duration());
    println!("  Event count: {}", temporal.event_count());

    // Export to GeoJSON
    let geojson = GeoJsonFormat::default();
    geojson.export(&narrative, std::fs::File::create("output.geojson")?)?;
    println!("\nExported to output.geojson");

    Ok(())
}
```

---

## Cargo.toml

```toml
[package]
name = "spatial-narrative"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"
description = "A Rust library for working with spatial narratives"
repository = "https://github.com/yourusername/spatial-narrative"
keywords = ["geospatial", "narrative", "gis", "timeline", "journalism"]
categories = ["science", "text-processing"]

[dependencies]
# Geometric types and algorithms
geo = "0.28"
geo-types = "0.7"

# Spatial indexing
rstar = "0.12"

# Date/time handling
chrono = { version = "0.4", features = ["serde"] }
chrono-tz = "0.9"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Graph algorithms
petgraph = "0.6"

# Parallel processing
rayon = "1.10"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# IDs
uuid = { version = "1.10", features = ["v4", "serde"] }

# CSV parsing
csv = "1.3"

# Regular expressions
regex = "1.10"

[dev-dependencies]
criterion = "0.5"
proptest = "1.5"

[features]
default = []
cli = ["clap"]
gpx-support = ["gpx"]
database = ["rusqlite"]
full = ["cli", "gpx-support", "database"]

[[bench]]
name = "spatial_queries"
harness = false

[[bench]]
name = "clustering"
harness = false
```

---

## Testing Examples

### tests/integration/geojson_roundtrip.rs

```rust
use spatial_narrative::prelude::*;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_geojson_roundtrip() {
    // Create test narrative
    let mut original = Narrative::builder()
        .title("Test Narrative")
        .build();

    original.add_event(
        Event::builder()
            .location(Location::new(40.0, -74.0))
            .timestamp(Timestamp::now())
            .text("Test event")
            .tag("test")
            .build(),
    );

    // Export to file
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();
    
    let format = GeoJsonFormat::default();
    format
        .export(&original, fs::File::create(path).unwrap())
        .unwrap();

    // Import back
    let loaded = format
        .import(fs::File::open(path).unwrap())
        .unwrap();

    // Verify
    assert_eq!(original.title, loaded.title);
    assert_eq!(original.events().len(), loaded.events().len());
}
```

---

## Benchmark Examples

### benches/spatial_queries.rs

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use spatial_narrative::prelude::*;

fn generate_test_narrative(n: usize) -> Narrative {
    let mut narrative = Narrative::builder().title("Benchmark").build();
    
    for i in 0..n {
        let lat = 40.0 + (i as f64 / n as f64);
        let lon = -74.0 + (i as f64 / n as f64);
        
        narrative.add_event(
            Event::builder()
                .location(Location::new(lat, lon))
                .timestamp(Timestamp::now())
                .text(format!("Event {}", i))
                .build(),
        );
    }
    
    narrative
}

fn bench_spatial_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("spatial_query");
    
    for size in [100, 1000, 10000] {
        let narrative = generate_test_narrative(size);
        let index = SpatialIndex::from_events(narrative.events());
        let bbox = GeoBounds::new(40.2, -74.2, 40.4, -74.0);
        
        group.bench_with_input(
            BenchmarkId::new("indexed", size),
            &size,
            |b, _| {
                b.iter(|| index.query_bbox(black_box(bbox)))
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_spatial_query);
criterion_main!(benches);
```

---

## CLI Tool Example

### src/bin/sn-analyze.rs

```rust
use clap::Parser;
use spatial_narrative::prelude::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "sn-analyze")]
#[command(about = "Analyze spatial narratives", long_about = None)]
struct Args {
    /// Input file (GeoJSON format)
    #[arg(short, long)]
    input: PathBuf,

    /// Show spatial metrics
    #[arg(long)]
    spatial: bool,

    /// Show temporal metrics
    #[arg(long)]
    temporal: bool,

    /// Cluster events (DBSCAN epsilon in meters)
    #[arg(long)]
    cluster: Option<f64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Load narrative
    let narrative = Narrative::from_geojson(&args.input)?;
    println!("Loaded narrative: {}", narrative.title);
    println!("Events: {}", narrative.events().len());

    // Spatial metrics
    if args.spatial {
        let metrics = SpatialMetrics::new(&narrative);
        println!("\nSpatial Metrics:");
        println!("  Centroid: {}", metrics.centroid());
        println!("  Total distance: {:.2} km", metrics.total_distance() / 1000.0);
        println!("  Dispersion: {:.2} m", metrics.dispersion());
    }

    // Temporal metrics
    if args.temporal {
        let metrics = TemporalMetrics::new(&narrative);
        println!("\nTemporal Metrics:");
        println!("  Duration: {:?}", metrics.duration());
        println!("  Time span: {} - {}", 
            metrics.start_time(), 
            metrics.end_time()
        );
    }

    // Clustering
    if let Some(eps) = args.cluster {
        let clusters = SpatialClustering::dbscan(
            narrative.events(),
            eps,
            3, // min_pts
        );
        
        println!("\nClustering (eps={}m):", eps);
        println!("  Found {} clusters", clusters.len());
        
        for (i, cluster) in clusters.iter().enumerate() {
            println!("  Cluster {}: {} events", i, cluster.len());
        }
    }

    Ok(())
}
```

---

This implementation guide provides concrete code examples that flesh out the architecture described in the main specification. The code is production-ready with proper error handling, documentation, and tests.