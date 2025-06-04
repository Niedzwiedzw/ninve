use {
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub by_key: HashMap<String, String>,
    pub list: Vec<MetadataEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataEntry {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditionList {
    pub count: i64,
    pub editions: Vec<Edition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edition {
    pub id: i64,
    pub title: Option<String>,
    pub default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemuxerCacheState {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeekableRange {
    pub start: f64,
    pub end: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsPerStream {
    pub r#type: String,
    pub cache_duration: Option<f64>,
    pub reader_pts: Option<f64>,
    pub cache_end: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioParams {
    pub format: String,
    pub samplerate: i64,
    pub channels: String,
    pub channel_count: i64,
    pub hr_channels: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoParams {
    pub pixelformat: String,
    pub hw_pixelformat: Option<String>,
    pub average_bpp: Option<i64>,
    pub w: i64,
    pub h: i64,
    pub dw: i64,
    pub dh: i64,
    pub crop_x: Option<i64>,
    pub crop_y: Option<i64>,
    pub crop_w: Option<i64>,
    pub crop_h: Option<i64>,
    pub aspect: f64,
    pub aspect_name: Option<String>,
    pub par: f64,
    pub sar: f64,
    pub sar_name: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFrameInfo {
    pub picture_type: Option<String>,
    pub interlaced: bool,
    pub tff: bool,
    pub repeat: bool,
    pub gop_timecode: Option<String>,
    pub smpte_timecode: Option<String>,
    pub estimated_smpte_timecode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsdDimensions {
    pub w: i64,
    pub h: i64,
    pub par: f64,
    pub aspect: f64,
    pub mt: i64,
    pub mb: i64,
    pub ml: i64,
    pub mr: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MousePos {
    pub x: i64,
    pub y: i64,
    pub hover: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TouchPos {
    pub count: i64,
    pub points: Vec<TouchPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TouchPoint {
    pub x: i64,
    pub y: i64,
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub count: i64,
    pub entries: Vec<PlaylistEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistEntry {
    pub filename: String,
    pub current: Option<bool>,
    pub playing: Option<bool>,
    pub title: Option<String>,
    pub id: i64,
    pub playlist_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackList {
    pub count: i64,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: i64,
    pub r#type: String,
    pub src_id: Option<i64>,
    pub title: Option<String>,
    pub lang: Option<String>,
    pub image: Option<bool>,
    pub albumart: Option<bool>,
    pub default: Option<bool>,
    pub forced: Option<bool>,
    pub dependent: Option<bool>,
    pub visual_impaired: Option<bool>,
    pub hearing_impaired: Option<bool>,
    pub hls_bitrate: Option<i64>,
    pub program_id: Option<i64>,
    pub selected: Option<bool>,
    pub main_selection: Option<i64>,
    pub external: Option<bool>,
    pub external_filename: Option<String>,
    pub codec: Option<String>,
    pub codec_desc: Option<String>,
    pub codec_profile: Option<String>,
    pub ff_index: Option<i64>,
    pub decoder: Option<String>,
    pub decoder_desc: Option<String>,
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
    pub format_name: Option<String>,
    pub audio_channels: Option<i64>,
    pub replaygain_track_peak: Option<f64>,
    pub replaygain_track_gain: Option<f64>,
    pub replaygain_album_peak: Option<f64>,
    pub replaygain_album_gain: Option<f64>,
    pub dolby_vision_profile: Option<i64>,
    pub dolby_vision_level: Option<i64>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterList {
    pub count: i64,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub title: Option<String>,
    pub time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub name: String,
    pub label: Option<String>,
    pub enabled: Option<bool>,
    pub params: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoPass {
    pub desc: String,
    pub last: i64,
    pub avg: i64,
    pub peak: i64,
    pub count: i64,
    pub samples: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoPasses {
    pub fresh: Vec<VoPass>,
    pub redraw: Vec<VoPass>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decoder {
    pub codec: String,
    pub driver: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputBinding {
    pub key: String,
    pub cmd: String,
    pub is_weak: bool,
    pub owner: Option<String>,
    pub section: Option<String>,
    pub priority: i64,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub r#type: String,
    pub title: Option<String>,
    pub cmd: Option<String>,
    pub shortcut: Option<String>,
    pub state: Vec<String>,
    pub submenu: Option<Vec<MenuItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionInfo {
    pub name: String,
    pub r#type: String,
    pub set_from_commandline: bool,
    pub set_locally: bool,
    pub expects_file: bool,
    pub default_value: Option<String>,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub choices: Option<Vec<String>>,
}
