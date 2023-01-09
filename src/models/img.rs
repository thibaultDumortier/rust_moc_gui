//! Contains code to generate an image from a given MOC

use std::{
    f64::consts::PI,
    fs::File,
    io::{self, BufWriter, Write},
    ops::RangeInclusive,
    path::Path,
};

use healpix::nested;

use mapproj::{
    img2celestial::Img2Celestial,
    img2proj::ReversedEastPngImgXY2ProjXY,
    math::{HALF_PI},
    pseudocyl::mol::Mol,
    zenithal::sin::Sin,
    CanonicalProjection, CenteredProjection, ImgXY, LonLat,
};

use moc::{
    elem::cell::Cell,
    idx::Idx,
    moc::{range::RangeMOC, CellMOCIterator, RangeMOCIntoIterator, RangeMOCIterator},
    qty::{Hpx, MocQty},
};

pub fn to_img_auto<T: Idx>(smoc: &RangeMOC<T, Hpx<T>>, img_size_y: u16) -> (Vec<u8>, (u16, u16)) {
    let (lon, lat) = smoc.into_range_moc_iter().cells().mean_center();
    let r_max = smoc
        .into_range_moc_iter()
        .cells()
        .max_distance_from(lon, lat);

    let proj_center = Some((lon, lat));
    if r_max > HALF_PI {
        // Mollweide, all sky
        let img_size = (img_size_y << 1, img_size_y);
        let rgba = to_img_default(smoc, img_size, proj_center, None);
        (rgba, img_size)
    } else {
        // Sinus, computed bounds from d_max
        let img_size = (img_size_y, img_size_y);
        let bound = r_max.sin() * 1.05; // add 5% of the distance
        let proj_bounds = Some((-bound..=bound, -bound..=bound));
        let rgba = to_img(smoc, img_size, Sin::new(), proj_center, proj_bounds);
        (rgba, img_size)
    }
}

/// Returns an RGBA array (each pixel is made of 4 successive u8: RGBA) using the Mollweide projection.
///
/// # Params
/// * `smoc`: the Spatial MOC to be print;
/// * `size`: the `(X, Y)` number of pixels in the image;
/// * `proj_center`: the `(lon, lat)` coordinates of the center of the projection, in radians,
///                      if different from `(0, 0)`;
/// * `proj_bounds`: the `(X, Y)` bounds of the projection, if different from the default values
///                  which depends on the projection. For unbounded projections, de default value
///                  is `(-PI..PI, -PI..PI)`.
pub fn to_img_default<T: Idx>(
    smoc: &RangeMOC<T, Hpx<T>>,
    img_size: (u16, u16),
    proj_center: Option<(f64, f64)>,
    proj_bounds: Option<(RangeInclusive<f64>, RangeInclusive<f64>)>,
) -> Vec<u8> {
    to_img(smoc, img_size, Mol::new(), proj_center, proj_bounds)
}

