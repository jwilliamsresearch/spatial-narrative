# Temporal Index (B-tree)

The `TemporalIndex` uses a B-tree for efficient time-based queries.

## Creating an Index

```rust
use spatial_narrative::index::TemporalIndex;
use spatial_narrative::core::{Event, Location, Timestamp};

// Empty index
let mut index: TemporalIndex<Event> = TemporalIndex::new();

// Insert items
let event = Event::new(
    Location::new(40.7128, -74.0060),
    Timestamp::parse("2024-01-15T10:00:00Z").unwrap(),
    "Morning meeting"
);
index.insert(event, &Timestamp::parse("2024-01-15T10:00:00Z").unwrap());
```

### From Iterator

```rust
let index = TemporalIndex::from_iter(
    events.iter().cloned(),
    |event| &event.timestamp
);
```

## Query Types

### Time Range Query

Find items within a time range:

```rust
use spatial_narrative::core::TimeRange;

let range = TimeRange::new(
    Timestamp::parse("2024-01-01T00:00:00Z").unwrap(),
    Timestamp::parse("2024-01-31T23:59:59Z").unwrap(),
);

let january_events = index.query_range(&range);
println!("Found {} events in January", january_events.len());
```

### Convenience Ranges

```rust
// Entire year
let results = index.query_range(&TimeRange::year(2024));

// Specific month
let results = index.query_range(&TimeRange::month(2024, 6));  // June 2024

// Specific day
let results = index.query_range(&TimeRange::day(2024, 7, 4));  // July 4th
```

### Before/After Queries

```rust
let cutoff = Timestamp::parse("2024-06-01T00:00:00Z").unwrap();

// All events before June
let before = index.before(&cutoff);

// All events after June
let after = index.after(&cutoff);

// Including the cutoff time
let at_or_before = index.at_or_before(&cutoff);
let at_or_after = index.at_or_after(&cutoff);
```

### First and Last

```rust
// Get earliest event
if let Some(first) = index.first() {
    println!("First event: {}", first.text);
}

// Get latest event
if let Some(last) = index.last() {
    println!("Last event: {}", last.text);
}
```

### Chronological Iteration

```rust
// Iterate in time order
for event in index.chronological() {
    println!("{}: {}", event.timestamp.to_rfc3339(), event.text);
}
```

### Reverse Chronological

```rust
// Most recent first
for event in index.reverse_chronological() {
    println!("{}: {}", event.timestamp.to_rfc3339(), event.text);
}
```

## Sliding Window

Iterate with a sliding time window:

```rust
use spatial_narrative::index::SlidingWindowIter;
use std::time::Duration;

let window_size = Duration::from_secs(3600);  // 1 hour
let step = Duration::from_secs(1800);         // 30 minute steps

for window in index.sliding_window(window_size, step) {
    println!("Window {}: {} events", 
        window.start.to_rfc3339(),
        window.events.len());
}
```

## Methods Reference

| Method | Description |
|--------|-------------|
| `new()` | Create empty index |
| `from_iter()` | Build from iterator |
| `insert(item, timestamp)` | Add an item |
| `query_range(range)` | Query time range |
| `before(timestamp)` | Items before (exclusive) |
| `after(timestamp)` | Items after (exclusive) |
| `at_or_before(timestamp)` | Items at or before (inclusive) |
| `at_or_after(timestamp)` | Items at or after (inclusive) |
| `first()` | Earliest item |
| `last()` | Latest item |
| `chronological()` | Time-ordered iterator |
| `reverse_chronological()` | Reverse time iterator |
| `sliding_window(size, step)` | Sliding window iterator |
| `time_range()` | Get overall time range |
| `len()` | Number of items |

## Use Cases

### Timeline Visualization

```rust
// Get events in order for display
let timeline: Vec<_> = index.chronological().collect();

for event in timeline {
    println!("{}: {}", 
        event.timestamp.format("%Y-%m-%d %H:%M"),
        event.text);
}
```

### Activity Analysis

```rust
use std::time::Duration;

// Count events per hour
let hour = Duration::from_secs(3600);
let mut hourly_counts = Vec::new();

for window in index.sliding_window(hour, hour) {
    hourly_counts.push((window.start, window.events.len()));
}

// Find busiest hour
if let Some((time, count)) = hourly_counts.iter().max_by_key(|(_, c)| c) {
    println!("Busiest hour: {} with {} events", 
        time.to_rfc3339(), count);
}
```

### Recent Events

```rust
use std::time::Duration;

// Get events from the last 24 hours
let now = Timestamp::now();
let yesterday = now.subtract(Duration::from_secs(86400));
let range = TimeRange::new(yesterday, now);

let recent = index.query_range(&range);
println!("{} events in the last 24 hours", recent.len());
```

### Gap Detection

```rust
// Find gaps between events
let ordered: Vec<_> = index.chronological().collect();

for window in ordered.windows(2) {
    let gap = window[1].timestamp.duration_since(&window[0].timestamp);
    if gap > Duration::from_secs(3600 * 6) {  // 6+ hour gap
        println!("Gap from {} to {}", 
            window[0].timestamp.to_rfc3339(),
            window[1].timestamp.to_rfc3339());
    }
}
```
