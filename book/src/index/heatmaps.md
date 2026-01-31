# Heatmaps

Generate spatial density data for visualization.

## Overview

The `Heatmap` struct represents event density across a grid, useful for:
- Visualizing activity hotspots
- Identifying high-density areas
- Creating choropleth maps

## Generating Heatmaps

```rust
use spatial_narrative::index::{SpatiotemporalIndex, GridSpec, Heatmap};
use spatial_narrative::core::GeoBounds;

// Build index from events
let index = SpatiotemporalIndex::from_iter(
    events.iter().cloned(),
    |e| &e.location,
    |e| &e.timestamp
);

// Define grid
let bounds = GeoBounds::new(40.0, -75.0, 42.0, -73.0);
let grid = GridSpec::new(bounds, 50, 50);  // 50x50 cells

// Generate heatmap
let heatmap = index.heatmap(grid);
```

## GridSpec

Configure the heatmap grid:

```rust
use spatial_narrative::index::GridSpec;
use spatial_narrative::core::GeoBounds;

let grid = GridSpec::new(
    GeoBounds::new(40.0, -75.0, 42.0, -73.0),  // bounds
    100,  // lat_cells (rows)
    100,  // lon_cells (columns)
);

// Get cell dimensions
let (lat_size, lon_size) = grid.cell_size();
println!("Cell size: {:.4}° lat x {:.4}° lon", lat_size, lon_size);
```

## Accessing Heatmap Data

### Raw Counts

```rust
// Get count at specific cell
let count = heatmap.get(25, 30);  // row 25, column 30
println!("Cell (25, 30): {} events", count);

// Maximum count
let max = heatmap.max_count();
println!("Maximum density: {} events", max);
```

### Normalized Values

```rust
// Get normalized value (0.0 to 1.0)
let intensity = heatmap.get_normalized(25, 30);
println!("Cell (25, 30) intensity: {:.2}", intensity);
```

### Cell Centers

```rust
// Get the center coordinates of a cell
let (lat, lon) = heatmap.cell_center(25, 30);
println!("Cell center: ({:.4}, {:.4})", lat, lon);
```

## Iterating Over Cells

```rust
// Iterate over all cells
for lat_idx in 0..heatmap.grid.lat_cells {
    for lon_idx in 0..heatmap.grid.lon_cells {
        let count = heatmap.get(lat_idx, lon_idx);
        if count > 0 {
            let (lat, lon) = heatmap.cell_center(lat_idx, lon_idx);
            println!("({:.4}, {:.4}): {} events", lat, lon, count);
        }
    }
}
```

## Exporting for Visualization

### As GeoJSON Grid

```rust
use serde_json::json;

let mut features = Vec::new();

for lat_idx in 0..heatmap.grid.lat_cells {
    for lon_idx in 0..heatmap.grid.lon_cells {
        let count = heatmap.get(lat_idx, lon_idx);
        if count > 0 {
            let (lat_size, lon_size) = heatmap.grid.cell_size();
            let min_lat = heatmap.grid.bounds.min_lat + lat_idx as f64 * lat_size;
            let min_lon = heatmap.grid.bounds.min_lon + lon_idx as f64 * lon_size;
            
            features.push(json!({
                "type": "Feature",
                "properties": {
                    "count": count,
                    "intensity": heatmap.get_normalized(lat_idx, lon_idx)
                },
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [[
                        [min_lon, min_lat],
                        [min_lon + lon_size, min_lat],
                        [min_lon + lon_size, min_lat + lat_size],
                        [min_lon, min_lat + lat_size],
                        [min_lon, min_lat]
                    ]]
                }
            }));
        }
    }
}

let geojson = json!({
    "type": "FeatureCollection",
    "features": features
});
```

### As CSV

```rust
use std::fs::File;
use std::io::Write;

let mut file = File::create("heatmap.csv")?;
writeln!(file, "lat,lon,count,intensity")?;

for lat_idx in 0..heatmap.grid.lat_cells {
    for lon_idx in 0..heatmap.grid.lon_cells {
        let count = heatmap.get(lat_idx, lon_idx);
        let (lat, lon) = heatmap.cell_center(lat_idx, lon_idx);
        let intensity = heatmap.get_normalized(lat_idx, lon_idx);
        writeln!(file, "{},{},{},{}", lat, lon, count, intensity)?;
    }
}
```

## Visualization Integration

### Leaflet.heat

```javascript
// Load heatmap data
const heatData = [];
for (const feature of geojson.features) {
    const coords = feature.geometry.coordinates[0][0];
    heatData.push([
        (coords[1] + feature.geometry.coordinates[0][2][1]) / 2,  // center lat
        (coords[0] + feature.geometry.coordinates[0][1][0]) / 2,  // center lon
        feature.properties.intensity  // intensity
    ]);
}

L.heatLayer(heatData, {radius: 25}).addTo(map);
```

### Mapbox GL JS

```javascript
map.addSource('heatmap', {
    type: 'geojson',
    data: geojson
});

map.addLayer({
    id: 'heat',
    type: 'fill',
    source: 'heatmap',
    paint: {
        'fill-color': [
            'interpolate',
            ['linear'],
            ['get', 'intensity'],
            0, 'rgba(0, 0, 255, 0)',
            0.5, 'rgba(255, 255, 0, 0.5)',
            1, 'rgba(255, 0, 0, 0.8)'
        ]
    }
});
```

## Use Cases

### Finding Hotspots

```rust
// Find cells with highest activity
let threshold = heatmap.max_count() / 2;
let mut hotspots = Vec::new();

for lat_idx in 0..heatmap.grid.lat_cells {
    for lon_idx in 0..heatmap.grid.lon_cells {
        if heatmap.get(lat_idx, lon_idx) >= threshold {
            hotspots.push(heatmap.cell_center(lat_idx, lon_idx));
        }
    }
}

println!("Found {} hotspot cells", hotspots.len());
```

### Comparing Time Periods

```rust
// Generate heatmaps for different periods
let morning = TimeRange::new(
    Timestamp::parse("2024-01-15T06:00:00Z").unwrap(),
    Timestamp::parse("2024-01-15T12:00:00Z").unwrap(),
);

let evening = TimeRange::new(
    Timestamp::parse("2024-01-15T18:00:00Z").unwrap(),
    Timestamp::parse("2024-01-16T00:00:00Z").unwrap(),
);

// Filter and generate separate heatmaps
let morning_events = index.query_temporal(&morning);
let evening_events = index.query_temporal(&evening);

// Compare distributions...
```
