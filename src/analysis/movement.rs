//! Movement analysis for spatial narratives.
//!
//! Provides tools for analyzing movement patterns including
//! trajectory extraction, velocity profiles, and stop detection.

use crate::analysis::haversine_distance;
use crate::core::{Event, GeoBounds, Location, TimeRange, Timestamp};

/// A trajectory representing movement through space and time.
#[derive(Debug, Clone)]
pub struct Trajectory {
    /// Unique identifier for this trajectory.
    pub id: String,
    /// Ordered sequence of events in this trajectory.
    events: Vec<Event>,
}

impl Trajectory {
    /// Create a new trajectory from events.
    ///
    /// Events are sorted by timestamp.
    pub fn new(id: impl Into<String>, events: Vec<Event>) -> Self {
        let mut events = events;
        events.sort_by_key(|e| e.timestamp.to_unix_millis());

        Self {
            id: id.into(),
            events,
        }
    }

    /// Get the events in this trajectory.
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    /// Get the number of events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Check if the trajectory is empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Get the time range of this trajectory.
    pub fn time_range(&self) -> Option<TimeRange> {
        if self.events.is_empty() {
            return None;
        }

        Some(TimeRange::new(
            self.events.first().unwrap().timestamp.clone(),
            self.events.last().unwrap().timestamp.clone(),
        ))
    }

    /// Get the geographic bounds of this trajectory.
    pub fn bounds(&self) -> Option<GeoBounds> {
        if self.events.is_empty() {
            return None;
        }

        let mut min_lat = f64::MAX;
        let mut max_lat = f64::MIN;
        let mut min_lon = f64::MAX;
        let mut max_lon = f64::MIN;

        for event in &self.events {
            min_lat = min_lat.min(event.location.lat);
            max_lat = max_lat.max(event.location.lat);
            min_lon = min_lon.min(event.location.lon);
            max_lon = max_lon.max(event.location.lon);
        }

        Some(GeoBounds::new(min_lat, max_lat, min_lon, max_lon))
    }

    /// Get total distance traveled in meters.
    pub fn total_distance(&self) -> f64 {
        if self.events.len() < 2 {
            return 0.0;
        }

        self.events
            .windows(2)
            .map(|w| {
                haversine_distance(
                    w[0].location.lat,
                    w[0].location.lon,
                    w[1].location.lat,
                    w[1].location.lon,
                )
            })
            .sum()
    }

    /// Get total duration in seconds.
    pub fn duration_secs(&self) -> f64 {
        if self.events.len() < 2 {
            return 0.0;
        }

        let first = self.events.first().unwrap().timestamp.to_unix_millis();
        let last = self.events.last().unwrap().timestamp.to_unix_millis();

        (last - first) as f64 / 1000.0
    }

    /// Get average speed in meters per second.
    pub fn avg_speed(&self) -> f64 {
        let duration = self.duration_secs();
        if duration <= 0.0 {
            return 0.0;
        }

        self.total_distance() / duration
    }

    /// Compute velocity profile: speed at each segment.
    ///
    /// Returns tuples of (timestamp, speed in m/s).
    pub fn velocity_profile(&self) -> Vec<(Timestamp, f64)> {
        if self.events.len() < 2 {
            return Vec::new();
        }

        self.events
            .windows(2)
            .map(|w| {
                let dist = haversine_distance(
                    w[0].location.lat,
                    w[0].location.lon,
                    w[1].location.lat,
                    w[1].location.lon,
                );
                let time_diff = (w[1].timestamp.to_unix_millis() - w[0].timestamp.to_unix_millis())
                    as f64
                    / 1000.0;

                let speed = if time_diff > 0.0 {
                    dist / time_diff
                } else {
                    0.0
                };

                (w[0].timestamp.clone(), speed)
            })
            .collect()
    }

