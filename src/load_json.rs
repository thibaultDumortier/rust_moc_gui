use moc::{
    deser::json::{cellmoc2d_from_json_aladin, from_json_aladin},
    moc::{CellMOCIntoIterator, CellMOCIterator, RangeMOCIterator},
    moc2d::{CellMOC2IntoIterator, RangeMOC2IntoIterator, RangeMOC2Iterator},
    qty::{Hpx, Time},
};
use wasm_bindgen::JsValue;

use crate::commons::InternalMoc;

pub(crate) fn smoc_from_json(data: &str) -> Result<InternalMoc, JsValue> {
    let cells =
        from_json_aladin::<u64, Hpx<u64>>(data).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let moc = cells.into_cell_moc_iter().ranges().into_range_moc();
    Ok(InternalMoc::Space(moc))
}

pub(crate) fn tmoc_from_json(data: &str) -> Result<InternalMoc, JsValue> {
    let cells =
        from_json_aladin::<u64, Time<u64>>(data).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let moc = cells.into_cell_moc_iter().ranges().into_range_moc();
    Ok(InternalMoc::Time(moc))
}

pub(crate) fn stmoc_from_json(data: &str) -> Result<InternalMoc, JsValue> {
    let cell2 = cellmoc2d_from_json_aladin::<u64, Time<u64>, u64, Hpx<u64>>(data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let moc2 = cell2
        .into_cell_moc2_iter()
        .into_range_moc2_iter()
        .into_range_moc2();
    Ok(InternalMoc::TimeSpace(moc2))
}