/// Returns an RGBA array (each pixel is made of 4 successive u8: RGBA).
///
/// # Params
/// * `smoc`: the Spatial MOC to be print;
/// * `size`: the `(X, Y)` number of pixels in the image;
/// * `proj`: a projection, if different from Mollweide;
/// * `proj_center`: the `(lon, lat)` coordinates of the center of the projection, in radians,
///                      if different from `(0, 0)`;
/// * `proj_bounds`: the `(X, Y)` bounds of the projection, if different from the default values
///                  which depends on the projection. For unbounded projections, de default value
///                  is `(-PI..PI, -PI..PI)`.
pub fn to_img<T: Idx, P: CanonicalProjection>(
    smoc: &RangeMOC<T, Hpx<T>>,
    img_size: (u16, u16),
    proj: P,
    proj_center: Option<(f64, f64)>,
    proj_bounds: Option<(RangeInclusive<f64>, RangeInclusive<f64>)>,
) -> Vec<u8> {
    let (size_x, size_y) = img_size;
    let mut v: Vec<u8> = Vec::with_capacity((size_x as usize * size_y as usize) << 2);

    let (proj_range_x, proj_range_y) = proj_bounds.unwrap_or((
        proj.bounds()
            .x_bounds()
            .as_ref()
            .cloned()
            .unwrap_or(-PI..=PI),
        proj.bounds()
            .y_bounds()
            .as_ref()
            .cloned()
            .unwrap_or(-PI..=PI),
    ));

    let img2proj =
        ReversedEastPngImgXY2ProjXY::from((size_x, size_y), (&proj_range_x, &proj_range_y));
    let mut img2cel = Img2Celestial::new(img2proj, CenteredProjection::new(proj));
    if let Some((lon, lat)) = proj_center {
        img2cel.set_proj_center_from_lonlat(&LonLat::new(lon, lat));
    }

    let hpx = nested::get(Hpx::<u64>::MAX_DEPTH);
    // First check all pixels and see if thier center is in the MOC
    for y in 0..size_y {
        for x in 0..size_x {
            if let Some(lonlat) = img2cel.img2lonlat(&ImgXY::new(x as f64, y as f64)) {
                let idx = hpx.hash(lonlat.lon(), lonlat.lat());
                if smoc.contains_val(&T::from_u64_idx(idx)) {
                    // in the moc
                    v.push(255);
                    v.push(0);
                    v.push(0);
                    v.push(255);
                } else {
                    // out of the moc
                    v.push(0);
                    v.push(0);
                    v.push(0);
                    v.push(255);
                }
            } else {
                // Not in the proj area
                v.push(255);
                v.push(255);
                v.push(255);
                v.push(0);
            }
        }
    }
    // But, in case of sparse MOC, also light up the pixel containing a cell center
    for Cell { depth, idx } in smoc.into_range_moc_iter().cells() {
        let (lon_rad, lat_rad) = nested::center(depth, idx.to_u64());
        if let Some(xy) = img2cel.lonlat2img(&LonLat::new(lon_rad, lat_rad)) {
            let ix = xy.x();
            let iy = xy.y();
            if proj_range_x.contains(&ix) && proj_range_y.contains(&iy) {
                let from = (iy as usize * size_x as usize + ix as usize) << 2; // <<2 <=> *4
                if v[from] == 0 {
                    v[from] = 255;
                    v[from + 1] = 0;
                    v[from + 2] = 0;
                    v[from + 3] = 128;
                }
            }
        }
    }
    v
}

/// # Params
/// * `smoc`: the Spatial MOC to be print;
/// * `size`: the `(X, Y)` number of pixels in the image;
/// * `proj`: a projection, if different from Mollweide;
/// * `proj_center`: the `(lon, lat)` coordinates of the center of the projection, in radians,
///                      if different from `(0, 0)`;
/// * `proj_bounds`: the `(X, Y)` bounds of the projection, if different from the default values
///                  which depends on the projection. For unbounded projections, de default value
///                  is `(-PI..PI, -PI..PI)`.
/// * `writer`: the writer in which the image is going to be written
pub fn to_png<T: Idx, P: CanonicalProjection, W: Write>(
    smoc: &RangeMOC<T, Hpx<T>>,
    img_size: (u16, u16),
    proj: Option<P>,
    proj_center: Option<(f64, f64)>,
    proj_bounds: Option<(RangeInclusive<f64>, RangeInclusive<f64>)>,
    writer: W,
) -> Result<(), io::Error> {
    let (xsize, ysize) = img_size;
    let data = if let Some(proj) = proj {
        to_img(smoc, img_size, proj, proj_center, proj_bounds)
    } else {
        to_img_default(smoc, img_size, proj_center, proj_bounds)
    };
    let mut encoder = png::Encoder::new(writer, xsize as u32, ysize as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&data).expect("Wrong encoding");
    Ok(())
}

