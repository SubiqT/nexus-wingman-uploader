use std::ffi::CString;

use anyhow::Result;
use nexus::event::Event;
use revtc::evtc::Encounter;

use crate::dpsreport::DpsReportResponse;

pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

// Events for other addons to subscribe to
pub const EV_LOG_DETECTED: Event<LogDetectedEvent> =
    unsafe { Event::new("EV_ARCDPS_LOG_UPLOADER_LOG_DETECTED") };
pub const EV_LOG_PARSED: Event<LogParsedEvent> =
    unsafe { Event::new("EV_ARCDPS_LOG_UPLOADER_LOG_PARSED") };
pub const EV_DPSREPORT: Event<DpsReportEvent> =
    unsafe { Event::new("EV_ARCDPS_LOG_UPLOADER_DPSREPORT") };
pub const EV_WINGMAN: Event<WingmanEvent> =
    unsafe { Event::new("EV_ARCDPS_LOG_UPLOADER_WINGMAN") };

#[repr(C)]
pub struct LogDetectedEvent {
    pub file_path: *const std::ffi::c_char,
}

#[repr(C)]
pub struct LogParsedEvent {
    pub file_path: *const std::ffi::c_char,
    pub boss_id: u16,
    pub player_count: u32,
}

#[repr(C)]
pub struct DpsReportEvent {
    pub file_path: *const std::ffi::c_char,
    pub permalink: *const std::ffi::c_char,
    pub boss_id: i64,
    pub success: bool,
}

#[repr(C)]
pub struct WingmanEvent {
    pub file_path: *const std::ffi::c_char,
    pub boss_id: u16,
    pub accepted: bool,
}

// helper to get a CString from a PathBuf, lossy
pub fn path_to_cstring(path: &std::path::Path) -> CString {
    CString::new(path.to_string_lossy().as_bytes()).unwrap_or_default()
}

#[derive(Debug)]
pub struct WorkerMessage {
    pub index: usize,
    pub payload: WorkerType,
}

impl WorkerMessage {
    pub fn evtc(index: usize, evtc: Result<Encounter>) -> Self {
        Self {
            index,
            payload: WorkerType::Evtc(evtc),
        }
    }

    pub fn dpsreport(
        index: usize,
        dpsreport: Result<Result<DpsReportResponse, std::time::Instant>>,
    ) -> WorkerMessage {
        WorkerMessage {
            index,
            payload: WorkerType::DpsReport(dpsreport),
        }
    }
    // should be a url later instead of bool
    pub fn wingman(index: usize, wingman: Result<bool>) -> WorkerMessage {
        WorkerMessage {
            index,
            payload: WorkerType::Wingman(wingman),
        }
    }
}

#[derive(Debug)]
pub enum WorkerType {
    DpsReport(Result<Result<DpsReportResponse, std::time::Instant>>),
    Wingman(Result<bool>),
    Evtc(Result<Encounter>),
}
