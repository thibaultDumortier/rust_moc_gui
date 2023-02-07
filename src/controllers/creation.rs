use core::fmt;

use moc::storage::u64idx::U64MocStore;

use crate::utils::commons::*;

const JD_TO_USEC: f64 = (24_u64 * 60 * 60 * 1_000_000) as f64;

#[derive(Copy, Clone)]
pub(crate) enum CreationType {
    Box,
    Cone,
    Coo,
    DecimalJd,
    DecimalJdRange,
    EllipticalCone,
    LargeCone,
    Polygon,
    Ring,
    SmallCone,
    ValuedCells,
    Zone,
}
impl fmt::Display for CreationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cone => write!(f, "Cone"),
            Self::Ring => write!(f, "Ring"),
            Self::EllipticalCone => write!(f, "EllipticalCone"),
            Self::Zone => write!(f, "Zone"),
            Self::Box => write!(f, "Box"),
            Self::Polygon => write!(f, "Polygon"),
            Self::Coo => write!(f, "Coo"),
            Self::SmallCone => write!(f, "SmallCone"),
            Self::LargeCone => write!(f, "LargeCone"),
            Self::DecimalJd => write!(f, "DecimalJd"),
            Self::DecimalJdRange => write!(f, "DecimalJdRange"),
            Self::ValuedCells => write!(f, "ValuedCells"),
        }
    }
}
impl PartialEq for CreationType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Cone, Self::Cone)
                | (Self::Ring, Self::Ring)
                | (Self::EllipticalCone, Self::EllipticalCone)
                | (Self::Zone, Self::Zone)
                | (Self::Box, Self::Box)
                | (Self::Polygon, Self::Polygon)
                | (Self::Coo, Self::Coo)
                | (Self::SmallCone, Self::SmallCone)
                | (Self::LargeCone, Self::LargeCone)
                | (Self::DecimalJd, Self::DecimalJd)
                | (Self::DecimalJdRange, Self::DecimalJdRange)
                | (Self::ValuedCells, Self::ValuedCells)
        )
    }
}
impl Default for CreationType {
    fn default() -> Self {
        CreationType::Cone
    }
}

/// Create a new MOC from the given polygon vertices.
/// # Params
/// * `name`: the name to be given to the MOC
/// * `depth`: MOC maximum depth in `[0, 29]`
/// * `vertices_deg`: vertices coordinates in degrees `[lon_v1, lat_v1, lon_v2, lat_v2, ..., lon_vn, lat_vn]`
/// * `complement`: reverse the default inside/outside of the polygon
pub fn from_polygon(depth: u8, content: String, complement: bool) -> Result<usize, String> {
    let v = vector_splitter(content);

    // An other solution would be to go unsafe to transmute in Box<[[f64; 2]]> ...
    U64MocStore.from_polygon(
        v.iter()
            .step_by(2)
            .zip(v.iter().skip(1).step_by(2))
            .filter_map(|(lon_deg, lat_deg)| {
                let lon = lon_deg2rad(*lon_deg).ok();
                let lat = lat_deg2rad(*lat_deg).ok();
                match (lon, lat) {
                    (Some(lon), Some(lat)) => Some((lon, lat)),
                    _ => None,
                }
            }),
        complement,
        depth,
    )
}

/// Create a new MOC from the given list of coordinates (assumed to be equatorial)
/// # Params
/// * `name`: the name to be given to the MOC
/// * `depth`: MOC maximum depth in `[0, 29]`
/// * `coos_deg`: list of coordinates in degrees `[lon_1, lat_1, lon_2, lat_2, ..., lon_n, lat_n]`
pub fn from_coo(depth: u8, content: String) -> Result<usize, String> {
    let v = vector_splitter(content);

    // An other solution would be to go unsafe to transmute coos_deg in Box<[[f64; 2]]> ...
    U64MocStore.from_coo(
        depth,
        v.iter()
            .step_by(2)
            .zip(v.iter().skip(1).step_by(2))
            .filter_map(|(lon_deg, lat_deg)| {
                let lon = lon_deg2rad(*lon_deg).ok();
                let lat = lat_deg2rad(*lat_deg).ok();
                match (lon, lat) {
                    (Some(lon), Some(lat)) => Some((lon, lat)),
                    _ => None,
                }
            }),
    )
}

/// Create a new MOC from the given list of cone centers and radii
/// Adapted for a large number of small cones (a few cells each).
/// # Params
/// * `name`: the name to be given to the MOC
/// * `depth`: MOC maximum depth in `[0, 29]`
/// * `coos_and_radius_deg`: list of coordinates adn radii in degrees
///   `[lon_1, lat_1, rad_1, lon_2, lat_2, rad_2, ..., lon_n, lat_n, rad_n]`
pub fn from_small_cones(depth: u8, content: String) -> Result<usize, String> {
    let v = vector_splitter(content);

    U64MocStore.from_small_cones(
        depth,
        2,
        v.iter()
            .step_by(3)
            .zip(v.iter().skip(1).step_by(3))
            .zip(v.iter().skip(2).step_by(3))
            .filter_map(|((lon_deg, lat_deg), radius_deg)| {
                let lon = lon_deg2rad(*lon_deg).ok();
                let lat = lat_deg2rad(*lat_deg).ok();
                match (lon, lat) {
                    (Some(lon), Some(lat)) => Some(((lon, lat), (*radius_deg).to_radians())),
                    _ => None,
                }
            }),
    )
}

