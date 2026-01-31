# Narrative Comparison

Compare narratives to find similarities and differences.

## Basic Comparison

```rust
use spatial_narrative::analysis::{compare_narratives, ComparisonConfig};
use spatial_narrative::core::Narrative;

let similarity = compare_narratives(&narrative1, &narrative2, &ComparisonConfig::default());

println!("Similarity scores:");
println!("  Spatial: {:.2}", similarity.spatial);
println!("  Temporal: {:.2}", similarity.temporal);
println!("  Thematic: {:.2}", similarity.thematic);
println!("  Overall: {:.2}", similarity.overall);
```

## NarrativeSimilarity

The comparison returns a `NarrativeSimilarity` struct:

| Field | Type | Description |
|-------|------|-------------|
| `spatial` | `f64` | Geographic overlap (0.0 to 1.0) |
| `temporal` | `f64` | Time overlap (0.0 to 1.0) |
| `thematic` | `f64` | Tag/topic similarity (0.0 to 1.0) |
| `overall` | `f64` | Weighted average score |

Scores range from 0.0 (no similarity) to 1.0 (identical).

## ComparisonConfig

Customize how similarity is calculated:

```rust
use spatial_narrative::analysis::ComparisonConfig;

let config = ComparisonConfig {
    spatial_weight: 0.4,    // 40% weight on geography
    temporal_weight: 0.3,   // 30% weight on time
    thematic_weight: 0.3,   // 30% weight on themes
    spatial_threshold_km: 10.0,  // Events within 10km considered "same place"
    temporal_threshold_hours: 24.0,  // Events within 24h considered "same time"
};

let similarity = compare_narratives(&n1, &n2, &config);
```

## Spatial Comparison Functions

### Spatial Similarity

Calculate geographic overlap:

```rust
use spatial_narrative::analysis::spatial_similarity;

let score = spatial_similarity(&narrative1, &narrative2);
println!("Spatial similarity: {:.2}", score);  // 0.0 to 1.0
```

### Spatial Intersection

Find events that occur in similar locations:

```rust
use spatial_narrative::analysis::spatial_intersection;

// Events within 5km of each other
let matching_pairs = spatial_intersection(&narrative1, &narrative2, 5000.0);

for (e1, e2) in matching_pairs {
    println!("Match: '{}' and '{}'", e1.text, e2.text);
    println!("  Distance: {:.0}m apart", 
        e1.location.distance_to(&e2.location));
}
```

### Spatial Union

Get combined geographic bounds:

```rust
use spatial_narrative::analysis::spatial_union;

let combined_bounds = spatial_union(&narrative1, &narrative2);
println!("Combined area: {:.2}° x {:.2}°",
    combined_bounds.lat_span(),
    combined_bounds.lon_span());
```

## Temporal Comparison

### Temporal Similarity

```rust
use spatial_narrative::analysis::temporal_similarity;

let score = temporal_similarity(&narrative1, &narrative2);
println!("Temporal similarity: {:.2}", score);
```

## Thematic Comparison

### Thematic Similarity

Compare based on shared tags:

```rust
use spatial_narrative::analysis::thematic_similarity;

let score = thematic_similarity(&narrative1, &narrative2);
println!("Thematic similarity: {:.2}", score);
```

### Common Locations

Find locations that appear in both narratives:

```rust
use spatial_narrative::analysis::common_locations;

// Locations within 1km considered the same
let shared = common_locations(&narrative1, &narrative2, 1000.0);

println!("Common locations ({}):", shared.len());
for loc in shared {
    println!("  ({:.4}, {:.4})", loc.lat, loc.lon);
}
```

## Use Cases

### Finding Related Stories

```rust
let threshold = 0.5;  // 50% similarity threshold
let mut related = Vec::new();

for candidate in &all_narratives {
    let sim = compare_narratives(&target, candidate, &ComparisonConfig::default());
    if sim.overall > threshold {
        related.push((candidate, sim.overall));
    }
}

// Sort by similarity
related.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

println!("Related narratives:");
for (narrative, score) in related.iter().take(5) {
    println!("  {}: {:.0}% similar", 
        narrative.title.as_deref().unwrap_or("Untitled"),
        score * 100.0);
}
```

### Detecting Duplicate Reports

```rust
let config = ComparisonConfig {
    spatial_threshold_km: 0.1,      // 100m
    temporal_threshold_hours: 1.0,   // 1 hour
    ..Default::default()
};

for i in 0..narratives.len() {
    for j in (i + 1)..narratives.len() {
        let sim = compare_narratives(&narratives[i], &narratives[j], &config);
        
        if sim.overall > 0.9 {
            println!("Potential duplicate:");
            println!("  '{}' and '{}'",
                narratives[i].title.as_deref().unwrap_or("?"),
                narratives[j].title.as_deref().unwrap_or("?"));
            println!("  Similarity: {:.0}%", sim.overall * 100.0);
        }
    }
}
```

### Clustering Narratives by Topic

```rust
// Group narratives by thematic similarity
let mut groups: Vec<Vec<&Narrative>> = Vec::new();

for narrative in &narratives {
    let mut found_group = false;
    
    for group in &mut groups {
        // Check if similar to group representative
        let sim = thematic_similarity(narrative, group[0]);
        if sim > 0.6 {
            group.push(narrative);
            found_group = true;
            break;
        }
    }
    
    if !found_group {
        groups.push(vec![narrative]);
    }
}

println!("Found {} thematic groups", groups.len());
for (i, group) in groups.iter().enumerate() {
    println!("  Group {}: {} narratives", i + 1, group.len());
}
```
