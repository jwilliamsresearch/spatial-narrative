//! Spatial clustering algorithms for narrative analysis.
//!
//! Provides DBSCAN and k-means clustering implementations
//! for grouping events by geographic location.

use crate::analysis::haversine_distance;
use crate::core::{Event, GeoBounds, Location};
use std::collections::HashSet;

/// A cluster of events.
#[derive(Debug, Clone)]
pub struct Cluster {
    /// Cluster identifier (0-indexed).
    pub id: usize,
    /// Indices of events in this cluster.
    pub event_indices: Vec<usize>,
    /// Centroid of the cluster.
    pub centroid: Location,
    /// Bounding box of the cluster.
    pub bounds: GeoBounds,
}

impl Cluster {
    /// Get the number of events in this cluster.
    pub fn len(&self) -> usize {
        self.event_indices.len()
    }

    /// Check if the cluster is empty.
    pub fn is_empty(&self) -> bool {
        self.event_indices.is_empty()
    }
}

/// Result of a clustering operation.
#[derive(Debug, Clone)]
pub struct ClusteringResult {
    /// The clusters found.
    pub clusters: Vec<Cluster>,
    /// Indices of noise points (not in any cluster).
    pub noise: Vec<usize>,
    /// Cluster assignment for each event (-1 for noise).
    pub labels: Vec<i32>,
}

impl ClusteringResult {
    /// Get the number of clusters.
    pub fn num_clusters(&self) -> usize {
        self.clusters.len()
    }

    /// Get cluster for a specific event index.
    pub fn cluster_of(&self, event_idx: usize) -> Option<&Cluster> {
        let label = self.labels.get(event_idx)?;
        if *label < 0 {
            return None;
        }
        self.clusters.get(*label as usize)
    }
}

/// DBSCAN clustering algorithm.
///
/// Density-Based Spatial Clustering of Applications with Noise.
#[derive(Debug, Clone)]
pub struct DBSCAN {
    /// Maximum distance between points in a cluster (in meters).
    pub eps: f64,
    /// Minimum number of points to form a dense region.
    pub min_points: usize,
}

impl DBSCAN {
    /// Create a new DBSCAN clusterer.
    ///
    /// # Arguments
    ///
    /// * `eps` - Maximum distance between points in meters
    /// * `min_points` - Minimum points to form a cluster
    ///
    /// # Examples
    ///
    /// ```
    /// use spatial_narrative::analysis::DBSCAN;
    ///
    /// let dbscan = DBSCAN::new(1000.0, 3); // 1km radius, min 3 points
    /// ```
    pub fn new(eps: f64, min_points: usize) -> Self {
        Self { eps, min_points }
    }

    /// Cluster events using DBSCAN algorithm.
    ///
    /// # Examples
    ///
    /// ```
    /// use spatial_narrative::core::{Event, Location, Timestamp};
    /// use spatial_narrative::analysis::DBSCAN;
    ///
    /// let events = vec![
    ///     Event::new(Location::new(40.0, -74.0), Timestamp::now(), "A"),
    ///     Event::new(Location::new(40.001, -74.001), Timestamp::now(), "B"),
    ///     Event::new(Location::new(40.002, -74.002), Timestamp::now(), "C"),
    ///     Event::new(Location::new(50.0, -80.0), Timestamp::now(), "D"), // Far away
    /// ];
    ///
    /// let dbscan = DBSCAN::new(1000.0, 2);
    /// let result = dbscan.cluster(&events);
    ///
    /// assert_eq!(result.num_clusters(), 1); // One cluster
    /// assert_eq!(result.noise.len(), 1); // One noise point
    /// ```
    pub fn cluster(&self, events: &[Event]) -> ClusteringResult {
        let n = events.len();
        if n == 0 {
            return ClusteringResult {
                clusters: Vec::new(),
                noise: Vec::new(),
                labels: Vec::new(),
            };
        }

        // Build distance cache (for efficiency)
        let locations: Vec<_> = events.iter().map(|e| &e.location).collect();

        // Labels: -1 = unvisited, -2 = noise, >= 0 = cluster id
        let mut labels: Vec<i32> = vec![-1; n];
        let mut current_cluster = 0;

        for i in 0..n {
            if labels[i] != -1 {
                continue; // Already processed
            }

            let neighbors = self.range_query(&locations, i);

            if neighbors.len() < self.min_points {
                labels[i] = -2; // Mark as noise
            } else {
                // Expand cluster
                self.expand_cluster(&locations, i, &neighbors, current_cluster, &mut labels);
                current_cluster += 1;
            }
        }

        // Convert noise markers to -1
        for label in &mut labels {
            if *label == -2 {
                *label = -1;
            }
        }

        self.build_result(events, labels)
    }

    fn range_query(&self, locations: &[&Location], point_idx: usize) -> Vec<usize> {
        let point = locations[point_idx];
        locations
            .iter()
            .enumerate()
            .filter(|(i, loc)| {
                *i != point_idx
                    && haversine_distance(point.lat, point.lon, loc.lat, loc.lon) <= self.eps
            })
            .map(|(i, _)| i)
            .collect()
    }

