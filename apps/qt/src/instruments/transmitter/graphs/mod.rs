/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded graphical presentation of actual transmitter boundaries.
mod physical;
use super::capture::Snapshot;
use serde_json::{json, Value};

/// Projects one retained stage page without changing sample values or provenance.
/// # Arguments
/// * `index` - Registered transmitter stage.
/// * `snapshot` - Immutable captured boundary.
/// * `offset`, `end` - Validated page inside the retained array.
/// * `mapped` - Separate source-mapped capture for an ordered-sequence comparison.
/// * `settings` - Frozen run geometry used by physical assembly.
/// # Returns
/// Bounded graph coordinates, exact readouts and explicit unsupported state.
pub(crate) fn project(
    index: usize,
    snapshot: &Snapshot,
    offset: usize,
    end: usize,
    mapped: Option<&Snapshot>,
    settings: &docsis_ofdm_fec::simulation::channel::ChannelRunSettings,
) -> Value {
    let values = snapshot.data["values"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default();
    let page = &values[offset..end];
    let first = snapshot.offset + offset;
    match index {
        0 | 2 => bits(page, first, false),
        1 => bits(page, first, true),
        3 => labels(page, first),
        4 | 7 | 8 => json!({"kind":"constellation","x_label":"I","y_label":"Q"}),
        5 => {
            let mut graph = sequence(page, first);
            graph["title"] = json!(format!(
                "After interleaving · channel symbol {}",
                snapshot
                    .symbol
                    .map(|symbol| symbol.to_string())
                    .unwrap_or_else(|| "unavailable".into())
            ));
            if let Some(source) = mapped {
                if let Some(values) = source.data["values"].as_array() {
                    let source_end = end.min(values.len());
                    if offset < source_end {
                        graph["before"] =
                            sequence(&values[offset..source_end], source.offset + offset);
                        let common = graph["extent"]
                            .as_f64()
                            .unwrap_or(1.0)
                            .max(graph["before"]["extent"].as_f64().unwrap_or(1.0));
                        rescale(&mut graph, common);
                        rescale(&mut graph["before"], common);
                        graph["before"]["title"] = json!(format!(
                            "Before interleaving · source symbol {}",
                            source
                                .source_symbol
                                .map(|symbol| symbol.to_string())
                                .unwrap_or_else(|| "unavailable".into())
                        ));
                    }
                }
            }
            graph
        }
        6 => physical::project(values, snapshot.offset, snapshot.symbol, settings),
        _ => json!({"kind":"empty"}),
    }
}

fn rescale(graph: &mut Value, extent: f64) {
    let previous = graph["extent"].as_f64().unwrap_or(extent);
    if let Some(points) = graph["points"].as_array_mut() {
        for point in points {
            for field in ["y", "q_y"] {
                if let Some(value) = point[field].as_f64() {
                    point[field] = json!(0.5 - (0.5 - value) * previous / extent);
                }
            }
        }
    }
    graph["extent"] = json!(extent);
}

fn bits(values: &[Value], offset: usize, raster: bool) -> Value {
    let columns = 64usize;
    let rows = values.len().div_ceil(columns).max(1);
    let points=values.iter().enumerate().map(|(index,value)| {
        let bit=value.as_u64().filter(|value|*value<=1);
        json!({"x":if raster {(index%columns) as f64/columns as f64} else {index as f64/values.len().saturating_sub(1).max(1) as f64},
            "y":if raster {Some((index/columns) as f64/rows as f64)} else {bit.map(|value|if value==0 {0.8} else {0.2})},
            "value":bit,"index":offset+index,"readout":format!("Bit {} · {}",offset+index,bit.map(|value|value.to_string()).unwrap_or_else(||"unavailable".into()))})
    }).collect::<Vec<_>>();
    json!({"kind":if raster {"raster"} else {"bits"},"points":points,"columns":columns,"rows":rows,"start":offset,"count":values.len(),"x_label":"Bit index","y_label":if raster {"Coded bits · 0 / 1"} else {"Bit · 0 / 1"},"first":offset.to_string(),"last":offset.saturating_add(values.len().saturating_sub(1)).to_string()})
}

fn labels(values: &[Value], offset: usize) -> Value {
    let maximum = values
        .iter()
        .filter_map(Value::as_u64)
        .max()
        .unwrap_or(1)
        .max(1);
    let points=values.iter().enumerate().map(|(index,value)| json!({"x":index as f64/values.len().max(1) as f64,"y":0.2,"value":value.as_u64(),"intensity":value.as_u64().map(|value|value as f64/maximum as f64),"index":offset+index,"readout":format!("Cell label {} · {}",offset+index,value)})).collect::<Vec<_>>();
    json!({"kind":"labels","points":points,"count":values.len(),"x_label":"Label index","y_label":"QAM label value","first":offset.to_string(),"last":offset.saturating_add(values.len().saturating_sub(1)).to_string()})
}

fn iq(value: &Value) -> Option<(f64, f64)> {
    let raw = if value.is_array() {
        value
    } else {
        &value["iq"]
    };
    raw.get(0)?
        .as_f64()
        .zip(raw.get(1)?.as_f64())
        .filter(|(i, q)| i.is_finite() && q.is_finite())
}

fn sequence(values: &[Value], offset: usize) -> Value {
    let extent = values
        .iter()
        .filter_map(iq)
        .fold(1.0f64, |maximum, (i, q)| maximum.max(i.abs()).max(q.abs()))
        * 1.1;
    let points=values.iter().enumerate().map(|(index,value)| {
        let sample=iq(value);
        json!({"x":index as f64/values.len().saturating_sub(1).max(1) as f64,"y":sample.map(|(i,_)|0.5-i/(2.0*extent)),"q_y":sample.map(|(_,q)|0.5-q/(2.0*extent)),"index":offset+index,"readout":sample.map(|(i,q)|format!("Cell {} · I {i:.5} · Q {q:.5}",offset+index)).unwrap_or_else(||format!("Cell {} · unavailable history",offset+index))})
    }).collect::<Vec<_>>();
    json!({"kind":"sequence","points":points,"x_label":"Cell index","y_label":"I / Q · normalized amplitude","extent":extent,"first":offset.to_string(),"last":offset.saturating_add(values.len().saturating_sub(1)).to_string()})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn digital_and_raster_coordinates_preserve_bits_offsets_and_unknowns() {
        let view = bits(&[json!(0), json!(1), Value::Null, json!(0)], 64, false);
        assert_eq!(view["points"][0]["y"], 0.8);
        assert_eq!(view["points"][1]["y"], 0.2);
        assert!(view["points"][2]["y"].is_null());
        assert_eq!(view["points"][3]["index"], 67);
        let raster = bits(&vec![json!(1); 128], 2048, true);
        assert_eq!(raster["rows"], 2);
        assert_eq!(raster["points"][64]["x"], 0.0);
        assert_eq!(raster["points"][64]["y"], 0.5);
    }
    #[test]
    fn ordered_iq_is_not_a_constellation_or_a_fabricated_permutation() {
        let view = sequence(
            &[
                json!([1.0, -1.0]),
                Value::Null,
                json!({"kind":"data","iq":[-1.0,1.0]}),
            ],
            20,
        );
        assert_eq!(view["points"][0]["x"], 0.0);
        assert_eq!(view["points"][2]["x"], 1.0);
        assert!(view["points"][1]["y"].is_null());
        assert!(view["points"][1]["q_y"].is_null());
        assert_eq!(view["points"][2]["index"], 22);
        assert_eq!(view["points"][0]["y"], view["points"][2]["q_y"]);
    }
}
