# Indexing Overview

The `index` module provides efficient data structures for spatial and temporal queries.

## Index Types

| Index | Data Structure | Best For |
|-------|----------------|----------|
| [`SpatialIndex`](./spatial.md) | R-tree | Geographic queries (bbox, radius, k-nearest) |
| [`TemporalIndex`](./temporal.md) | B-tree | Time range queries |
| [`SpatiotemporalIndex`](./spatiotemporal.md) | Combined | Space + time queries together |

## Quick Example

```rust
use spatial_narrative::index::{SpatialIndex, TemporalIndex, SpatiotemporalIndex};
use spatial_narrative::core::{Event, Location, Timestamp, GeoBounds, TimeRange};

// Create events
let events = vec![
    Event::new(Location::new(40.7128, -74.0060), 
        Timestamp::parse("2024-01-15T10:00:00Z").unwrap(), "NYC Event"),
    Event::new(Location::new(34.0522, -118.2437), 
        Timestamp::parse("2024-01-15T14:00:00Z").unwrap(), "LA Event"),
];

// Spatial-only queries
let mut spatial = SpatialIndex::new();
for e in &events {
    spatial.insert(e.clone(), &e.location);
}
let nearby = spatial.query_radius(40.7, -74.0, 0.5);

// Temporal-only queries
let mut temporal = TemporalIndex::new();
for e in &events {
    temporal.insert(e.clone(), &e.timestamp);
}
let range = TimeRange::new(
    Timestamp::parse("2024-01-15T09:00:00Z").unwrap(),
    Timestamp::parse("2024-01-15T12:00:00Z").unwrap(),
);
let morning = temporal.query_range(&range);

// Combined queries
let mut combined = SpatiotemporalIndex::new();
for e in &events {
    combined.insert(e.clone(), &e.location, &e.timestamp);
}
let bounds = GeoBounds::new(40.0, -75.0, 41.0, -73.0);
let results = combined.query(&bounds, &range);
```

## Performance Characteristics

| Operation | SpatialIndex | TemporalIndex | SpatiotemporalIndex |
|-----------|--------------|---------------|---------------------|
| Insert | O(log n) | O(log n) | O(log n) |
| Point query | O(log n) | O(log n) | O(log n) |
| Range query | O(log n + k) | O(log n + k) | O(log n + k) |
| K-nearest | O(k log n) | - | - |

Where `n` is the number of items and `k` is the number of results.

## Choosing an Index

### Use SpatialIndex when:
- You only care about location
- Finding events near a point
- Bounding box searches
- K-nearest neighbor queries

### Use TemporalIndex when:
- You only care about time
- Finding events in a time range
- Before/after queries
- Chronological iteration

### Use SpatiotemporalIndex when:
- You need to filter by both space AND time
- Events have meaningful locations and timestamps
- Generating heatmaps

## Generic Types

All indexes are generic over the stored item type:

```rust
// Index of events
let event_index: SpatialIndex<Event> = SpatialIndex::new();

// Index of custom structs
struct MyData { name: String, location: Location }
let my_index: SpatialIndex<MyData> = SpatialIndex::new();

// Index of references
let ref_index: SpatialIndex<&Event> = SpatialIndex::new();

// Index of IDs (lookup in separate collection)
let id_index: SpatialIndex<usize> = SpatialIndex::new();
```

## Building from Iterators

Efficiently build indexes from collections:

```rust
// From events with extractors
let spatial = SpatialIndex::from_iter(
    events.iter().cloned(),
    |e| &e.location
);

let temporal = TemporalIndex::from_iter(
    events.iter().cloned(),
    |e| &e.timestamp
);

let combined = SpatiotemporalIndex::from_iter(
    events.iter().cloned(),
    |e| &e.location,
    |e| &e.timestamp
);
```

## Next Steps

- [Spatial Index](./spatial.md) - R-tree geographic queries
- [Temporal Index](./temporal.md) - B-tree time queries
- [Spatiotemporal Index](./spatiotemporal.md) - Combined queries
- [Heatmaps](./heatmaps.md) - Density visualization
