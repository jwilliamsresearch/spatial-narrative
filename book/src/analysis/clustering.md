# Clustering

Group events based on spatial proximity.

## DBSCAN

Density-Based Spatial Clustering of Applications with Noise.

DBSCAN finds clusters of arbitrary shape and identifies outliers (noise).

```rust
use spatial_narrative::analysis::DBSCAN;
use spatial_narrative::core::{Event, Location, Timestamp};

let events = vec![
    // Cluster 1: NYC area
    Event::new(Location::new(40.7128, -74.0060), Timestamp::now(), "NYC 1"),
    Event::new(Location::new(40.7138, -74.0050), Timestamp::now(), "NYC 2"),
    Event::new(Location::new(40.7118, -74.0070), Timestamp::now(), "NYC 3"),
    // Cluster 2: LA area  
    Event::new(Location::new(34.0522, -118.2437), Timestamp::now(), "LA 1"),
    Event::new(Location::new(34.0532, -118.2427), Timestamp::now(), "LA 2"),
    // Noise: isolated point
    Event::new(Location::new(41.8781, -87.6298), Timestamp::now(), "Chicago"),
];

// Create DBSCAN with 1km radius and minimum 2 points per cluster
let dbscan = DBSCAN::new(1000.0, 2);
let result = dbscan.cluster(&events);

println!("Found {} clusters", result.num_clusters());
println!("Noise points: {}", result.noise.len());
```

### DBSCAN Parameters

| Parameter | Description |
|-----------|-------------|
| `eps` | Maximum distance (meters) between points in a cluster |
| `min_points` | Minimum points required to form a cluster |

### Choosing Parameters

```rust
// Dense urban data: small radius, more points
let urban = DBSCAN::new(500.0, 5);    // 500m, 5+ points

// Sparse regional data: larger radius, fewer points  
let regional = DBSCAN::new(10_000.0, 3);  // 10km, 3+ points

// Very dense data (GPS tracks): tight clustering
let gps = DBSCAN::new(50.0, 10);      // 50m, 10+ points
```

### ClusteringResult

```rust
// Iterate over clusters
for (i, cluster) in result.clusters.iter().enumerate() {
    println!("Cluster {}:", i);
    println!("  Events: {}", cluster.events.len());
    println!("  Center: ({:.4}, {:.4})", 
        cluster.centroid.0, cluster.centroid.1);
    
    for event in &cluster.events {
        println!("    - {}", event.text);
    }
}

// Handle noise points
if !result.noise.is_empty() {
    println!("Noise points (unclustered):");
    for event in &result.noise {
        println!("  - {} at ({:.4}, {:.4})", 
            event.text, event.location.lat, event.location.lon);
    }
}
```

## K-Means

Partition events into exactly K clusters.

```rust
use spatial_narrative::analysis::KMeans;

// Partition into 3 clusters
let kmeans = KMeans::new(3);
let result = kmeans.cluster(&events);

for (i, cluster) in result.clusters.iter().enumerate() {
    println!("Cluster {} ({} events):", i, cluster.events.len());
    println!("  Centroid: ({:.4}, {:.4})", 
        cluster.centroid.0, cluster.centroid.1);
}
```

### K-Means Parameters

| Parameter | Description |
|-----------|-------------|
| `k` | Number of clusters to create |
| `max_iterations` | Maximum iterations (default: 100) |

### Choosing K

```rust
// Try different values of K and evaluate
for k in 2..=5 {
    let kmeans = KMeans::new(k);
    let result = kmeans.cluster(&events);
    
    // Calculate average cluster size
    let avg_size = events.len() as f64 / k as f64;
    println!("K={}: avg cluster size = {:.1}", k, avg_size);
}
```

## Cluster Struct

Both algorithms return `Cluster` objects:

| Field | Type | Description |
|-------|------|-------------|
| `events` | `Vec<&Event>` | Events in the cluster |
| `centroid` | `(f64, f64)` | Geographic center (lat, lon) |
| `bounds` | `GeoBounds` | Bounding box |

## When to Use Which

| Algorithm | Best For |
|-----------|----------|
| **DBSCAN** | Unknown number of clusters, irregular shapes, noise handling |
| **K-Means** | Known number of clusters, roughly equal-sized groups |

## Use Cases

### Finding Activity Hotspots

```rust
let dbscan = DBSCAN::new(1000.0, 5);  // 1km, 5+ events
let result = dbscan.cluster(&events);

// Sort clusters by size
let mut clusters: Vec<_> = result.clusters.iter().collect();
clusters.sort_by(|a, b| b.events.len().cmp(&a.events.len()));

println!("Top activity hotspots:");
for (i, cluster) in clusters.iter().take(5).enumerate() {
    println!("  {}. {} events at ({:.4}, {:.4})",
        i + 1, cluster.events.len(),
        cluster.centroid.0, cluster.centroid.1);
}
```

### Regional Grouping

```rust
// Group events into regions for separate analysis
let kmeans = KMeans::new(4);  // 4 regions
let result = kmeans.cluster(&events);

for (i, cluster) in result.clusters.iter().enumerate() {
    let region_name = match i {
        0 => "Northeast",
        1 => "Southeast", 
        2 => "Midwest",
        3 => "West",
        _ => "Unknown",
    };
    
    println!("{} region: {} events", region_name, cluster.events.len());
}
```

### Outlier Detection

```rust
let dbscan = DBSCAN::new(5000.0, 2);
let result = dbscan.cluster(&events);

// Noise points are potential outliers or unique events
if !result.noise.is_empty() {
    println!("Potential outliers ({}):", result.noise.len());
    for event in &result.noise {
        println!("  - {} at ({:.4}, {:.4})", 
            event.text, 
            event.location.lat, 
            event.location.lon);
    }
}
```
