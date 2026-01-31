# Connection Strategies

Automatically connect events based on different criteria.

## Temporal Connection

Connect events in chronological order:

```rust
let mut graph = NarrativeGraph::from_events(events);

// Connect A → B if A happens before B
graph.connect_temporal();
```

This creates a chain:
```
Event 1 ─temporal→ Event 2 ─temporal→ Event 3 ─temporal→ ...
```

### How It Works

- Events are sorted by timestamp
- Each event connects to the next in sequence
- Creates `EdgeType::Temporal` edges
- Existing edges are not duplicated

## Spatial Connection

Connect events that are geographically close:

```rust
// Connect events within 10km of each other
graph.connect_spatial(10.0);

// Closer threshold for dense areas
graph.connect_spatial(1.0);  // 1km
```

### How It Works

- Compares all pairs of events
- Uses Haversine distance (great-circle)
- Creates bidirectional `EdgeType::Spatial` edges
- Edge weight = 1.0 - (distance / max_distance)

```rust
// Edge weight indicates proximity
// Weight 1.0 = same location
// Weight 0.5 = half the max distance apart
// Weight 0.0 = at max distance
```

## Thematic Connection

Connect events that share tags:

```rust
graph.connect_thematic();
```

### How It Works

- Events sharing one or more tags are connected
- Creates bidirectional `EdgeType::Thematic` edges
- Edge weight = shared_tags / max_tags

```rust
// Example: Event A has tags ["news", "politics"]
//          Event B has tags ["politics", "election"]
// Shared: 1 tag, Max: 2 tags
// Weight: 0.5
```

## Manual Connections

For relationships that can't be auto-detected:

```rust
use spatial_narrative::graph::{EdgeType, EdgeWeight};

// Get nodes
let cause = graph.get_node(&event1.id).unwrap();
let effect = graph.get_node(&event2.id).unwrap();

// Simple connection
graph.connect(cause, effect, EdgeType::Causal);

// With weight and label
let weight = EdgeWeight::with_weight(EdgeType::Reference, 0.9)
    .with_label("cites");
graph.connect_weighted(cause, effect, weight);
```

## Edge Types Reference

| Type | Auto-Connect | Direction | Weight Meaning |
|------|--------------|-----------|----------------|
| `Temporal` | `connect_temporal()` | Unidirectional | Always 1.0 |
| `Spatial` | `connect_spatial(km)` | Bidirectional | Proximity |
| `Thematic` | `connect_thematic()` | Bidirectional | Tag overlap |
| `Causal` | Manual | Unidirectional | Strength |
| `Reference` | Manual | Unidirectional | Relevance |
| `Custom` | Manual | Either | User-defined |

## Combining Strategies

Apply multiple connection strategies:

```rust
let mut graph = NarrativeGraph::from_events(events);

// Build a rich connection graph
graph.connect_temporal();      // Time sequence
graph.connect_spatial(5.0);    // 5km proximity
graph.connect_thematic();      // Shared topics

println!("Created {} connections", graph.edge_count());

// Analyze by type
let temporal = graph.edges_of_type(EdgeType::Temporal).len();
let spatial = graph.edges_of_type(EdgeType::Spatial).len();
let thematic = graph.edges_of_type(EdgeType::Thematic).len();

println!("  Temporal: {}", temporal);
println!("  Spatial: {}", spatial);
println!("  Thematic: {}", thematic);
```

## Selective Connection

Connect only certain events:

```rust
// First, add all events
let mut graph = NarrativeGraph::from_events(events);

// Connect only important events temporally
let important: Vec<_> = graph.nodes()
    .filter(|(_, e)| e.has_tag("important"))
    .map(|(id, _)| id)
    .collect();

// Sort by time and connect
let mut sorted: Vec<_> = important.iter()
    .filter_map(|&id| graph.event(id).map(|e| (id, e.timestamp.clone())))
    .collect();
sorted.sort_by(|a, b| a.1.cmp(&b.1));

for window in sorted.windows(2) {
    graph.connect(window[0].0, window[1].0, EdgeType::Temporal);
}
```

## Use Cases

### News Story Tracking

```rust
// Connect news events
graph.connect_temporal();     // Story progression
graph.connect_thematic();     // Related topics

// Find story threads
for root in graph.roots() {
    println!("Story starts: {}", graph.event(root).unwrap().text);
    let thread: Vec<_> = std::iter::successors(Some(root), |&n| {
        graph.successors(n).into_iter().next()
    }).collect();
    println!("  {} events in thread", thread.len());
}
```

### Location-Based Analysis

```rust
// Focus on spatial relationships
graph.connect_spatial(0.5);  // 500m - same block
graph.connect_spatial(2.0);  // 2km - neighborhood

// Find location clusters
let high_degree: Vec<_> = graph.nodes()
    .filter(|(id, _)| graph.in_degree(*id) + graph.out_degree(*id) > 5)
    .collect();

println!("Highly connected locations: {}", high_degree.len());
```

### Topic Networks

```rust
// Build topic network
graph.connect_thematic();

// Find central topics (most connected)
let mut connections: Vec<_> = graph.nodes()
    .map(|(id, _)| (id, graph.in_degree(id) + graph.out_degree(id)))
    .collect();
connections.sort_by(|a, b| b.1.cmp(&a.1));

println!("Most connected events:");
for (id, degree) in connections.iter().take(5) {
    let event = graph.event(*id).unwrap();
    println!("  {} ({} connections)", event.text, degree);
}
```
