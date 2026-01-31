# Analysis Overview

The `analysis` module provides tools for extracting insights from spatial narratives.

## Features

| Feature | Description | Types |
|---------|-------------|-------|
| **Spatial Metrics** | Geographic extent, distances, dispersion | [`SpatialMetrics`] |
| **Temporal Metrics** | Duration, rates, gaps, bursts | [`TemporalMetrics`] |
| **Movement** | Trajectory extraction and stop detection | [`Trajectory`], [`Stop`] |
| **Clustering** | Density and centroid-based clustering | [`DBSCAN`], [`KMeans`] |
| **Comparison** | Similarity scoring between narratives | [`compare_narratives`] |

## Quick Examples

### Spatial Metrics

```rust
use spatial_narrative::analysis::SpatialMetrics;
use spatial_narrative::core::{Event, Location, Timestamp};

let events = vec![
    Event::new(Location::new(40.7128, -74.0060), Timestamp::now(), "NYC"),
    Event::new(Location::new(34.0522, -118.2437), Timestamp::now(), "LA"),
    Event::new(Location::new(41.8781, -87.6298), Timestamp::now(), "Chicago"),
];

let metrics = SpatialMetrics::from_events(&events);

println!("Total distance: {:.0} km", metrics.total_distance / 1000.0);
println!("Geographic extent: {:.0} km", metrics.max_extent / 1000.0);
println!("Centroid: ({:.4}, {:.4})", metrics.centroid.0, metrics.centroid.1);
```

### Temporal Metrics

```rust
use spatial_narrative::analysis::TemporalMetrics;

let metrics = TemporalMetrics::from_events(&events);

println!("Duration: {:?}", metrics.duration);
println!("Event count: {}", metrics.event_count);
println!("Average interval: {:?}", metrics.avg_interval);
```

### DBSCAN Clustering

```rust
use spatial_narrative::analysis::DBSCAN;

// Cluster events within 5km, requiring 2+ points per cluster
let dbscan = DBSCAN::new(5000.0, 2);
let result = dbscan.cluster(&events);

println!("Found {} clusters", result.num_clusters());
println!("Noise points: {}", result.noise.len());
```

### K-Means Clustering

```rust
use spatial_narrative::analysis::KMeans;

// Partition events into 3 clusters
let kmeans = KMeans::new(3);
let result = kmeans.cluster(&events);

for (i, cluster) in result.clusters.iter().enumerate() {
    println!("Cluster {}: {} events", i, cluster.events.len());
}
```

### Trajectory Analysis

```rust
use spatial_narrative::analysis::{Trajectory, detect_stops, StopThreshold};

// Extract trajectory from events
let trajectory = Trajectory::from_events(&events);

println!("Trajectory length: {:.0} m", trajectory.total_distance());
println!("Duration: {:?}", trajectory.duration());

// Detect stops (stationary periods)
let threshold = StopThreshold::new(50.0, std::time::Duration::from_secs(300));
let stops = detect_stops(&events, &threshold);

for stop in stops {
    println!("Stop at ({:.4}, {:.4}) for {:?}", 
        stop.location.lat, stop.location.lon, stop.duration);
}
```

### Comparing Narratives

```rust
use spatial_narrative::analysis::{compare_narratives, ComparisonConfig};
use spatial_narrative::core::Narrative;

let similarity = compare_narratives(&narrative1, &narrative2, &ComparisonConfig::default());

println!("Spatial similarity: {:.2}", similarity.spatial);
println!("Temporal similarity: {:.2}", similarity.temporal);
println!("Thematic similarity: {:.2}", similarity.thematic);
println!("Overall similarity: {:.2}", similarity.overall);
```

## Module Structure

```
spatial_narrative::analysis
├── SpatialMetrics      # Geographic analysis
├── TemporalMetrics     # Time-based analysis
├── Trajectory          # Movement paths
├── DBSCAN              # Density clustering
├── KMeans              # K-means clustering
├── detect_stops        # Stop detection
├── detect_gaps         # Gap detection
├── detect_bursts       # Burst detection
└── compare_narratives  # Narrative comparison
```

## Next Steps

- [Spatial Metrics](./spatial.md) - Geographic analysis
- [Temporal Metrics](./temporal.md) - Time-based analysis
- [Movement Analysis](./movement.md) - Trajectories and stops
- [Clustering](./clustering.md) - Event grouping
- [Comparison](./comparison.md) - Narrative similarity
