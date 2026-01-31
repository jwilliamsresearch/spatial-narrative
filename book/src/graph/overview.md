# Graph Overview

The `graph` module provides tools for representing narratives as directed graphs.

## Concepts

In a narrative graph:
- **Nodes** are events
- **Edges** are relationships between events
- **Edge types** describe the nature of relationships

## NarrativeGraph

```rust
use spatial_narrative::graph::{NarrativeGraph, EdgeType};
use spatial_narrative::core::{Event, Location, Timestamp};

// Create events
let e1 = Event::new(Location::new(40.7, -74.0), Timestamp::now(), "Event 1");
let e2 = Event::new(Location::new(40.7, -74.0), Timestamp::now(), "Event 2");

// Build graph
let mut graph = NarrativeGraph::new();
let n1 = graph.add_event(e1);
let n2 = graph.add_event(e2);

// Connect events
graph.connect(n1, n2, EdgeType::Temporal);

println!("Nodes: {}, Edges: {}", graph.node_count(), graph.edge_count());
```

## Edge Types

| Type | Description | Use Case |
|------|-------------|----------|
| `Temporal` | Time sequence | A happens before B |
| `Spatial` | Geographic proximity | A and B are near each other |
| `Causal` | Cause and effect | A causes B |
| `Thematic` | Shared themes/tags | A and B cover same topic |
| `Reference` | Citation/mention | A references B |
| `Custom` | User-defined | Domain-specific relationships |

## Quick Example

```rust
use spatial_narrative::graph::{NarrativeGraph, EdgeType};
use spatial_narrative::core::Narrative;

// Create from narrative
let mut graph = NarrativeGraph::from_events(narrative.events.clone());

// Auto-connect by different strategies
graph.connect_temporal();      // Connect events in time order
graph.connect_spatial(10.0);   // Connect events within 10km
graph.connect_thematic();      // Connect events sharing tags

// Query the graph
println!("Connected events: {}", graph.edge_count());

// Find paths
if let Some(path) = graph.shortest_path(node_a, node_b) {
    println!("Path length: {} nodes", path.nodes.len());
}
```

## Why Use Graphs?

Graphs enable powerful analysis:

- **Path finding**: How are two events connected?
- **Clustering**: Which events form natural groups?
- **Centrality**: Which events are most connected?
- **Subgraphs**: Extract related event clusters
- **Visualization**: Export to Graphviz, D3.js, etc.

## Module Contents

- [`NarrativeGraph`](./building.md) - Main graph structure
- [Connection strategies](./connections.md) - Auto-connecting events
- [Path finding](./pathfinding.md) - Shortest paths and connectivity
- [Export](./export.md) - DOT format for visualization

## Visualization Preview

Export to DOT and render with Graphviz:

```rust
let dot = graph.to_dot();
std::fs::write("graph.dot", dot)?;
// Run: dot -Tpng graph.dot -o graph.png
```

```
┌─────────┐  temporal  ┌─────────┐  spatial  ┌─────────┐
│ Event 1 │ ──────────▶│ Event 2 │ ──────────▶│ Event 3 │
└─────────┘            └─────────┘            └─────────┘
     │                      │                      
     │ thematic             │ causal              
     ▼                      ▼                     
┌─────────┐            ┌─────────┐               
│ Event 4 │            │ Event 5 │               
└─────────┘            └─────────┘               
```
