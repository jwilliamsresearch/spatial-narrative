# GeoJSON Format

Export narratives as GeoJSON FeatureCollections for use with mapping libraries.

## Basic Usage

```rust
use spatial_narrative::io::{GeoJsonFormat, Format};
use spatial_narrative::core::{Narrative, Event, Location, Timestamp};

// Export to GeoJSON
let geojson = GeoJsonFormat::new().export_str(&narrative)?;

// Import from GeoJSON
let narrative = GeoJsonFormat::new().import_str(&geojson)?;
```

## Output Structure

Events are exported as Point features:

```json
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [-74.006, 40.7128]
      },
      "properties": {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "timestamp": "2024-01-15T10:00:00Z",
        "text": "Event description",
        "tags": ["conference", "technology"]
      }
    }
  ]
}
```

> **Note**: GeoJSON uses [longitude, latitude] order per the specification.

## Configuration Options

Customize the export with `GeoJsonOptions`:

```rust
use spatial_narrative::io::{GeoJsonFormat, GeoJsonOptions};

let options = GeoJsonOptions {
    include_ids: true,           // Include event IDs
    include_tags: true,          // Include tags array
    include_sources: true,       // Include source information
    timestamp_property: "time".to_string(),      // Property name for timestamp
    text_property: "description".to_string(),    // Property name for text
};

let format = GeoJsonFormat::with_options(options);
let geojson = format.export_str(&narrative)?;
```

### Default Options

```rust
GeoJsonOptions {
    include_ids: true,
    include_tags: true,
    include_sources: false,
    timestamp_property: "timestamp".to_string(),
    text_property: "text".to_string(),
}
```

## Web Mapping Integration

### Leaflet

```javascript
// Load exported GeoJSON
fetch('narrative.geojson')
  .then(res => res.json())
  .then(geojson => {
    L.geoJSON(geojson, {
      pointToLayer: (feature, latlng) => {
        return L.circleMarker(latlng, {
          radius: 8,
          fillColor: '#ff7800',
          color: '#000',
          weight: 1,
          opacity: 1,
          fillOpacity: 0.8
        });
      },
      onEachFeature: (feature, layer) => {
        layer.bindPopup(`
          <b>${feature.properties.text}</b><br>
          ${feature.properties.timestamp}
        `);
      }
    }).addTo(map);
  });
```

### Mapbox GL JS

```javascript
map.on('load', () => {
  map.addSource('narrative', {
    type: 'geojson',
    data: 'narrative.geojson'
  });
  
  map.addLayer({
    id: 'events',
    type: 'circle',
    source: 'narrative',
    paint: {
      'circle-radius': 8,
      'circle-color': '#ff7800'
    }
  });
});
```

## Importing GeoJSON

Import existing GeoJSON data:

```rust
let geojson = r#"{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "geometry": {"type": "Point", "coordinates": [-74.006, 40.7128]},
      "properties": {
        "timestamp": "2024-01-15T10:00:00Z",
        "text": "Event description"
      }
    }
  ]
}"#;

let narrative = GeoJsonFormat::new().import_str(geojson)?;
println!("Imported {} events", narrative.events.len());
```

### Property Mapping

The importer looks for these properties:

| Property | Maps To | Required |
|----------|---------|----------|
| `timestamp`, `time`, `datetime` | `Event.timestamp` | Yes |
| `text`, `description`, `name` | `Event.text` | No |
| `tags` | `Event.tags` | No |
| `id` | `Event.id` | No (generates UUID) |

## File Operations

```rust
use std::fs::File;
use std::io::{BufReader, BufWriter};

// Export
let file = File::create("events.geojson")?;
GeoJsonFormat::new().export(&narrative, &mut BufWriter::new(file))?;

// Import
let file = File::open("events.geojson")?;
let narrative = GeoJsonFormat::new().import(&mut BufReader::new(file))?;
```

## When to Use GeoJSON

**Use GeoJSON when:**
- Displaying events on web maps (Leaflet, Mapbox, Google Maps)
- Importing into GIS software (QGIS, ArcGIS)
- Sharing with systems that expect standard geographic formats
- Building map visualizations

**Consider other formats when:**
- You need to preserve all metadata → use [JSON](./json.md)
- Importing to spreadsheets → use [CSV](./csv.md)