    fn expand_cluster(
        &self,
        locations: &[&Location],
        seed_idx: usize,
        seed_neighbors: &[usize],
        cluster_id: i32,
        labels: &mut [i32],
    ) {
        labels[seed_idx] = cluster_id;

        let mut seeds: Vec<usize> = seed_neighbors.to_vec();
        let mut processed: HashSet<usize> = HashSet::new();

        while let Some(current_idx) = seeds.pop() {
            if processed.contains(&current_idx) {
                continue;
            }
            processed.insert(current_idx);

            if labels[current_idx] == -2 {
                labels[current_idx] = cluster_id; // Was noise, now in cluster
            }

            if labels[current_idx] != -1 {
                continue; // Already in a cluster
            }

            labels[current_idx] = cluster_id;

            let neighbors = self.range_query(locations, current_idx);

            if neighbors.len() >= self.min_points {
                seeds.extend(neighbors);
            }
        }
    }

    fn build_result(&self, events: &[Event], labels: Vec<i32>) -> ClusteringResult {
        let max_label = labels.iter().max().copied().unwrap_or(-1);
        let num_clusters = if max_label >= 0 {
            (max_label + 1) as usize
        } else {
            0
        };

        let mut clusters = Vec::with_capacity(num_clusters);
        let mut noise = Vec::new();

        for cluster_id in 0..num_clusters {
            let event_indices: Vec<usize> = labels
                .iter()
                .enumerate()
                .filter(|(_, &l)| l == cluster_id as i32)
                .map(|(i, _)| i)
                .collect();

            let cluster_events: Vec<&Event> =
                event_indices.iter().map(|&i| &events[i]).collect();

            let centroid = compute_centroid(&cluster_events);
            let bounds = compute_bounds(&cluster_events);

            clusters.push(Cluster {
                id: cluster_id,
                event_indices,
                centroid,
                bounds,
            });
        }

        for (i, &label) in labels.iter().enumerate() {
            if label < 0 {
                noise.push(i);
            }
        }

        ClusteringResult {
            clusters,
            noise,
            labels,
        }
    }
}

/// K-means clustering with geographic distance.
#[derive(Debug, Clone)]
pub struct KMeans {
    /// Number of clusters to create.
    pub k: usize,
    /// Maximum iterations.
    pub max_iterations: usize,
    /// Convergence threshold in meters.
    pub tolerance: f64,
}

impl KMeans {
    /// Create a new K-means clusterer.
    ///
    /// # Arguments
    ///
    /// * `k` - Number of clusters
    pub fn new(k: usize) -> Self {
        Self {
            k,
            max_iterations: 100,
            tolerance: 1.0, // 1 meter
        }
    }

    /// Create with custom parameters.
    pub fn with_params(k: usize, max_iterations: usize, tolerance: f64) -> Self {
        Self {
            k,
            max_iterations,
            tolerance,
        }
    }

    /// Cluster events using k-means algorithm.
    ///
    /// # Examples
    ///
    /// ```
    /// use spatial_narrative::core::{Event, Location, Timestamp};
    /// use spatial_narrative::analysis::KMeans;
    ///
    /// let events = vec![
    ///     Event::new(Location::new(40.0, -74.0), Timestamp::now(), "A"),
    ///     Event::new(Location::new(40.001, -74.001), Timestamp::now(), "B"),
    ///     Event::new(Location::new(50.0, -80.0), Timestamp::now(), "C"),
    ///     Event::new(Location::new(50.001, -80.001), Timestamp::now(), "D"),
    /// ];
    ///
    /// let kmeans = KMeans::new(2);
    /// let result = kmeans.cluster(&events);
    ///
    /// assert_eq!(result.num_clusters(), 2);
    /// ```
    pub fn cluster(&self, events: &[Event]) -> ClusteringResult {
        let n = events.len();
        if n == 0 || self.k == 0 {
            return ClusteringResult {
                clusters: Vec::new(),
                noise: Vec::new(),
                labels: Vec::new(),
            };
        }

        let k = self.k.min(n);
        let locations: Vec<_> = events.iter().map(|e| &e.location).collect();

        // Initialize centroids (spread evenly across data)
        let mut centroids: Vec<Location> = (0..k)
            .map(|i| {
                let idx = (i * n) / k;
                locations[idx].clone()
            })
            .collect();

        let mut labels = vec![0i32; n];

        for _ in 0..self.max_iterations {
            // Assign points to nearest centroid
            for (i, loc) in locations.iter().enumerate() {
                let mut min_dist = f64::MAX;
                let mut min_cluster = 0;

                for (c, centroid) in centroids.iter().enumerate() {
                    let dist = haversine_distance(loc.lat, loc.lon, centroid.lat, centroid.lon);
                    if dist < min_dist {
                        min_dist = dist;
                        min_cluster = c;
                    }
                }

                labels[i] = min_cluster as i32;
            }

            // Update centroids
            let mut converged = true;
            for (c, centroid) in centroids.iter_mut().enumerate().take(k) {
                let cluster_points: Vec<&&Location> = locations
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| labels[*i] == c as i32)
                    .map(|(_, loc)| loc)
                    .collect();

                if cluster_points.is_empty() {
                    continue;
                }

                let new_centroid = compute_centroid_from_locations(&cluster_points);
                let shift = haversine_distance(
                    centroid.lat,
                    centroid.lon,
                    new_centroid.lat,
                    new_centroid.lon,
                );

                if shift > self.tolerance {
                    converged = false;
                }

                *centroid = new_centroid;
            }

            if converged {
                break;
            }
        }

