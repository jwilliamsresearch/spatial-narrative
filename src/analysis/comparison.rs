//! Narrative comparison and similarity analysis.
//!
//! Provides tools for comparing narratives based on spatial,
//! temporal, and thematic properties.

use crate::analysis::haversine_distance;
use crate::core::{Event, GeoBounds, Narrative, TimeRange};

/// Similarity scores between two narratives.
#[derive(Debug, Clone)]
pub struct NarrativeSimilarity {
    /// Overall similarity score (0.0 to 1.0).
    pub overall: f64,
    /// Spatial similarity score (0.0 to 1.0).
    pub spatial: f64,
    /// Temporal similarity score (0.0 to 1.0).
    pub temporal: f64,
    /// Tag/thematic similarity score (0.0 to 1.0).
    pub thematic: f64,
}

/// Configuration for narrative comparison.
#[derive(Debug, Clone)]
pub struct ComparisonConfig {
    /// Weight for spatial similarity in overall score.
    pub spatial_weight: f64,
    /// Weight for temporal similarity in overall score.
    pub temporal_weight: f64,
    /// Weight for thematic similarity in overall score.
    pub thematic_weight: f64,
    /// Maximum distance (meters) to consider locations as "same".
    pub location_threshold_m: f64,
}

impl Default for ComparisonConfig {
    fn default() -> Self {
        Self {
            spatial_weight: 0.4,
            temporal_weight: 0.4,
            thematic_weight: 0.2,
            location_threshold_m: 1000.0, // 1 km
        }
    }
}

/// Compare two narratives and compute similarity.
///
/// # Examples
///
/// ```
/// use spatial_narrative::core::{Event, Location, Timestamp, Narrative, NarrativeBuilder};
/// use spatial_narrative::analysis::{compare_narratives, ComparisonConfig};
///
/// let events1 = vec![
///     Event::new(Location::new(40.0, -74.0), Timestamp::parse("2024-01-01T10:00:00Z").unwrap(), "A"),
/// ];
/// let events2 = vec![
///     Event::new(Location::new(40.001, -74.001), Timestamp::parse("2024-01-01T11:00:00Z").unwrap(), "B"),
/// ];
///
/// let n1 = NarrativeBuilder::new().events(events1).build();
/// let n2 = NarrativeBuilder::new().events(events2).build();
///
/// let similarity = compare_narratives(&n1, &n2, &ComparisonConfig::default());
/// assert!(similarity.spatial > 0.0);
/// ```
pub fn compare_narratives(
    n1: &Narrative,
    n2: &Narrative,
    config: &ComparisonConfig,
) -> NarrativeSimilarity {
    let spatial = spatial_similarity(n1.events(), n2.events(), config.location_threshold_m);
    let temporal = temporal_similarity(n1.events(), n2.events());
    let thematic = thematic_similarity(n1.events(), n2.events());

    let total_weight = config.spatial_weight + config.temporal_weight + config.thematic_weight;
    let overall = if total_weight > 0.0 {
        (spatial * config.spatial_weight
            + temporal * config.temporal_weight
            + thematic * config.thematic_weight)
            / total_weight
    } else {
        0.0
    };

    NarrativeSimilarity {
        overall,
        spatial,
        temporal,
        thematic,
    }
}

/// Compute spatial similarity between two event sets.
///
/// Based on Jaccard-like overlap of locations within threshold.
pub fn spatial_similarity(events1: &[Event], events2: &[Event], threshold_m: f64) -> f64 {
    if events1.is_empty() || events2.is_empty() {
        return 0.0;
    }

    // Count how many events in e1 have a nearby event in e2
    let mut matches = 0;
    for e1 in events1 {
        for e2 in events2 {
            let dist = haversine_distance(
                e1.location.lat,
                e1.location.lon,
                e2.location.lat,
                e2.location.lon,
            );
            if dist <= threshold_m {
                matches += 1;
                break;
            }
        }
    }

    // Jaccard-like: matches / union
    let union = events1.len() + events2.len() - matches;
    if union > 0 {
        matches as f64 / union as f64
    } else {
        0.0
    }
}

