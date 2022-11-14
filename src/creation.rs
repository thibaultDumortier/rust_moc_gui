use core::fmt;

use moc::{
    elem::valuedcell::valued_cells_to_moc_with_opt,
    elemset::range::HpxRanges,
    moc::range::RangeMOC,
    qty::{Hpx, Time},
};

use crate::{commons::*, store};

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

pub fn from_cone(
    name: &str,
    depth: u8,
    lon_deg: f64,
    lat_deg: f64,
    radius_deg: f64,
) -> Result<(), String> {
    let lon = lon_deg2rad(lon_deg)?;
    let lat = lat_deg2rad(lat_deg)?;
    let r = radius_deg.to_radians();
    if r <= 0.0 || PI <= r {
        Err("Radius must be in ]0, pi[".to_string())
    } else {
        let moc: RangeMOC<u64, Hpx<u64>> = RangeMOC::from_cone(lon, lat, r, depth, 2);
        store::add(name, InternalMoc::Space(moc))
    }
}

pub fn from_ring(
    name: &str,
    depth: u8,
    lon_deg: f64,
    lat_deg: f64,
    internal_radius_deg: f64,
    external_radius_deg: f64,
) -> Result<(), String> {
    let lon = lon_deg2rad(lon_deg)?;
    let lat = lat_deg2rad(lat_deg)?;
    let r_int = internal_radius_deg.to_radians();
    let r_ext = external_radius_deg.to_radians();
    if r_int <= 0.0 || PI <= r_int {
        Err("Internal radius must be in ]0, pi[".to_string())
    } else if r_ext <= 0.0 || PI <= r_ext {
        Err("External radius must be in ]0, pi[".to_string())
    } else if r_ext < r_int {
        Err("External radius must be larger than the internal radius".to_string())
    } else {
        let moc: RangeMOC<u64, Hpx<u64>> = RangeMOC::from_ring(lon, lat, r_int, r_ext, depth, 2);
        store::add(name, InternalMoc::Space(moc))
    }
}

pub fn from_elliptical_cone(
    name: &str,
    depth: u8,
    lon_deg: f64,
    lat_deg: f64,
    a_deg: f64,
    b_deg: f64,
    pa_deg: f64,
) -> Result<(), String> {
    let lon = lon_deg2rad(lon_deg)?;
    let lat = lat_deg2rad(lat_deg)?;
    let a = a_deg.to_radians();
    let b = b_deg.to_radians();
    let pa = pa_deg.to_radians();
    if a <= 0.0 || HALF_PI <= a {
        Err("Semi-major axis must be in ]0, pi/2]".to_string())
    } else if b <= 0.0 || a <= b {
        Err("Semi-minor axis must be in ]0, a[".to_string())
    } else if !(0.0..HALF_PI).contains(&pa) {
        Err("Position angle must be in [0, pi[".to_string())
    } else {
        let moc: RangeMOC<u64, Hpx<u64>> =
            RangeMOC::from_elliptical_cone(lon, lat, a, b, pa, depth, 2);
        store::add(name, InternalMoc::Space(moc))
    }
}

pub fn from_zone(
    name: &str,
    depth: u8,
    lon_deg_min: f64,
    lat_deg_min: f64,
    lon_deg_max: f64,
    lat_deg_max: f64,
) -> Result<(), String> {
    let lon_min = lon_deg2rad(lon_deg_min)?;
    let lat_min = lat_deg2rad(lat_deg_min)?;
    let lon_max = lon_deg2rad(lon_deg_max)?;
    let lat_max = lat_deg2rad(lat_deg_max)?;
    let moc: RangeMOC<u64, Hpx<u64>> =
        RangeMOC::from_zone(lon_min, lat_min, lon_max, lat_max, depth);
    store::add(name, InternalMoc::Space(moc))
}

pub fn from_box(
    name: &str,
    depth: u8,
    lon_deg: f64,
    lat_deg: f64,
    a_deg: f64,
    b_deg: f64,
    pa_deg: f64,
) -> Result<(), String> {
    let lon = lon_deg2rad(lon_deg)?;
    let lat = lat_deg2rad(lat_deg)?;
    let a = a_deg.to_radians();
    let b = b_deg.to_radians();
    let pa = pa_deg.to_radians();
    if a <= 0.0 || HALF_PI <= a {
        Err("Semi-major axis must be in ]0, pi/2]".to_string())
    } else if b <= 0.0 || a < b {
        Err("Semi-minor axis must be in ]0, a[".to_string())
    } else if !(0.0..PI).contains(&pa) {
        Err("Position angle must be in [0, pi[".to_string())
    } else {
        let moc: RangeMOC<u64, Hpx<u64>> = RangeMOC::from_box(lon, lat, a, b, pa, depth);
        store::add(name, InternalMoc::Space(moc))
    }
}

