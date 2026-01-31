//! Spatial metrics for narrative analysis.
//!
//! Provides tools for computing geographic extent, distances,
//! dispersion, and density of events in a narrative.

use crate::core::{Event, GeoBounds, Location};

/// Spatial metrics computed from a collection of events.
#[derive(Debug, Clone)]
pub struct SpatialMetrics {
    /// Number of events analyzed.
    pub event_count: usize,
    /// Geographic bounding box containing all events.
    pub bounds: Option<GeoBounds>,
    /// Geographic centroid (center of mass).
    pub centroid: Option<Location>,
    /// Total distance traveled between consecutive events (in meters).
    pub total_distance: f64,
    /// Average distance between consecutive events (in meters).
    pub avg_distance: f64,
    /// Maximum distance between any two consecutive events (in meters).
    pub max_distance: f64,
    /// Dispersion: average distance from centroid (in meters).
    pub dispersion: f64,
    /// Convex hull area (in square meters), if computable.
    pub area: Option<f64>,
}

impl Default for SpatialMetrics {
    fn default() -> Self {
        Self {
            event_count: 0,
            bounds: None,
            centroid: None,
            total_distance: 0.0,
            avg_distance: 0.0,
            max_distance: 0.0,
            dispersion: 0.0,
            area: None,
        }
    }
}

impl SpatialMetrics {
    /// Compute spatial metrics from a slice of events.
    ///
    /// Events are assumed to be in chronological order for distance calculations.
    ///
    /// # Examples
    ///
    /// ```
    /// use spatial_narrative::core::{Event, Location, Timestamp};
    /// use spatial_narrative::analysis::SpatialMetrics;
    ///
    /// let events = vec![
    ///     Event::new(Location::new(40.7128, -74.0060), Timestamp::now(), "NYC"),
    ///     Event::new(Location::new(34.0522, -118.2437), Timestamp::now(), "LA"),
    /// ];
    ///
    /// let metrics = SpatialMetrics::from_events(&events);
    /// assert_eq!(metrics.event_count, 2);
    /// assert!(metrics.total_distance > 0.0);
    /// ```
    pub fn from_events(events: &[Event]) -> Self {
        if events.is_empty() {
            return Self::default();
        }

        let locations: Vec<&Location> = events.iter().map(|e| &e.location).collect();
        Self::from_locations(&locations)
    }

    /// Compute spatial metrics from a slice of locations.
    pub fn from_locations(locations: &[&Location]) -> Self {
        if locations.is_empty() {
            return Self::default();
        }

        let event_count = locations.len();

        // Compute bounds
        let bounds = Self::compute_bounds(locations);

        // Compute centroid
        let centroid = Self::compute_centroid(locations);

        // Compute distances between consecutive locations
        let (total_distance, avg_distance, max_distance) =
            Self::compute_consecutive_distances(locations);

        // Compute dispersion from centroid
        let dispersion = centroid
            .as_ref()
            .map(|c| Self::compute_dispersion(locations, c))
            .unwrap_or(0.0);

        // Approximate area using bounding box (simplified)
        let area = bounds.as_ref().map(|b| {
            let width_m = haversine_distance(b.min_lat, b.min_lon, b.min_lat, b.max_lon);
            let height_m = haversine_distance(b.min_lat, b.min_lon, b.max_lat, b.min_lon);
            width_m * height_m
        });

        Self {
            event_count,
            bounds,
            centroid,
            total_distance,
            avg_distance,
            max_distance,
            dispersion,
            area,
        }
    }

    fn compute_bounds(locations: &[&Location]) -> Option<GeoBounds> {
        if locations.is_empty() {
            return None;
        }

        let mut min_lat = f64::MAX;
        let mut max_lat = f64::MIN;
        let mut min_lon = f64::MAX;
        let mut max_lon = f64::MIN;

        for loc in locations {
            min_lat = min_lat.min(loc.lat);
            max_lat = max_lat.max(loc.lat);
            min_lon = min_lon.min(loc.lon);
            max_lon = max_lon.max(loc.lon);
        }

        Some(GeoBounds::new(min_lat, max_lat, min_lon, max_lon))
    }

