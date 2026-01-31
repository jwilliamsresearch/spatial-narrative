//! Temporal metrics for narrative analysis.
//!
//! Provides tools for analyzing the temporal characteristics
//! of narratives including duration, event rates, gaps, and clustering.

use crate::core::{Event, TimeRange, Timestamp};
use std::collections::HashMap;

/// Temporal metrics computed from a collection of events.
#[derive(Debug, Clone)]
pub struct TemporalMetrics {
    /// Number of events analyzed.
    pub event_count: usize,
    /// Time range spanning all events.
    pub time_range: Option<TimeRange>,
    /// Total duration in seconds.
    pub duration_secs: f64,
    /// Average time between consecutive events in seconds.
    pub avg_inter_event_time: f64,
    /// Minimum time between consecutive events in seconds.
    pub min_inter_event_time: f64,
    /// Maximum time between consecutive events in seconds.
    pub max_inter_event_time: f64,
    /// Standard deviation of inter-event times in seconds.
    pub inter_event_std_dev: f64,
}

impl Default for TemporalMetrics {
    fn default() -> Self {
        Self {
            event_count: 0,
            time_range: None,
            duration_secs: 0.0,
            avg_inter_event_time: 0.0,
            min_inter_event_time: 0.0,
            max_inter_event_time: 0.0,
            inter_event_std_dev: 0.0,
        }
    }
}

impl TemporalMetrics {
    /// Compute temporal metrics from a slice of events.
    ///
    /// Events will be sorted by timestamp for analysis.
    ///
    /// # Examples
    ///
    /// ```
    /// use spatial_narrative::core::{Event, Location, Timestamp};
    /// use spatial_narrative::analysis::TemporalMetrics;
    ///
    /// let events = vec![
    ///     Event::new(Location::new(0.0, 0.0), Timestamp::parse("2024-01-01T10:00:00Z").unwrap(), "First"),
    ///     Event::new(Location::new(0.0, 0.0), Timestamp::parse("2024-01-01T11:00:00Z").unwrap(), "Second"),
    /// ];
    ///
    /// let metrics = TemporalMetrics::from_events(&events);
    /// assert_eq!(metrics.event_count, 2);
    /// assert_eq!(metrics.duration_secs, 3600.0); // 1 hour
    /// ```
    pub fn from_events(events: &[Event]) -> Self {
        if events.is_empty() {
            return Self::default();
        }

        let timestamps: Vec<&Timestamp> = events.iter().map(|e| &e.timestamp).collect();
        Self::from_timestamps(&timestamps)
    }

    /// Compute temporal metrics from a slice of timestamps.
    pub fn from_timestamps(timestamps: &[&Timestamp]) -> Self {
        if timestamps.is_empty() {
            return Self::default();
        }

        let event_count = timestamps.len();

        // Sort timestamps
        let mut sorted: Vec<_> = timestamps.to_vec();
        sorted.sort_by_key(|t| t.to_unix_millis());

        let first = sorted.first().unwrap();
        let last = sorted.last().unwrap();

        let time_range = Some(TimeRange::new((*first).clone(), (*last).clone()));
        let duration_secs = (last.to_unix_millis() - first.to_unix_millis()) as f64 / 1000.0;

        // Compute inter-event times
        let (avg, min, max, std_dev) = Self::compute_inter_event_stats(&sorted);

        Self {
            event_count,
            time_range,
            duration_secs,
            avg_inter_event_time: avg,
            min_inter_event_time: min,
            max_inter_event_time: max,
            inter_event_std_dev: std_dev,
        }
    }

    fn compute_inter_event_stats(sorted: &[&Timestamp]) -> (f64, f64, f64, f64) {
        if sorted.len() < 2 {
            return (0.0, 0.0, 0.0, 0.0);
        }

        let mut gaps: Vec<f64> = Vec::with_capacity(sorted.len() - 1);

        for window in sorted.windows(2) {
            let gap_ms = window[1].to_unix_millis() - window[0].to_unix_millis();
            gaps.push(gap_ms as f64 / 1000.0);
        }

        let sum: f64 = gaps.iter().sum();
        let avg = sum / gaps.len() as f64;
        let min = gaps.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = gaps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // Standard deviation
        let variance: f64 = gaps.iter().map(|g| (g - avg).powi(2)).sum::<f64>() / gaps.len() as f64;
        let std_dev = variance.sqrt();

        (avg, min, max, std_dev)
    }
}

/// Time bin for temporal aggregation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeBin {
    /// Hourly bins
    Hour,
    /// Daily bins
    Day,
    /// Weekly bins
    Week,
    /// Monthly bins
    Month,
    /// Yearly bins
    Year,
}