/// Create a new MOC from the given list of cone centers and radii
/// Adapted for a reasonable number of possibly large cones.
/// # Params
/// * `name`: the name to be given to the MOC
/// * `depth`: MOC maximum depth in `[0, 29]`
/// * `coos_and_radius_deg`: list of coordinates adn radii in degrees
///   `[lon_1, lat_1, rad_1, lon_2, lat_2, rad_2, ..., lon_n, lat_n, rad_n]`
pub fn from_large_cones(depth: u8, content: String) -> Result<usize, String> {
    let v = vector_splitter(content);

    U64MocStore.from_large_cones(
        depth,
        2,
        v.iter()
            .step_by(3)
            .zip(v.iter().skip(1).step_by(3))
            .zip(v.iter().skip(2).step_by(3))
            .filter_map(|((lon_deg, lat_deg), radius_deg)| {
                let lon = lon_deg2rad(*lon_deg).ok();
                let lat = lat_deg2rad(*lat_deg).ok();
                match (lon, lat) {
                    (Some(lon), Some(lat)) => Some(((lon, lat), (*radius_deg).to_radians())),
                    _ => None,
                }
            }),
    )
}

/// Create a new T-MOC from the given list of decimal Julian Days (JD) times.
/// # Params
/// * `name`: the name to be given to the MOC
/// * `depth`: T-MOC maximum depth in `[0, 61]`
/// * `jd`: array of decimal JD time (`f64`)
/// # WARNING
/// Using decimal Julian Days stored on `f64`, the precision does not reach the microsecond
/// since JD=0.
/// In Javascript, there is no `u64` type (integers are stored on the mantissa of
/// a double -- a `f64` --, which is made of 52 bits).
/// The other approach is to use a couple of `f64`: one for the integer part of the JD, the
/// other for the fractional part of the JD.
/// We will add such a method later if required by users.
pub fn from_decimal_jd(depth: u8, content: String) -> Result<usize, String> {
    let v = vector_splitter(content);

    U64MocStore.from_decimal_jd_values(depth, v.iter().map(|jd| (jd * JD_TO_USEC)))
}

pub fn from_decimal_jd_range(depth: u8, content: String) -> Result<usize, String> {
    let v = vector_splitter(content);

    U64MocStore.from_decimal_jd_ranges(
        depth,
        v.iter()
            .step_by(2)
            .zip(v.iter().skip(1).step_by(2))
            .map(|(jd_min, jd_max)| (jd_min * JD_TO_USEC)..(jd_max * JD_TO_USEC)),
    )
}

/// Create a new S-MOC from the given lists of UNIQ and Values.
/// # Params
/// * `name`: the name to be given to the MOC
/// * `depth`: S-MOC maximum depth in `[0, 29]`, Must be >= largest input cells depth.
/// * `density`: Input values are densities, i.e. they are not proportional to the area of their associated cells.
/// * `from_threshold`: Cumulative value at which we start putting cells in he MOC (often = 0).
/// * `to_threshold`: Cumulative value at which we stop putting cells in the MOC.
/// * `asc`: Compute cumulative value from ascending density values instead of descending (often = false).
/// * `not_strict`: Cells overlapping with the upper or the lower cumulative bounds are not rejected (often = false).
/// * `split`: Split recursively the cells overlapping the upper or the lower cumulative bounds (often = false).
/// * `revese_recursive_descent`: Perform the recursive descent from the highest to the lowest sub-cell, only with option 'split' (set both flags to be compatibile with Aladin)
/// * `uniqs`: array of uniq HEALPix cells
/// * `values`: array of values associated to the HEALPix cells
pub fn from_valued_cells(
    depth: u8,
    density: bool,
    from_threshold: f64,
    to_threshold: f64,
    asc: bool,
    not_strict: bool,
    split: bool,
    revese_recursive_descent: bool,
    content: String,
) -> Result<usize, String> {
    let mut v: Vec<f64> = Vec::default();

    let f: Vec<&str> = content.split(|c| c == ',' || c == '\n').collect();
    // Split on line returns too
    let mut tmp: Vec<f64> = f.iter().filter_map(|f| (*f).parse::<f64>().ok()).collect();
    v.append(&mut tmp);

    let uniq_vals = v
        .iter()
        .zip(v.iter().step_by(2))
        .map(|(uniq, val)| (*uniq as u64, *val));
    U64MocStore.from_valued_cells(
        depth,
        density,
        from_threshold,
        to_threshold,
        asc,
        not_strict,
        split,
        revese_recursive_descent,
        uniq_vals,
    )
}

pub(crate) fn lon_deg2rad(lon_deg: f64) -> Result<f64, String> {
    let lon = lon_deg.to_radians();
    if !(0.0..TWICE_PI).contains(&lon) {
        Err("Longitude must be in [0, 2pi[".to_string())
    } else {
        Ok(lon)
    }
}

pub(crate) fn lat_deg2rad(lat_deg: f64) -> Result<f64, String> {
    let lat = lat_deg.to_radians();
    if !(-HALF_PI..HALF_PI).contains(&lat) {
        Err("Latitude must be in [-pi/2, pi/2]".to_string())
    } else {
        Ok(lat)
    }
}

fn vector_splitter(content: String) -> Vec<f64> {
    let mut v: Vec<f64> = Vec::default();

    let f: Vec<&str> = content.split(|c| c == ',' || c == '\n').collect();
    // Split on line returns too
    let mut tmp: Vec<f64> = f.iter().filter_map(|f| (*f).parse::<f64>().ok()).collect();
    v.append(&mut tmp);

    v
}