        // Build result
        let mut clusters = Vec::with_capacity(k);
        for (cluster_id, centroid) in centroids.iter().enumerate().take(k) {
            let event_indices: Vec<usize> = labels
                .iter()
                .enumerate()
                .filter(|(_, &l)| l == cluster_id as i32)
                .map(|(i, _)| i)
                .collect();

            if event_indices.is_empty() {
                continue;
            }

            let cluster_events: Vec<&Event> =
                event_indices.iter().map(|&i| &events[i]).collect();

            let centroid = centroid.clone();
            let bounds = compute_bounds(&cluster_events);

            clusters.push(Cluster {
                id: clusters.len(),
                event_indices,
                centroid,
                bounds,
            });
        }

        ClusteringResult {
            clusters,
            noise: Vec::new(), // K-means has no noise concept
            labels,
        }
    }
}

fn compute_centroid(events: &[&Event]) -> Location {
    if events.is_empty() {
        return Location::new(0.0, 0.0);
    }

    let sum_lat: f64 = events.iter().map(|e| e.location.lat).sum();
    let sum_lon: f64 = events.iter().map(|e| e.location.lon).sum();
    let n = events.len() as f64;

    Location::new(sum_lat / n, sum_lon / n)
}

fn compute_centroid_from_locations(locations: &[&&Location]) -> Location {
    if locations.is_empty() {
        return Location::new(0.0, 0.0);
    }

    let sum_lat: f64 = locations.iter().map(|l| l.lat).sum();
    let sum_lon: f64 = locations.iter().map(|l| l.lon).sum();
    let n = locations.len() as f64;

    Location::new(sum_lat / n, sum_lon / n)
}

fn compute_bounds(events: &[&Event]) -> GeoBounds {
    if events.is_empty() {
        return GeoBounds::new(0.0, 0.0, 0.0, 0.0);
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

    GeoBounds::new(min_lat, max_lat, min_lon, max_lon)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Timestamp;

    fn make_event(lat: f64, lon: f64) -> Event {
        Event::new(Location::new(lat, lon), Timestamp::now(), "test")
    }

    #[test]
    fn test_dbscan_empty() {
        let dbscan = DBSCAN::new(1000.0, 2);
        let result = dbscan.cluster(&[]);
        assert_eq!(result.num_clusters(), 0);
    }

    #[test]
    fn test_dbscan_single_cluster() {
        let events = vec![
            make_event(40.0, -74.0),
            make_event(40.001, -74.001),
            make_event(40.002, -74.002),
        ];

        let dbscan = DBSCAN::new(1000.0, 2); // 1km, min 2 points
        let result = dbscan.cluster(&events);

        assert_eq!(result.num_clusters(), 1);
        assert_eq!(result.clusters[0].len(), 3);
    }

    #[test]
    fn test_dbscan_with_noise() {
        let events = vec![
            make_event(40.0, -74.0),
            make_event(40.001, -74.001),
            make_event(40.002, -74.002),
            make_event(50.0, -80.0), // Far away noise
        ];

        let dbscan = DBSCAN::new(1000.0, 2);
        let result = dbscan.cluster(&events);

        assert_eq!(result.num_clusters(), 1);
        assert_eq!(result.noise.len(), 1);
        assert_eq!(result.labels[3], -1); // Last point is noise
    }

    #[test]
    fn test_kmeans_basic() {
        let events = vec![
            make_event(40.0, -74.0),
            make_event(40.001, -74.001),
            make_event(50.0, -80.0),
            make_event(50.001, -80.001),
        ];

        let kmeans = KMeans::new(2);
        let result = kmeans.cluster(&events);

        assert_eq!(result.num_clusters(), 2);
    }

    #[test]
    fn test_kmeans_too_many_clusters() {
        let events = vec![make_event(40.0, -74.0), make_event(40.001, -74.001)];

        let kmeans = KMeans::new(10); // More clusters than points
        let result = kmeans.cluster(&events);

        assert!(result.num_clusters() <= 2);
    }

    #[test]
    fn test_cluster_of() {
        let events = vec![
            make_event(40.0, -74.0),
            make_event(40.001, -74.001),
            make_event(40.002, -74.002), // Third point to ensure cluster
            make_event(50.0, -80.0),     // Noise - far away
        ];

        let dbscan = DBSCAN::new(1000.0, 2);
        let result = dbscan.cluster(&events);

        // First 3 should be in a cluster
        assert!(result.cluster_of(0).is_some() || result.cluster_of(1).is_some());
        assert!(result.cluster_of(3).is_none()); // Far point should be noise
    }
}