/// A count of events in a time period.
#[derive(Debug, Clone)]
pub struct TimeBinCount {
    /// Start of the bin.
    pub start: Timestamp,
    /// End of the bin.
    pub end: Timestamp,
    /// Number of events in this bin.
    pub count: usize,
}

/// Compute event counts per time bin.
///
/// # Arguments
///
/// * `events` - Slice of events to analyze
/// * `bin_size` - The size of time bins
///
/// # Returns
///
/// Vector of bin counts in chronological order.
///
/// # Examples
///
/// ```
/// use spatial_narrative::core::{Event, Location, Timestamp};
/// use spatial_narrative::analysis::{event_rate, TimeBin};
///
/// let events = vec![
///     Event::new(Location::new(0.0, 0.0), Timestamp::parse("2024-01-01T10:00:00Z").unwrap(), "A"),
///     Event::new(Location::new(0.0, 0.0), Timestamp::parse("2024-01-01T10:30:00Z").unwrap(), "B"),
///     Event::new(Location::new(0.0, 0.0), Timestamp::parse("2024-01-01T11:15:00Z").unwrap(), "C"),
/// ];
///
/// let rates = event_rate(&events, TimeBin::Hour);
/// assert_eq!(rates.len(), 2); // 2 hours covered
/// assert_eq!(rates[0].count, 2); // 2 events in first hour
/// assert_eq!(rates[1].count, 1); // 1 event in second hour
/// ```
pub fn event_rate(events: &[Event], bin_size: TimeBin) -> Vec<TimeBinCount> {
    if events.is_empty() {
        return Vec::new();
    }

    // Sort events by timestamp
    let mut sorted: Vec<_> = events.iter().collect();
    sorted.sort_by_key(|e| e.timestamp.to_unix_millis());

    let first_ts = sorted.first().unwrap().timestamp.to_unix_millis();
    let last_ts = sorted.last().unwrap().timestamp.to_unix_millis();

    let bin_millis = match bin_size {
        TimeBin::Hour => 3_600_000,
        TimeBin::Day => 86_400_000,
        TimeBin::Week => 604_800_000,
        TimeBin::Month => 2_629_800_000, // ~30.44 days
        TimeBin::Year => 31_557_600_000, // ~365.25 days
    };

    // Compute bin counts
    let mut bins: HashMap<i64, usize> = HashMap::new();

    for event in &sorted {
        let ts = event.timestamp.to_unix_millis();
        let bin_start = (ts / bin_millis) * bin_millis;
        *bins.entry(bin_start).or_insert(0) += 1;
    }

    // Generate continuous bins from first to last
    let first_bin = (first_ts / bin_millis) * bin_millis;
    let last_bin = (last_ts / bin_millis) * bin_millis;

    let mut result = Vec::new();
    let mut bin_start = first_bin;

    while bin_start <= last_bin {
        let count = bins.get(&bin_start).copied().unwrap_or(0);
        let start = Timestamp::from_unix_millis(bin_start).unwrap();
        let end = Timestamp::from_unix_millis(bin_start + bin_millis).unwrap();

        result.push(TimeBinCount { start, end, count });
        bin_start += bin_millis;
    }

    result
}

/// Detect gaps in event timeline.
///
/// Returns time ranges where no events occurred for longer than the threshold.
///
/// # Arguments
///
/// * `events` - Slice of events to analyze
/// * `threshold_secs` - Minimum gap duration to report (in seconds)
///
/// # Returns
///
/// Vector of time ranges representing gaps.
///
/// # Examples
///
/// ```
/// use spatial_narrative::core::{Event, Location, Timestamp};
/// use spatial_narrative::analysis::detect_gaps;
///
/// let events = vec![
///     Event::new(Location::new(0.0, 0.0), Timestamp::parse("2024-01-01T10:00:00Z").unwrap(), "A"),
///     Event::new(Location::new(0.0, 0.0), Timestamp::parse("2024-01-01T15:00:00Z").unwrap(), "B"), // 5 hour gap
/// ];
///
/// let gaps = detect_gaps(&events, 3600.0); // 1 hour threshold
/// assert_eq!(gaps.len(), 1);
/// ```
pub fn detect_gaps(events: &[Event], threshold_secs: f64) -> Vec<TimeRange> {
    if events.len() < 2 {
        return Vec::new();
    }

    let threshold_millis = (threshold_secs * 1000.0) as i64;

    // Sort events by timestamp
    let mut sorted: Vec<_> = events.iter().collect();
    sorted.sort_by_key(|e| e.timestamp.to_unix_millis());

    let mut gaps = Vec::new();

    for window in sorted.windows(2) {
        let start_ts = window[0].timestamp.to_unix_millis();
        let end_ts = window[1].timestamp.to_unix_millis();

        if end_ts - start_ts > threshold_millis {
            gaps.push(TimeRange::new(
                window[0].timestamp.clone(),
                window[1].timestamp.clone(),
            ));
        }
    }

    gaps
}

