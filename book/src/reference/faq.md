# FAQ

Frequently asked questions about spatial-narrative.

## General

### What is a spatial narrative?

A spatial narrative is a sequence of events that are anchored in both space (geographic location) and time. Think of it as a story that unfolds across a map.

Examples:
- A news story with events happening in different cities over time
- GPS tracks showing a journey
- Historical events with known locations and dates
- Sensor readings with coordinates and timestamps

### When should I use this library?

Use spatial-narrative when you need to:
- Store events with both location AND time information
- Query events by geography (bounding box, radius, nearest)
- Query events by time range
- Analyze relationships between events
- Export to mapping formats (GeoJSON) or data formats (CSV)
- Build graphs showing how events connect

### What makes this different from just using a Vec<Event>?

The library provides:
- **Efficient indexing**: O(log n) spatial and temporal queries
- **Rich types**: Location with validation, Timestamp with timezone handling
- **Relationship graphs**: Connect events and find paths
- **Analysis tools**: Clustering, metrics, movement detection
- **I/O formats**: GeoJSON, CSV, JSON with proper handling

## Core Types

### How precise are coordinates?

Coordinates use `f64` (64-bit floating point), which provides about 15-16 significant digits. For geographic coordinates, this is sub-millimeter precision — far more than any real-world application needs.

### What timezone does Timestamp use?

`Timestamp` is timezone-aware and stores time in UTC internally. When parsing:

```rust
// Explicit UTC
Timestamp::parse("2024-01-15T10:00:00Z")

// With offset
Timestamp::parse("2024-01-15T10:00:00+05:00")

// Local time (uses system timezone)
Timestamp::parse("2024-01-15T10:00:00")
```

### Can I use custom IDs for events?

Events get auto-generated UUIDs, but you can store custom IDs in metadata:

```rust
event.set_metadata("custom_id", json!("MY-001"));
```

## Indexing

### Which index should I use?

| Use Case | Index |
|----------|-------|
| Only location queries | `SpatialIndex` |
| Only time queries | `TemporalIndex` |
| Both location AND time | `SpatiotemporalIndex` |

### Are indexes updated automatically?

No, indexes are immutable after creation. If you add events, rebuild the index:

```rust
// Add new events
events.push(new_event);

// Rebuild index
let index = SpatialIndex::from_iter(events.iter().cloned(), |e| &e.location);
```

### How accurate is the radius query?

`query_radius_meters` uses the Haversine formula for great-circle distance, which is accurate for the Earth as a sphere. Error is typically < 0.5% vs a full ellipsoid model.

## Graphs

### What's the difference between EdgeType variants?

| Type | Meaning | Auto-Generated |
|------|---------|----------------|
| `Temporal` | A happens before B | `connect_temporal()` |
| `Spatial` | A is near B | `connect_spatial(km)` |
| `Thematic` | A and B share tags | `connect_thematic()` |
| `Causal` | A causes B | Manual only |
| `Reference` | A mentions B | Manual only |
| `Custom` | User-defined | Manual only |

### Why is connect_spatial slow?

`connect_spatial` compares all pairs of events (O(n²)). For 10,000 events, that's 100 million comparisons. 

Workarounds:
- Use a spatial index to pre-filter candidates
- Sample events before connecting
- Skip for large datasets

## I/O

### Which format preserves all data?

`JsonFormat` preserves everything including custom metadata. `GeoJsonFormat` and `CsvFormat` may lose some fields.

### Can I import from other GeoJSON sources?

Yes, as long as the GeoJSON has Point features with timestamp information:

```json
{
  "type": "Feature",
  "geometry": {"type": "Point", "coordinates": [-74.0, 40.7]},
  "properties": {"timestamp": "2024-01-15T10:00:00Z"}
}
```

### How do I handle large files?

Use streaming I/O:

```rust
let file = File::open("large.geojson")?;
let reader = BufReader::new(file);
let narrative = GeoJsonFormat::new().import(&mut reader)?;
```

## Analysis

### What distance unit does DBSCAN use?

`DBSCAN::new(eps, min_points)` takes `eps` in **meters**.

```rust
// 500 meter radius
let dbscan = DBSCAN::new(500.0, 3);

// 5 kilometer radius
let dbscan = DBSCAN::new(5000.0, 3);
```

### How does clustering work with timestamps?

Current clustering is spatial only. For spatiotemporal clustering, filter by time first:

```rust
// Cluster events from January only
let january_events: Vec<_> = events.iter()
    .filter(|e| TimeRange::month(2024, 1).contains(&e.timestamp))
    .collect();

let dbscan = DBSCAN::new(1000.0, 3);
let result = dbscan.cluster(&january_events);
```

## Troubleshooting

### "No events found" after import

Check that:
1. File path is correct
2. Format matches file content
3. Required fields exist (coordinates, timestamp)

```rust
// Debug: print what was imported
let narrative = GeoJsonFormat::new().import_str(&content)?;
println!("Imported {} events", narrative.events.len());
if let Some(first) = narrative.events.first() {
    println!("First event: {:?}", first);
}
```

### Timestamp parse errors

Ensure ISO 8601 format:
- ✅ `2024-01-15T10:00:00Z`
- ✅ `2024-01-15T10:00:00+00:00`
- ❌ `2024/01/15 10:00` (wrong separators)
- ❌ `Jan 15, 2024` (not ISO format)

### Graph visualization looks wrong

Try different layout options:

```rust
// Timeline (left-to-right)
let dot = graph.to_dot_with_options(DotOptions::timeline());

// Hierarchical (top-to-bottom)
let dot = graph.to_dot_with_options(DotOptions::hierarchical());
```

Different Graphviz engines:
```bash
neato -Tpng graph.dot -o graph.png   # Force-directed
circo -Tpng graph.dot -o graph.png   # Circular
fdp -Tpng graph.dot -o graph.png     # Spring model
```
