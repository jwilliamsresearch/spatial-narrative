# Spatial Metrics

Analyze the geographic characteristics of events.

## SpatialMetrics

The `SpatialMetrics` struct provides geographic measurements for a set of events.

```rust
use spatial_narrative::analysis::SpatialMetrics;
use spatial_narrative::core::{Event, Location, Timestamp};

let events = vec![
    Event::new(Location::new(40.7128, -74.0060), Timestamp::now(), "NYC"),
    Event::new(Location::new(34.0522, -118.2437), Timestamp::now(), "LA"),
    Event::new(Location::new(41.8781, -87.6298), Timestamp::now(), "Chicago"),
];

let metrics = SpatialMetrics::from_events(&events);
```

### Available Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `total_distance` | `f64` | Sum of distances between consecutive events (meters) |
| `max_extent` | `f64` | Maximum distance between any two events (meters) |
| `centroid` | `(f64, f64)` | Geographic center (lat, lon) |
| `bounds` | `GeoBounds` | Bounding box containing all events |
| `dispersion` | `f64` | Standard distance from centroid (meters) |

### Example Output

```rust
println!("Metrics for {} events:", events.len());
println!("  Total distance: {:.1} km", metrics.total_distance / 1000.0);
println!("  Maximum extent: {:.1} km", metrics.max_extent / 1000.0);
println!("  Centroid: ({:.4}°, {:.4}°)", metrics.centroid.0, metrics.centroid.1);
println!("  Dispersion: {:.1} km", metrics.dispersion / 1000.0);
println!("  Bounds: {:.2}°N to {:.2}°N, {:.2}°W to {:.2}°W",
    metrics.bounds.min_lat, metrics.bounds.max_lat,
    metrics.bounds.min_lon.abs(), metrics.bounds.max_lon.abs());
```

## Distance Functions

### Haversine Distance

Calculate great-circle distance between two points:

```rust
use spatial_narrative::analysis::haversine_distance;
use spatial_narrative::core::Location;

let nyc = Location::new(40.7128, -74.0060);
let london = Location::new(51.5074, -0.1278);

let distance_m = haversine_distance(&nyc, &london);
println!("NYC to London: {:.0} km", distance_m / 1000.0);  // ~5570 km
```

### Bearing

Calculate initial bearing between two points:

```rust
use spatial_narrative::analysis::bearing;

let from = Location::new(40.7128, -74.0060);  // NYC
let to = Location::new(51.5074, -0.1278);     // London

let degrees = bearing(&from, &to);
println!("Bearing: {:.1}°", degrees);  // ~51° (northeast)
```

### Destination Point

Calculate destination given start, bearing, and distance:

```rust
use spatial_narrative::analysis::destination_point;

let start = Location::new(40.7128, -74.0060);
let bearing_deg = 45.0;   // Northeast
let distance_m = 100_000.0;  // 100 km

let dest = destination_point(&start, bearing_deg, distance_m);
println!("Destination: ({:.4}, {:.4})", dest.lat, dest.lon);
```

## Density Mapping

Generate a density map of event locations:

```rust
use spatial_narrative::analysis::{density_map, DensityCell};
use spatial_narrative::core::GeoBounds;

// Define grid
let bounds = GeoBounds::new(34.0, -118.5, 41.0, -73.5);
let lat_cells = 50;
let lon_cells = 50;

// Generate density map
let cells = density_map(&events, &bounds, lat_cells, lon_cells);

// Find hotspots
let max_density = cells.iter().map(|c| c.count).max().unwrap_or(0);
let hotspots: Vec<_> = cells.iter()
    .filter(|c| c.count > max_density / 2)
    .collect();

println!("Found {} hotspot cells", hotspots.len());
```

### DensityCell

Each cell in the density map contains:

| Field | Type | Description |
|-------|------|-------------|
| `lat_idx` | `usize` | Row index |
| `lon_idx` | `usize` | Column index |
| `count` | `usize` | Number of events in cell |
| `center` | `(f64, f64)` | Cell center coordinates |

## Use Cases

### Analyzing Geographic Spread

```rust
let metrics = SpatialMetrics::from_events(&events);

if metrics.max_extent > 1_000_000.0 {
    println!("Events span over 1000 km - continental scale");
} else if metrics.max_extent > 100_000.0 {
    println!("Events span over 100 km - regional scale");
} else {
    println!("Events are localized - city scale");
}
```

### Finding the Geographic Center

```rust
let metrics = SpatialMetrics::from_events(&events);
let center = Location::new(metrics.centroid.0, metrics.centroid.1);

// Find events nearest to center
for event in &events {
    let dist = haversine_distance(&event.location, &center);
    if dist < 10_000.0 {  // Within 10km of center
        println!("Near center: {}", event.text);
    }
}
```
