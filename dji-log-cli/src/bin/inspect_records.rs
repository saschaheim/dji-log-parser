use dji_log_parser::record::smart_battery_group::SmartBatteryGroup;
use dji_log_parser::record::Record;
use dji_log_parser::DJILog;
use std::collections::BTreeMap;
use std::env;
use std::fs;

fn main() {
    let log_path = env::args().nth(1).expect("log path required");
    let api_key = env::args().nth(2).expect("api key required");

    let bytes = fs::read(log_path).expect("unable to read log");
    let parser = DJILog::from_bytes(bytes).expect("unable to parse log");
    let keychains = parser
        .fetch_keychains(&api_key)
        .expect("unable to fetch keychains");
    let records = parser
        .records(Some(keychains))
        .expect("unable to parse records");

    let mut unknown_counts = BTreeMap::<u8, usize>::new();
    let mut unknown_lengths = BTreeMap::<u8, Vec<usize>>::new();
    let mut voltage_candidates = BTreeMap::<u8, CandidateCounts>::new();
    let mut invalid_count = 0usize;
    let mut dynamic = BTreeMap::<u8, usize>::new();
    let mut single_voltage = BTreeMap::<u8, usize>::new();
    let mut single_voltage_samples = Vec::new();

    for record in records {
        match record {
            Record::Unknown(record_type, data) => {
                *unknown_counts.entry(record_type).or_default() += 1;
                unknown_lengths
                    .entry(record_type)
                    .or_default()
                    .push(data.len());
                voltage_candidates
                    .entry(record_type)
                    .or_default()
                    .add(scan_candidates(&data));
            }
            Record::Invalid(_) => invalid_count += 1,
            Record::SmartBatteryGroup(SmartBatteryGroup::SmartBatteryDynamic(battery)) => {
                *dynamic.entry(battery.index).or_default() += 1;
            }
            Record::SmartBatteryGroup(SmartBatteryGroup::SmartBatterySingleVoltage(battery)) => {
                *single_voltage.entry(battery.index).or_default() += 1;
                if single_voltage_samples.len() < 12 {
                    single_voltage_samples.push(format!(
                        "index={} cell_count={} cell_voltages={:?}",
                        battery.index, battery.cell_count, battery.cell_voltages
                    ));
                }
            }
            _ => {}
        }
    }

    println!("unknown_counts={unknown_counts:?}");
    println!("unknown_lengths={}", format_lengths(&unknown_lengths));
    println!(
        "unknown_voltage_candidates={}",
        format_candidates(&voltage_candidates)
    );
    println!("invalid_count={invalid_count}");
    println!("smart_battery_dynamic={dynamic:?}");
    println!("smart_battery_single_voltage={single_voltage:?}");
    println!("smart_battery_single_voltage_samples={single_voltage_samples:#?}");
}

#[derive(Default)]
struct CandidateCounts {
    u16_le_mv: usize,
    u16_be_mv: usize,
    f32_le_volts: usize,
    f32_be_volts: usize,
}

impl CandidateCounts {
    fn add(&mut self, other: CandidateCounts) {
        self.u16_le_mv += other.u16_le_mv;
        self.u16_be_mv += other.u16_be_mv;
        self.f32_le_volts += other.f32_le_volts;
        self.f32_be_volts += other.f32_be_volts;
    }
}

fn scan_candidates(data: &[u8]) -> CandidateCounts {
    let mut counts = CandidateCounts::default();

    for window in data.windows(12) {
        if window
            .chunks_exact(2)
            .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]))
            .all(is_mv_cell_voltage)
        {
            counts.u16_le_mv += 1;
        }
        if window
            .chunks_exact(2)
            .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]))
            .all(is_mv_cell_voltage)
        {
            counts.u16_be_mv += 1;
        }
    }

    for window in data.windows(24) {
        if window
            .chunks_exact(4)
            .map(|bytes| f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            .all(is_float_cell_voltage)
        {
            counts.f32_le_volts += 1;
        }
        if window
            .chunks_exact(4)
            .map(|bytes| f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            .all(is_float_cell_voltage)
        {
            counts.f32_be_volts += 1;
        }
    }

    counts
}

fn is_mv_cell_voltage(value: u16) -> bool {
    (3000..=4400).contains(&value)
}

fn is_float_cell_voltage(value: f32) -> bool {
    value.is_finite() && (3.0..=4.4).contains(&value)
}

fn format_lengths(lengths: &BTreeMap<u8, Vec<usize>>) -> String {
    let parts = lengths
        .iter()
        .map(|(record_type, lengths)| {
            let min = lengths.iter().min().unwrap_or(&0);
            let max = lengths.iter().max().unwrap_or(&0);
            format!("{record_type}:{{min:{min},max:{max}}}")
        })
        .collect::<Vec<_>>();
    format!("{{{}}}", parts.join(", "))
}

fn format_candidates(candidates: &BTreeMap<u8, CandidateCounts>) -> String {
    let parts = candidates
        .iter()
        .filter(|(_, counts)| {
            counts.u16_le_mv + counts.u16_be_mv + counts.f32_le_volts + counts.f32_be_volts > 0
        })
        .map(|(record_type, counts)| {
            format!(
                "{record_type}:{{u16_le_mv:{},u16_be_mv:{},f32_le_volts:{},f32_be_volts:{}}}",
                counts.u16_le_mv, counts.u16_be_mv, counts.f32_le_volts, counts.f32_be_volts
            )
        })
        .collect::<Vec<_>>();
    format!("{{{}}}", parts.join(", "))
}
