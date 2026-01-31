# I/O Overview

The `io` module provides import and export functionality for narratives in various formats.

## Supported Formats

| Format | Type | Best For |
|--------|------|----------|
| [**JSON**](./json.md) | Native | Full fidelity, all metadata preserved |
| [**GeoJSON**](./geojson.md) | Standard | Web mapping, GIS tools, Leaflet/Mapbox |
| [**CSV**](./csv.md) | Tabular | Spreadsheets, data analysis, pandas |

## The Format Trait

All formats implement the `Format` trait:

```rust
use spatial_narrative::io::Format;

pub trait Format {
    fn import<R: Read>(&self, reader: &mut R) -> Result<Narrative>;
    fn export<W: Write>(&self, narrative: &Narrative, writer: &mut W) -> Result<()>;
    fn import_str(&self, s: &str) -> Result<Narrative>;
    fn export_str(&self, narrative: &Narrative) -> Result<String>;
}
```

## Quick Examples

### Export to Multiple Formats

```rust
use spatial_narrative::io::{GeoJsonFormat, CsvFormat, JsonFormat, Format};
use spatial_narrative::core::{Narrative, NarrativeBuilder, Event, Location, Timestamp};

let narrative = NarrativeBuilder::new()
    .title("My Story")
    .event(Event::new(
        Location::new(40.7128, -74.0060),
        Timestamp::now(),
        "Something happened"
    ))
    .build();

// Export to GeoJSON (for web maps)
let geojson = GeoJsonFormat::new().export_str(&narrative)?;

// Export to CSV (for spreadsheets)
let csv = CsvFormat::new().export_str(&narrative)?;

// Export to JSON (for full fidelity)
let json = JsonFormat::new().export_str(&narrative)?;
```

### Import from File

```rust
use std::fs::File;
use std::io::BufReader;

// Import from GeoJSON file
let file = File::open("data.geojson")?;
let mut reader = BufReader::new(file);
let narrative = GeoJsonFormat::new().import(&mut reader)?;

println!("Loaded {} events", narrative.events.len());
```

### Export to File

```rust
use std::fs::File;
use std::io::BufWriter;

// Export to CSV file
let file = File::create("output.csv")?;
let mut writer = BufWriter::new(file);
CsvFormat::new().export(&narrative, &mut writer)?;
```

## Format Comparison

| Feature | JSON | GeoJSON | CSV |
|---------|------|---------|-----|
| All metadata | ✅ | ⚠️ Partial | ⚠️ Limited |
| Tags | ✅ | ✅ | ✅ |
| Sources | ✅ | ✅ | ⚠️ Optional |
| Custom metadata | ✅ | ⚠️ Properties | ❌ |
| Human readable | ⚠️ | ✅ | ✅ |
| GIS compatible | ❌ | ✅ | ⚠️ |
| Spreadsheet ready | ❌ | ❌ | ✅ |
| File size | Medium | Large | Small |

## Round-Trip Fidelity

For lossless round-trips, use `JsonFormat`:

```rust
let json = JsonFormat::new();

// Export
let exported = json.export_str(&narrative)?;

// Import
let imported = json.import_str(&exported)?;

// Verify
assert_eq!(narrative.events.len(), imported.events.len());
```

## Next Steps

- [JSON Format](./json.md) - Native format with full fidelity
- [GeoJSON Format](./geojson.md) - Standard geographic format
- [CSV Format](./csv.md) - Tabular export for spreadsheets
- [Custom Formats](./custom.md) - Implement your own format
