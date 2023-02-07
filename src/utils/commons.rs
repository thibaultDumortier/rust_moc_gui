use core::fmt;
use std::str::from_utf8_unchecked;

use moc::{
    storage::u64idx::{common::MocQType, U64MocStore},
};
use crate::utils::namestore::add;

#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Uint8Array};
#[cfg(target_arch = "wasm32")]
use rfd::AsyncFileDialog;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;
#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs::File,
    io::{Read, Write},
};

pub(crate) const HALF_PI: f64 = 0.5 * std::f64::consts::PI;
pub(crate) const TWICE_PI: f64 = 2.0 * std::f64::consts::PI;

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

pub(crate) fn type_reading(rtype: &str, moct: &Qty, data: &[u8]) -> Result<usize, String> {
    match rtype {
        "fits" => U64MocStore.load_from_fits(data),
        "json" => match moct {
            Qty::Space => U64MocStore.load_smoc_from_json(unsafe { from_utf8_unchecked(data) }),
            Qty::Time => U64MocStore.load_tmoc_from_json(unsafe { from_utf8_unchecked(data) }),
            Qty::Timespace => {
                U64MocStore.load_stmoc_from_json(unsafe { from_utf8_unchecked(data) })
            }
        },
        "txt" | "ascii" => match moct {
            Qty::Space => U64MocStore.load_smoc_from_ascii(unsafe { from_utf8_unchecked(data) }),
            Qty::Time => U64MocStore.load_tmoc_from_ascii(unsafe { from_utf8_unchecked(data) }),
            Qty::Timespace => {
                U64MocStore.load_stmoc_from_ascii(unsafe { from_utf8_unchecked(data) })
            }
        },
        _ => unreachable!(), // since file_input.set_attribute("accept", ".fits, .json, .ascii, .txt");
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn to_file(name: &str, ext: &str, _mime: &str, data: Box<[u8]>) -> Result<(), String> {
    let path = rfd::FileDialog::new()
        .set_directory("../")
        .set_file_name(&(name.to_owned() + ext))
        .save_file();
    if let Some(path) = path {
        let file = File::create(path);
        match file {
            Ok(_) => {
                if file.unwrap().write_all(&data).is_err() {
                    return Err("Erreur dans l'ecriture du fichier".to_string());
                }
            }
            Err(_) => return Err("Erreur dans la creation du fichier".to_string()),
        };
    } else {
        // path is equal to none
        return Err("Annulé".to_string());
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn to_file(name: &str, ext: &str, mime: &str, data: Box<[u8]>) -> Result<(), String> {
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
    body.append_child(&anchor)
        .map_err(|_| "Body child appending has failed".to_string())?;
    // Simulate a click
    anchor.click();
    // Clean
    body.remove_child(&anchor)
        .map_err(|_| "Body child removing has failed".to_string())?;
    Url::revoke_object_url(&url).map_err(|_| "URL revoking object url has failed".to_string())?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn load(rtype: &[&str], moct: Qty) -> Result<(), String> {
    let reading = if rtype.contains(&"fits") {
        "fits"
    } else if rtype.contains(&"json") {
        "json"
    } else if rtype.contains(&"ascii") {
        "ascii"
    } else {
        "l.301 commons.rs [NOT SUPPOSED TO HAPPEN]"
    };

    if let Some(handle) = FileDialog::new().add_filter("MOCs", rtype).pick_files() {
        for path in handle {
            let mut file = File::open(&path).map_err(|_| "Error while opening file".to_string())?;
            //Reads name and adds it to be shown to user
            let file_name = path
                .file_name()
                .ok_or_else(|| "error while reading file name".to_string())?
                .to_str()
                .ok_or_else(|| "error while reading file name".to_string())?;
            //Reads file contents and adds it to the data
            let mut file_content = Vec::default();
            file.read_to_end(&mut file_content)
                .map_err(|e| format!("Error while reading file: {}", e))?;

            if let Ok(id) = type_reading(reading, &moct, file_content.as_slice()) {
                add(file_name, id)?;
            }
        }
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
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
        "l.339 commons.rs [NOT SUPPOSED TO HAPPEN]"
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
                if let Ok(id) = type_reading(reading, &moct, file_content.as_slice()) {
                    add(&file_name, id);
                }
            }
        }
    });
    Ok(())
}
#[cfg(target_arch = "wasm32")]
fn execute<F: std::future::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

pub fn fmt_qty(typ: MocQType) -> String {
    match typ {
        MocQType::Space => "Space".to_string(),
        MocQType::Time => "Time".to_string(),
        MocQType::TimeSpace => "Timespace".to_string(),
        MocQType::Frequency => todo!(), //TODO ADD FREQUENCY ERROR
    }
}
