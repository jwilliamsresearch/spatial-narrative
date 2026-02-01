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

**Extract geographic narratives from text.**

[📖 Documentation](https://docs.rs/spatial-narrative) · [🚀 Getting Started](#quick-start) · [📦 Crates.io](https://crates.io/crates/spatial-narrative)

</div>

---

## What It Does

`spatial-narrative` extracts **locations and events from unstructured text**, turning documents into structured geospatial data.

```
Input:  "The summit in Paris brought together leaders from Berlin and Tokyo.
         Negotiations continued through the week before concluding in Geneva."

Output: [
  { location: Paris (48.86°, 2.35°), text: "summit" },
  { location: Berlin (52.52°, 13.41°), text: "leaders" },
  { location: Tokyo (35.68°, 139.65°), text: "leaders" },
  { location: Geneva (46.20°, 6.14°), text: "concluding" }
]
```

## Core Features

| Feature | Description |
|---------|-------------|
| **Geoparsing** | Extract place names from text and resolve to coordinates |
| **Built-in Gazetteer** | 2,500+ world cities with coordinates, population, aliases |
| **Coordinate Detection** | Parse decimal degrees, DMS, and other coordinate formats |
| **ML-NER** | Transformer-based Named Entity Recognition (optional) |
| **Online Gazetteers** | Optional Nominatim, GeoNames, Wikidata integration |
| **Event Modeling** | Structure extracted locations into events with timestamps |
| **Analysis** | Clustering, spatial metrics, trajectory detection |
| **Export** | GeoJSON, CSV, JSON for mapping tools |

## Quick Start

```rust
use spatial_narrative::parser::{GeoParser, BuiltinGazetteer};

// Create parser with built-in gazetteer (2500+ cities, no API needed)
let gazetteer = BuiltinGazetteer::new();
let parser = GeoParser::with_gazetteer(gazetteer);

// Extract locations from text
let text = "Fighting broke out near Kyiv before spreading to Kharkiv and Odesa.";
let mentions = parser.extract(text);

for mention in &mentions {
    if let Some(loc) = &mention.location {
        println!("{}: ({:.2}°, {:.2}°)", mention.text, loc.lat, loc.lon);
    }
}
// Kyiv: (50.45°, 30.52°)
// Kharkiv: (49.99°, 36.23°)
// Odesa: (46.48°, 30.73°)
```

## Installation

```toml
[dependencies]
spatial-narrative = "0.1"
```

For online geocoding (Nominatim, GeoNames, Wikidata):

```toml
[dependencies]
spatial-narrative = { version = "0.1", features = ["geocoding"] }
```

For ML-powered Named Entity Recognition:

```toml
[dependencies]
spatial-narrative = { version = "0.1", features = ["ml-ner-download"] }
```

**Note**: ML-NER requires [ONNX Runtime](https://github.com/microsoft/onnxruntime/releases). See the [ML-NER guide](https://docs.rs/spatial-narrative) for installation.

## Geoparsing

### Built-in Gazetteer

Works offline with 2,500+ major world cities:

```rust
use spatial_narrative::parser::{GeoParser, BuiltinGazetteer, Gazetteer};

let gazetteer = BuiltinGazetteer::new();

// Direct lookup
if let Some(location) = gazetteer.lookup("Tokyo") {
    println!("Tokyo: {}, {}", location.lat, location.lon);
}

// Check for aliases
gazetteer.lookup("NYC");        // → New York
gazetteer.lookup("München");    // → Munich

// Use with parser
let parser = GeoParser::with_gazetteer(gazetteer);
let mentions = parser.extract("Protests erupted in Cairo and Alexandria.");
```

### Coordinate Detection

Automatically detects coordinates in text:

```rust
let parser = GeoParser::new();

let text = "The vessel was last seen at 40.7128° N, 74.0060° W";
let mentions = parser.extract_coordinates(text);

// Also handles:
// - Decimal: 40.7128, -74.0060
// - DMS: 40°42'46"N 74°0'22"W
// - With symbols: 40.7128°N, 74.0060°W
```

### Online Gazetteers

For comprehensive coverage beyond the built-in cities:

```rust
use spatial_narrative::parser::{GeoParser, MultiGazetteer, BuiltinGazetteer};

#[cfg(feature = "geocoding")]
{
    use spatial_narrative::parser::GazetteerNominatim;
    
    let multi = MultiGazetteer::new(vec![
        Box::new(BuiltinGazetteer::new()),
        Box::new(GazetteerNominatim::new()),
    ]);
    
    // Falls back to Nominatim if not in built-in gazetteer
    let parser = GeoParser::with_gazetteer(multi);
}
```

## ML-NER (Advanced)

For high-accuracy Named Entity Recognition using transformer models:

```rust
use spatial_narrative::text::{MlNerModel, NerModel};

// Auto-download model (first run downloads ~65MB, then cached locally)
let model = MlNerModel::download_blocking(NerModel::DistilBertQuantized)?;

let text = "Dr. Chen presented findings in Paris on March 15, 2024.";
let entities = model.extract(text)?;

for entity in entities {
    println!("{}: \"{}\" (confidence: {:.2})", 
        entity.label, entity.text, entity.score);
}
// PER: "Dr. Chen" (confidence: 0.99)
// LOC: "Paris" (confidence: 0.98)
// MISC: "March 15, 2024" (confidence: 0.95)
```

**Available models**: DistilBERT (~65MB), BERT Base (~400MB), BERT Large (~1.2GB), Multilingual (~700MB)

**Requires**: `ml-ner-download` feature + ONNX Runtime ([installation guide](https://docs.rs/spatial-narrative))

#[cfg(feature = "geocoding")]
use spatial_narrative::parser::{GazetteerNominatim, GazetteerGeoNames};

// Chain multiple sources: try built-in first, fall back to API
let gazetteer = MultiGazetteer::new()
    .add(BuiltinGazetteer::new())           // Fast, offline
    .add(GazetteerNominatim::new());        // Comprehensive, online

let parser = GeoParser::with_gazetteer(gazetteer);
```

## Building Narratives

Once you've extracted locations, structure them as events:

```rust
use spatial_narrative::core::{Event, EventBuilder, NarrativeBuilder, Timestamp};
use spatial_narrative::parser::{GeoParser, BuiltinGazetteer};

let parser = GeoParser::with_gazetteer(BuiltinGazetteer::new());

// Process a document
let article = "
    March 15: Ceasefire announced in Damascus.
    March 17: Aid convoys reached Aleppo.
    March 20: Talks resumed in Geneva.
";

// Extract and build events (you'd parse dates from text too)
let mentions = parser.extract(article);
let events: Vec<Event> = mentions
    .into_iter()
    .filter_map(|m| {
        Some(EventBuilder::new()
            .location(m.location?)
            .timestamp(Timestamp::now())  // Parse from text in practice
            .text(&m.text)
            .build())
    })
    .collect();

let narrative = NarrativeBuilder::new()
    .title("Syria Crisis Timeline")
    .events(events)
    .build();
```

## Analysis

After extraction, analyze spatial patterns:

```rust
use spatial_narrative::analysis::{DBSCAN, SpatialMetrics};

// Find geographic clusters
let dbscan = DBSCAN::new(100_000.0, 2);  // 100km radius, min 2 points
let clusters = dbscan.cluster(&narrative.events);

println!("Found {} event clusters", clusters.num_clusters());

// Compute spatial extent
let metrics = SpatialMetrics::from_events(&narrative.events);
if let Some(centroid) = metrics.centroid {
    println!("Narrative centered around: {:.2}°, {:.2}°", centroid.lat, centroid.lon);
}
println!("Geographic spread: {:.0} km", metrics.dispersion / 1000.0);
```

## Export

Export to standard formats for visualization:

```rust
use spatial_narrative::io::{Format, GeoJsonFormat};

// GeoJSON → Leaflet, Mapbox, QGIS, Google Earth
let mut output = std::fs::File::create("narrative.geojson")?;
GeoJsonFormat::new().export(&narrative, &mut output)?;
```

```javascript
// Load in Leaflet
fetch('narrative.geojson')
  .then(res => res.json())
  .then(data => L.geoJSON(data).addTo(map));
```

## Use Cases

- **Journalism**: Extract locations from news articles to map story development
- **Intelligence**: Geolocate events from reports and social media
- **Historical Research**: Map events from historical documents
- **Disaster Response**: Extract affected locations from situation reports
- **Academic Research**: Ground qualitative text data in geography

## Modules

| Module | Purpose |
|--------|---------|
| `parser` | **Geoparsing**: extract locations from text |
| `text` | **NER & ML-NER**: entity extraction, keyword analysis |
| `core` | Data types: Event, Location, Timestamp, Narrative |
| `analysis` | Clustering, metrics, trajectory analysis |
| `index` | Spatial/temporal indexing for large datasets |
| `graph` | Event relationship networks |
| `io` | GeoJSON, CSV, JSON export |

## Performance

| Operation | Notes |
|-----------|-------|
| Built-in gazetteer lookup | O(1) hash lookup, ~2500 cities |
| Coordinate extraction | Regex-based, single pass |
| DBSCAN clustering | O(n²), suitable for <10k events |
| Spatial queries | O(log n + k) via R-tree |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
cargo test
cargo test --all-features
cargo clippy
```

## License

MIT License — see [LICENSE](LICENSE).

## Acknowledgments

- Gazetteer data from [GeoNames](https://www.geonames.org/) (CC BY 4.0)
- ML models from [HuggingFace](https://huggingface.co/) (Apache 2.0 / CC BY-NC-SA 4.0)
- Built with [rstar](https://docs.rs/rstar), [chrono](https://docs.rs/chrono), [geo](https://docs.rs/geo), [ort](https://docs.rs/ort)
