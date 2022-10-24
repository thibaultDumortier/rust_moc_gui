use core::fmt;
use std::{io::Cursor, str::from_utf8_unchecked};

use crate::{load_ascii::*, load_fits::*, load_json::*, store::*};
use moc::{
    deser::fits::{from_fits_ivoa, ranges2d_to_fits_ivoa, MocIdxType},
    elemset::range::MocRanges,
    moc::{
        range::RangeMOC, CellMOCIterator, CellOrCellRangeMOCIterator, RangeMOCIntoIterator,
        RangeMOCIterator,
    },
    moc2d::{
        range::RangeMOC2, CellMOC2IntoIterator, CellMOC2Iterator, CellOrCellRangeMOC2IntoIterator,
        CellOrCellRangeMOC2Iterator, RangeMOC2IntoIterator,
    },
    qty::{Hpx, Time},
};
use rfd::AsyncFileDialog;
use unreachable::UncheckedResultExt;
use wasm_bindgen::JsValue;

/// Convenient type for Space-MOCs
pub(crate) type Smoc = RangeMOC<u64, Hpx<u64>>;
/// Convenient type for Time-MOCs
pub(crate) type Tmoc = RangeMOC<u64, Time<u64>>;
/// Convenient type for SpaceTime-MOCs
pub(crate) type Stmoc = RangeMOC2<u64, Time<u64>, u64, Hpx<u64>>;

#[derive(PartialEq, Clone)]
pub(crate) enum Qty {
    Space,
    Time,
    Timespace,
}
impl Default for Qty {
    fn default() -> Self {
        Qty::Space
    }
}
impl fmt::Display for Qty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Qty::Space => write!(f, "Space"),
            Qty::Time => write!(f, "Time"),
            Qty::Timespace => write!(f, "Timespace"),
        }
    }
}

#[derive(Clone)]
pub(crate) enum InternalMoc {
    Space(Smoc),
    Time(Tmoc),
    TimeSpace(Stmoc),
}
impl Default for InternalMoc {
    fn default() -> Self {
        InternalMoc::Space(Smoc::new(0, MocRanges::default()))
    }
}
impl InternalMoc {
    pub(crate) fn to_fits(&self) -> Box<[u8]> {
        let mut buf: Vec<u8> = Default::default();
        // Uses unsafe [unchecked_unwrap_ok](https://docs.rs/unreachable/1.0.0/unreachable/trait.UncheckedResultExt.html)
        // for wasm size optimisation.
        // We do it because no I/O error can occurs since we are writing in memory.
        unsafe {
            match self {
                InternalMoc::Space(moc) => moc
                    .into_range_moc_iter()
                    .to_fits_ivoa(None, None, &mut buf)
                    .unchecked_unwrap_ok(),
                InternalMoc::Time(moc) => moc
                    .into_range_moc_iter()
                    .to_fits_ivoa(None, None, &mut buf)
                    .unchecked_unwrap_ok(),
                InternalMoc::TimeSpace(moc) => {
                    ranges2d_to_fits_ivoa(moc.into_range_moc2_iter(), None, None, &mut buf)
                        .unchecked_unwrap_ok()
                }
            }
        }
        buf.into_boxed_slice()
    }

    pub(crate) fn to_json(&self, fold: Option<usize>) -> String {
        let mut buf: Vec<u8> = Default::default();
        // Uses unsafe [unchecked_unwrap_ok](https://docs.rs/unreachable/1.0.0/unreachable/trait.UncheckedResultExt.html)
        // for wasm size optimisation.
        // We do it because no I/O error can occurs since we are writing in memory.
        unsafe {
            match self {
                InternalMoc::Space(moc) => moc
                    .into_range_moc_iter()
                    .cells()
                    .to_json_aladin(fold, &mut buf)
                    .unchecked_unwrap_ok(),
                InternalMoc::Time(moc) => moc
                    .into_range_moc_iter()
                    .cells()
                    .to_json_aladin(fold, &mut buf)
                    .unchecked_unwrap_ok(),
                InternalMoc::TimeSpace(moc) => moc
                    .into_range_moc2_iter()
                    .into_cell_moc2_iter()
                    .to_json_aladin(&fold, &mut buf)
                    .unchecked_unwrap_ok(),
            }
        }
        unsafe { String::from_utf8_unchecked(buf) }
    }