    fn compute_centroid(locations: &[&Location]) -> Option<Location> {
        if locations.is_empty() {
            return None;
        }

        // Convert to Cartesian for proper averaging across antimeridian
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        for loc in locations {
            let lat_rad = loc.lat.to_radians();
            let lon_rad = loc.lon.to_radians();

            x += lat_rad.cos() * lon_rad.cos();
            y += lat_rad.cos() * lon_rad.sin();
            z += lat_rad.sin();
        }

        let n = locations.len() as f64;
        x /= n;
        y /= n;
        z /= n;

        let lon = y.atan2(x).to_degrees();
        let hyp = (x * x + y * y).sqrt();
        let lat = z.atan2(hyp).to_degrees();

        // Average elevation if any have it
        let total_elev: f64 = locations.iter().filter_map(|l| l.elevation).sum();
        let elev_count = locations.iter().filter(|l| l.elevation.is_some()).count();
        let elevation = if elev_count > 0 {
            Some(total_elev / elev_count as f64)
        } else {
            None
        };

        Some(Location {
            lat,
            lon,
            elevation,
            uncertainty_meters: None,
            name: None,
        })
    }

    fn compute_consecutive_distances(locations: &[&Location]) -> (f64, f64, f64) {
        if locations.len() < 2 {
            return (0.0, 0.0, 0.0);
        }

        let mut total = 0.0;
        let mut max = 0.0_f64;

        for window in locations.windows(2) {
            let dist =
                haversine_distance(window[0].lat, window[0].lon, window[1].lat, window[1].lon);
            total += dist;
            max = max.max(dist);
        }

        let avg = total / (locations.len() - 1) as f64;
        (total, avg, max)
    }

    fn compute_dispersion(locations: &[&Location], centroid: &Location) -> f64 {
        if locations.is_empty() {
            return 0.0;
        }

        let total_dist: f64 = locations
            .iter()
            .map(|loc| haversine_distance(loc.lat, loc.lon, centroid.lat, centroid.lon))
            .sum();

        total_dist / locations.len() as f64
    }
}

/// Earth radius in meters.
const EARTH_RADIUS_M: f64 = 6_371_000.0;

/// Compute the Haversine distance between two points in meters.
///
/// # Examples
///
/// ```
/// use spatial_narrative::analysis::haversine_distance;
///
/// // NYC to LA
/// let dist = haversine_distance(40.7128, -74.0060, 34.0522, -118.2437);
/// assert!((dist - 3_944_000.0).abs() < 10_000.0); // ~3944 km
/// ```
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    EARTH_RADIUS_M * c
}

/// Compute the initial bearing from point 1 to point 2 in degrees.
///
/// Returns a value between 0 and 360 degrees.
pub fn bearing(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lon = (lon2 - lon1).to_radians();

    let x = delta_lon.sin() * lat2_rad.cos();
    let y = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * delta_lon.cos();

    let bearing_rad = x.atan2(y);
    (bearing_rad.to_degrees() + 360.0) % 360.0
}

/// Compute destination point given start, bearing, and distance.
///
/// # Arguments
///
/// * `lat` - Starting latitude in degrees
/// * `lon` - Starting longitude in degrees
/// * `bearing_deg` - Bearing in degrees (0 = North, 90 = East)
/// * `distance_m` - Distance in meters
///
/// # Returns
///
/// Tuple of (latitude, longitude) for the destination point.
pub fn destination_point(lat: f64, lon: f64, bearing_deg: f64, distance_m: f64) -> (f64, f64) {
    let lat_rad = lat.to_radians();
    let lon_rad = lon.to_radians();
    let bearing_rad = bearing_deg.to_radians();
    let angular_dist = distance_m / EARTH_RADIUS_M;

    let dest_lat = (lat_rad.sin() * angular_dist.cos()
        + lat_rad.cos() * angular_dist.sin() * bearing_rad.cos())
    .asin();

    let dest_lon = lon_rad
        + (bearing_rad.sin() * angular_dist.sin() * lat_rad.cos())
            .atan2(angular_dist.cos() - lat_rad.sin() * dest_lat.sin());

    (dest_lat.to_degrees(), dest_lon.to_degrees())
}

/// Density map cell for spatial density analysis.
#[derive(Debug, Clone)]
pub struct DensityCell {
    /// Cell center latitude.
    pub lat: f64,
    /// Cell center longitude.
    pub lon: f64,
    /// Event count in this cell.
    pub count: usize,
    /// Density (events per square km).
    pub density: f64,
}

