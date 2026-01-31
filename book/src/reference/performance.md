# Performance

Performance characteristics and optimization tips.

## Complexity

### Indexing Operations

| Operation | SpatialIndex | TemporalIndex | SpatiotemporalIndex |
|-----------|--------------|---------------|---------------------|
| Insert | O(log n) | O(log n) | O(log n) |
| Build (bulk) | O(n log n) | O(n log n) | O(n log n) |
| Point query | O(log n) | O(log n) | O(log n) |
| Range query | O(log n + k) | O(log n + k) | O(log n + k) |
| K-nearest | O(k log n) | N/A | N/A |

Where n = number of items, k = number of results.

### Graph Operations

| Operation | Complexity |
|-----------|------------|
| Add node | O(1) |
| Add edge | O(1) |
| Get neighbors | O(degree) |
| Has path (BFS) | O(V + E) |
| Shortest path | O((V + E) log V) |
| connect_temporal() | O(n log n) |
| connect_spatial(d) | O(n²) |
| connect_thematic() | O(n² × tags) |

Where V = vertices, E = edges.

## Memory Usage

### Per-Event Overhead

| Component | Approximate Size |
|-----------|------------------|
| Event base | ~200 bytes |
| Location | ~40 bytes |
| Timestamp | ~32 bytes |
| EventId (UUID) | 16 bytes |
| Text (per char) | 1 byte |
| Tag (each) | ~24 bytes + string |

### Index Overhead

| Index | Overhead per item |
|-------|-------------------|
| SpatialIndex | ~64 bytes |
| TemporalIndex | ~48 bytes |
| SpatiotemporalIndex | ~112 bytes |

### Approximate Memory Formula

```
Memory (MB) ≈ events × (200 + avg_text_len + tags × 30) / 1_000_000
           + index_overhead
```

## Optimization Tips

### Bulk Loading

Use `from_iter` instead of repeated `insert`:

```rust
// ✅ Fast: O(n log n) bulk load
let index = SpatialIndex::from_iter(events, |e| &e.location);

// ❌ Slow: O(n log n) per insert
let mut index = SpatialIndex::new();
for event in events {
    index.insert(event, &event.location);
}
```

### Query Then Filter

Use indexes for initial filtering, then apply additional criteria:

```rust
// Get spatial candidates (fast)
let candidates = spatial_index.query_bounds(&bounds);

// Apply additional filters (linear)
let filtered: Vec<_> = candidates.into_iter()
    .filter(|e| e.has_tag("important"))
    .filter(|e| time_range.contains(&e.timestamp))
    .collect();
```

### Avoid O(n²) Operations

The `connect_spatial` and `connect_thematic` methods are O(n²):

```rust
// For large graphs, consider:

// 1. Sample events
let sample: Vec<_> = events.choose_multiple(&mut rng, 1000).collect();
let mut graph = NarrativeGraph::from_events(sample);
graph.connect_spatial(10.0);  // Now O(sample²)

// 2. Use spatial index for proximity
let index = SpatialIndex::from_iter(events.clone(), |e| &e.location);
for event in &events {
    let nearby = index.query_radius_meters(event.location.lat, event.location.lon, 1000.0);
    // Only connect to nearby events
}

// 3. Skip if graph is too large
if events.len() > 10_000 {
    println!("Skipping spatial connections (too large)");
} else {
    graph.connect_spatial(10.0);
}
```

### Lazy Evaluation

Build indexes only when needed:

```rust
struct LazyNarrative {
    events: Vec<Event>,
    spatial_index: Option<SpatialIndex<Event>>,
}

impl LazyNarrative {
    fn get_spatial_index(&mut self) -> &SpatialIndex<Event> {
        if self.spatial_index.is_none() {
            self.spatial_index = Some(SpatialIndex::from_iter(
                self.events.iter().cloned(),
                |e| &e.location
            ));
        }
        self.spatial_index.as_ref().unwrap()
    }
}
```

### Parallel Processing

Use rayon for parallel operations:

```rust
use rayon::prelude::*;

// Parallel filtering
let filtered: Vec<_> = events.par_iter()
    .filter(|e| expensive_check(e))
    .collect();

// Parallel analysis
let metrics: Vec<_> = narratives.par_iter()
    .map(|n| SpatialMetrics::from_events(&n.events))
    .collect();
```

## Benchmarks

### Index Query Performance

Tested on 100,000 events:

| Query Type | Time |
|------------|------|
| Spatial bbox (1% area) | ~50 μs |
| Spatial radius (1km) | ~80 μs |
| Temporal range (1 month) | ~40 μs |
| Combined (1% × 1 month) | ~100 μs |
| K-nearest (k=10) | ~120 μs |

### Graph Operations

| Operation | 1K events | 10K events | 100K events |
|-----------|-----------|------------|-------------|
| from_events | 1 ms | 10 ms | 100 ms |
| connect_temporal | 2 ms | 25 ms | 300 ms |
| connect_spatial(10km) | 50 ms | 5 s | too slow |
| shortest_path | < 1 ms | 5 ms | 50 ms |
| to_dot | 5 ms | 50 ms | 500 ms |

### I/O Performance

| Format | Export 10K events | Import 10K events |
|--------|-------------------|-------------------|
| JSON | 15 ms | 20 ms |
| GeoJSON | 20 ms | 25 ms |
| CSV | 10 ms | 15 ms |

## Memory Optimization

### String Interning

For repeated tags, consider interning:

```rust
use std::collections::HashSet;
use std::sync::Arc;

// Share common strings
let tag_pool: HashSet<Arc<str>> = HashSet::new();

fn intern_tag(pool: &mut HashSet<Arc<str>>, tag: &str) -> Arc<str> {
    if let Some(existing) = pool.get(tag) {
        existing.clone()
    } else {
        let new = Arc::from(tag);
        pool.insert(new.clone());
        new
    }
}
```

### Streaming Processing

For very large datasets, process in chunks:

```rust
use std::io::{BufRead, BufReader};

let file = File::open("huge_file.csv")?;
let reader = BufReader::new(file);

for chunk in reader.lines().chunks(10_000) {
    let events: Vec<Event> = chunk
        .filter_map(|line| parse_event(&line.ok()?).ok())
        .collect();
    
    // Process chunk
    process_events(&events);
}
```
