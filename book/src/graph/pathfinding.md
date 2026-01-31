# Path Finding

Find paths and analyze connectivity in narrative graphs.

## Connectivity Checks

### Direct Connection

```rust
// Are two nodes directly connected?
if graph.are_connected(node_a, node_b) {
    println!("A connects directly to B");
}
```

### Path Exists

```rust
// Is there any path from A to B?
if graph.has_path(node_a, node_b) {
    println!("A can reach B (directly or indirectly)");
} else {
    println!("No path from A to B");
}
```

## Shortest Path

Find the shortest path between two nodes:

```rust
if let Some(path) = graph.shortest_path(start, end) {
    println!("Path found:");
    println!("  Nodes: {}", path.nodes.len());
    println!("  Total weight: {:.2}", path.total_weight);
    
    // Print path
    for node_id in &path.nodes {
        let event = graph.event(*node_id).unwrap();
        println!("  → {}", event.text);
    }
} else {
    println!("No path exists");
}
```

### PathInfo

```rust
pub struct PathInfo {
    pub nodes: Vec<NodeId>,    // Nodes in order
    pub total_weight: f64,     // Sum of edge weights
}

impl PathInfo {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}
```

## Neighborhood Analysis

### Successors (Outgoing)

```rust
// Events this event leads to
let following = graph.successors(node);
for next in following {
    let event = graph.event(next).unwrap();
    println!("Leads to: {}", event.text);
}
```

### Predecessors (Incoming)

```rust
// Events that lead to this event
let previous = graph.predecessors(node);
for prev in previous {
    let event = graph.event(prev).unwrap();
    println!("Preceded by: {}", event.text);
}
```

## Graph Structure

### Entry Points (Roots)

Events with no predecessors - story starting points:

```rust
let roots = graph.roots();
println!("Story begins at {} events", roots.len());

for root in roots {
    let event = graph.event(root).unwrap();
    println!("  Start: {}", event.text);
}
```

### End Points (Leaves)

Events with no successors - story endings:

```rust
let leaves = graph.leaves();
println!("Story ends at {} events", leaves.len());

for leaf in leaves {
    let event = graph.event(leaf).unwrap();
    println!("  End: {}", event.text);
}
```

## Degree Analysis

```rust
// Incoming connections
let in_deg = graph.in_degree(node);

// Outgoing connections
let out_deg = graph.out_degree(node);

println!("Node has {} incoming, {} outgoing connections", in_deg, out_deg);

// Find highly connected nodes
for (node_id, event) in graph.nodes() {
    let total_degree = graph.in_degree(node_id) + graph.out_degree(node_id);
    if total_degree > 5 {
        println!("Hub: {} ({} connections)", event.text, total_degree);
    }
}
```

## Traversal Patterns

### Follow Timeline

```rust
// Follow temporal connections from a starting point
fn follow_timeline(graph: &NarrativeGraph, start: NodeId) -> Vec<NodeId> {
    let mut path = vec![start];
    let mut current = start;
    
    while let Some(next) = graph.successors(current)
        .into_iter()
        .filter(|&n| {
            graph.edges()
                .any(|(from, to, w)| from == current && to == n 
                    && w.edge_type == EdgeType::Temporal)
        })
        .next()
    {
        path.push(next);
        current = next;
    }
    
    path
}
```

### Find All Paths

```rust
// Find all paths between two nodes (BFS)
fn find_all_paths(
    graph: &NarrativeGraph, 
    start: NodeId, 
    end: NodeId,
    max_depth: usize
) -> Vec<Vec<NodeId>> {
    let mut paths = Vec::new();
    let mut queue = vec![(vec![start], 0)];
    
    while let Some((path, depth)) = queue.pop() {
        if depth > max_depth {
            continue;
        }
        
        let current = *path.last().unwrap();
        if current == end {
            paths.push(path);
            continue;
        }
        
        for next in graph.successors(current) {
            if !path.contains(&next) {
                let mut new_path = path.clone();
                new_path.push(next);
                queue.push((new_path, depth + 1));
            }
        }
    }
    
    paths
}
```

## Use Cases

### Story Thread Extraction

```rust
// Extract complete story threads from roots to leaves
for root in graph.roots() {
    let thread = follow_timeline(&graph, root);
    
    println!("Story thread ({} events):", thread.len());
    for node_id in &thread {
        let event = graph.event(*node_id).unwrap();
        println!("  {}: {}", event.timestamp.to_rfc3339(), event.text);
    }
}
```

### Finding Connection Between Events

```rust
// How are two events connected?
let event_a = graph.get_node(&event1.id).unwrap();
let event_b = graph.get_node(&event2.id).unwrap();

if let Some(path) = graph.shortest_path(event_a, event_b) {
    println!("Connection found via {} intermediate events:", path.len() - 2);
    
    for window in path.nodes.windows(2) {
        let from = graph.event(window[0]).unwrap();
        let to = graph.event(window[1]).unwrap();
        
        // Find edge type
        let edge_type = graph.edges()
            .find(|(f, t, _)| *f == window[0] && *t == window[1])
            .map(|(_, _, w)| w.edge_type);
        
        println!("  {} →[{:?}]→ {}", from.text, edge_type, to.text);
    }
}
```

### Hub Detection

```rust
// Find events that connect many other events
let mut hubs: Vec<_> = graph.nodes()
    .map(|(id, event)| {
        let degree = graph.in_degree(id) + graph.out_degree(id);
        (id, event, degree)
    })
    .collect();

hubs.sort_by(|a, b| b.2.cmp(&a.2));

println!("Top 5 hub events:");
for (_, event, degree) in hubs.iter().take(5) {
    println!("  {} ({} connections)", event.text, degree);
}
```