/// Create a new MOC from the given polygon vertices.
/// # Params
/// * `name`: the name to be given to the MOC
/// * `depth`: MOC maximum depth in `[0, 29]`
/// * `vertices_deg`: vertices coordinates in degrees `[lon_v1, lat_v1, lon_v2, lat_v2, ..., lon_vn, lat_vn]`
/// * `complement`: reverse the default inside/outside of the polygon
pub fn from_polygon(
    name: &str,
    depth: u8,
    vertices_deg: Box<[f64]>,
    complement: bool,
) -> Result<(), String> {
    // An other solution would be to go unsafe to transmute in Box<[[f64; 2]]> ...
    let vertices = vertices_deg
        .iter()
        .step_by(2)
        .zip(vertices_deg.iter().skip(1).step_by(2))
        .map(|(lon_deg, lat_deg)| {
            let lon = lon_deg2rad(*lon_deg)?;
            let lat = lat_deg2rad(*lat_deg)?;
            Ok((lon, lat))
        })
        .collect::<Result<Vec<(f64, f64)>, String>>()?;
    let moc: RangeMOC<u64, Hpx<u64>> = RangeMOC::from_polygon(&vertices, complement, depth);
    store::add(name, InternalMoc::Space(moc))
}

/// Create a new MOC from the given list of coordinates (assumed to be equatorial)
/// # Params
/// * `name`: the name to be given to the MOC
/// * `depth`: MOC maximum depth in `[0, 29]`
/// * `coos_deg`: list of coordinates in degrees `[lon_1, lat_1, lon_2, lat_2, ..., lon_n, lat_n]`
pub fn from_coo(name: &str, depth: u8, coos_deg: Box<[f64]>) -> Result<(), String> {
    // An other solution would be to go unsafe to transmute coos_deg in Box<[[f64; 2]]> ...
    let moc: RangeMOC<u64, Hpx<u64>> = RangeMOC::from_coos(
        depth,
        coos_deg
            .iter()
            .step_by(2)
            .zip(coos_deg.iter().skip(1).step_by(2))
            .filter_map(|(lon_deg, lat_deg)| {
                let lon = lon_deg2rad(*lon_deg).ok();
                let lat = lat_deg2rad(*lat_deg).ok();
                match (lon, lat) {
                    (Some(lon), Some(lat)) => Some((lon, lat)),
                    _ => None,
                }
            }),
        None,
    );
    store::add(name, InternalMoc::Space(moc))
}

/// Create a new MOC from the given list of cone centers and radii
/// Adapted for a large number of small cones (a few cells each).
/// # Params
/// * `name`: the name to be given to the MOC
/// * `depth`: MOC maximum depth in `[0, 29]`
/// * `coos_and_radius_deg`: list of coordinates adn radii in degrees
///   `[lon_1, lat_1, rad_1, lon_2, lat_2, rad_2, ..., lon_n, lat_n, rad_n]`
pub fn from_small_cones(
    name: &str,
    depth: u8,
    coos_and_radius_deg: Box<[f64]>,
) -> Result<(), String> {
    let coos_rad = coos_and_radius_deg
        .iter()
        .step_by(3)
        .zip(coos_and_radius_deg.iter().skip(1).step_by(3))
        .zip(coos_and_radius_deg.iter().skip(2).step_by(3))
        .filter_map(|((lon_deg, lat_deg), radius_deg)| {
            let lon = lon_deg2rad(*lon_deg).ok();
            let lat = lat_deg2rad(*lat_deg).ok();
            match (lon, lat) {
                (Some(lon), Some(lat)) => Some((lon, lat, (*radius_deg).to_radians())),
                _ => None,
            }
        });
    let moc: RangeMOC<u64, Hpx<u64>> = RangeMOC::from_small_cones(depth, 2, coos_rad, None);
    store::add(name, InternalMoc::Space(moc))
}

