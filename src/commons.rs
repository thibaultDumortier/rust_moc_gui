use core::fmt;
use std::{io::Cursor, str::from_utf8_unchecked};

use crate::{
    load_ascii::*,
    load_fits::*,
    load_json::*,
    store::{self, *},
};
use js_sys::{Array, Uint8Array};
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
use wasm_bindgen::JsCast;
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

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

pub(crate) fn from_fits(data: &[u8]) -> Result<InternalMoc, String> {
    // Build the MOC
    let moc = match from_fits_ivoa(Cursor::new(data)).map_err(|e| e.to_string())? {
        MocIdxType::U16(moc) => from_fits_gen(moc),
        MocIdxType::U32(moc) => from_fits_gen(moc),
        MocIdxType::U64(moc) => from_fits_u64(moc),
    }
    .map_err(|e| e.to_string())?;
    Ok(moc)
}

pub(crate) fn load(rtype: &[&str], moct: Qty) -> Result<(), String> {
    let task = AsyncFileDialog::new()
        .add_filter("MOCs", rtype)
        .pick_files();

    let reading = if rtype.contains(&"fits") {
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
fn type_reading(rtype: &str, moct: &Qty, data: &[u8]) -> Result<InternalMoc, String> {
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

pub fn to_ascii_file(name: &str, fold: Option<usize>) -> Result<(), String> {
    let data: String = store::exec(name, move |moc| moc.to_ascii(fold))
        .ok_or_else(|| "MOC not found".to_string())?;
    to_file(
        name,
        ".txt",
        "text/plain",
        data.into_bytes().into_boxed_slice(),
    )
}

pub fn to_json_file(name: &str, fold: Option<usize>) -> Result<(), String> {
    let data: String = store::exec(name, move |moc| moc.to_json(fold))
        .ok_or_else(|| "MOC not found".to_string())?;
    to_file(
        name,
        ".json",
        "application/json",
        data.into_bytes().into_boxed_slice(),
    )
}

pub fn to_fits_file(name: &str) -> Result<(), String> {
    let data: Box<[u8]> =
        store::exec(name, move |moc| moc.to_fits()).ok_or_else(|| "MOC not found".to_string())?;
    to_file(name, ".fits", "application/fits", data)
}

fn to_file(name: &str, ext: &str, mime: &str, data: Box<[u8]>) -> Result<(), String> {
    // Set filename
    let mut filename = String::from(name);
    if !filename.ends_with(ext) {
        filename.push_str(ext);
    }
    // Put data in a blob
    let data: Uint8Array = data.as_ref().into();
    let bytes = Array::new();
    bytes.push(&data);
    let mut blob_prop = BlobPropertyBag::new();
    blob_prop.type_(mime);

    let blob = Blob::new_with_u8_array_sequence_and_options(&bytes, &blob_prop)
        .map_err(|e| e.as_string())
        .unwrap();

    // Generate the URL with the attached data
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|e| e.as_string())
        .unwrap();

    // Create a temporary download link
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .unwrap()
        .dyn_into()
        .map_err(|e| e.as_string())
        .unwrap();
    anchor.set_href(&url);
    anchor.set_download(&filename);
    if !body
        .append_child(&anchor)
        .map_err(|e| e.as_string())
        .is_ok()
    {
        return Err("Body child appending has failed".to_string());
    }
    // Simulate a click
    anchor.click();
    // Clean
    if !body
        .remove_child(&anchor)
        .map_err(|e| e.as_string())
        .is_ok()
    {
        return Err("Body child removing has failed".to_string());
    }
    if Url::revoke_object_url(&url)
        .map_err(|e| e.as_string())
        .is_ok()
    {
        return Err("URL revoking object url has failed".to_string());
    }
    Ok(())
}
