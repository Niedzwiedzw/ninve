#![allow(dead_code)]

use {
    serde::Serialize,
    serde_json::{Value, json},
    std::collections::HashMap,
};

pub trait PropertyValue {
    fn to_json_array(&self) -> Vec<Value>;
}

#[derive(Debug)]
pub(crate) struct Pause(pub bool);

impl PropertyValue for Pause {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("pause"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct AudioSpeedCorrection(pub f64);

impl PropertyValue for AudioSpeedCorrection {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("audio-speed-correction"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct VideoSpeedCorrection(pub f64);

impl PropertyValue for VideoSpeedCorrection {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("video-speed-correction"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DisplaySyncActive(pub bool);

impl PropertyValue for DisplaySyncActive {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("display-sync-active"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Filename(pub String);

impl PropertyValue for Filename {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("filename"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct FilenameNoExt(pub String);

impl PropertyValue for FilenameNoExt {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("filename/no-ext"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct FileSize(pub i64);

impl PropertyValue for FileSize {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("file-size"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct EstimatedFrameCount(pub i64);

impl PropertyValue for EstimatedFrameCount {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("estimated-frame-count"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct EstimatedFrameNumber(pub i64);

impl PropertyValue for EstimatedFrameNumber {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("estimated-frame-number"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Pid(pub i64);

impl PropertyValue for Pid {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("pid"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Path(pub String);

impl PropertyValue for Path {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("path"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct StreamOpenFilename(pub String);

impl PropertyValue for StreamOpenFilename {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("stream-open-filename"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct MediaTitle(pub String);

impl PropertyValue for MediaTitle {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("media-title"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct FileFormat(pub String);

impl PropertyValue for FileFormat {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("file-format"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct CurrentDemuxer(pub String);

impl PropertyValue for CurrentDemuxer {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("current-demuxer"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct StreamPath(pub String);

impl PropertyValue for StreamPath {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("stream-path"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct StreamPos(pub i64);

impl PropertyValue for StreamPos {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("stream-pos"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct StreamEnd(pub i64);

impl PropertyValue for StreamEnd {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("stream-end"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Duration {
    pub seconds: Option<f64>,
    pub full: Option<f64>,
}

impl PropertyValue for Duration {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("duration"), json!(self.seconds)]
    }
}

#[derive(Debug)]
pub(crate) struct DurationFull(pub f64);

impl PropertyValue for DurationFull {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("duration/full"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Avsync(pub Option<f64>);

impl PropertyValue for Avsync {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("avsync"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct TotalAvsyncChange(pub Option<f64>);

impl PropertyValue for TotalAvsyncChange {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("total-avsync-change"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DecoderFrameDropCount(pub Option<i64>);

impl PropertyValue for DecoderFrameDropCount {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("decoder-frame-drop-count"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct FrameDropCount(pub Option<i64>);

impl PropertyValue for FrameDropCount {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("frame-drop-count"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct MistimedFrameCount(pub i64);

impl PropertyValue for MistimedFrameCount {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("mistimed-frame-count"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct VsyncRatio(pub f64);

impl PropertyValue for VsyncRatio {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("vsync-ratio"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct VoDelayedFrameCount(pub i64);

impl PropertyValue for VoDelayedFrameCount {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("vo-delayed-frame-count"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct PercentPos(pub i64);

impl PropertyValue for PercentPos {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("percent-pos"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct TimePos {
    pub seconds: f64,
    pub full: Option<f64>,
}

impl PropertyValue for TimePos {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("time-pos"), json!(self.seconds)]
    }
}

#[derive(Debug)]
pub(crate) struct TimePosFull(pub f64);

impl PropertyValue for TimePosFull {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("time-pos/full"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct TimeStart(pub i64);

impl PropertyValue for TimeStart {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("time-start"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct TimeRemaining {
    pub seconds: Option<f64>,
    pub full: Option<f64>,
}

impl PropertyValue for TimeRemaining {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("time-remaining"), json!(self.seconds)]
    }
}

#[derive(Debug)]
pub(crate) struct TimeRemainingFull(pub f64);

impl PropertyValue for TimeRemainingFull {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("time-remaining/full"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct AudioPts {
    pub seconds: Option<f64>,
    pub full: Option<f64>,
}

impl PropertyValue for AudioPts {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("audio-pts"), json!(self.seconds)]
    }
}

#[derive(Debug)]
pub(crate) struct AudioPtsFull(pub f64);

impl PropertyValue for AudioPtsFull {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("audio-pts/full"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct PlaytimeRemaining {
    pub seconds: Option<f64>,
    pub full: Option<f64>,
}

impl PropertyValue for PlaytimeRemaining {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("playtime-remaining"), json!(self.seconds)]
    }
}

#[derive(Debug)]
pub(crate) struct PlaytimeRemainingFull(pub f64);

impl PropertyValue for PlaytimeRemainingFull {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("playtime-remaining/full"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct PlaybackTime {
    pub seconds: f64,
    pub full: Option<f64>,
}

impl PropertyValue for PlaybackTime {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("playback-time"), json!(self.seconds)]
    }
}

#[derive(Debug)]
pub(crate) struct PlaybackTimeFull(pub f64);

impl PropertyValue for PlaybackTimeFull {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("playback-time/full"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct RemainingFileLoops(pub i64);

impl PropertyValue for RemainingFileLoops {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("remaining-file-loops"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct RemainingAbLoops(pub i64);

impl PropertyValue for RemainingAbLoops {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("remaining-ab-loops"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Chapter(pub i64);

impl PropertyValue for Chapter {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("chapter"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Edition(pub i64);

impl PropertyValue for Edition {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("edition"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct CurrentEdition(pub i64);

impl PropertyValue for CurrentEdition {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("current-edition"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Chapters(pub i64);

impl PropertyValue for Chapters {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("chapters"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Editions(pub i64);

impl PropertyValue for Editions {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("editions"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct EditionList(pub Vec<EditionEntry>);

#[derive(Debug)]
pub(crate) struct EditionEntry {
    pub id: i64,
    pub default: bool,
    pub title: Option<String>,
}

impl PropertyValue for EditionList {
    fn to_json_array(&self) -> Vec<Value> {
        let editions: Vec<Value> = self
            .0
            .iter()
            .map(|e| {
                json!({
                    "id": e.id,
                    "default": e.default,
                    "title": e.title,
                })
            })
            .collect();
        vec![json!("edition-list"), json!(editions)]
    }
}

#[derive(Debug)]
pub(crate) struct Metadata(pub HashMap<String, String>);

impl PropertyValue for Metadata {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("metadata"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct FilteredMetadata(pub HashMap<String, String>);

impl PropertyValue for FilteredMetadata {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("filtered-metadata"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct ChapterMetadata(pub HashMap<String, String>);

impl PropertyValue for ChapterMetadata {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("chapter-metadata"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct VfMetadata(pub HashMap<String, String>);

impl PropertyValue for VfMetadata {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("vf-metadata"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct AfMetadata(pub HashMap<String, String>);

impl PropertyValue for AfMetadata {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("af-metadata"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DeinterlaceActive(pub bool);

impl PropertyValue for DeinterlaceActive {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("deinterlace-active"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct IdleActive(pub bool);

impl PropertyValue for IdleActive {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("idle-active"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct CoreIdle(pub bool);

impl PropertyValue for CoreIdle {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("core-idle"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct CacheSpeed(pub i64);

impl PropertyValue for CacheSpeed {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("cache-speed"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DemuxerCacheDuration(pub Option<f64>);

impl PropertyValue for DemuxerCacheDuration {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("demuxer-cache-duration"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DemuxerCacheTime(pub Option<f64>);

impl PropertyValue for DemuxerCacheTime {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("demuxer-cache-time"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DemuxerCacheIdle(pub bool);

impl PropertyValue for DemuxerCacheIdle {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("demuxer-cache-idle"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DemuxerCacheState {
    pub seekable_ranges: Vec<SeekableRange>,
    pub bof_cached: bool,
    pub eof_cached: bool,
    pub fw_bytes: i64,
    pub file_cache_bytes: Option<i64>,
    pub cache_end: Option<f64>,
    pub reader_pts: Option<f64>,
    pub cache_duration: Option<f64>,
    pub raw_input_rate: Option<i64>,
    pub ts_per_stream: Vec<TsPerStream>,
}

#[derive(Debug)]
pub(crate) struct SeekableRange {
    pub start: f64,
    pub end: f64,
}

#[derive(Debug)]
pub(crate) struct TsPerStream {
    pub type_: String,
    pub cache_duration: Option<f64>,
    pub reader_pts: Option<f64>,
    pub cache_end: Option<f64>,
}

impl PropertyValue for DemuxerCacheState {
    fn to_json_array(&self) -> Vec<Value> {
        let ranges: Vec<Value> = self
            .seekable_ranges
            .iter()
            .map(|r| {
                json!({
                    "start": r.start,
                    "end": r.end,
                })
            })
            .collect();
        let ts_streams: Vec<Value> = self
            .ts_per_stream
            .iter()
            .map(|ts| {
                json!({
                    "type": ts.type_,
                    "cache-duration": ts.cache_duration,
                    "reader-pts": ts.reader_pts,
                    "cache-end": ts.cache_end,
                })
            })
            .collect();
        vec![
            json!("demuxer-cache-state"),
            json!({
                "seekable-ranges": ranges,
                "bof-cached": self.bof_cached,
                "eof-cached": self.eof_cached,
                "fw-bytes": self.fw_bytes,
                "file-cache-bytes": self.file_cache_bytes,
                "cache-end": self.cache_end,
                "reader-pts": self.reader_pts,
                "cache-duration": self.cache_duration,
                "raw-input-rate": self.raw_input_rate,
                "ts-per-stream": ts_streams,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct DemuxerViaNetwork(pub bool);

impl PropertyValue for DemuxerViaNetwork {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("demuxer-via-network"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DemuxerStartTime(pub f64);

impl PropertyValue for DemuxerStartTime {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("demuxer-start-time"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct PausedForCache(pub bool);

impl PropertyValue for PausedForCache {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("paused-for-cache"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct CacheBufferingState(pub i64);

impl PropertyValue for CacheBufferingState {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("cache-buffering-state"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct EofReached(pub bool);

impl PropertyValue for EofReached {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("eof-reached"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Seeking(pub bool);

impl PropertyValue for Seeking {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("seeking"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct MixerActive(pub bool);

impl PropertyValue for MixerActive {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("mixer-active"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct AoVolume(pub f64);

impl PropertyValue for AoVolume {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("ao-volume"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct AoMute(pub bool);

impl PropertyValue for AoMute {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("ao-mute"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct AudioParams {
    pub format: String,
    pub samplerate: i64,
    pub channels: String,
    pub hr_channels: String,
    pub channel_count: i64,
}

impl PropertyValue for AudioParams {
    fn to_json_array(&self) -> Vec<Value> {
        vec![
            json!("audio-params"),
            json!({
                "format": self.format,
                "samplerate": self.samplerate,
                "channels": self.channels,
                "hr-channels": self.hr_channels,
                "channel-count": self.channel_count,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct AudioOutParams {
    pub format: String,
    pub samplerate: i64,
    pub channels: String,
    pub hr_channels: String,
    pub channel_count: i64,
}

impl PropertyValue for AudioOutParams {
    fn to_json_array(&self) -> Vec<Value> {
        vec![
            json!("audio-out-params"),
            json!({
                "format": self.format,
                "samplerate": self.samplerate,
                "channels": self.channels,
                "hr-channels": self.hr_channels,
                "channel-count": self.channel_count,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct Colormatrix(pub String);

impl PropertyValue for Colormatrix {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("colormatrix"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct ColormatrixInputRange(pub String);

impl PropertyValue for ColormatrixInputRange {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("colormatrix-input-range"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct ColormatrixPrimaries(pub String);

impl PropertyValue for ColormatrixPrimaries {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("colormatrix-primaries"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Hwdec(pub String);

impl PropertyValue for Hwdec {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("hwdec"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct HwdecCurrent(pub String);

impl PropertyValue for HwdecCurrent {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("hwdec-current"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct HwdecInterop(pub String);

impl PropertyValue for HwdecInterop {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("hwdec-interop"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Width(pub i64);

impl PropertyValue for Width {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("width"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Height(pub i64);

impl PropertyValue for Height {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("height"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct VideoParams {
    pub pixelformat: String,
    pub hw_pixelformat: Option<String>,
    pub average_bpp: Option<i64>,
    pub w: i64,
    pub h: i64,
    pub dw: i64,
    pub dh: i64,
    pub crop_x: i64,
    pub crop_y: i64,
    pub crop_w: i64,
    pub crop_h: i64,
    pub aspect: f64,
    pub aspect_name: String,
    pub par: f64,
    pub sar: f64,
    pub sar_name: String,
    pub colormatrix: String,
    pub colorlevels: String,
    pub primaries: String,
    pub gamma: String,
    pub sig_peak: Option<f64>,
    pub light: String,
    pub chroma_location: String,
    pub rotate: i64,
    pub stereo_in: String,
    pub alpha: Option<String>,
    pub min_luma: Option<f64>,
    pub max_luma: Option<f64>,
    pub max_cll: Option<f64>,
    pub max_fall: Option<f64>,
    pub scene_max_r: Option<f64>,
    pub scene_max_g: Option<f64>,
    pub scene_max_b: Option<f64>,
    pub max_pq_y: Option<f64>,
    pub avg_pq_y: Option<f64>,
}

impl PropertyValue for VideoParams {
    fn to_json_array(&self) -> Vec<Value> {
        vec![
            json!("video-params"),
            json!({
                "pixelformat": self.pixelformat,
                "hw-pixelformat": self.hw_pixelformat,
                "average-bpp": self.average_bpp,
                "w": self.w,
                "h": self.h,
                "dw": self.dw,
                "dh": self.dh,
                "crop-x": self.crop_x,
                "crop-y": self.crop_y,
                "crop-w": self.crop_w,
                "crop-h": self.crop_h,
                "aspect": self.aspect,
                "aspect-name": self.aspect_name,
                "par": self.par,
                "sar": self.sar,
                "sar-name": self.sar_name,
                "colormatrix": self.colormatrix,
                "colorlevels": self.colorlevels,
                "primaries": self.primaries,
                "gamma": self.gamma,
                "sig-peak": self.sig_peak,
                "light": self.light,
                "chroma-location": self.chroma_location,
                "rotate": self.rotate,
                "stereo-in": self.stereo_in,
                "alpha": self.alpha,
                "min-luma": self.min_luma,
                "max-luma": self.max_luma,
                "max-cll": self.max_cll,
                "max-fall": self.max_fall,
                "scene-max-r": self.scene_max_r,
                "scene-max-g": self.scene_max_g,
                "scene-max-b": self.scene_max_b,
                "max-pq-y": self.max_pq_y,
                "avg-pq-y": self.avg_pq_y,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct Dwidth(pub i64);

impl PropertyValue for Dwidth {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("dwidth"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Dheight(pub i64);

impl PropertyValue for Dheight {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("dheight"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct VideoDecParams {
    pub pixelformat: String,
    pub hw_pixelformat: Option<String>,
    pub average_bpp: Option<i64>,
    pub w: i64,
    pub h: i64,
    pub dw: i64,
    pub dh: i64,
    pub crop_x: i64,
    pub crop_y: i64,
    pub crop_w: i64,
    pub crop_h: i64,
    pub aspect: f64,
    pub aspect_name: String,
    pub par: f64,
    pub sar: f64,
    pub sar_name: String,
    pub colormatrix: String,
    pub colorlevels: String,
    pub primaries: String,
    pub gamma: String,
    pub sig_peak: Option<f64>,
    pub light: String,
    pub chroma_location: String,
    pub rotate: i64,
    pub stereo_in: String,
    pub alpha: Option<String>,
    pub min_luma: Option<f64>,
    pub max_luma: Option<f64>,
    pub max_cll: Option<f64>,
    pub max_fall: Option<f64>,
    pub scene_max_r: Option<f64>,
    pub scene_max_g: Option<f64>,
    pub scene_max_b: Option<f64>,
    pub max_pq_y: Option<f64>,
    pub avg_pq_y: Option<f64>,
}

impl PropertyValue for VideoDecParams {
    fn to_json_array(&self) -> Vec<Value> {
        vec![
            json!("video-dec-params"),
            json!({
                "pixelformat": self.pixelformat,
                "hw-pixelformat": self.hw_pixelformat,
                "average-bpp": self.average_bpp,
                "w": self.w,
                "h": self.h,
                "dw": self.dw,
                "dh": self.dh,
                "crop-x": self.crop_x,
                "crop-y": self.crop_y,
                "crop-w": self.crop_w,
                "crop-h": self.crop_h,
                "aspect": self.aspect,
                "aspect-name": self.aspect_name,
                "par": self.par,
                "sar": self.sar,
                "sar-name": self.sar_name,
                "colormatrix": self.colormatrix,
                "colorlevels": self.colorlevels,
                "primaries": self.primaries,
                "gamma": self.gamma,
                "sig-peak": self.sig_peak,
                "light": self.light,
                "chroma-location": self.chroma_location,
                "rotate": self.rotate,
                "stereo-in": self.stereo_in,
                "alpha": self.alpha,
                "min-luma": self.min_luma,
                "max-luma": self.max_luma,
                "max-cll": self.max_cll,
                "max-fall": self.max_fall,
                "scene-max-r": self.scene_max_r,
                "scene-max-g": self.scene_max_g,
                "scene-max-b": self.scene_max_b,
                "max-pq-y": self.max_pq_y,
                "avg-pq-y": self.avg_pq_y,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct VideoOutParams {
    pub pixelformat: String,
    pub hw_pixelformat: Option<String>,
    pub average_bpp: Option<i64>,
    pub w: i64,
    pub h: i64,
    pub dw: i64,
    pub dh: i64,
    pub crop_x: i64,
    pub crop_y: i64,
    pub crop_w: i64,
    pub crop_h: i64,
    pub aspect: f64,
    pub aspect_name: String,
    pub par: f64,
    pub sar: f64,
    pub sar_name: String,
    pub colormatrix: String,
    pub colorlevels: String,
    pub primaries: String,
    pub gamma: String,
    pub sig_peak: Option<f64>,
    pub light: String,
    pub chroma_location: String,
    pub rotate: i64,
    pub stereo_in: String,
    pub alpha: Option<String>,
    pub min_luma: Option<f64>,
    pub max_luma: Option<f64>,
    pub max_cll: Option<f64>,
    pub max_fall: Option<f64>,
    pub scene_max_r: Option<f64>,
    pub scene_max_g: Option<f64>,
    pub scene_max_b: Option<f64>,
    pub max_pq_y: Option<f64>,
    pub avg_pq_y: Option<f64>,
}

impl PropertyValue for VideoOutParams {
    fn to_json_array(&self) -> Vec<Value> {
        vec![
            json!("video-out-params"),
            json!({
                "pixelformat": self.pixelformat,
                "hw-pixelformat": self.hw_pixelformat,
                "average-bpp": self.average_bpp,
                "w": self.w,
                "h": self.h,
                "dw": self.dw,
                "dh": self.dh,
                "crop-x": self.crop_x,
                "crop-y": self.crop_y,
                "crop-w": self.crop_w,
                "crop-h": self.crop_h,
                "aspect": self.aspect,
                "aspect-name": self.aspect_name,
                "par": self.par,
                "sar": self.sar,
                "sar-name": self.sar_name,
                "colormatrix": self.colormatrix,
                "colorlevels": self.colorlevels,
                "primaries": self.primaries,
                "gamma": self.gamma,
                "sig-peak": self.sig_peak,
                "light": self.light,
                "chroma-location": self.chroma_location,
                "rotate": self.rotate,
                "stereo-in": self.stereo_in,
                "alpha": self.alpha,
                "min-luma": self.min_luma,
                "max-luma": self.max_luma,
                "max-cll": self.max_cll,
                "max-fall": self.max_fall,
                "scene-max-r": self.scene_max_r,
                "scene-max-g": self.scene_max_g,
                "scene-max-b": self.scene_max_b,
                "max-pq-y": self.max_pq_y,
                "avg-pq-y": self.avg_pq_y,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct VideoTargetParams {
    pub pixelformat: String,
    pub hw_pixelformat: Option<String>,
    pub average_bpp: Option<i64>,
    pub w: i64,
    pub h: i64,
    pub dw: i64,
    pub dh: i64,
    pub crop_x: i64,
    pub crop_y: i64,
    pub crop_w: i64,
    pub crop_h: i64,
    pub aspect: f64,
    pub aspect_name: String,
    pub par: f64,
    pub sar: f64,
    pub sar_name: String,
    pub colormatrix: String,
    pub colorlevels: String,
    pub primaries: String,
    pub gamma: String,
    pub sig_peak: Option<f64>,
    pub light: String,
    pub chroma_location: String,
    pub rotate: i64,
    pub stereo_in: String,
    pub alpha: Option<String>,
    pub min_luma: Option<f64>,
    pub max_luma: Option<f64>,
    pub max_cll: Option<f64>,
    pub max_fall: Option<f64>,
    pub scene_max_r: Option<f64>,
    pub scene_max_g: Option<f64>,
    pub scene_max_b: Option<f64>,
    pub max_pq_y: Option<f64>,
    pub avg_pq_y: Option<f64>,
}

impl PropertyValue for VideoTargetParams {
    fn to_json_array(&self) -> Vec<Value> {
        vec![
            json!("video-target-params"),
            json!({
                "pixelformat": self.pixelformat,
                "hw-pixelformat": self.hw_pixelformat,
                "average-bpp": self.average_bpp,
                "w": self.w,
                "h": self.h,
                "dw": self.dw,
                "dh": self.dh,
                "crop-x": self.crop_x,
                "crop-y": self.crop_y,
                "crop-w": self.crop_w,
                "crop-h": self.crop_h,
                "aspect": self.aspect,
                "aspect-name": self.aspect_name,
                "par": self.par,
                "sar": self.sar,
                "sar-name": self.sar_name,
                "colormatrix": self.colormatrix,
                "colorlevels": self.colorlevels,
                "primaries": self.primaries,
                "gamma": self.gamma,
                "sig-peak": self.sig_peak,
                "light": self.light,
                "chroma-location": self.chroma_location,
                "rotate": self.rotate,
                "stereo-in": self.stereo_in,
                "alpha": self.alpha,
                "min-luma": self.min_luma,
                "max-luma": self.max_luma,
                "max-cll": self.max_cll,
                "max-fall": self.max_fall,
                "scene-max-r": self.scene_max_r,
                "scene-max-g": self.scene_max_g,
                "scene-max-b": self.scene_max_b,
                "max-pq-y": self.max_pq_y,
                "avg-pq-y": self.avg_pq_y,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct VideoFrameInfo {
    pub picture_type: String,
    pub interlaced: bool,
    pub tff: bool,
    pub repeat: bool,
    pub gop_timecode: String,
    pub smpte_timecode: String,
    pub estimated_smpte_timecode: String,
}

impl PropertyValue for VideoFrameInfo {
    fn to_json_array(&self) -> Vec<Value> {
        vec![
            json!("video-frame-info"),
            json!({
                "picture-type": self.picture_type,
                "interlaced": self.interlaced,
                "tff": self.tff,
                "repeat": self.repeat,
                "gop-timecode": self.gop_timecode,
                "smpte-timecode": self.smpte_timecode,
                "estimated-smpte-timecode": self.estimated_smpte_timecode,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct ContainerFps(pub f64);

impl PropertyValue for ContainerFps {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("container-fps"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct EstimatedVfFps(pub f64);

impl PropertyValue for EstimatedVfFps {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("estimated-vf-fps"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct CurrentWindowScale(pub f64);

impl PropertyValue for CurrentWindowScale {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("current-window-scale"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Focused(pub bool);

impl PropertyValue for Focused {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("focused"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct AmbientLight(pub f64);

impl PropertyValue for AmbientLight {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("ambient-light"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DisplayNames(pub Vec<String>);

impl PropertyValue for DisplayNames {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("display-names"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DisplayFps(pub f64);

impl PropertyValue for DisplayFps {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("display-fps"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct EstimatedDisplayFps(pub f64);

impl PropertyValue for EstimatedDisplayFps {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("estimated-display-fps"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct VsyncJitter(pub f64);

impl PropertyValue for VsyncJitter {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("vsync-jitter"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DisplayWidth(pub i64);

impl PropertyValue for DisplayWidth {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("display-width"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DisplayHeight(pub i64);

impl PropertyValue for DisplayHeight {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("display-height"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DisplayHidpiScale(pub f64);

impl PropertyValue for DisplayHidpiScale {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("display-hidpi-scale"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct OsdWidth(pub i64);

impl PropertyValue for OsdWidth {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("osd-width"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct OsdHeight(pub i64);

impl PropertyValue for OsdHeight {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("osd-height"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct OsdPar(pub f64);

impl PropertyValue for OsdPar {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("osd-par"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct OsdDimensions {
    pub w: i64,
    pub h: i64,
    pub par: f64,
    pub aspect: f64,
    pub mt: i64,
    pub mb: i64,
    pub ml: i64,
    pub mr: i64,
}

impl PropertyValue for OsdDimensions {
    fn to_json_array(&self) -> Vec<Value> {
        vec![
            json!("osd-dimensions"),
            json!({
                "w": self.w,
                "h": self.h,
                "par": self.par,
                "aspect": self.aspect,
                "mt": self.mt,
                "mb": self.mb,
                "ml": self.ml,
                "mr": self.mr,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct TermSize {
    pub w: i64,
    pub h: i64,
}

impl PropertyValue for TermSize {
    fn to_json_array(&self) -> Vec<Value> {
        vec![
            json!("term-size"),
            json!({
                "w": self.w,
                "h": self.h,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct WindowId(pub i64);

impl PropertyValue for WindowId {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("window-id"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct MousePos {
    pub x: i64,
    pub y: i64,
    pub hover: bool,
}

impl PropertyValue for MousePos {
    fn to_json_array(&self) -> Vec<Value> {
        vec![
            json!("mouse-pos"),
            json!({
                "x": self.x,
                "y": self.y,
                "hover": self.hover,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct TouchPos(pub Vec<TouchPoint>);

#[derive(Debug)]
pub(crate) struct TouchPoint {
    pub x: i64,
    pub y: i64,
    pub id: i64,
}

impl PropertyValue for TouchPos {
    fn to_json_array(&self) -> Vec<Value> {
        let points: Vec<Value> = self
            .0
            .iter()
            .map(|p| {
                json!({
                    "x": p.x,
                    "y": p.y,
                    "id": p.id,
                })
            })
            .collect();
        vec![json!("touch-pos"), json!(points)]
    }
}

#[derive(Debug)]
pub(crate) struct SubAssExtradata(pub String);

impl PropertyValue for SubAssExtradata {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("sub-ass-extradata"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct SubText {
    pub text: String,
    pub ass: String,
    pub ass_full: String,
}

impl PropertyValue for SubText {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("sub-text"), json!(self.text)]
    }
}

#[derive(Debug)]
pub(crate) struct SubTextAss(pub String);

impl PropertyValue for SubTextAss {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("sub-text/ass"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct SubTextAssFull(pub String);

impl PropertyValue for SubTextAssFull {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("sub-text/ass-full"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct SecondarySubText {
    pub text: String,
    pub ass: String,
    pub ass_full: String,
}

impl PropertyValue for SecondarySubText {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("secondary-sub-text"), json!(self.text)]
    }
}

#[derive(Debug)]
pub(crate) struct SubStart {
    pub seconds: Option<f64>,
    pub full: Option<f64>,
}

impl PropertyValue for SubStart {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("sub-start"), json!(self.seconds)]
    }
}

#[derive(Debug)]
pub(crate) struct SubStartFull(pub f64);

impl PropertyValue for SubStartFull {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("sub-start/full"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct SecondarySubStart {
    pub seconds: Option<f64>,
    pub full: Option<f64>,
}

impl PropertyValue for SecondarySubStart {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("secondary-sub-start"), json!(self.seconds)]
    }
}

#[derive(Debug)]
pub(crate) struct SubEnd {
    pub seconds: Option<f64>,
    pub full: Option<f64>,
}

impl PropertyValue for SubEnd {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("sub-end"), json!(self.seconds)]
    }
}

#[derive(Debug)]
pub(crate) struct SubEndFull(pub f64);

impl PropertyValue for SubEndFull {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("sub-end/full"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct SecondarySubEnd {
    pub seconds: Option<f64>,
    pub full: Option<f64>,
}

impl PropertyValue for SecondarySubEnd {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("secondary-sub-end"), json!(self.seconds)]
    }
}

#[derive(Debug)]
pub(crate) struct PlaylistPos(pub i64);

impl PropertyValue for PlaylistPos {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("playlist-pos"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct PlaylistPos1(pub i64);

impl PropertyValue for PlaylistPos1 {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("playlist-pos-1"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct PlaylistCurrentPos(pub i64);

impl PropertyValue for PlaylistCurrentPos {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("playlist-current-pos"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct PlaylistPlayingPos(pub i64);

impl PropertyValue for PlaylistPlayingPos {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("playlist-playing-pos"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct PlaylistCount(pub i64);

impl PropertyValue for PlaylistCount {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("playlist-count"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct PlaylistPath(pub String);

impl PropertyValue for PlaylistPath {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("playlist-path"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Playlist(pub Vec<PlaylistEntry>);

#[derive(Debug)]
pub(crate) struct PlaylistEntry {
    pub filename: String,
    pub playing: bool,
    pub current: bool,
    pub title: Option<String>,
    pub id: i64,
    pub playlist_path: Option<String>,
}

impl PropertyValue for Playlist {
    fn to_json_array(&self) -> Vec<Value> {
        let entries: Vec<Value> = self
            .0
            .iter()
            .map(|e| {
                json!({
                    "filename": e.filename,
                    "playing": e.playing,
                    "current": e.current,
                    "title": e.title,
                    "id": e.id,
                    "playlist-path": e.playlist_path,
                })
            })
            .collect();
        vec![json!("playlist"), json!(entries)]
    }
}

#[derive(Debug)]
pub(crate) struct TrackList(pub Vec<Track>);

#[derive(Debug)]
pub(crate) struct Track {
    pub id: i64,
    pub type_: String,
    pub src_id: Option<i64>,
    pub title: Option<String>,
    pub lang: Option<String>,
    pub image: bool,
    pub albumart: bool,
    pub default: bool,
    pub forced: bool,
    pub dependent: bool,
    pub visual_impaired: bool,
    pub hearing_impaired: bool,
    pub hls_bitrate: Option<i64>,
    pub program_id: Option<i64>,
    pub codec: String,
    pub codec_desc: String,
    pub codec_profile: Option<String>,
    pub external: bool,
    pub external_filename: Option<String>,
    pub selected: bool,
    pub main_selection: Option<i64>,
    pub ff_index: i64,
    pub decoder: String,
    pub decoder_desc: String,
    pub demux_w: Option<i64>,
    pub demux_h: Option<i64>,
    pub demux_crop_x: Option<i64>,
    pub demux_crop_y: Option<i64>,
    pub demux_crop_w: Option<i64>,
    pub demux_crop_h: Option<i64>,
    pub demux_channel_count: Option<i64>,
    pub demux_channels: Option<String>,
    pub demux_samplerate: Option<i64>,
    pub demux_fps: Option<f64>,
    pub demux_bitrate: Option<i64>,
    pub demux_rotation: Option<i64>,
    pub demux_par: Option<f64>,
    pub format_name: String,
    pub replaygain_track_peak: Option<f64>,
    pub replaygain_track_gain: Option<f64>,
    pub replaygain_album_peak: Option<f64>,
    pub replaygain_album_gain: Option<f64>,
    pub dolby_vision_profile: Option<i64>,
    pub dolby_vision_level: Option<i64>,
    pub metadata: HashMap<String, String>,
}

impl PropertyValue for TrackList {
    fn to_json_array(&self) -> Vec<Value> {
        let tracks: Vec<Value> = self
            .0
            .iter()
            .map(|t| {
                json!({
                    "id": t.id,
                    "type": t.type_,
                    "src-id": t.src_id,
                    "title": t.title,
                    "lang": t.lang,
                    "image": t.image,
                    "albumart": t.albumart,
                    "default": t.default,
                    "forced": t.forced,
                    "dependent": t.dependent,
                    "visual-impaired": t.visual_impaired,
                    "hearing-impaired": t.hearing_impaired,
                    "hls-bitrate": t.hls_bitrate,
                    "program-id": t.program_id,
                    "codec": t.codec,
                    "codec-desc": t.codec_desc,
                    "codec-profile": t.codec_profile,
                    "external": t.external,
                    "external-filename": t.external_filename,
                    "selected": t.selected,
                    "main-selection": t.main_selection,
                    "ff-index": t.ff_index,
                    "decoder": t.decoder,
                    "decoder-desc": t.decoder_desc,
                    "demux-w": t.demux_w,
                    "demux-h": t.demux_h,
                    "demux-crop-x": t.demux_crop_x,
                    "demux-crop-y": t.demux_crop_y,
                    "demux-crop-w": t.demux_crop_w,
                    "demux-crop-h": t.demux_crop_h,
                    "demux-channel-count": t.demux_channel_count,
                    "demux-channels": t.demux_channels,
                    "demux-samplerate": t.demux_samplerate,
                    "demux-fps": t.demux_fps,
                    "demux-bitrate": t.demux_bitrate,
                    "demux-rotation": t.demux_rotation,
                    "demux-par": t.demux_par,
                    "format-name": t.format_name,
                    "replaygain-track-peak": t.replaygain_track_peak,
                    "replaygain-track-gain": t.replaygain_track_gain,
                    "replaygain-album-peak": t.replaygain_album_peak,
                    "replaygain-album-gain": t.replaygain_album_gain,
                    "dolby-vision-profile": t.dolby_vision_profile,
                    "dolby-vision-level": t.dolby_vision_level,
                    "metadata": t.metadata,
                })
            })
            .collect();
        vec![json!("track-list"), json!(tracks)]
    }
}

#[derive(Debug)]
pub(crate) struct ChapterList(pub Vec<ChapterListChapter>);

#[derive(Debug)]
pub(crate) struct ChapterListChapter {
    pub title: Option<String>,
    pub time: f64,
}

impl PropertyValue for ChapterList {
    fn to_json_array(&self) -> Vec<Value> {
        let chapters: Vec<Value> = self
            .0
            .iter()
            .map(|c| {
                json!({
                    "title": c.title,
                    "time": c.time,
                })
            })
            .collect();
        vec![json!("chapter-list"), json!(chapters)]
    }
}

#[derive(Debug)]
pub(crate) struct Af(pub Vec<Filter>);

#[derive(Debug)]
pub(crate) struct Filter {
    pub name: String,
    pub label: Option<String>,
    pub enabled: bool,
    pub params: HashMap<String, String>,
}

impl PropertyValue for Af {
    fn to_json_array(&self) -> Vec<Value> {
        let filters: Vec<Value> = self
            .0
            .iter()
            .map(|f| {
                json!({
                    "name": f.name,
                    "label": f.label,
                    "enabled": f.enabled,
                    "params": f.params,
                })
            })
            .collect();
        vec![json!("af"), json!(filters)]
    }
}

#[derive(Debug)]
pub(crate) struct Vf(pub Vec<Filter>);

impl PropertyValue for Vf {
    fn to_json_array(&self) -> Vec<Value> {
        let filters: Vec<Value> = self
            .0
            .iter()
            .map(|f| {
                json!({
                    "name": f.name,
                    "label": f.label,
                    "enabled": f.enabled,
                    "params": f.params,
                })
            })
            .collect();
        vec![json!("vf"), json!(filters)]
    }
}

#[derive(Debug)]
pub(crate) struct Seekable(pub bool);

impl PropertyValue for Seekable {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("seekable"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct PartiallySeekable(pub bool);

impl PropertyValue for PartiallySeekable {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("partially-seekable"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct PlaybackAbort(pub bool);

impl PropertyValue for PlaybackAbort {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("playback-abort"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct CursorAutohide(pub i64);

impl PropertyValue for CursorAutohide {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("cursor-autohide"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct TermClipCc(pub String);

impl PropertyValue for TermClipCc {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("term-clip-cc"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct OsdSymCc(pub String);

impl PropertyValue for OsdSymCc {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("osd-sym-cc"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct OsdAssCc(pub bool);

impl PropertyValue for OsdAssCc {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("osd-ass-cc"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct VoConfigured(pub bool);

impl PropertyValue for VoConfigured {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("vo-configured"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct VoPasses {
    pub fresh: Vec<Pass>,
    pub redraw: Vec<Pass>,
}

#[derive(Debug)]
pub(crate) struct Pass {
    pub desc: String,
    pub last: i64,
    pub avg: i64,
    pub peak: i64,
    pub count: i64,
    pub samples: Vec<i64>,
}

impl PropertyValue for VoPasses {
    fn to_json_array(&self) -> Vec<Value> {
        let fresh: Vec<Value> = self
            .fresh
            .iter()
            .map(|p| {
                json!({
                    "desc": p.desc,
                    "last": p.last,
                    "avg": p.avg,
                    "peak": p.peak,
                    "count": p.count,
                    "samples": p.samples,
                })
            })
            .collect();
        let redraw: Vec<Value> = self
            .redraw
            .iter()
            .map(|p| {
                json!({
                    "desc": p.desc,
                    "last": p.last,
                    "avg": p.avg,
                    "peak": p.peak,
                    "count": p.count,
                    "samples": p.samples,
                })
            })
            .collect();
        vec![
            json!("vo-passes"),
            json!({
                "fresh": fresh,
                "redraw": redraw,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct PerfInfo(pub HashMap<String, Value>);

impl PropertyValue for PerfInfo {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("perf-info"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct VideoBitrate(pub i64);

impl PropertyValue for VideoBitrate {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("video-bitrate"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct AudioBitrate(pub i64);

impl PropertyValue for AudioBitrate {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("audio-bitrate"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct SubBitrate(pub i64);

impl PropertyValue for SubBitrate {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("sub-bitrate"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct AudioDeviceList(pub Vec<AudioDevice>);

#[derive(Debug)]
pub(crate) struct AudioDevice {
    pub name: String,
    pub description: String,
}

impl PropertyValue for AudioDeviceList {
    fn to_json_array(&self) -> Vec<Value> {
        let devices: Vec<Value> = self
            .0
            .iter()
            .map(|d| {
                json!({
                    "name": d.name,
                    "description": d.description,
                })
            })
            .collect();
        vec![json!("audio-device-list"), json!(devices)]
    }
}

#[derive(Debug)]
pub(crate) struct AudioDeviceListItem(pub String);

impl PropertyValue for AudioDeviceListItem {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("audio-device"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct CurrentVo(pub String);

impl PropertyValue for CurrentVo {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("current-vo"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct CurrentGpuContext(pub String);

impl PropertyValue for CurrentGpuContext {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("current-gpu-context"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct CurrentAo(pub String);

impl PropertyValue for CurrentAo {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("current-ao"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct UserData(pub HashMap<String, Value>);

impl PropertyValue for UserData {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("user-data"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct MenuData(pub Vec<MenuItem>);

#[derive(Debug, serde::Serialize)]
pub(crate) struct MenuItem {
    pub type_: String,
    pub title: Option<String>,
    pub cmd: Option<String>,
    pub shortcut: Option<String>,
    pub state: Vec<String>,
    pub submenu: Option<Vec<MenuItem>>,
}

impl PropertyValue for MenuData {
    fn to_json_array(&self) -> Vec<Value> {
        let items: Vec<Value> = self
            .0
            .iter()
            .map(|i| {
                json!({
                    "type": i.type_,
                    "title": i.title,
                    "cmd": i.cmd,
                    "shortcut": i.shortcut,
                    "state": i.state,
                    "submenu": i.submenu,
                })
            })
            .collect();
        vec![json!("menu-data"), json!(items)]
    }
}

#[derive(Debug)]
pub(crate) struct WorkingDirectory(pub String);

impl PropertyValue for WorkingDirectory {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("working-directory"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct CurrentWatchLaterDir(pub String);

impl PropertyValue for CurrentWatchLaterDir {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("current-watch-later-dir"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct ProtocolList(pub Vec<String>);

impl PropertyValue for ProtocolList {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("protocol-list"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct DecoderList(pub Vec<Decoder>);

#[derive(Debug)]
pub(crate) struct Decoder {
    pub codec: String,
    pub driver: String,
    pub description: String,
}

impl PropertyValue for DecoderList {
    fn to_json_array(&self) -> Vec<Value> {
        let decoders: Vec<Value> = self
            .0
            .iter()
            .map(|d| {
                json!({
                    "codec": d.codec,
                    "driver": d.driver,
                    "description": d.description,
                })
            })
            .collect();
        vec![json!("decoder-list"), json!(decoders)]
    }
}

#[derive(Debug)]
pub(crate) struct EncoderList(pub Vec<Encoder>);

#[derive(Debug)]
pub(crate) struct Encoder {
    pub codec: String,
    pub driver: String,
    pub description: String,
}

impl PropertyValue for EncoderList {
    fn to_json_array(&self) -> Vec<Value> {
        let encoders: Vec<Value> = self
            .0
            .iter()
            .map(|e| {
                json!({
                    "codec": e.codec,
                    "driver": e.driver,
                    "description": e.description,
                })
            })
            .collect();
        vec![json!("encoder-list"), json!(encoders)]
    }
}

#[derive(Debug)]
pub(crate) struct DemuxerLavfList(pub Vec<String>);

impl PropertyValue for DemuxerLavfList {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("demuxer-lavf-list"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct InputKeyList(pub Vec<String>);

impl PropertyValue for InputKeyList {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("input-key-list"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct MpvVersion(pub String);

impl PropertyValue for MpvVersion {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("mpv-version"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct MpvConfiguration(pub String);

impl PropertyValue for MpvConfiguration {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("mpv-configuration"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct FfmpegVersion(pub String);

impl PropertyValue for FfmpegVersion {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("ffmpeg-version"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct LibassVersion(pub i64);

impl PropertyValue for LibassVersion {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("libass-version"), json!(self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Platform(pub String);

impl PropertyValue for Platform {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("platform"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Options(pub HashMap<String, String>);

impl PropertyValue for Options {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("options"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct FileLocalOptions(pub HashMap<String, String>);

impl PropertyValue for FileLocalOptions {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("file-local-options"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct OptionInfo {
    pub name: String,
    pub type_: String,
    pub set_from_commandline: bool,
    pub set_locally: bool,
    pub expects_file: bool,
    pub default_value: Option<String>,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub choices: Vec<String>,
}

impl PropertyValue for OptionInfo {
    fn to_json_array(&self) -> Vec<Value> {
        vec![
            json!("option-info"),
            json!({
                "name": self.name,
                "type": self.type_,
                "set-from-commandline": self.set_from_commandline,
                "set-locally": self.set_locally,
                "expects-file": self.expects_file,
                "default-value": self.default_value,
                "min": self.min,
                "max": self.max,
                "choices": self.choices,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct PropertyList(pub Vec<String>);

impl PropertyValue for PropertyList {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("property-list"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct ProfileList(pub Vec<Profile>);

#[derive(Debug)]
pub(crate) struct Profile {
    pub name: String,
    pub options: Vec<(String, String)>,
}

impl PropertyValue for ProfileList {
    fn to_json_array(&self) -> Vec<Value> {
        let profiles: Vec<Value> = self
            .0
            .iter()
            .map(|p| {
                json!({
                    "name": p.name,
                    "options": p.options,
                })
            })
            .collect();
        vec![json!("profile-list"), json!(profiles)]
    }
}

#[derive(Debug)]
pub(crate) struct CommandList(pub Vec<Command>);

#[derive(Debug)]
pub(crate) struct Command {
    pub name: String,
}

impl PropertyValue for CommandList {
    fn to_json_array(&self) -> Vec<Value> {
        let commands: Vec<Value> = self
            .0
            .iter()
            .map(|c| {
                json!({
                    "name": c.name,
                })
            })
            .collect();
        vec![json!("command-list"), json!(commands)]
    }
}

#[derive(Debug)]
pub(crate) struct InputBindings(pub Vec<InputBinding>);

#[derive(Debug)]
pub(crate) struct InputBinding {
    pub key: String,
    pub cmd: String,
    pub is_weak: bool,
    pub owner: Option<String>,
    pub section: Option<String>,
    pub priority: i64,
    pub comment: Option<String>,
}

impl PropertyValue for InputBindings {
    fn to_json_array(&self) -> Vec<Value> {
        let bindings: Vec<Value> = self
            .0
            .iter()
            .map(|b| {
                json!({
                    "key": b.key,
                    "cmd": b.cmd,
                    "is-weak": b.is_weak,
                    "owner": b.owner,
                    "section": b.section,
                    "priority": b.priority,
                    "comment": b.comment,
                })
            })
            .collect();
        vec![json!("input-bindings"), json!(bindings)]
    }
}

#[derive(Debug)]
pub(crate) struct Clipboard {
    pub text: String,
    pub text_primary: Option<String>,
}

impl PropertyValue for Clipboard {
    fn to_json_array(&self) -> Vec<Value> {
        vec![
            json!("clipboard"),
            json!({
                "text": self.text,
                "text-primary": self.text_primary,
            }),
        ]
    }
}

#[derive(Debug)]
pub(crate) struct CurrentClipboardBackend(pub String);

impl PropertyValue for CurrentClipboardBackend {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("current-clipboard-backend"), json!(&self.0)]
    }
}

#[derive(Debug)]
pub(crate) struct Clock(pub String);

impl PropertyValue for Clock {
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!("clock"), json!(&self.0)]
    }
}
