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

**Extract geographic narratives from text.**

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

## Core Workflow

```
┌──────────────┐      ┌──────────────┐      ┌──────────────┐      ┌──────────────┐
│     Text     │  →   │  Geoparser   │  →   │   Narrative  │  →   │    Export    │
│  (documents, │      │  (extract    │      │   (events,   │      │  (GeoJSON,   │
│   articles)  │      │   locations) │      │   analysis)  │      │   mapping)   │
└──────────────┘      └──────────────┘      └──────────────┘      └──────────────┘
```

## Key Features

| Feature | Description |
|---------|-------------|
| **Geoparsing** | Extract place names from text and resolve to coordinates |
| **Built-in Gazetteer** | 2,500+ world cities with coordinates, population, aliases |
| **Coordinate Detection** | Parse decimal degrees, DMS, and other coordinate formats |
| **Online Gazetteers** | Optional Nominatim, GeoNames, Wikidata integration |
| **Event Modeling** | Structure extracted locations into events with timestamps |
| **Analysis** | Clustering, spatial metrics, trajectory detection |
| **Export** | GeoJSON, CSV, JSON for mapping tools |

## Quick Example

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
| `core` | Data types: Event, Location, Timestamp, Narrative |
| `analysis` | Clustering, metrics, trajectory analysis |
| `index` | Spatial/temporal indexing for large datasets |
| `graph` | Event relationship networks |
| `io` | GeoJSON, CSV, JSON export |

## Getting Started

Ready to extract locations from text? Start with the [Installation](./getting-started/installation.md) guide!

## Links

- [📦 Crates.io](https://crates.io/crates/spatial-narrative)
- [📖 API Documentation](https://docs.rs/spatial-narrative)
- [🐙 GitHub Repository](https://github.com/jwilliamsresearch/spatial-narrative)
