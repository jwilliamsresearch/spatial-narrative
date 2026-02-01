# spatial-narrative

<div align="center">

<!-- Animated Logo -->
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/jwilliamsresearch/spatial-narrative/master/assets/logo-dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/jwilliamsresearch/spatial-narrative/master/assets/logo-light.svg">
  <img alt="spatial-narrative logo" src="https://raw.githubusercontent.com/jwilliamsresearch/spatial-narrative/master/assets/logo-light.svg" width="500">
</picture>

<br><br>

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/spatial-narrative.svg?logo=rust)](https://crates.io/crates/spatial-narrative)
[![Downloads](https://img.shields.io/crates/d/spatial-narrative.svg?logo=rust)](https://crates.io/crates/spatial-narrative)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-rustdoc-green.svg?logo=docsdotrs)](https://docs.rs/spatial-narrative)
[![CI](https://img.shields.io/github/actions/workflow/status/jwilliamsresearch/spatial-narrative/ci.yml?branch=master&logo=github)](https://github.com/jwilliamsresearch/spatial-narrative/actions)
[![codecov](https://img.shields.io/codecov/c/github/jwilliamsresearch/spatial-narrative?logo=codecov)](https://codecov.io/gh/jwilliamsresearch/spatial-narrative)

[![GitHub stars](https://img.shields.io/github/stars/jwilliamsresearch/spatial-narrative?style=social)](https://github.com/jwilliamsresearch/spatial-narrative)
[![GitHub forks](https://img.shields.io/github/forks/jwilliamsresearch/spatial-narrative?style=social)](https://github.com/jwilliamsresearch/spatial-narrative/fork)

**Composable building blocks for spatial narratives in Rust.**

[📖 Documentation](https://docs.rs/spatial-narrative) · [🚀 Getting Started](#quick-start) · [📦 Crates.io](https://crates.io/crates/spatial-narrative) · [💬 Discussions](https://github.com/jwilliamsresearch/spatial-narrative/discussions)

</div>

---

## Philosophy

`spatial-narrative` provides **focused, interoperable components** for working with geospatial event data. It's designed to fit into your existing data pipeline, not replace it.

**What it provides:**
- Standard data types for events, locations, timestamps, and narratives
- Spatial indexing (R-tree) and temporal indexing (B-tree)
- Analysis algorithms: DBSCAN clustering, spatial/temporal metrics, trajectory detection
- Format conversion: GeoJSON, CSV, JSON import/export
- Graph structures for event relationships

**What it doesn't try to do:**
- Fetch data from APIs (use `reqwest`)
- Parse domain-specific formats (use your parser + our types)
- Render visualizations (export to GeoJSON, use Leaflet/Mapbox)
- Replace `geo`/`geo-types` (we interop with them)

The library is most valuable when you need **analysis and indexing** on top of your own data ingestion pipeline.

## Ecosystem Integration

`spatial-narrative` is built to compose with the Rust geospatial ecosystem:

```
┌─────────────────────────────────────────────────────────────────┐
│                     Your Application                             │
├─────────────────────────────────────────────────────────────────┤
│  Data Sources          spatial-narrative         Outputs        │
│  ─────────────         ─────────────────         ───────        │
│  reqwest (HTTP)   →    Event, Narrative    →    GeoJSON         │
│  csv (parsing)    →    SpatialIndex        →    Leaflet/Mapbox  │
│  serde_json       →    DBSCAN, Metrics     →    QGIS            │
│  Custom parsers   →    NarrativeGraph      →    CSV/Excel       │
└─────────────────────────────────────────────────────────────────┘
```

**Direct interop:**
- `Location` ↔ `geo_types::Point<f64>` (via `From`/`Into`)
- All types implement `Serialize`/`Deserialize`
- R-tree backed by `rstar`
- Graphs powered by `petgraph`

## Installation

```toml
[dependencies]
spatial-narrative = "0.1"

# Your data pipeline dependencies
reqwest = { version = "0.12", features = ["blocking"] }  # HTTP
csv = "1.3"                                               # CSV parsing
serde_json = "1.0"                                        # JSON
```

## Quick Start

```rust
use spatial_narrative::core::{Event, Location, Timestamp, NarrativeBuilder};
use spatial_narrative::analysis::{DBSCAN, SpatialMetrics};
use spatial_narrative::io::{Format, GeoJsonFormat};

// Your events (from whatever source)
let events = vec![
    Event::new(Location::new(40.7128, -74.0060), Timestamp::now(), "NYC event"),
    Event::new(Location::new(40.7580, -73.9855), Timestamp::now(), "Times Square"),
    Event::new(Location::new(51.5074, -0.1278), Timestamp::now(), "London event"),
];

// Wrap in a narrative
let narrative = NarrativeBuilder::new()
    .title("Global Events")
    .events(events)
    .build();

// Analyze: find geographic clusters
let dbscan = DBSCAN::new(50_000.0, 2);  // 50km radius, min 2 points
let clusters = dbscan.cluster(&narrative.events);
println!("Found {} clusters", clusters.num_clusters());

// Analyze: compute spatial metrics
let metrics = SpatialMetrics::from_events(&narrative.events);
if let Some(centroid) = metrics.centroid {
    println!("Centroid: {:.2}°, {:.2}°", centroid.lat, centroid.lon);
}

// Export: GeoJSON for web visualization
let mut output = Vec::new();
GeoJsonFormat::new().export(&narrative, &mut output)?;
// → Use with Leaflet, Mapbox, QGIS, etc.
```

## Modules

| Module | What it does | Key types |
|--------|--------------|-----------|
| `core` | Data structures | `Event`, `Location`, `Timestamp`, `Narrative`, `GeoBounds`, `TimeRange` |
| `index` | Fast spatial/temporal queries | `SpatialIndex`, `TemporalIndex`, `SpatiotemporalIndex` |
| `analysis` | Algorithms | `DBSCAN`, `KMeans`, `SpatialMetrics`, `TemporalMetrics`, `Trajectory` |
| `graph` | Event relationships | `NarrativeGraph`, `EdgeType` |
| `io` | Format conversion | `GeoJsonFormat`, `CsvFormat`, `JsonFormat` |
| `parser` | Text → locations | `GeoParser`, `Gazetteer`, `BuiltinGazetteer` |

## Core Types

### Event

The fundamental unit — something that happened at a place and time:

```rust
use spatial_narrative::core::{Event, EventBuilder, Location, Timestamp};

// Simple construction
let event = Event::new(
    Location::new(48.8566, 2.3522),
    Timestamp::parse("2024-07-14T10:00:00Z").unwrap(),
    "Bastille Day celebrations"
);

// Builder for richer data
let event = EventBuilder::new()
    .location(Location::new(48.8566, 2.3522))
    .timestamp(Timestamp::parse("2024-07-14T10:00:00Z").unwrap())
    .text("Bastille Day celebrations")
    .tag("celebration")
    .tag("national-holiday")
    .metadata("source", "reuters")
    .metadata("confidence", "0.95")
    .build();
```

### Narrative

A collection of related events:

```rust
use spatial_narrative::core::{NarrativeBuilder, GeoBounds, TimeRange};

let narrative = NarrativeBuilder::new()
    .title("European Summit 2024")
    .author("Research Team")
    .events(events)
    .build();

// Built-in queries
let chronological = narrative.events_chronological();
let bounds = narrative.bounds();        // Geographic extent
let range = narrative.time_range();     // Temporal extent

// Filtering
let paris = narrative.filter_spatial(&GeoBounds::new(48.0, 2.0, 49.0, 3.0));
let january = narrative.filter_temporal(&TimeRange::month(2024, 1));
```

## Indexing

For datasets where you need fast queries (not just iteration):

```rust
use spatial_narrative::index::SpatiotemporalIndex;
use spatial_narrative::core::{Event, GeoBounds};

// Build index from events
let index = SpatiotemporalIndex::from_iter(
    events.clone(),
    |e| &e.location,
    |e| &e.timestamp,
);

// Spatial query: events in a bounding box
let europe = GeoBounds::new(35.0, -10.0, 72.0, 40.0);
let european_events = index.query_spatial(&europe);

// Combined query: space + time
let range = TimeRange::month(2024, 1);
let january_in_europe = index.query(&europe, &range);
```

**Performance:** O(log n + k) for spatial and temporal queries (R-tree / B-tree).

## Analysis

### Clustering

```rust
use spatial_narrative::analysis::DBSCAN;

// DBSCAN: density-based clustering
let dbscan = DBSCAN::new(100_000.0, 3);  // 100km radius, min 3 points
let result = dbscan.cluster(&events);

for cluster in &result.clusters {
    println!(
        "Cluster: {} events around ({:.1}°, {:.1}°)",
        cluster.len(),
        cluster.centroid.lat,
        cluster.centroid.lon
    );
}

// Noise points (not in any cluster)
println!("{} noise points", result.noise.len());
```

### Metrics

```rust
use spatial_narrative::analysis::{SpatialMetrics, TemporalMetrics};

let spatial = SpatialMetrics::from_events(&events);
println!("Total distance: {:.0} km", spatial.total_distance / 1000.0);
println!("Dispersion: {:.0} m", spatial.dispersion);

let temporal = TemporalMetrics::from_events(&events);
println!("Duration: {:.0} hours", temporal.duration_secs / 3600.0);
println!("Avg inter-event time: {:.0} min", temporal.avg_inter_event_time / 60.0);
```

## I/O Formats

Export to formats your visualization tools understand:

```rust
use spatial_narrative::io::{Format, GeoJsonFormat, CsvFormat, JsonFormat};

// GeoJSON → Leaflet, Mapbox, QGIS, Google Earth
let mut geojson = Vec::new();
GeoJsonFormat::new().export(&narrative, &mut geojson)?;

// CSV → Excel, pandas, R, databases
let mut csv = Vec::new();
CsvFormat::new().export(&narrative, &mut csv)?;

// JSON → full fidelity, all metadata preserved
let mut json = Vec::new();
JsonFormat::pretty().export(&narrative, &mut json)?;
```

**Round-trip:** All formats support import back to `Narrative`.

## Geoparsing

Extract locations from unstructured text:

```rust
use spatial_narrative::parser::{GeoParser, BuiltinGazetteer};

let gazetteer = BuiltinGazetteer::new();  // 2500+ world cities
let parser = GeoParser::with_gazetteer(gazetteer);

let text = "The summit in Paris brought together leaders from Berlin and Tokyo.";
let mentions = parser.extract(text);

for mention in mentions {
    println!("{}: {:?}", mention.text, mention.location);
}
// Paris: Some(Location { lat: 48.8566, lon: 2.3522, ... })
// Berlin: Some(Location { lat: 52.52, lon: 13.405, ... })
// Tokyo: Some(Location { lat: 35.6762, lon: 139.6503, ... })
```

## Example: Building a Pipeline

Here's how the pieces fit together for a typical use case:

```rust
use spatial_narrative::core::{Event, EventBuilder, Location, Timestamp, NarrativeBuilder};
use spatial_narrative::analysis::{DBSCAN, SpatialMetrics};
use spatial_narrative::index::SpatiotemporalIndex;
use spatial_narrative::io::{Format, GeoJsonFormat};

fn process_my_data(raw_records: Vec<MyRecord>) -> Result<(), Box<dyn Error>> {
    // 1. Transform your data → Events
    let events: Vec<Event> = raw_records
        .into_iter()
        .filter_map(|r| {
            Some(EventBuilder::new()
                .location(Location::new(r.lat?, r.lon?))
                .timestamp(Timestamp::parse(&r.date).ok()?)
                .text(&r.description)
                .tag(&r.category)
                .build())
        })
        .collect();

    // 2. Wrap in narrative
    let narrative = NarrativeBuilder::new()
        .title("My Analysis")
        .events(events)
        .build();

    // 3. Index for queries
    let index = SpatiotemporalIndex::from_iter(
        narrative.events.clone(),
        |e| &e.location,
        |e| &e.timestamp,
    );

    // 4. Analyze
    let clusters = DBSCAN::new(50_000.0, 2).cluster(&narrative.events);
    let metrics = SpatialMetrics::from_events(&narrative.events);

    // 5. Export for visualization
    let mut file = File::create("output.geojson")?;
    GeoJsonFormat::new().export(&narrative, &mut file)?;

    Ok(())
}
```

## When to Use This Library

**Good fit:**
- You have event data with locations and times
- You need spatial/temporal indexing and queries
- You want clustering or trajectory analysis
- You're exporting to mapping tools (Leaflet, QGIS, etc.)

**Probably overkill:**
- Simple point-in-polygon checks (just use `geo`)
- Static coordinate lists (just use `geo-types`)
- No analysis needed (just serialize directly)

**Not the right tool:**
- Real-time visualization (use a JS mapping library)
- Heavy GIS operations (use GDAL bindings or PostGIS)
- Machine learning on spatial data (use specialized ML crates)

## Performance

| Operation | Complexity | Notes |
|-----------|------------|-------|
| Spatial bbox query | O(log n + k) | R-tree |
| Temporal range query | O(log n + k) | B-tree |
| K-nearest neighbors | O(log n + k) | R-tree |
| DBSCAN clustering | O(n²) | Can be slow for n > 10k |
| Metrics computation | O(n) | Single pass |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
cargo test
cargo test --all-features
cargo fmt --check
cargo clippy
```

## License

MIT License — see [LICENSE](LICENSE) for details.

## Acknowledgments

Built on top of excellent Rust crates:
- [rstar](https://docs.rs/rstar) — R-tree spatial indexing
- [petgraph](https://docs.rs/petgraph) — Graph data structures
- [chrono](https://docs.rs/chrono) — Date and time handling
- [serde](https://docs.rs/serde) — Serialization
- [geo](https://docs.rs/geo) — Geospatial primitives