/// Automatically determines the center of the projection and if the projection to be used
/// is an allsky (Mollweide) ou bound limited (Sinus) projection.
/// In the first case, the image size along the x-axis is `2 * size_y`, and `size_y`
/// # Params
/// * `smoc`: the Spatial MOC to be print;
/// * `img_size_y`: the size of the image along the y-axis.
pub fn to_png_auto<T: Idx, W: Write>(
    smoc: &RangeMOC<T, Hpx<T>>,
    img_size_y: u16,
    writer: W,
) -> Result<(u16, u16), io::Error> {
    let (data, img_size) = to_img_auto(smoc, img_size_y);
    let mut encoder = png::Encoder::new(writer, img_size.0 as u32, img_size.1 as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&data).expect("Wrong encoding");
    Ok(img_size)
}

/// # Params
/// * `smoc`: the Spatial MOC to be print;
/// * `size`: the `(X, Y)` number of pixels in the image;
/// * `proj`: a projection, if different from Mollweide;
/// * `proj_center`: the `(lon, lat)` coordinates of the center of the projection, in radians,
///                      if different from `(0, 0)`;
/// * `proj_bounds`: the `(X, Y)` bounds of the projection, if different from the default values
///                  which depends on the projection. For unbounded projections, de default value
///                  is `(-PI..PI, -PI..PI)`.
/// * `path`: the path of th PNG file to be written.
/// * `view`: set to true to visualize the saved image.
#[cfg(not(target_arch = "wasm32"))]
pub fn to_png_file<T: Idx, P: CanonicalProjection>(
    smoc: &RangeMOC<T, Hpx<T>>,
    img_size: (u16, u16),
    proj: Option<P>,
    proj_center: Option<(f64, f64)>,
    proj_bounds: Option<(RangeInclusive<f64>, RangeInclusive<f64>)>,
    path: &Path,
    view: bool,
) -> Result<(), io::Error> {
    // Brackets are important to be sure the file is closed before trying to open it.
    {
        let file = File::create(path)?;
        let ref mut writer = BufWriter::new(file);
        to_png(smoc, img_size, proj, proj_center, proj_bounds, writer)?;
    }
    if view {
        show_with_default_app(path.to_string_lossy().as_ref())?;
    }
    Ok(())
}

/// Automatically determines the center of the projection and if the projection to be used
/// is an allsky (Mollweide) ou bound limited (Sinus) projection.
/// In the first case, the image size along the x-axis is `2 * size_y`, and `size_y`
/// # Params
/// * `smoc`: the Spatial MOC to be print;
/// * `img_size_y`: the size of the image along the y-axis.
/// * `path`: the path of th PNG file to be written.
/// * `view`: set to true to visualize the saved image.
#[cfg(not(target_arch = "wasm32"))]
pub fn to_png_file_auto<T: Idx>(
    smoc: &RangeMOC<T, Hpx<T>>,
    img_size_y: u16,
    path: &Path,
    view: bool,
) -> Result<(u16, u16), io::Error> {
    // Brackets are important to be sure the file is closed before trying to open it.
    let img_size = {
        let file = File::create(path)?;
        let ref mut writer = BufWriter::new(file);
        to_png_auto(smoc, img_size_y, writer)?
    };
    if view {
        show_with_default_app(path.to_string_lossy().as_ref())?;
    }
    Ok(img_size)
}

// Adapted from https://github.com/igiagkiozis/plotly/blob/master/plotly/src/plot.rs

#[cfg(target_os = "linux")]
fn show_with_default_app(path: &str) -> Result<(), io::Error> {
    use std::process::Command;
    Command::new("xdg-open").args([path]).output()?;
    // .map_err(|e| e.into())?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn show_with_default_app(path: &str) -> Result<(), io::Error> {
    use std::process::Command;
    Command::new("open").args(&[path]).output()?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn show_with_default_app(path: &str) -> Result<(), io::Error> {
    use std::process::Command;
    Command::new("cmd")
        .arg("/C")
        .arg(format!(r#"start {}"#, path))
        .output()?;
    Ok(())
}
