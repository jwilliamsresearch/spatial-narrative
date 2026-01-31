# Building Graphs

Create and populate narrative graphs.

## Creating a Graph

### Empty Graph

```rust
use spatial_narrative::graph::NarrativeGraph;

let mut graph = NarrativeGraph::new();
```

### From Events

```rust
use spatial_narrative::graph::NarrativeGraph;
use spatial_narrative::core::{Event, Location, Timestamp};

let events = vec![
    Event::new(Location::new(40.7, -74.0), Timestamp::now(), "Event 1"),
    Event::new(Location::new(40.8, -74.1), Timestamp::now(), "Event 2"),
];

let graph = NarrativeGraph::from_events(events);
println!("Graph has {} nodes", graph.node_count());
```

### From Narrative

```rust
let graph = NarrativeGraph::from_events(narrative.events.clone());
```

## Adding Events

```rust
use spatial_narrative::graph::NarrativeGraph;

let mut graph = NarrativeGraph::new();

// Add returns NodeId for later reference
let node1 = graph.add_event(event1);
let node2 = graph.add_event(event2);

// Look up node by event ID
let node = graph.get_node(&event.id);
```

## Connecting Events

### Basic Connection

```rust
use spatial_narrative::graph::EdgeType;

graph.connect(node1, node2, EdgeType::Temporal);
```

### Weighted Connection

```rust
use spatial_narrative::graph::EdgeWeight;

// Create weighted edge
let weight = EdgeWeight::with_weight(EdgeType::Spatial, 0.8);
graph.connect_weighted(node1, node2, weight);

// With label
let weight = EdgeWeight::new(EdgeType::Causal)
    .with_label("caused by");
graph.connect_weighted(node1, node2, weight);
```

## Accessing Nodes and Edges

### Get Event by Node

```rust
if let Some(event) = graph.event(node_id) {
    println!("Event: {}", event.text);
}

// Mutable access
if let Some(event) = graph.event_mut(node_id) {
    event.add_tag("processed");
}
```

### Iterate Nodes

```rust
for (node_id, event) in graph.nodes() {
    println!("Node {}: {}", node_id.index(), event.text);
}
```

### Iterate Edges

```rust
for (from, to, weight) in graph.edges() {
    println!("{} -> {} ({:?})", 
        from.index(), to.index(), weight.edge_type);
}
```

### Filter Edges by Type

```rust
let temporal_edges = graph.edges_of_type(EdgeType::Temporal);
println!("Temporal connections: {}", temporal_edges.len());
```

## Graph Properties

```rust
// Counts
println!("Nodes: {}", graph.node_count());
println!("Edges: {}", graph.edge_count());
println!("Empty: {}", graph.is_empty());

// Degree analysis
let in_degree = graph.in_degree(node);    // Incoming edges
let out_degree = graph.out_degree(node);  // Outgoing edges
println!("Node has {} in, {} out", in_degree, out_degree);
```

## Neighbors

```rust
// Get connected nodes
let successors = graph.successors(node);   // Outgoing connections
let predecessors = graph.predecessors(node);  // Incoming connections

for successor in successors {
    if let Some(event) = graph.event(successor) {
        println!("Leads to: {}", event.text);
    }
}
```

## Checking Connectivity

```rust
// Direct connection
if graph.are_connected(node1, node2) {
    println!("Directly connected");
}

// Any path exists
if graph.has_path(node1, node2) {
    println!("Path exists");
}
```

## Special Nodes

```rust
// Entry points (no predecessors)
let roots = graph.roots();
println!("Entry points: {}", roots.len());

// End points (no successors)
let leaves = graph.leaves();
println!("End points: {}", leaves.len());
```

## Subgraphs

Extract portions of the graph:

```rust
use spatial_narrative::core::{TimeRange, GeoBounds};

// By time
let january = TimeRange::month(2024, 1);
let subgraph = graph.subgraph_temporal(&january);
println!("January subgraph: {} nodes", subgraph.graph.node_count());

// By location
let nyc = GeoBounds::new(40.4, -74.3, 41.0, -73.7);
let subgraph = graph.subgraph_spatial(&nyc);
println!("NYC subgraph: {} nodes", subgraph.graph.node_count());
```

### SubgraphResult

```rust
let result = graph.subgraph_temporal(&range);

// The new graph
let new_graph = result.graph;

// Mapping from old to new node IDs
for (old_id, new_id) in result.node_mapping {
    println!("Node {} -> {}", old_id.index(), new_id.index());
}
```

## Example: Building a Story Graph

```rust
// Create graph from news events
let mut graph = NarrativeGraph::from_events(news_events);

// Connect by time (earlier → later)
graph.connect_temporal();

// Connect nearby events (within 1km)
graph.connect_spatial(1.0);

// Connect events sharing topics
graph.connect_thematic();

// Add manual causal links
let cause = graph.get_node(&cause_event.id).unwrap();
let effect = graph.get_node(&effect_event.id).unwrap();
graph.connect(cause, effect, EdgeType::Causal);

println!("Story graph: {} events, {} connections",
    graph.node_count(), graph.edge_count());
```
