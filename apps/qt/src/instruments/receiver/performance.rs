/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded profile performance retention, separate from the latest detailed instrument snapshot.
use crate::simulation::closed_loop::ResultData;
use docsis_ofdm_fec::{
    profile::candidate::CapturedProfile, signal_processing::shannon::minimum_snr_db,
    types::modulation::CarrierType,
};
use serde_json::{json, Value};
use std::cell::RefCell;

pub(crate) const MAX_PROFILES: usize = 32;
struct Point {
    hz: u32,
    mer: Option<f64>,
    ideal: Option<f64>,
    samples: u64,
}
struct Entry {
    id: u8,
    channel: u8,
    status: String,
    fec: docsis_ofdm_fec::simulation::channel::results::FecCounters,
    ber: Option<f64>,
    pending: u64,
    points: Vec<Point>,
    error: Option<String>,
}
thread_local! { static ENTRIES: RefCell<Vec<Entry>> = const {RefCell::new(Vec::new())}; }
/// Clears all transient profile results after new accepted input or Start.
pub(crate) fn clear() {
    ENTRIES.with(|entries| entries.borrow_mut().clear());
}
/// Registers the finite profile catalog so unstarted jobs remain visible after Stop.
/// # Arguments
/// * `profiles` - Prevalidated captured identities, bounded by MAX_PROFILES.
pub(crate) fn begin(profiles: &[CapturedProfile]) {
    clear();
    for profile in profiles.iter().take(MAX_PROFILES) {
        insert(Entry {
            id: profile.profile_id,
            channel: profile.key.channel_id,
            status: "not_started".into(),
            fec: Default::default(),
            ber: None,
            pending: 0,
            points: vec![],
            error: None,
        });
    }
}
fn insert(entry: Entry) {
    ENTRIES.with(|entries| {
        let mut entries = entries.borrow_mut();
        if let Some(index) = entries.iter().position(|old| old.id == entry.id) {
            entries[index] = entry;
        } else if entries.len() < MAX_PROFILES {
            entries.push(entry);
        }
    });
}
/// Retains one bounded scalar/curve result while leaving the large latest snapshot elsewhere.
/// # Arguments
/// * `profile` - Exact applied profile for this independent run.
/// * `result` - Actual completed or partial job, including observation power sums.
pub(crate) fn retain(profile: &CapturedProfile, result: &ResultData) {
    let points = profile
        .assignments
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let bin = usize::from(profile.key.grid.first_subcarrier_index) + i;
            let mer = result.job.carrier_mer.get(bin);
            Point {
                hz: profile
                    .key
                    .grid
                    .frequency(i)
                    .expect("Validated captured grid"),
                mer: mer.and_then(|m| m.mer_db()),
                samples: mer.map_or(0, |m| m.samples()),
                ideal: if matches!(
                    a.carrier_type,
                    CarrierType::Data | CarrierType::ContinuousPilot | CarrierType::Plc
                ) {
                    a.bit_loading.and_then(minimum_snr_db)
                } else {
                    None
                },
            }
        })
        .collect();
    let job = &result.job;
    let entry = Entry {
        id: profile.profile_id,
        channel: profile.key.channel_id,
        status: serde_json::to_value(job.status)
            .unwrap()
            .as_str()
            .unwrap()
            .into(),
        fec: job
            .summary
            .window
            .as_ref()
            .map(|w| w.fec.snapshot())
            .unwrap_or_default(),
        ber: job.summary.window.as_ref().and_then(|w| w.ber),
        pending: job.unrecovered_generated_payloads,
        points,
        error: job.error.as_ref().map(ToString::to_string),
    };
    insert(entry);
}
/// Records an explicit per-profile setup failure without fabricated decoder counters.
/// # Arguments
/// * `id`, `channel` - Captured identities.
/// * `error` - Bounded setup diagnostic.
pub(crate) fn reject(id: u8, channel: u8, error: String) {
    insert(Entry {
        id,
        channel,
        status: "failed".into(),
        fec: Default::default(),
        ber: None,
        pending: 0,
        points: vec![],
        error: Some(error),
    });
}
/// Queries one profile on its original frequency grid, independently of other instruments.
/// # Arguments
/// * `id` - Captured ID; -1 selects the most recently completed profile.
/// * `start`, `count` - Source offsets; zero count restores Full Span.
/// # Returns
/// Bounded extrema, exact frequency axes, counters and complete/partial status.
pub(crate) fn view(id: i32, start: i32, count: i32) -> Value {
    ENTRIES.with(|entries| {
        let entries=entries.borrow();
        let profiles=entries.iter().map(|e|json!({"id":e.id,"status":e.status})).collect::<Vec<_>>();
        let Some(entry)=entries.iter().find(|e|i32::from(e.id)==id).or_else(|| if id<0 {entries.iter().rev().find(|entry|entry.status!="not_started").or_else(||entries.first())}else{None}) else {return json!({"available":false,"profiles":profiles,"reason":"Run the simulation to compare profiles."});};
        let total=entry.points.len();
        let (start,count)=if count==0 {(0,total)} else {let (Ok(s),Ok(c))=(usize::try_from(start),usize::try_from(count)) else {return json!({"error":"Invalid profile range."});};(s,c)};
        let Some(end)=start.checked_add(count).filter(|end|*end<=total) else {return json!({"error":"Choose a range within this profile."});};
        if count==0 {return json!({"available":false,"profiles":profiles,"profile_id":entry.id,"channel":entry.channel,"status":entry.status,"reason":entry.error.clone().unwrap_or_else(||"This profile has not run.".into())});}
        let first=entry.points[start].hz;let last=entry.points[end-1].hz;let span=f64::from((last-first).max(1));
        let mut maximum=45.0_f64;let mut minimum=0.0_f64;
        for p in &entry.points[start..end] {if let Some(v)=p.mer {maximum=maximum.max((v/5.0).ceil()*5.0);minimum=minimum.min((v/5.0).floor()*5.0);}}
        let mut series=Vec::new();
        for ideal in [false,true] {
            let mut points=Vec::new();
            for bucket in entry.points[start..end].chunks(count.div_ceil(1024)) {
                let values=bucket.iter().filter_map(|p|if ideal {p.ideal}else{p.mer}).collect::<Vec<_>>();
                if values.is_empty() {points.push(json!({"y":null}));continue;}
                let low=values.iter().copied().fold(f64::INFINITY,f64::min);let high=values.iter().copied().fold(f64::NEG_INFINITY,f64::max);
                points.push(json!({"x":f64::from(bucket[0].hz-first)/span,"y":(maximum-high)/(maximum-minimum),"minimum_y":(maximum-low)/(maximum-minimum),"gap":values.len()!=bucket.len(),"readout":format!("{:.3} MHz · {} · {low:.2}–{high:.2} dB",f64::from(bucket[0].hz)/1e6,if ideal {"Ideal Profile Minimum"}else{"Receiver MER"})}));
            }
            series.push(json!({"id":if ideal {0}else{1},"points":points}));
        }
        json!({"available":true,"profiles":profiles,"profile_id":entry.id,"channel":entry.channel,"status":entry.status,"fec":entry.fec,"ber":entry.ber,"unrecovered":entry.pending,"error":entry.error,
            "total":total,"start":start,"count":count,"points":[{"x":0},{"x":1}],"series":series,"lanes":[],"markers":[],"axis_label":"MER (dB)",
            "samples":entry.points.iter().map(|p|p.samples).sum::<u64>(),"scope":"Measured data-region cells · FEC: contained payload words · Each captured profile runs as Profile A (wire ID 0)",
            "summary":format!("Frequency selection: {:.3} – {:.3} MHz",f64::from(first)/1e6,f64::from(last)/1e6),
            "x_ticks":(0..=4).map(|i|json!({"x":f64::from(i)/4.0,"label":format!("{:.3}",(f64::from(first)+f64::from(i)/4.0*f64::from(last-first))/1e6)})).collect::<Vec<_>>(),
            "y_ticks":(0..=4).map(|i|json!({"x":f64::from(i)/4.0,"label":format!("{:.1}",maximum-f64::from(i)/4.0*(maximum-minimum))})).collect::<Vec<_>>()})
    })
}