    pub(crate) fn to_ascii(&self, fold: Option<usize>) -> String {
        let mut buf: Vec<u8> = Default::default();
        // Uses unsafe [unchecked_unwrap_ok](https://docs.rs/unreachable/1.0.0/unreachable/trait.UncheckedResultExt.html)
        // for wasm size optimisation.
        // We do it because no I/O error can occurs since we are writing in memory.
        unsafe {
            match self {
                InternalMoc::Space(moc) => moc
                    .into_range_moc_iter()
                    .cells()
                    .cellranges()
                    .to_ascii_ivoa(fold, false, &mut buf)
                    .unchecked_unwrap_ok(),
                InternalMoc::Time(moc) => moc
                    .into_range_moc_iter()
                    .cells()
                    .cellranges()
                    .to_ascii_ivoa(fold, false, &mut buf)
                    .unchecked_unwrap_ok(),
                InternalMoc::TimeSpace(moc) => moc
                    .into_range_moc2_iter()
                    .into_cellcellrange_moc2_iter()
                    .to_ascii_ivoa(fold, false, &mut buf)
                    .unchecked_unwrap_ok(),
            }
        }
        unsafe { String::from_utf8_unchecked(buf) }
    }
}

#[derive(PartialEq)]
pub(crate) enum MocWType {
    Fits,
    Json,
    Ascii,
}
impl Default for MocWType {
    fn default() -> Self {
        MocWType::Fits
    }
}
impl fmt::Display for MocWType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MocWType::Fits => write!(f, "Fits"),
            MocWType::Json => write!(f, "Json"),
            MocWType::Ascii => write!(f, "Ascii"),
        }
    }
}

pub(crate) fn from_fits(data: &[u8]) -> Result<InternalMoc, JsValue> {
    // Build the MOC
    let moc =
        match from_fits_ivoa(Cursor::new(data)).map_err(|e| JsValue::from_str(&e.to_string()))? {
            MocIdxType::U16(moc) => from_fits_gen(moc),
            MocIdxType::U32(moc) => from_fits_gen(moc),
            MocIdxType::U64(moc) => from_fits_u64(moc),
        }
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(moc)
}

pub(crate) fn load(rtype: &[&str], moct: Qty) -> Result<(), String> {
    let task = AsyncFileDialog::new()
        .add_filter("MOCs", rtype)
        .pick_files();

    let mut reading = if rtype.contains(&"fits") {
        "fits"
    } else if rtype.contains(&"json") {
        "json"
    } else if rtype.contains(&"ascii") {
        "ascii"
    } else {
        "error [NOT SUPPOSED TO HAPPEN]"
    };

    execute(async move {
        let handle = task.await;

        if let Some(handle) = handle {
            //If you care about wasm support you just read() the file
            for path in handle {
                //Reads name and adds it to be shown to user
                let file_name = path.file_name();
                //Reads file contents and adds it to the data
                let file_content = path.read().await;
                let res = type_reading(reading, &moct, file_content.as_slice());
                if res.is_ok() {
                    add(&file_name, res.unwrap())
                        .expect("A problem has occured while trying to add the MOC");
                }
            }
        }
    });
    Ok(())
}
fn execute<F: std::future::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
fn type_reading(rtype: &str, moct: &Qty, data: &[u8]) -> Result<InternalMoc, JsValue> {
    match rtype {
        "fits" => from_fits(data),
        "json" => match moct {
            Qty::Space => smoc_from_json(unsafe { from_utf8_unchecked(data) }),
            Qty::Time => tmoc_from_json(unsafe { from_utf8_unchecked(data) }),
            Qty::Timespace => stmoc_from_json(unsafe { from_utf8_unchecked(data) }),
        },
        "txt" | "ascii" => match moct {
            Qty::Space => smoc_from_ascii(unsafe { from_utf8_unchecked(data) }),
            Qty::Time => tmoc_from_ascii(unsafe { from_utf8_unchecked(data) }),
            Qty::Timespace => stmoc_from_ascii(unsafe { from_utf8_unchecked(data) }),
        },
        _ => unreachable!(), // since file_input.set_attribute("accept", ".fits, .json, .ascii, .txt");
    }
}