/// Compute temporal similarity between two event sets.
///
/// Based on overlap of time ranges.
pub fn temporal_similarity(events1: &[Event], events2: &[Event]) -> f64 {
    let range1 = match compute_time_range(events1) {
        Some(r) => r,
        None => return 0.0,
    };

    let range2 = match compute_time_range(events2) {
        Some(r) => r,
        None => return 0.0,
    };

    let start1 = range1.start.to_unix_millis();
    let end1 = range1.end.to_unix_millis();
    let start2 = range2.start.to_unix_millis();
    let end2 = range2.end.to_unix_millis();

    // Compute overlap
    let overlap_start = start1.max(start2);
    let overlap_end = end1.min(end2);

    if overlap_start >= overlap_end {
        return 0.0; // No overlap
    }

    let overlap = (overlap_end - overlap_start) as f64;
    let union = ((end1.max(end2)) - (start1.min(start2))) as f64;

    if union > 0.0 {
        overlap / union
    } else {
        0.0
    }
}

/// Compute thematic similarity between two event sets.
///
/// Based on Jaccard similarity of tags.
pub fn thematic_similarity(events1: &[Event], events2: &[Event]) -> f64 {
    use std::collections::HashSet;

    let tags1: HashSet<_> = events1.iter().flat_map(|e| e.tags.iter()).collect();
    let tags2: HashSet<_> = events2.iter().flat_map(|e| e.tags.iter()).collect();

    if tags1.is_empty() && tags2.is_empty() {
        return 0.0;
    }

    let intersection = tags1.intersection(&tags2).count();
    let union = tags1.union(&tags2).count();

    if union > 0 {
        intersection as f64 / union as f64
    } else {
        0.0
    }
}

fn compute_time_range(events: &[Event]) -> Option<TimeRange> {
    if events.is_empty() {
        return None;
    }

    let mut min_ts = events[0].timestamp.to_unix_millis();
    let mut max_ts = min_ts;

    for event in events.iter().skip(1) {
        let ts = event.timestamp.to_unix_millis();
        min_ts = min_ts.min(ts);
        max_ts = max_ts.max(ts);
    }

    use crate::core::Timestamp;
    Some(TimeRange::new(
        Timestamp::from_unix_millis(min_ts)?,
        Timestamp::from_unix_millis(max_ts)?,
    ))
}

/// Find events that occur near the same location in both narratives.
///
/// Returns pairs of event indices (index in n1, index in n2).
pub fn common_locations(
    n1: &Narrative,
    n2: &Narrative,
    threshold_m: f64,
) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();

    for (i, e1) in n1.events().iter().enumerate() {
        for (j, e2) in n2.events().iter().enumerate() {
            let dist = haversine_distance(
                e1.location.lat,
                e1.location.lon,
                e2.location.lat,
                e2.location.lon,
            );
            if dist <= threshold_m {
                pairs.push((i, j));
            }
        }
    }

    pairs
}

/// Find the geographic intersection of two narratives.
///
/// Returns events from n1 that have a nearby event in n2.
pub fn spatial_intersection<'a>(
    n1: &'a Narrative,
    n2: &Narrative,
    threshold_m: f64,
) -> Vec<&'a Event> {
    n1.events()
        .iter()
        .filter(|e1| {
            n2.events().iter().any(|e2| {
                haversine_distance(
                    e1.location.lat,
                    e1.location.lon,
                    e2.location.lat,
                    e2.location.lon,
                ) <= threshold_m
            })
        })
        .collect()
}

