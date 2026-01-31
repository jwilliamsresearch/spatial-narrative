# Movement Analysis

Analyze movement patterns and detect significant locations.

## Trajectory

A `Trajectory` represents a path through space and time.

```rust
use spatial_narrative::analysis::Trajectory;
use spatial_narrative::core::{Event, Location, Timestamp};

let events = vec![
    Event::new(Location::new(40.7128, -74.0060), 
        Timestamp::parse("2024-01-01T10:00:00Z").unwrap(), "Start"),
    Event::new(Location::new(40.7480, -73.9850), 
        Timestamp::parse("2024-01-01T10:30:00Z").unwrap(), "Midtown"),
    Event::new(Location::new(40.7580, -73.9855), 
        Timestamp::parse("2024-01-01T11:00:00Z").unwrap(), "Times Square"),
];

let trajectory = Trajectory::from_events(&events);
```

### Trajectory Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `total_distance()` | `f64` | Total path length in meters |
| `duration()` | `Duration` | Time from start to end |
| `average_speed()` | `f64` | Average speed in m/s |
| `points()` | `&[TrajectoryPoint]` | All points on the path |
| `bounds()` | `GeoBounds` | Bounding box of path |

### Example Usage

```rust
println!("Trajectory analysis:");
println!("  Distance: {:.2} km", trajectory.total_distance() / 1000.0);
println!("  Duration: {:?}", trajectory.duration());
println!("  Avg speed: {:.1} km/h", trajectory.average_speed() * 3.6);

// Access individual segments
for (i, segment) in trajectory.segments().enumerate() {
    println!("  Segment {}: {:.0}m in {:?}", 
        i, segment.distance, segment.duration);
}
```

## Stop Detection

Identify locations where movement paused.

```rust
use spatial_narrative::analysis::{detect_stops, StopThreshold};
use std::time::Duration;

// Configure stop detection
let threshold = StopThreshold::new(
    50.0,                            // Max radius in meters
    Duration::from_secs(300),        // Min duration (5 minutes)
);

let stops = detect_stops(&events, &threshold);

for stop in &stops {
    println!("Stop at ({:.4}, {:.4})", stop.location.lat, stop.location.lon);
    println!("  Duration: {:?}", stop.duration);
    println!("  Events: {}", stop.events.len());
}
```

### StopThreshold

Configure stop detection sensitivity:

| Field | Type | Description |
|-------|------|-------------|
| `radius_meters` | `f64` | Maximum spread to consider a stop |
| `min_duration` | `Duration` | Minimum time to qualify as a stop |

### Stop Struct

Each detected stop contains:

| Field | Type | Description |
|-------|------|-------------|
| `location` | `Location` | Center of the stop |
| `start` | `Timestamp` | When the stop began |
| `end` | `Timestamp` | When the stop ended |
| `duration` | `Duration` | How long the stop lasted |
| `events` | `Vec<&Event>` | Events during the stop |

## Movement Analyzer

For more advanced movement analysis:

```rust
use spatial_narrative::analysis::MovementAnalyzer;

let analyzer = MovementAnalyzer::new(&events);

// Get movement statistics
let stats = analyzer.statistics();
println!("Movement statistics:");
println!("  Total distance: {:.2} km", stats.total_distance / 1000.0);
println!("  Moving time: {:?}", stats.moving_time);
println!("  Stopped time: {:?}", stats.stopped_time);
println!("  Max speed: {:.1} km/h", stats.max_speed * 3.6);

// Detect mode changes
let segments = analyzer.segment_by_speed(5.0); // 5 m/s threshold
for segment in segments {
    let mode = if segment.avg_speed > 5.0 { "vehicle" } else { "walking" };
    println!("  {} segment: {:.0}m", mode, segment.distance);
}
```

## Use Cases

### GPS Track Analysis

```rust
// Analyze a GPS track
let trajectory = Trajectory::from_events(&gps_points);
let stops = detect_stops(&gps_points, &StopThreshold::new(25.0, Duration::from_secs(120)));

println!("Track summary:");
println!("  Total distance: {:.2} km", trajectory.total_distance() / 1000.0);
println!("  Duration: {:?}", trajectory.duration());
println!("  Stops: {}", stops.len());

// Calculate moving vs stopped time
let stopped_time: Duration = stops.iter().map(|s| s.duration).sum();
let moving_time = trajectory.duration() - stopped_time;
println!("  Moving time: {:?}", moving_time);
println!("  Stopped time: {:?}", stopped_time);
```

### Delivery Route Analysis

```rust
// Identify delivery stops
let threshold = StopThreshold::new(
    30.0,                          // 30m radius (parking area)
    Duration::from_secs(60),       // At least 1 minute
);

let delivery_stops = detect_stops(&route_events, &threshold);

println!("Delivery route analysis:");
println!("  Total stops: {}", delivery_stops.len());
println!("  Route distance: {:.1} km", 
    Trajectory::from_events(&route_events).total_distance() / 1000.0);

// Average time per stop
if !delivery_stops.is_empty() {
    let avg_stop_time: Duration = delivery_stops.iter()
        .map(|s| s.duration)
        .sum::<Duration>() / delivery_stops.len() as u32;
    println!("  Avg stop time: {:?}", avg_stop_time);
}
```

### Anomaly Detection

```rust
let trajectory = Trajectory::from_events(&events);

// Find unusually fast segments (potential GPS errors)
for segment in trajectory.segments() {
    let speed_kmh = segment.speed() * 3.6;
    if speed_kmh > 200.0 {
        println!("Warning: Unrealistic speed {:.0} km/h detected", speed_kmh);
        println!("  From {} to {}", 
            segment.start.to_rfc3339(), 
            segment.end.to_rfc3339());
    }
}
```