/// Create a new MOC from the given list of cone centers and radii
/// Adapted for a reasonable number of possibly large cones.
/// # Params
/// * `name`: the name to be given to the MOC
/// * `depth`: MOC maximum depth in `[0, 29]`
/// * `coos_and_radius_deg`: list of coordinates adn radii in degrees
///   `[lon_1, lat_1, rad_1, lon_2, lat_2, rad_2, ..., lon_n, lat_n, rad_n]`
pub fn from_large_cones(
    name: &str,
    depth: u8,
    coos_and_radius_deg: Box<[f64]>,
) -> Result<(), String> {
    let coos_rad = coos_and_radius_deg
        .iter()
        .step_by(3)
        .zip(coos_and_radius_deg.iter().skip(1).step_by(3))
        .zip(coos_and_radius_deg.iter().skip(2).step_by(3))
        .filter_map(|((lon_deg, lat_deg), radius_deg)| {
            let lon = lon_deg2rad(*lon_deg).ok();
            let lat = lat_deg2rad(*lat_deg).ok();
            match (lon, lat) {
                (Some(lon), Some(lat)) => Some((lon, lat, (*radius_deg).to_radians())),
                _ => None,
            }
        });
    let moc: RangeMOC<u64, Hpx<u64>> = RangeMOC::from_large_cones(depth, 2, coos_rad);
    store::add(name, InternalMoc::Space(moc))
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
pub fn from_decimal_jd(name: &str, depth: u8, jd: Box<[f64]>) -> Result<(), String> {
    let moc = RangeMOC::<u64, Time<u64>>::from_microsec_since_jd0(
        depth,
        jd.iter().map(|jd| (jd * JD_TO_USEC) as u64),
        None,
    );
    store::add(name, InternalMoc::Time(moc))
}

pub fn from_decimal_jd_range(name: &str, depth: u8, jd_ranges: Box<[f64]>) -> Result<(), String> {
    let moc = RangeMOC::<u64, Time<u64>>::from_microsec_ranges_since_jd0(
        depth,
        jd_ranges
            .iter()
            .step_by(2)
            .zip(jd_ranges.iter().skip(1).step_by(2))
            .map(|(jd_min, jd_max)| (jd_min * JD_TO_USEC) as u64..(jd_max * JD_TO_USEC) as u64),
        None,
    );
    store::add(name, InternalMoc::Time(moc))
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
    name: &str,
    depth: u8,
    density: bool,
    from_threshold: f64,
    to_threshold: f64,
    asc: bool,
    not_strict: bool,
    split: bool,
    revese_recursive_descent: bool,
    uniqs: Box<[f64]>,
    values: Box<[f64]>,
) -> Result<(), String> {
    let depth = depth.max(
        uniqs
            .iter()
            .map(|uniq| Hpx::<u64>::from_uniq_hpx(*uniq as u64).0)
            .max()
            .unwrap_or(depth),
    );
    let area_per_cell = (PI / 3.0) / (1_u64 << (depth << 1) as u32) as f64; // = 4pi / (12*4^depth)
    let ranges: HpxRanges<u64> = if density {
        valued_cells_to_moc_with_opt::<u64, f64>(
            depth,
            uniqs
                .iter()
                .zip(values.iter())
                .map(|(uniq, dens)| {
                    let uniq = *uniq as u64;
                    let (cdepth, _ipix) = Hpx::<u64>::from_uniq_hpx(uniq);
                    let n_sub_cells = (1_u64 << (((depth - cdepth) << 1) as u32)) as f64;
                    (uniq, dens * n_sub_cells * area_per_cell, *dens)
                })
                .collect(),
            from_threshold,
            to_threshold,
            asc,
            !not_strict,
            !split,
            revese_recursive_descent,
        )
    } else {
        valued_cells_to_moc_with_opt::<u64, f64>(
            depth,
            uniqs
                .iter()
                .zip(values.iter())
                .map(|(uniq, val)| {
                    let uniq = *uniq as u64;
                    let (cdepth, _ipix) = Hpx::<u64>::from_uniq_hpx(uniq);
                    let n_sub_cells = (1_u64 << (((depth - cdepth) << 1) as u32)) as f64;
                    (uniq, *val, val / (n_sub_cells * area_per_cell))
                })
                .collect(),
            from_threshold,
            to_threshold,
            asc,
            !not_strict,
            !split,
            revese_recursive_descent,
        )
    };
    let moc = RangeMOC::new(depth, ranges);
    store::add(name, InternalMoc::Space(moc))
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
