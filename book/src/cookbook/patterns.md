# Common Patterns

Practical patterns for working with spatial narratives.

## Creating Events

### From Raw Data

```rust
use spatial_narrative::core::{Event, EventBuilder, Location, Timestamp};

// Simple event
let event = Event::new(
    Location::new(40.7128, -74.0060),
    Timestamp::parse("2024-01-15T10:00:00Z").unwrap(),
    "Event description"
);

// Rich event with builder
let event = EventBuilder::new()
    .location(Location::builder()
        .lat(40.7128)
        .lon(-74.0060)
        .name("New York City")
        .build()
        .unwrap())
    .timestamp(Timestamp::parse("2024-01-15T10:00:00Z").unwrap())
    .text("Conference begins")
    .tag("conference")
    .tag("technology")
    .build();
```

### From Database Records

```rust
fn event_from_row(row: &Row) -> Result<Event> {
    let event = EventBuilder::new()
        .location(Location::new(
            row.get::<f64>("latitude")?,
            row.get::<f64>("longitude")?
        ))
        .timestamp(Timestamp::parse(&row.get::<String>("datetime")?)?)
        .text(row.get::<String>("description")?)
        .build();
    Ok(event)
}
```

### From CSV Line

```rust
fn event_from_csv(line: &str) -> Result<Event> {
    let parts: Vec<&str> = line.split(',').collect();
    let event = Event::new(
        Location::new(parts[0].parse()?, parts[1].parse()?),
        Timestamp::parse(parts[2])?,
        parts[3]
    );
    Ok(event)
}
```

## Building Narratives

### From Event Collection

```rust
use spatial_narrative::core::{Narrative, NarrativeBuilder};

let narrative = NarrativeBuilder::new()
    .title("My Narrative")
    .description("A collection of events")
    .author("Research Team")
    .events(events)
    .tag("research")
    .build();
```

### Incremental Building

```rust
let mut builder = NarrativeBuilder::new()
    .title("Growing Narrative");

for data in data_stream {
    if let Ok(event) = process_data(data) {
        builder = builder.event(event);
    }
}

let narrative = builder.build();
```

## Filtering Patterns

### By Location

```rust
use spatial_narrative::core::GeoBounds;

let nyc_bounds = GeoBounds::new(40.4, -74.3, 41.0, -73.7);
let nyc_events = narrative.filter_spatial(&nyc_bounds);
```

### By Time

```rust
use spatial_narrative::core::TimeRange;

let january = TimeRange::month(2024, 1);
let january_events = narrative.filter_temporal(&january);
```

### By Tags

```rust
let important: Vec<_> = narrative.events.iter()
    .filter(|e| e.has_tag("important"))
    .collect();
```

### Combined Filters

```rust
let filtered: Vec<_> = narrative.events.iter()
    .filter(|e| nyc_bounds.contains(&e.location))
    .filter(|e| january.contains(&e.timestamp))
    .filter(|e| e.has_tag("verified"))
    .collect();
```

## Index Usage Patterns

### Build Once, Query Many

```rust
use spatial_narrative::index::SpatiotemporalIndex;

// Build index once
let index = SpatiotemporalIndex::from_iter(
    events.iter().cloned(),
    |e| &e.location,
    |e| &e.timestamp
);

// Query multiple times
let q1 = index.query(&bounds1, &range1);
let q2 = index.query(&bounds2, &range2);
let q3 = index.query_spatial(&bounds3);
```

### Nearest Neighbor Search

```rust
use spatial_narrative::index::SpatialIndex;

let index = SpatialIndex::from_iter(events, |e| &e.location);

// Find 5 nearest events to a point
let nearest = index.nearest(user_lat, user_lon, 5);
```

## Graph Patterns

### Complete Analysis Pipeline

```rust
use spatial_narrative::graph::{NarrativeGraph, EdgeType};

// Build graph
let mut graph = NarrativeGraph::from_events(events);

// Apply connection strategies
graph.connect_temporal();
graph.connect_spatial(10.0);
graph.connect_thematic();

// Analyze
let roots = graph.roots();      // Starting points
let leaves = graph.leaves();    // End points
let hub_count = graph.nodes()
    .filter(|(id, _)| graph.in_degree(*id) + graph.out_degree(*id) > 5)
    .count();

println!("Graph: {} roots, {} leaves, {} hubs", 
    roots.len(), leaves.len(), hub_count);
```

### Path Analysis

```rust
// Find how events are connected
let start = graph.get_node(&event1.id).unwrap();
let end = graph.get_node(&event2.id).unwrap();

if let Some(path) = graph.shortest_path(start, end) {
    println!("Connected via {} events", path.nodes.len());
}
```

## IO Patterns

### Multi-Format Export

```rust
use spatial_narrative::io::{Format, GeoJsonFormat, CsvFormat, JsonFormat};

// Export to all formats
let geojson = GeoJsonFormat::new().export_str(&narrative)?;
let csv = CsvFormat::new().export_str(&narrative)?;
let json = JsonFormat::pretty().export_str(&narrative)?;

std::fs::write("data.geojson", &geojson)?;
std::fs::write("data.csv", &csv)?;
std::fs::write("data.json", &json)?;
```

### Round-Trip Preservation

```rust
// Use JSON for lossless round-trips
let json = JsonFormat::new();
let exported = json.export_str(&narrative)?;
let imported = json.import_str(&exported)?;

assert_eq!(narrative.events.len(), imported.events.len());
```

## Analysis Patterns

### Quick Metrics

```rust
use spatial_narrative::analysis::{SpatialMetrics, TemporalMetrics};

let spatial = SpatialMetrics::from_events(&events);
let temporal = TemporalMetrics::from_events(&events);

println!("Spatial extent: {:.0} km", spatial.max_extent / 1000.0);
println!("Time span: {:?}", temporal.duration);
println!("Event rate: {:.2}/hour", 
    events.len() as f64 / temporal.duration.as_secs_f64() * 3600.0);
```

### Clustering

```rust
use spatial_narrative::analysis::DBSCAN;

let dbscan = DBSCAN::new(5000.0, 3);  // 5km radius, min 3 points
let result = dbscan.cluster(&events);

println!("Found {} clusters, {} noise points", 
    result.num_clusters(), 
    result.noise.len());
```

## Error Handling

### Graceful Parsing

```rust
let events: Vec<Event> = raw_data.iter()
    .filter_map(|row| {
        match parse_event(row) {
            Ok(event) => Some(event),
            Err(e) => {
                eprintln!("Skipping invalid row: {}", e);
                None
            }
        }
    })
    .collect();
```

### Validation

```rust
fn validate_event(event: &Event) -> Result<()> {
    if event.text.is_empty() {
        return Err(Error::Validation("Event text is empty".into()));
    }
    if event.location.lat < -90.0 || event.location.lat > 90.0 {
        return Err(Error::Validation("Invalid latitude".into()));
    }
    Ok(())
}
```
