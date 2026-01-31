# JSON Format

The native JSON format provides full fidelity for narratives, preserving all metadata.

## Basic Usage

```rust
use spatial_narrative::io::{JsonFormat, Format};
use spatial_narrative::core::{Narrative, NarrativeBuilder, Event, Location, Timestamp};

let narrative = NarrativeBuilder::new()
    .title("My Narrative")
    .event(Event::new(
        Location::new(40.7128, -74.0060),
        Timestamp::now(),
        "Event description"
    ))
    .build();

// Export
let json = JsonFormat::new().export_str(&narrative)?;

// Import
let imported = JsonFormat::new().import_str(&json)?;
```

## Pretty Printing

For human-readable output:

```rust
let format = JsonFormat::pretty();
let json = format.export_str(&narrative)?;

// Output is indented and formatted
println!("{}", json);
```

## Output Structure

The JSON format produces:

```json
{
  "version": "1.0",
  "narrative": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "My Narrative",
    "description": null,
    "author": null,
    "created": "2024-01-15T10:00:00Z",
    "modified": "2024-01-15T10:00:00Z",
    "tags": ["example"],
    "metadata": {}
  },
  "events": [
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "location": {
        "lat": 40.7128,
        "lon": -74.006,
        "elevation": null,
        "uncertainty_meters": null,
        "name": "New York City"
      },
      "timestamp": "2024-01-15T10:00:00Z",
      "text": "Event description",
      "tags": ["conference"],
      "source": {
        "title": "Source Name",
        "source_type": "article",
        "url": "https://example.com"
      },
      "metadata": {}
    }
  ]
}
```

## Version Compatibility

The format includes version information for forward compatibility:

```rust
// Current version is "1.0"
// Future versions will maintain backward compatibility

let json = r#"{"version": "1.0", "narrative": {...}, "events": [...]}"#;
let narrative = JsonFormat::new().import_str(json)?;
```

## File Operations

### Export to File

```rust
use std::fs::File;
use std::io::BufWriter;

let file = File::create("narrative.json")?;
let mut writer = BufWriter::new(file);
JsonFormat::pretty().export(&narrative, &mut writer)?;
```

### Import from File

```rust
use std::fs::File;
use std::io::BufReader;

let file = File::open("narrative.json")?;
let mut reader = BufReader::new(file);
let narrative = JsonFormat::new().import(&mut reader)?;
```

## When to Use JSON

**Use JSON when:**
- Archiving narratives for later use
- Transferring between systems using this library
- You need to preserve all metadata
- Round-trip fidelity is important

**Consider other formats when:**
- Integrating with GIS tools → use [GeoJSON](./geojson.md)
- Importing to spreadsheets → use [CSV](./csv.md)
- Minimizing file size is critical

## Metadata Preservation

JSON preserves all custom metadata:

```rust
use serde_json::json;

let mut event = Event::new(location, timestamp, "Description");
event.set_metadata("custom_field", json!({"nested": "data"}));
event.set_metadata("priority", json!(1));

// All metadata is preserved in JSON export
let json = JsonFormat::new().export_str(&narrative)?;
let imported = JsonFormat::new().import_str(&json)?;

assert_eq!(
    imported.events[0].get_metadata("custom_field"),
    Some(&json!({"nested": "data"}))
);
```