/// Compute the geographic union bounding box of two narratives.
pub fn spatial_union(n1: &Narrative, n2: &Narrative) -> Option<GeoBounds> {
    let bounds1 = compute_bounds(n1.events());
    let bounds2 = compute_bounds(n2.events());

    match (bounds1, bounds2) {
        (Some(b1), Some(b2)) => Some(GeoBounds::new(
            b1.min_lat.min(b2.min_lat),
            b1.max_lat.max(b2.max_lat),
            b1.min_lon.min(b2.min_lon),
            b1.max_lon.max(b2.max_lon),
        )),
        (Some(b), None) | (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn compute_bounds(events: &[Event]) -> Option<GeoBounds> {
    if events.is_empty() {
        return None;
    }

    let mut min_lat = f64::MAX;
    let mut max_lat = f64::MIN;
    let mut min_lon = f64::MAX;
    let mut max_lon = f64::MIN;

    for event in events {
        min_lat = min_lat.min(event.location.lat);
        max_lat = max_lat.max(event.location.lat);
        min_lon = min_lon.min(event.location.lon);
        max_lon = max_lon.max(event.location.lon);
    }

    Some(GeoBounds::new(min_lat, max_lat, min_lon, max_lon))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Location, NarrativeBuilder, Timestamp};

    fn make_event(lat: f64, lon: f64, time: &str, tags: &[&str]) -> Event {
        let mut event = Event::new(
            Location::new(lat, lon),
            Timestamp::parse(time).unwrap(),
            "test",
        );
        for tag in tags {
            event.tags.push((*tag).to_string());
        }
        event
    }

    #[test]
    fn test_spatial_similarity_identical() {
        let events1 = vec![make_event(40.0, -74.0, "2024-01-01T10:00:00Z", &[])];
        let events2 = vec![make_event(40.0, -74.0, "2024-01-01T10:00:00Z", &[])];

        let sim = spatial_similarity(&events1, &events2, 100.0);
        assert!((sim - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_spatial_similarity_far_apart() {
        let events1 = vec![make_event(40.0, -74.0, "2024-01-01T10:00:00Z", &[])];
        let events2 = vec![make_event(50.0, -80.0, "2024-01-01T10:00:00Z", &[])];

        let sim = spatial_similarity(&events1, &events2, 100.0);
        assert!(sim < 0.01);
    }

    #[test]
    fn test_temporal_similarity_overlapping() {
        let events1 = vec![
            make_event(0.0, 0.0, "2024-01-01T10:00:00Z", &[]),
            make_event(0.0, 0.0, "2024-01-01T12:00:00Z", &[]),
        ];
        let events2 = vec![
            make_event(0.0, 0.0, "2024-01-01T11:00:00Z", &[]),
            make_event(0.0, 0.0, "2024-01-01T13:00:00Z", &[]),
        ];

        let sim = temporal_similarity(&events1, &events2);
        assert!(sim > 0.0);
        assert!(sim < 1.0);
    }

    #[test]
    fn test_temporal_similarity_no_overlap() {
        let events1 = vec![make_event(0.0, 0.0, "2024-01-01T10:00:00Z", &[])];
        let events2 = vec![make_event(0.0, 0.0, "2024-02-01T10:00:00Z", &[])];

        let sim = temporal_similarity(&events1, &events2);
        assert!(sim < 0.01);
    }

    #[test]
    fn test_thematic_similarity() {
        let events1 = vec![make_event(0.0, 0.0, "2024-01-01T10:00:00Z", &["politics", "protest"])];
        let events2 = vec![make_event(0.0, 0.0, "2024-01-01T10:00:00Z", &["protest", "march"])];

        let sim = thematic_similarity(&events1, &events2);
        assert!(sim > 0.0);
        assert!(sim < 1.0);
    }

    #[test]
    fn test_compare_narratives() {
        let events1 = vec![make_event(40.0, -74.0, "2024-01-01T10:00:00Z", &["news"])];
        let events2 = vec![make_event(40.001, -74.001, "2024-01-01T11:00:00Z", &["news"])];

        let n1 = NarrativeBuilder::new().events(events1).build();
        let n2 = NarrativeBuilder::new().events(events2).build();

        let config = ComparisonConfig::default();
        let sim = compare_narratives(&n1, &n2, &config);

        assert!(sim.overall > 0.0);
        assert!(sim.spatial > 0.0);
        assert!(sim.thematic > 0.0);
    }

    #[test]
    fn test_common_locations() {
        let events1 = vec![
            make_event(40.0, -74.0, "2024-01-01T10:00:00Z", &[]),
            make_event(50.0, -80.0, "2024-01-01T11:00:00Z", &[]),
        ];
        let events2 = vec![make_event(40.001, -74.001, "2024-01-01T12:00:00Z", &[])];

        let n1 = NarrativeBuilder::new().events(events1).build();
        let n2 = NarrativeBuilder::new().events(events2).build();

        let pairs = common_locations(&n1, &n2, 1000.0);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0], (0, 0));
    }

    #[test]
    fn test_spatial_intersection() {
        let events1 = vec![
            make_event(40.0, -74.0, "2024-01-01T10:00:00Z", &[]),
            make_event(50.0, -80.0, "2024-01-01T11:00:00Z", &[]),
        ];
        let events2 = vec![make_event(40.001, -74.001, "2024-01-01T12:00:00Z", &[])];

        let n1 = NarrativeBuilder::new().events(events1).build();
        let n2 = NarrativeBuilder::new().events(events2).build();

        let intersection = spatial_intersection(&n1, &n2, 1000.0);
        assert_eq!(intersection.len(), 1);
    }
}