/// Detect bursts of activity (periods of high event frequency).
///
/// # Arguments
///
/// * `events` - Slice of events to analyze
/// * `window_secs` - Time window size in seconds
/// * `min_events` - Minimum events in window to qualify as burst
///
/// # Returns
///
/// Vector of time ranges representing burst periods.
pub fn detect_bursts(events: &[Event], window_secs: f64, min_events: usize) -> Vec<TimeRange> {
    if events.is_empty() || min_events == 0 {
        return Vec::new();
    }

    let window_millis = (window_secs * 1000.0) as i64;

    // Sort events by timestamp
    let mut sorted: Vec<_> = events.iter().collect();
    sorted.sort_by_key(|e| e.timestamp.to_unix_millis());

    let timestamps: Vec<i64> = sorted.iter().map(|e| e.timestamp.to_unix_millis()).collect();

    let mut bursts = Vec::new();
    let mut i = 0;

    while i < timestamps.len() {
        // Count events in window starting at i
        let window_start = timestamps[i];
        let window_end = window_start + window_millis;

        let count = timestamps[i..]
            .iter()
            .take_while(|&&t| t < window_end)
            .count();

        if count >= min_events {
            // Find actual end of burst
            let burst_end_idx = i + count - 1;
            bursts.push(TimeRange::new(
                sorted[i].timestamp.clone(),
                sorted[burst_end_idx].timestamp.clone(),
            ));
            i = burst_end_idx + 1;
        } else {
            i += 1;
        }
    }

    bursts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Location;

    fn make_event(time_str: &str) -> Event {
        Event::new(
            Location::new(0.0, 0.0),
            Timestamp::parse(time_str).unwrap(),
            "test",
        )
    }

    #[test]
    fn test_temporal_metrics_empty() {
        let metrics = TemporalMetrics::from_events(&[]);
        assert_eq!(metrics.event_count, 0);
        assert!(metrics.time_range.is_none());
    }

    #[test]
    fn test_temporal_metrics_single() {
        let events = vec![make_event("2024-01-01T10:00:00Z")];
        let metrics = TemporalMetrics::from_events(&events);

        assert_eq!(metrics.event_count, 1);
        assert!(metrics.time_range.is_some());
        assert_eq!(metrics.duration_secs, 0.0);
    }

    #[test]
    fn test_temporal_metrics_multiple() {
        let events = vec![
            make_event("2024-01-01T10:00:00Z"),
            make_event("2024-01-01T11:00:00Z"),
            make_event("2024-01-01T12:00:00Z"),
        ];
        let metrics = TemporalMetrics::from_events(&events);

        assert_eq!(metrics.event_count, 3);
        assert_eq!(metrics.duration_secs, 7200.0); // 2 hours
        assert_eq!(metrics.avg_inter_event_time, 3600.0); // 1 hour average
    }

    #[test]
    fn test_event_rate() {
        let events = vec![
            make_event("2024-01-01T10:00:00Z"),
            make_event("2024-01-01T10:30:00Z"),
            make_event("2024-01-01T11:15:00Z"),
        ];

        let rates = event_rate(&events, TimeBin::Hour);
        assert_eq!(rates.len(), 2);
        assert_eq!(rates[0].count, 2);
        assert_eq!(rates[1].count, 1);
    }

    #[test]
    fn test_detect_gaps() {
        let events = vec![
            make_event("2024-01-01T10:00:00Z"),
            make_event("2024-01-01T10:30:00Z"),
            make_event("2024-01-01T15:00:00Z"), // 4.5 hour gap
        ];

        let gaps = detect_gaps(&events, 3600.0); // 1 hour threshold
        assert_eq!(gaps.len(), 1);
    }

    #[test]
    fn test_detect_bursts() {
        let events = vec![
            make_event("2024-01-01T10:00:00Z"),
            make_event("2024-01-01T10:01:00Z"),
            make_event("2024-01-01T10:02:00Z"),
            make_event("2024-01-01T15:00:00Z"), // Isolated event
        ];

        let bursts = detect_bursts(&events, 300.0, 3); // 5 min window, 3+ events
        assert_eq!(bursts.len(), 1);
    }
}