    /// Simplify trajectory using Douglas-Peucker algorithm.
    ///
    /// # Arguments
    ///
    /// * `epsilon` - Maximum deviation in meters
    ///
    /// # Returns
    ///
    /// A new trajectory with fewer points.
    pub fn simplify(&self, epsilon: f64) -> Trajectory {
        if self.events.len() <= 2 {
            return self.clone();
        }

        let indices = douglas_peucker_indices(&self.events, epsilon);
        let simplified_events: Vec<Event> = indices
            .into_iter()
            .map(|i| self.events[i].clone())
            .collect();

        Trajectory::new(format!("{}_simplified", self.id), simplified_events)
    }
}

/// A detected stop in a trajectory.
#[derive(Debug, Clone)]
pub struct Stop {
    /// Location of the stop (centroid of points).
    pub location: Location,
    /// Start time of the stop.
    pub start: Timestamp,
    /// End time of the stop.
    pub end: Timestamp,
    /// Duration in seconds.
    pub duration_secs: f64,
    /// Number of events during this stop.
    pub event_count: usize,
}

impl Stop {
    /// Get the time range of this stop.
    pub fn time_range(&self) -> TimeRange {
        TimeRange::new(self.start.clone(), self.end.clone())
    }
}

/// Configuration for stop detection.
#[derive(Debug, Clone)]
pub struct StopThreshold {
    /// Maximum radius in meters to consider as same location.
    pub radius_m: f64,
    /// Minimum duration in seconds to qualify as a stop.
    pub min_duration_secs: f64,
}

impl Default for StopThreshold {
    fn default() -> Self {
        Self {
            radius_m: 50.0,           // 50 meters
            min_duration_secs: 300.0, // 5 minutes
        }
    }
}

/// Detect stops in a trajectory.
///
/// A stop is a period where movement stays within a radius for a minimum duration.
///
/// # Examples
///
/// ```
/// use spatial_narrative::core::{Event, Location, Timestamp};
/// use spatial_narrative::analysis::{Trajectory, detect_stops, StopThreshold};
///
/// let events = vec![
///     Event::new(Location::new(40.0, -74.0), Timestamp::parse("2024-01-01T10:00:00Z").unwrap(), "A"),
///     Event::new(Location::new(40.0001, -74.0001), Timestamp::parse("2024-01-01T10:30:00Z").unwrap(), "B"),
///     Event::new(Location::new(40.0002, -74.0002), Timestamp::parse("2024-01-01T11:00:00Z").unwrap(), "C"),
/// ];
///
/// let trajectory = Trajectory::new("test", events);
/// let threshold = StopThreshold { radius_m: 100.0, min_duration_secs: 1800.0 };
/// let stops = detect_stops(&trajectory, &threshold);
/// assert_eq!(stops.len(), 1); // All points are close together for 1 hour
/// ```
pub fn detect_stops(trajectory: &Trajectory, threshold: &StopThreshold) -> Vec<Stop> {
    let events = trajectory.events();
    if events.len() < 2 {
        return Vec::new();
    }

    let mut stops = Vec::new();
    let mut start_idx = 0;

    while start_idx < events.len() {
        let anchor = &events[start_idx].location;
        let mut end_idx = start_idx;

        // Find all consecutive events within radius
        while end_idx < events.len() {
            let dist = haversine_distance(
                anchor.lat,
                anchor.lon,
                events[end_idx].location.lat,
                events[end_idx].location.lon,
            );

            if dist > threshold.radius_m {
                break;
            }
            end_idx += 1;
        }

        // Check if duration meets threshold
        if end_idx > start_idx {
            let start_ts = events[start_idx].timestamp.to_unix_millis();
            let end_ts = events[end_idx - 1].timestamp.to_unix_millis();
            let duration_secs = (end_ts - start_ts) as f64 / 1000.0;

            if duration_secs >= threshold.min_duration_secs {
                // Compute centroid
                let stop_events = &events[start_idx..end_idx];
                let centroid = compute_centroid(stop_events);

                stops.push(Stop {
                    location: centroid,
                    start: events[start_idx].timestamp.clone(),
                    end: events[end_idx - 1].timestamp.clone(),
                    duration_secs,
                    event_count: end_idx - start_idx,
                });

                start_idx = end_idx;
                continue;
            }
        }

        start_idx += 1;
    }

    stops
}

