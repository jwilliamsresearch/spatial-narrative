# Spatial Index (R-tree)

The `SpatialIndex` uses an R-tree for efficient geographic queries.

## Creating an Index

```rust
use spatial_narrative::index::SpatialIndex;
use spatial_narrative::core::{Event, Location, Timestamp};

// Empty index
let mut index: SpatialIndex<Event> = SpatialIndex::new();

// Insert items
let event = Event::new(
    Location::new(40.7128, -74.0060),
    Timestamp::now(),
    "NYC Event"
);
index.insert(event, &Location::new(40.7128, -74.0060));
```

### From Iterator

Build an index efficiently from a collection:

```rust
let index = SpatialIndex::from_iter(
    events.iter().cloned(),
    |event| &event.location
);
```

## Query Types

### Bounding Box Query

Find all items within a rectangular region:

```rust
// Query by lat/lon bounds
let results = index.query_bbox(
    40.0,   // min_lat
    -75.0,  // min_lon
    41.0,   // max_lat
    -73.0   // max_lon
);

println!("Found {} events in region", results.len());
```

### Using GeoBounds

```rust
use spatial_narrative::core::GeoBounds;

let nyc_area = GeoBounds::new(40.4, -74.3, 41.0, -73.7);
let results = index.query_bounds(&nyc_area);
```

### Radius Query (Degrees)

Find items within a radius (in degrees):

```rust
// Approximate radius query using degrees
// Note: This is Euclidean distance in degrees, not great-circle distance
let results = index.query_radius(
    40.7128,  // center lat
    -74.0060, // center lon
    0.1       // radius in degrees (~11km at this latitude)
);
```

### Radius Query (Meters)

For accurate distance-based queries:

```rust
// Great-circle distance using Haversine formula
let results = index.query_radius_meters(
    40.7128,  // center lat
    -74.0060, // center lon
    5000.0    // radius in meters (5km)
);
```

### K-Nearest Neighbors

Find the K closest items to a point:

```rust
// Find 10 nearest events
let nearest = index.nearest(40.7128, -74.0060, 10);

for event in nearest {
    println!("Near event: {}", event.text);
}
```

## Methods Reference

| Method | Description |
|--------|-------------|
| `new()` | Create empty index |
| `from_iter()` | Build from iterator with location extractor |
| `insert(item, location)` | Add an item |
| `query_bbox(min_lat, min_lon, max_lat, max_lon)` | Bounding box query |
| `query_bounds(bounds)` | Query using GeoBounds |
| `query_radius(lat, lon, radius_deg)` | Radius query (degrees) |
| `query_radius_meters(lat, lon, radius_m)` | Radius query (meters) |
| `nearest(lat, lon, k)` | K-nearest neighbors |
| `len()` | Number of items |
| `is_empty()` | Check if empty |

## Performance Tips

### Bulk Loading

For large datasets, use `from_iter` instead of repeated `insert`:

```rust
// Efficient: bulk load
let index = SpatialIndex::from_iter(events, |e| &e.location);

// Less efficient: repeated inserts
let mut index = SpatialIndex::new();
for event in events {
    index.insert(event.clone(), &event.location);  // O(log n) each
}
```

### Query Optimization

Start with broad queries, then filter:

```rust
// Get candidates from index
let candidates = index.query_bbox(39.0, -76.0, 42.0, -72.0);

// Apply additional filters
let filtered: Vec<_> = candidates
    .into_iter()
    .filter(|e| e.has_tag("important"))
    .filter(|e| e.timestamp.year() == 2024)
    .collect();
```

## Use Cases

### Finding Nearby Events

```rust
let my_location = Location::new(40.7580, -73.9855);  // Times Square

// Find events within 1km
let nearby = index.query_radius_meters(
    my_location.lat, 
    my_location.lon, 
    1000.0
);

println!("Found {} events within 1km", nearby.len());
```

### Regional Analysis

```rust
// Define regions
let regions = vec![
    ("NYC", GeoBounds::new(40.4, -74.3, 41.0, -73.7)),
    ("LA", GeoBounds::new(33.7, -118.7, 34.4, -117.9)),
    ("Chicago", GeoBounds::new(41.6, -88.0, 42.1, -87.5)),
];

// Count events per region
for (name, bounds) in &regions {
    let count = index.query_bounds(bounds).len();
    println!("{}: {} events", name, count);
}
```

### Clustering Preprocessing

```rust
// Use spatial index to speed up DBSCAN-style clustering
let candidates = index.query_radius_meters(point.lat, point.lon, eps);

// Only check distances for nearby points
for candidate in candidates {
    let distance = haversine_distance(&point, &candidate.location);
    if distance <= eps {
        // Add to cluster
    }
}
```
