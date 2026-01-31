# Filtering Events

Techniques for filtering and selecting events.

## Spatial Filtering

### By Bounding Box

```rust
use spatial_narrative::core::GeoBounds;

// Define bounds
let nyc = GeoBounds::new(40.4, -74.3, 41.0, -73.7);

// Filter narrative
let nyc_narrative = narrative.filter_spatial(&nyc);

// Or filter events directly
let nyc_events: Vec<_> = events.iter()
    .filter(|e| nyc.contains(&e.location))
    .collect();
```

### By Radius

```rust
use spatial_narrative::analysis::haversine_distance;
use spatial_narrative::core::Location;

let center = Location::new(40.7128, -74.0060);
let radius_m = 5000.0;  // 5km

let nearby: Vec<_> = events.iter()
    .filter(|e| haversine_distance(&center, &e.location) <= radius_m)
    .collect();
```

### By Multiple Regions

```rust
let regions = vec![
    GeoBounds::new(40.4, -74.3, 41.0, -73.7),   // NYC
    GeoBounds::new(33.7, -118.7, 34.4, -117.9), // LA
    GeoBounds::new(41.6, -88.0, 42.1, -87.5),   // Chicago
];

let in_any_region: Vec<_> = events.iter()
    .filter(|e| regions.iter().any(|r| r.contains(&e.location)))
    .collect();
```

## Temporal Filtering

### By Time Range

```rust
use spatial_narrative::core::TimeRange;

// Specific range
let range = TimeRange::new(
    Timestamp::parse("2024-01-01T00:00:00Z").unwrap(),
    Timestamp::parse("2024-01-31T23:59:59Z").unwrap(),
);
let january = narrative.filter_temporal(&range);

// Convenience constructors
let q1 = events.iter()
    .filter(|e| TimeRange::month(2024, 1).contains(&e.timestamp)
             || TimeRange::month(2024, 2).contains(&e.timestamp)
             || TimeRange::month(2024, 3).contains(&e.timestamp));
```

### Before/After

```rust
let cutoff = Timestamp::parse("2024-06-01T00:00:00Z").unwrap();

let before: Vec<_> = events.iter()
    .filter(|e| e.timestamp < cutoff)
    .collect();

let after: Vec<_> = events.iter()
    .filter(|e| e.timestamp >= cutoff)
    .collect();
```

### By Day of Week

```rust
let weekends: Vec<_> = events.iter()
    .filter(|e| {
        let weekday = e.timestamp.weekday();
        weekday == 0 || weekday == 6  // Sunday or Saturday
    })
    .collect();
```

### By Time of Day

```rust
// Business hours (9 AM to 5 PM)
let business_hours: Vec<_> = events.iter()
    .filter(|e| {
        let hour = e.timestamp.hour();
        hour >= 9 && hour < 17
    })
    .collect();
```

## Tag Filtering

### Single Tag

```rust
let important: Vec<_> = events.iter()
    .filter(|e| e.has_tag("important"))
    .collect();
```

### Any of Tags

```rust
let priority_tags = ["urgent", "important", "critical"];

let priority: Vec<_> = events.iter()
    .filter(|e| priority_tags.iter().any(|t| e.has_tag(t)))
    .collect();
```

### All of Tags

```rust
let required_tags = ["verified", "published"];

let complete: Vec<_> = events.iter()
    .filter(|e| required_tags.iter().all(|t| e.has_tag(t)))
    .collect();
```

### Excluding Tags

```rust
let not_spam: Vec<_> = events.iter()
    .filter(|e| !e.has_tag("spam") && !e.has_tag("duplicate"))
    .collect();
```

## Text Filtering

### Contains Keyword

```rust
let mentions_storm: Vec<_> = events.iter()
    .filter(|e| e.text.to_lowercase().contains("storm"))
    .collect();
```

### Regex Matching

```rust
use regex::Regex;

let phone_pattern = Regex::new(r"\d{3}-\d{3}-\d{4}").unwrap();

let with_phone: Vec<_> = events.iter()
    .filter(|e| phone_pattern.is_match(&e.text))
    .collect();
```

### Minimum Length

```rust
let substantive: Vec<_> = events.iter()
    .filter(|e| e.text.len() >= 50)
    .collect();
```

## Combined Filtering

### Chained Filters

```rust
let filtered: Vec<_> = events.iter()
    .filter(|e| nyc_bounds.contains(&e.location))
    .filter(|e| january.contains(&e.timestamp))
    .filter(|e| e.has_tag("verified"))
    .filter(|e| !e.has_tag("duplicate"))
    .collect();
```

### With Predicate Function

```rust
fn is_relevant(event: &Event) -> bool {
    // Complex filtering logic
    let in_region = NYC_BOUNDS.contains(&event.location);
    let in_timeframe = STUDY_PERIOD.contains(&event.timestamp);
    let has_content = event.text.len() >= 10;
    let is_verified = event.has_tag("verified");
    
    in_region && in_timeframe && has_content && is_verified
}

let relevant: Vec<_> = events.iter()
    .filter(|e| is_relevant(e))
    .collect();
```

## Using Indexes for Filtering

### Spatial Index

```rust
use spatial_narrative::index::SpatialIndex;

let index = SpatialIndex::from_iter(events.clone(), |e| &e.location);

// Fast bounding box query
let in_region = index.query_bounds(&bounds);

// Fast radius query
let nearby = index.query_radius_meters(lat, lon, radius);
```

### Temporal Index

```rust
use spatial_narrative::index::TemporalIndex;

let index = TemporalIndex::from_iter(events.clone(), |e| &e.timestamp);

// Fast range query
let in_period = index.query_range(&time_range);

// Ordered access
let chronological = index.chronological();
```

### Combined Index

```rust
use spatial_narrative::index::SpatiotemporalIndex;

let index = SpatiotemporalIndex::from_iter(
    events.clone(), 
    |e| &e.location, 
    |e| &e.timestamp
);

// Query both dimensions efficiently
let filtered = index.query(&bounds, &time_range);
```

## Sampling

### Random Sample

```rust
use rand::seq::SliceRandom;

let mut rng = rand::thread_rng();
let sample: Vec<_> = events.choose_multiple(&mut rng, 100).collect();
```

### Systematic Sample

```rust
// Every 10th event
let systematic: Vec<_> = events.iter()
    .enumerate()
    .filter(|(i, _)| i % 10 == 0)
    .map(|(_, e)| e)
    .collect();
```

### Stratified Sample

```rust
// Sample from each region
let mut samples = Vec::new();
for region in &regions {
    let in_region: Vec<_> = events.iter()
        .filter(|e| region.contains(&e.location))
        .collect();
    samples.extend(in_region.choose_multiple(&mut rng, 10));
}
```