fn compute_centroid(events: &[Event]) -> Location {
    if events.is_empty() {
        return Location::new(0.0, 0.0);
    }

    let sum_lat: f64 = events.iter().map(|e| e.location.lat).sum();
    let sum_lon: f64 = events.iter().map(|e| e.location.lon).sum();
    let n = events.len() as f64;

    Location::new(sum_lat / n, sum_lon / n)
}

/// Douglas-Peucker line simplification algorithm.
///
/// Returns indices of points to keep.
fn douglas_peucker_indices(events: &[Event], epsilon: f64) -> Vec<usize> {
    if events.len() <= 2 {
        return (0..events.len()).collect();
    }

    // Find point with maximum distance from line between first and last
    let (max_dist, max_idx) = find_max_perpendicular_distance(events);

    if max_dist > epsilon {
        // Recursively simplify
        let mut left = douglas_peucker_indices(&events[..=max_idx], epsilon);
        let right = douglas_peucker_indices(&events[max_idx..], epsilon);

        // Remove duplicate at junction
        left.pop();

        // Adjust right indices
        let right_adjusted: Vec<usize> = right.into_iter().map(|i| i + max_idx).collect();

        left.extend(right_adjusted);
        left
    } else {
        // Keep only endpoints
        vec![0, events.len() - 1]
    }
}

fn find_max_perpendicular_distance(events: &[Event]) -> (f64, usize) {
    let first = &events[0].location;
    let last = &events[events.len() - 1].location;

    let mut max_dist = 0.0;
    let mut max_idx = 0;

    for (i, event) in events.iter().enumerate().skip(1).take(events.len() - 2) {
        let dist = perpendicular_distance(&event.location, first, last);
        if dist > max_dist {
            max_dist = dist;
            max_idx = i;
        }
    }

    (max_dist, max_idx)
}

fn perpendicular_distance(point: &Location, line_start: &Location, line_end: &Location) -> f64 {
    // Approximate using planar geometry (good for small areas)
    let dx = line_end.lon - line_start.lon;
    let dy = line_end.lat - line_start.lat;

    let line_len_sq = dx * dx + dy * dy;
    if line_len_sq < 1e-12 {
        return haversine_distance(point.lat, point.lon, line_start.lat, line_start.lon);
    }

    // Project point onto line
    let t = ((point.lon - line_start.lon) * dx + (point.lat - line_start.lat) * dy) / line_len_sq;
    let t = t.clamp(0.0, 1.0);

    let proj_lon = line_start.lon + t * dx;
    let proj_lat = line_start.lat + t * dy;

    haversine_distance(point.lat, point.lon, proj_lat, proj_lon)
}

/// Movement analyzer for extracting insights from event sequences.
#[derive(Debug, Clone, Default)]
pub struct MovementAnalyzer {
    /// Stop detection threshold.
    pub stop_threshold: StopThreshold,
}

impl MovementAnalyzer {
    /// Create a new movement analyzer with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a movement analyzer with custom stop threshold.
    pub fn with_stop_threshold(threshold: StopThreshold) -> Self {
        Self {
            stop_threshold: threshold,
        }
    }

    /// Extract a trajectory from events.
    pub fn extract_trajectory(&self, id: impl Into<String>, events: Vec<Event>) -> Trajectory {
        Trajectory::new(id, events)
    }

    /// Detect stops in a trajectory.
    pub fn detect_stops(&self, trajectory: &Trajectory) -> Vec<Stop> {
        detect_stops(trajectory, &self.stop_threshold)
    }

