# Spatiotemporal Index

The `SpatiotemporalIndex` combines spatial and temporal indexing for efficient queries on both dimensions.

## Creating an Index

```rust
use spatial_narrative::index::SpatiotemporalIndex;
use spatial_narrative::core::{Event, Location, Timestamp};

// Empty index
let mut index: SpatiotemporalIndex<Event> = SpatiotemporalIndex::new();

// Insert items
let event = Event::new(
    Location::new(40.7128, -74.0060),
    Timestamp::parse("2024-01-15T10:00:00Z").unwrap(),
    "NYC Event"
);
index.insert(event, 
    &Location::new(40.7128, -74.0060),
    &Timestamp::parse("2024-01-15T10:00:00Z").unwrap()
);
```

### From Iterator

```rust
let index = SpatiotemporalIndex::from_iter(
    events.iter().cloned(),
    |e| &e.location,
    |e| &e.timestamp
);
```

## Combined Queries

Query by both space and time:

```rust
use spatial_narrative::core::{GeoBounds, TimeRange};

let bounds = GeoBounds::new(40.0, -75.0, 41.0, -73.0);  // NYC area
let range = TimeRange::month(2024, 1);                   // January 2024

// Events in NYC during January
let results = index.query(&bounds, &range);
println!("Found {} events", results.len());
```

## Individual Dimension Queries

### Spatial Only

```rust
let bounds = GeoBounds::new(40.0, -75.0, 41.0, -73.0);
let in_area = index.query_spatial(&bounds);
```

### Temporal Only

```rust
let range = TimeRange::year(2024);
let in_2024 = index.query_temporal(&range);
```

## Nearest in Time Range

Find spatially nearest items within a time range:

```rust
let range = TimeRange::day(2024, 1, 15);

// 5 nearest events on January 15th
let nearest = index.nearest_in_range(40.7128, -74.0060, 5, &range);

for event in nearest {
    println!("{}: {}", event.timestamp.to_rfc3339(), event.text);
}
```

## Index Properties

```rust
// Number of items
println!("Items: {}", index.len());

// Check if empty
if !index.is_empty() {
    // Get bounds
    if let Some(bounds) = index.bounds() {
        println!("Geographic extent: {:.2}° x {:.2}°", 
            bounds.lat_span(), bounds.lon_span());
    }
    
    // Get time range
    if let Some(range) = index.time_range() {
        println!("Time span: {} to {}", 
            range.start.to_rfc3339(), 
            range.end.to_rfc3339());
    }
}

// Access all items
for item in index.items() {
    println!("{}", item.text);
}
```

## Methods Reference

| Method | Description |
|--------|-------------|
| `new()` | Create empty index |
| `from_iter()` | Build from iterator |
| `insert(item, location, timestamp)` | Add an item |
| `query(bounds, range)` | Query by space AND time |
| `query_spatial(bounds)` | Query by space only |
| `query_temporal(range)` | Query by time only |
| `nearest_in_range(lat, lon, k, range)` | K-nearest in time window |
| `bounds()` | Geographic extent |
| `time_range()` | Temporal extent |
| `len()` | Number of items |
| `is_empty()` | Check if empty |
| `items()` | Access all items |

## Use Cases

### Incident Analysis

```rust
// Find incidents in downtown during business hours
let downtown = GeoBounds::new(40.70, -74.02, 40.75, -73.98);

let business_hours = TimeRange::new(
    Timestamp::parse("2024-01-15T09:00:00Z").unwrap(),
    Timestamp::parse("2024-01-15T17:00:00Z").unwrap(),
);

let incidents = index.query(&downtown, &business_hours);
println!("Found {} downtown incidents during business hours", incidents.len());
```

### Event Correlation

```rust
// Find events near a location around a specific time
let location = (40.7580, -73.9855);  // Times Square
let time = Timestamp::parse("2024-01-01T00:00:00Z").unwrap();

// 2 hours around midnight on NYE
let window = TimeRange::new(
    time.subtract(std::time::Duration::from_secs(3600)),
    time.add(std::time::Duration::from_secs(3600)),
);

let times_square = GeoBounds::new(40.755, -73.990, 40.760, -73.982);
let nye_events = index.query(&times_square, &window);
```

### Coverage Analysis

```rust
// Check coverage across regions and time periods
let regions = vec![
    ("North", GeoBounds::new(41.0, -74.5, 42.0, -73.5)),
    ("Central", GeoBounds::new(40.5, -74.5, 41.0, -73.5)),
    ("South", GeoBounds::new(40.0, -74.5, 40.5, -73.5)),
];

let months = (1..=12).map(|m| TimeRange::month(2024, m));

println!("Coverage by region and month:");
for (name, bounds) in &regions {
    print!("{}: ", name);
    for month in months.clone() {
        let count = index.query(bounds, &month).len();
        print!("{} ", count);
    }
    println!();
}
```

### Anomaly Detection

```rust
// Find unusual activity spikes
let normal_bounds = GeoBounds::new(40.0, -75.0, 42.0, -73.0);

// Compare weekday vs weekend activity
let weekday = TimeRange::new(
    Timestamp::parse("2024-01-15T00:00:00Z").unwrap(),  // Monday
    Timestamp::parse("2024-01-15T23:59:59Z").unwrap(),
);

let weekend = TimeRange::new(
    Timestamp::parse("2024-01-13T00:00:00Z").unwrap(),  // Saturday
    Timestamp::parse("2024-01-13T23:59:59Z").unwrap(),
);

let weekday_count = index.query(&normal_bounds, &weekday).len();
let weekend_count = index.query(&normal_bounds, &weekend).len();

println!("Weekday: {} events, Weekend: {} events", weekday_count, weekend_count);
```
