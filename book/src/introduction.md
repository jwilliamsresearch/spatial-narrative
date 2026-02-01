# Spatial Narrative

<div align="center">

<svg viewBox="0 0 600 160" xmlns="http://www.w3.org/2000/svg" style="max-width: 500px; width: 100%;">
    <defs>
        <linearGradient id="primaryGrad" x1="0%" y1="0%" x2="100%" y2="0%">
            <stop offset="0%" style="stop-color:#1e3a8a;stop-opacity:1" />
            <stop offset="100%" style="stop-color:#f97316;stop-opacity:1" />
        </linearGradient>
    </defs>
    <text x="40" y="115" font-family="'Segoe UI', sans-serif" font-weight="800" font-size="64" fill="#1e3a8a" letter-spacing="-1.5">spatial</text>
    <text x="270" y="85" font-family="'Segoe UI', sans-serif" font-weight="300" font-size="64" fill="#f97316" letter-spacing="-1">narrative</text>
    <path id="pathStepUp" d="M45,135 L230,135 C250,135 250,105 270,105 L540,105" fill="none" stroke="url(#primaryGrad)" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"/>
    <circle cx="45" cy="135" r="5" fill="#1e3a8a"/>
    <circle cx="540" cy="105" r="5" fill="#f97316"/>
    <circle r="5" fill="white" stroke="#f97316" stroke-width="2">
        <animateMotion dur="4s" repeatCount="indefinite" keyPoints="0;1" keyTimes="0;1" calcMode="spline" keySplines="0.4 0 0.2 1">
            <mpath href="#pathStepUp"/>
        </animateMotion>
        <animate attributeName="opacity" values="0;1;1;0" keyTimes="0;0.1;0.9;1" dur="4s" repeatCount="indefinite" />
    </circle>
</svg>

</div>

**Composable building blocks for spatial narratives in Rust.**

## Philosophy

`spatial-narrative` provides **focused, interoperable components** for working with geospatial event data. It's designed to fit into your existing data pipeline, not replace it.

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

### What it provides

- Standard data types for events, locations, timestamps, and narratives
- Spatial indexing (R-tree) and temporal indexing (B-tree)
- Analysis algorithms: DBSCAN clustering, spatial/temporal metrics, trajectory detection
- Format conversion: GeoJSON, CSV, JSON import/export
- Graph structures for event relationships
- Geoparsing: extract locations from unstructured text

### What it doesn't try to do

- Fetch data from APIs (use `reqwest`)
- Parse domain-specific formats (use your parser + our types)
- Render visualizations (export to GeoJSON, use Leaflet/Mapbox)
- Replace `geo`/`geo-types` (we interop with them)

The library is most valuable when you need **analysis and indexing** on top of your own data ingestion pipeline.

## Modules

| Module | What it does | Key types |
|--------|--------------|-----------|
| `core` | Data structures | `Event`, `Location`, `Timestamp`, `Narrative`, `GeoBounds`, `TimeRange` |
| `index` | Fast spatial/temporal queries | `SpatialIndex`, `TemporalIndex`, `SpatiotemporalIndex` |
| `analysis` | Algorithms | `DBSCAN`, `KMeans`, `SpatialMetrics`, `TemporalMetrics`, `Trajectory` |
| `graph` | Event relationships | `NarrativeGraph`, `EdgeType` |
| `io` | Format conversion | `GeoJsonFormat`, `CsvFormat`, `JsonFormat` |
| `parser` | Text → locations | `GeoParser`, `Gazetteer`, `BuiltinGazetteer` |

## Quick Example

```rust
use spatial_narrative::core::{Event, Location, Timestamp, NarrativeBuilder};
use spatial_narrative::analysis::{DBSCAN, SpatialMetrics};
use spatial_narrative::io::{Format, GeoJsonFormat};

// Your events (from whatever source you fetch them)
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

// Export: GeoJSON for web visualization
let mut output = Vec::new();
GeoJsonFormat::new().export(&narrative, &mut output)?;
// → Use with Leaflet, Mapbox, QGIS, etc.
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

## Getting Started

Ready to dive in? Start with the [Installation](./getting-started/installation.md) guide!

## Links

- [📦 Crates.io](https://crates.io/crates/spatial-narrative)
- [📖 API Documentation](https://docs.rs/spatial-narrative)
- [🐙 GitHub Repository](https://github.com/jwilliamsresearch/spatial-narrative)
- [💬 Discussions](https://github.com/jwilliamsresearch/spatial-narrative/discussions)