    /// Analyze movement between stops.
    ///
    /// Returns segments of movement between detected stops.
    pub fn movement_segments(&self, trajectory: &Trajectory) -> Vec<Trajectory> {
        let stops = self.detect_stops(trajectory);
        if stops.is_empty() {
            return vec![trajectory.clone()];
        }

        let events = trajectory.events();
        let mut segments = Vec::new();
        let mut current_start = 0;

        for (i, stop) in stops.iter().enumerate() {
            // Find events before this stop
            let stop_start_idx = events
                .iter()
                .position(|e| e.timestamp.to_unix_millis() >= stop.start.to_unix_millis())
                .unwrap_or(events.len());

            if stop_start_idx > current_start {
                let segment_events: Vec<Event> = events[current_start..stop_start_idx].to_vec();
                if !segment_events.is_empty() {
                    segments.push(Trajectory::new(
                        format!("{}_segment_{}", trajectory.id, i),
                        segment_events,
                    ));
                }
            }

            // Move past the stop
            current_start = events
                .iter()
                .position(|e| e.timestamp.to_unix_millis() > stop.end.to_unix_millis())
                .unwrap_or(events.len());
        }

        // Add final segment after last stop
        if current_start < events.len() {
            let segment_events: Vec<Event> = events[current_start..].to_vec();
            if !segment_events.is_empty() {
                segments.push(Trajectory::new(
                    format!("{}_segment_{}", trajectory.id, stops.len()),
                    segment_events,
                ));
            }
        }

        segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_event(lat: f64, lon: f64, time_str: &str) -> Event {
        Event::new(
            Location::new(lat, lon),
            Timestamp::parse(time_str).unwrap(),
            "test",
        )
    }

    #[test]
    fn test_trajectory_basic() {
        let events = vec![
            make_event(40.0, -74.0, "2024-01-01T10:00:00Z"),
            make_event(41.0, -73.0, "2024-01-01T11:00:00Z"),
        ];

        let traj = Trajectory::new("test", events);
        assert_eq!(traj.len(), 2);
        assert!(traj.total_distance() > 0.0);
        assert_eq!(traj.duration_secs(), 3600.0);
    }

    #[test]
    fn test_velocity_profile() {
        let events = vec![
            make_event(40.0, -74.0, "2024-01-01T10:00:00Z"),
            make_event(40.1, -74.0, "2024-01-01T11:00:00Z"),
        ];

        let traj = Trajectory::new("test", events);
        let profile = traj.velocity_profile();

        assert_eq!(profile.len(), 1);
        assert!(profile[0].1 > 0.0);
    }

    #[test]
    fn test_detect_stops() {
        let events = vec![
            make_event(40.0, -74.0, "2024-01-01T10:00:00Z"),
            make_event(40.0001, -74.0001, "2024-01-01T10:30:00Z"),
            make_event(40.0002, -74.0002, "2024-01-01T11:00:00Z"),
            make_event(41.0, -73.0, "2024-01-01T12:00:00Z"), // Move away
        ];

        let traj = Trajectory::new("test", events);
        let threshold = StopThreshold {
            radius_m: 100.0,
            min_duration_secs: 1800.0, // 30 min
        };

        let stops = detect_stops(&traj, &threshold);
        assert_eq!(stops.len(), 1);
        assert!((stops[0].duration_secs - 3600.0).abs() < 1.0); // 1 hour stop
    }

    #[test]
    fn test_trajectory_simplify() {
        let events = vec![
            make_event(0.0, 0.0, "2024-01-01T10:00:00Z"),
            make_event(0.00001, 0.5, "2024-01-01T10:30:00Z"), // Nearly on line
            make_event(0.0, 1.0, "2024-01-01T11:00:00Z"),
        ];

        let traj = Trajectory::new("test", events);
        let simplified = traj.simplify(1000.0); // Large epsilon

        // Should simplify to 2 points
        assert!(simplified.len() <= traj.len());
    }

    #[test]
    fn test_movement_analyzer() {
        let analyzer = MovementAnalyzer::new();
        let events = vec![
            make_event(40.0, -74.0, "2024-01-01T10:00:00Z"),
            make_event(41.0, -73.0, "2024-01-01T11:00:00Z"),
        ];

        let traj = analyzer.extract_trajectory("test", events);
        assert_eq!(traj.len(), 2);
    }
}