/// Compute a density map for the given events.
///
/// Divides the bounding box into a grid and counts events per cell.
///
/// # Arguments
///
/// * `events` - Slice of events to analyze
/// * `rows` - Number of rows in the grid
/// * `cols` - Number of columns in the grid
///
/// # Returns
///
/// Vector of density cells, row-major order.
pub fn density_map(events: &[Event], rows: usize, cols: usize) -> Vec<DensityCell> {
    if events.is_empty() || rows == 0 || cols == 0 {
        return Vec::new();
    }

    let locations: Vec<&Location> = events.iter().map(|e| &e.location).collect();
    let bounds = match SpatialMetrics::compute_bounds(&locations) {
        Some(b) => b,
        None => return Vec::new(),
    };

    let lat_step = (bounds.max_lat - bounds.min_lat) / rows as f64;
    let lon_step = (bounds.max_lon - bounds.min_lon) / cols as f64;

    // Count events per cell
    let mut counts = vec![vec![0usize; cols]; rows];

    for loc in &locations {
        let row = ((loc.lat - bounds.min_lat) / lat_step).floor() as usize;
        let col = ((loc.lon - bounds.min_lon) / lon_step).floor() as usize;

        let row = row.min(rows - 1);
        let col = col.min(cols - 1);

        counts[row][col] += 1;
    }

    // Compute cell area in square km (approximate)
    let mut cells = Vec::with_capacity(rows * cols);

    for (row, count_row) in counts.iter().enumerate() {
        for (col, &count) in count_row.iter().enumerate() {
            let cell_lat = bounds.min_lat + (row as f64 + 0.5) * lat_step;
            let cell_lon = bounds.min_lon + (col as f64 + 0.5) * lon_step;

            // Approximate cell area
            let width_m = haversine_distance(
                cell_lat,
                bounds.min_lon,
                cell_lat,
                bounds.min_lon + lon_step,
            );
            let height_m = haversine_distance(
                bounds.min_lat,
                cell_lon,
                bounds.min_lat + lat_step,
                cell_lon,
            );
            let area_km2 = (width_m * height_m) / 1_000_000.0;

            let density = if area_km2 > 0.0 {
                count as f64 / area_km2
            } else {
                0.0
            };

            cells.push(DensityCell {
                lat: cell_lat,
                lon: cell_lon,
                count,
                density,
            });
        }
    }

    cells
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Timestamp;

    fn make_event(lat: f64, lon: f64) -> Event {
        Event::new(Location::new(lat, lon), Timestamp::now(), "test")
    }

    #[test]
    fn test_haversine_distance() {
        // NYC to LA
        let dist = haversine_distance(40.7128, -74.0060, 34.0522, -118.2437);
        assert!((dist - 3_944_000.0).abs() < 50_000.0); // Within 50km

        // Same point
        let dist = haversine_distance(40.7128, -74.0060, 40.7128, -74.0060);
        assert!(dist < 1.0); // Less than 1 meter
    }

    #[test]
    fn test_bearing() {
        // Due east
        let b = bearing(0.0, 0.0, 0.0, 1.0);
        assert!((b - 90.0).abs() < 0.1);

        // Due north
        let b = bearing(0.0, 0.0, 1.0, 0.0);
        assert!(b < 0.1 || (b - 360.0).abs() < 0.1);
    }

    #[test]
    fn test_spatial_metrics_empty() {
        let metrics = SpatialMetrics::from_events(&[]);
        assert_eq!(metrics.event_count, 0);
        assert!(metrics.bounds.is_none());
        assert!(metrics.centroid.is_none());
    }

    #[test]
    fn test_spatial_metrics_single_event() {
        let events = vec![make_event(40.0, -74.0)];
        let metrics = SpatialMetrics::from_events(&events);

        assert_eq!(metrics.event_count, 1);
        assert!(metrics.bounds.is_some());
        assert!(metrics.centroid.is_some());
        assert_eq!(metrics.total_distance, 0.0);
        assert_eq!(metrics.dispersion, 0.0);
    }

    #[test]
    fn test_spatial_metrics_multiple_events() {
        let events = vec![
            make_event(40.0, -74.0),
            make_event(41.0, -73.0),
            make_event(39.0, -75.0),
        ];
        let metrics = SpatialMetrics::from_events(&events);

        assert_eq!(metrics.event_count, 3);
        assert!(metrics.total_distance > 0.0);
        assert!(metrics.dispersion > 0.0);

        let centroid = metrics.centroid.unwrap();
        assert!((centroid.lat - 40.0).abs() < 1.0);
        assert!((centroid.lon - (-74.0)).abs() < 1.0);
    }

    #[test]
    fn test_density_map() {
        let events = vec![
            make_event(0.0, 0.0),
            make_event(0.1, 0.1),
            make_event(0.9, 0.9),
        ];

        let cells = density_map(&events, 2, 2);
        assert_eq!(cells.len(), 4);

        // Total counts should equal number of events
        let total: usize = cells.iter().map(|c| c.count).sum();
        assert_eq!(total, 3);
    }
}
