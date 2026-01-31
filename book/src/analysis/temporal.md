# Temporal Metrics

Analyze the time-based characteristics of events.

## TemporalMetrics

The `TemporalMetrics` struct provides time-based measurements for a set of events.

```rust
use spatial_narrative::analysis::TemporalMetrics;
use spatial_narrative::core::{Event, Location, Timestamp};

let events = vec![
    Event::new(Location::new(40.7, -74.0), 
        Timestamp::parse("2024-01-01T10:00:00Z").unwrap(), "Morning"),
    Event::new(Location::new(40.7, -74.0), 
        Timestamp::parse("2024-01-01T14:00:00Z").unwrap(), "Afternoon"),
    Event::new(Location::new(40.7, -74.0), 
        Timestamp::parse("2024-01-02T09:00:00Z").unwrap(), "Next day"),
];

let metrics = TemporalMetrics::from_events(&events);
```

### Available Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `duration` | `Duration` | Time span from first to last event |
| `event_count` | `usize` | Number of events |
| `avg_interval` | `Duration` | Average time between events |
| `min_interval` | `Duration` | Shortest time between events |
| `max_interval` | `Duration` | Longest time between events |
| `start` | `Timestamp` | First event timestamp |
| `end` | `Timestamp` | Last event timestamp |

### Example Output

```rust
println!("Temporal metrics for {} events:", metrics.event_count);
println!("  Duration: {:?}", metrics.duration);
println!("  Average interval: {:?}", metrics.avg_interval);
println!("  Min interval: {:?}", metrics.min_interval);
println!("  Max interval: {:?}", metrics.max_interval);
println!("  Time range: {} to {}", 
    metrics.start.to_rfc3339(), 
    metrics.end.to_rfc3339());
```

## Event Rate

Calculate the rate of events over time:

```rust
use spatial_narrative::analysis::event_rate;
use std::time::Duration;

// Events per hour
let hourly_rate = event_rate(&events, Duration::from_secs(3600));
println!("Events per hour: {:.2}", hourly_rate);

// Events per day
let daily_rate = event_rate(&events, Duration::from_secs(86400));
println!("Events per day: {:.2}", daily_rate);
```

## Gap Detection

Find significant gaps in the event timeline:

```rust
use spatial_narrative::analysis::detect_gaps;
use std::time::Duration;

// Find gaps longer than 1 hour
let min_gap = Duration::from_secs(3600);
let gaps = detect_gaps(&events, min_gap);

for gap in gaps {
    println!("Gap from {} to {} ({:?})",
        gap.start.to_rfc3339(),
        gap.end.to_rfc3339(),
        gap.duration);
}
```

### Gap Struct

Each detected gap contains:

| Field | Type | Description |
|-------|------|-------------|
| `start` | `Timestamp` | End of previous event |
| `end` | `Timestamp` | Start of next event |
| `duration` | `Duration` | Length of the gap |

## Burst Detection

Identify periods of high event activity:

```rust
use spatial_narrative::analysis::detect_bursts;
use std::time::Duration;

// Find bursts with 5+ events in 1-hour windows
let window = Duration::from_secs(3600);
let min_events = 5;
let bursts = detect_bursts(&events, window, min_events);

for burst in bursts {
    println!("Burst: {} events from {} to {}",
        burst.count,
        burst.start.to_rfc3339(),
        burst.end.to_rfc3339());
}
```

## Time Binning

Group events into time bins:

```rust
use spatial_narrative::analysis::TimeBin;
use std::time::Duration;

// Group by hour
let bins = TimeBin::from_events(&events, Duration::from_secs(3600));

for bin in bins {
    println!("{}: {} events", bin.start.to_rfc3339(), bin.count);
}
```

## Use Cases

### Activity Pattern Analysis

```rust
let metrics = TemporalMetrics::from_events(&events);

// Calculate activity intensity
let events_per_hour = metrics.event_count as f64 
    / metrics.duration.as_secs_f64() * 3600.0;

if events_per_hour > 10.0 {
    println!("High activity: {:.1} events/hour", events_per_hour);
} else if events_per_hour > 1.0 {
    println!("Moderate activity: {:.1} events/hour", events_per_hour);
} else {
    println!("Low activity: {:.2} events/hour", events_per_hour);
}
```

### Finding Quiet Periods

```rust
use std::time::Duration;

// Gaps longer than 6 hours
let quiet_periods = detect_gaps(&events, Duration::from_secs(6 * 3600));

println!("Found {} quiet periods (>6 hours):", quiet_periods.len());
for gap in quiet_periods {
    println!("  {:?} gap starting {}", 
        gap.duration, 
        gap.start.to_rfc3339());
}
```

### Rush Hour Detection

```rust
use std::time::Duration;

// Detect 30-minute windows with 10+ events
let rush_periods = detect_bursts(&events, Duration::from_secs(1800), 10);

if !rush_periods.is_empty() {
    println!("Detected {} rush periods", rush_periods.len());
    for rush in rush_periods {
        println!("  {} events in 30min window at {}", 
            rush.count, 
            rush.start.to_rfc3339());
    }
}
```
