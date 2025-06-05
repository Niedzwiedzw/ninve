use super::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Pause(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PauseKind;

impl PropertyValueName for PauseKind {
    type Value = Pause;
    fn as_str() -> &'static str {
        "pause"
    }
}
impl SetPropertyValue for Pause {
    type Kind = PauseKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AudioSpeedCorrection(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AudioSpeedCorrectionKind;

impl PropertyValueName for AudioSpeedCorrectionKind {
    type Value = AudioSpeedCorrection;
    fn as_str() -> &'static str {
        "audio-speed-correction"
    }
}
impl SetPropertyValue for AudioSpeedCorrection {
    type Kind = AudioSpeedCorrectionKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VideoSpeedCorrection(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VideoSpeedCorrectionKind;

impl PropertyValueName for VideoSpeedCorrectionKind {
    type Value = VideoSpeedCorrection;
    fn as_str() -> &'static str {
        "video-speed-correction"
    }
}
impl SetPropertyValue for VideoSpeedCorrection {
    type Kind = VideoSpeedCorrectionKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DisplaySyncActive(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DisplaySyncActiveKind;

impl PropertyValueName for DisplaySyncActiveKind {
    type Value = DisplaySyncActive;
    fn as_str() -> &'static str {
        "display-sync-active"
    }
}
impl SetPropertyValue for DisplaySyncActive {
    type Kind = DisplaySyncActiveKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Filename(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FilenameKind;

impl PropertyValueName for FilenameKind {
    type Value = Filename;
    fn as_str() -> &'static str {
        "filename"
    }
}
impl SetPropertyValue for Filename {
    type Kind = FilenameKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FilenameNoExt(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FilenameNoExtKind;

impl PropertyValueName for FilenameNoExtKind {
    type Value = FilenameNoExt;
    fn as_str() -> &'static str {
        "filename/no-ext"
    }
}
impl SetPropertyValue for FilenameNoExt {
    type Kind = FilenameNoExtKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FileSize(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FileSizeKind;

impl PropertyValueName for FileSizeKind {
    type Value = FileSize;
    fn as_str() -> &'static str {
        "file-size"
    }
}
impl SetPropertyValue for FileSize {
    type Kind = FileSizeKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EstimatedFrameCount(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EstimatedFrameCountKind;

impl PropertyValueName for EstimatedFrameCountKind {
    type Value = EstimatedFrameCount;
    fn as_str() -> &'static str {
        "estimated-frame-count"
    }
}
impl SetPropertyValue for EstimatedFrameCount {
    type Kind = EstimatedFrameCountKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EstimatedFrameNumber(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EstimatedFrameNumberKind;

impl PropertyValueName for EstimatedFrameNumberKind {
    type Value = EstimatedFrameNumber;
    fn as_str() -> &'static str {
        "estimated-frame-number"
    }
}
impl SetPropertyValue for EstimatedFrameNumber {
    type Kind = EstimatedFrameNumberKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Pid(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PidKind;

impl PropertyValueName for PidKind {
    type Value = Pid;
    fn as_str() -> &'static str {
        "pid"
    }
}
impl SetPropertyValue for Pid {
    type Kind = PidKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Path(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PathKind;

impl PropertyValueName for PathKind {
    type Value = Path;
    fn as_str() -> &'static str {
        "path"
    }
}
impl SetPropertyValue for Path {
    type Kind = PathKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct StreamOpenFilename(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StreamOpenFilenameKind;

impl PropertyValueName for StreamOpenFilenameKind {
    type Value = StreamOpenFilename;
    fn as_str() -> &'static str {
        "stream-open-filename"
    }
}
impl SetPropertyValue for StreamOpenFilename {
    type Kind = StreamOpenFilenameKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MediaTitle(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MediaTitleKind;

impl PropertyValueName for MediaTitleKind {
    type Value = MediaTitle;
    fn as_str() -> &'static str {
        "media-title"
    }
}
impl SetPropertyValue for MediaTitle {
    type Kind = MediaTitleKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FileFormat(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FileFormatKind;

impl PropertyValueName for FileFormatKind {
    type Value = FileFormat;
    fn as_str() -> &'static str {
        "file-format"
    }
}
impl SetPropertyValue for FileFormat {
    type Kind = FileFormatKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CurrentDemuxer(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrentDemuxerKind;

impl PropertyValueName for CurrentDemuxerKind {
    type Value = CurrentDemuxer;
    fn as_str() -> &'static str {
        "current-demuxer"
    }
}
impl SetPropertyValue for CurrentDemuxer {
    type Kind = CurrentDemuxerKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct StreamPath(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StreamPathKind;

impl PropertyValueName for StreamPathKind {
    type Value = StreamPath;
    fn as_str() -> &'static str {
        "stream-path"
    }
}
impl SetPropertyValue for StreamPath {
    type Kind = StreamPathKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct StreamPos(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StreamPosKind;

impl PropertyValueName for StreamPosKind {
    type Value = StreamPos;
    fn as_str() -> &'static str {
        "stream-pos"
    }
}
impl SetPropertyValue for StreamPos {
    type Kind = StreamPosKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct StreamEnd(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StreamEndKind;

impl PropertyValueName for StreamEndKind {
    type Value = StreamEnd;
    fn as_str() -> &'static str {
        "stream-end"
    }
}
impl SetPropertyValue for StreamEnd {
    type Kind = StreamEndKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Duration(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DurationKind;

impl PropertyValueName for DurationKind {
    type Value = Duration;
    fn as_str() -> &'static str {
        "duration"
    }
}
impl SetPropertyValue for Duration {
    type Kind = DurationKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DurationFull(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DurationFullKind;

impl PropertyValueName for DurationFullKind {
    type Value = DurationFull;
    fn as_str() -> &'static str {
        "duration/full"
    }
}
impl SetPropertyValue for DurationFull {
    type Kind = DurationFullKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Avsync(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AvsyncKind;

impl PropertyValueName for AvsyncKind {
    type Value = Avsync;
    fn as_str() -> &'static str {
        "avsync"
    }
}
impl SetPropertyValue for Avsync {
    type Kind = AvsyncKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TotalAvsyncChange(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TotalAvsyncChangeKind;

impl PropertyValueName for TotalAvsyncChangeKind {
    type Value = TotalAvsyncChange;
    fn as_str() -> &'static str {
        "total-avsync-change"
    }
}
impl SetPropertyValue for TotalAvsyncChange {
    type Kind = TotalAvsyncChangeKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DecoderFrameDropCount(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DecoderFrameDropCountKind;

impl PropertyValueName for DecoderFrameDropCountKind {
    type Value = DecoderFrameDropCount;
    fn as_str() -> &'static str {
        "decoder-frame-drop-count"
    }
}
impl SetPropertyValue for DecoderFrameDropCount {
    type Kind = DecoderFrameDropCountKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FrameDropCount(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FrameDropCountKind;

impl PropertyValueName for FrameDropCountKind {
    type Value = FrameDropCount;
    fn as_str() -> &'static str {
        "frame-drop-count"
    }
}
impl SetPropertyValue for FrameDropCount {
    type Kind = FrameDropCountKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MistimedFrameCount(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MistimedFrameCountKind;

impl PropertyValueName for MistimedFrameCountKind {
    type Value = MistimedFrameCount;
    fn as_str() -> &'static str {
        "mistimed-frame-count"
    }
}
impl SetPropertyValue for MistimedFrameCount {
    type Kind = MistimedFrameCountKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VsyncRatio(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VsyncRatioKind;

impl PropertyValueName for VsyncRatioKind {
    type Value = VsyncRatio;
    fn as_str() -> &'static str {
        "vsync-ratio"
    }
}
impl SetPropertyValue for VsyncRatio {
    type Kind = VsyncRatioKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VoDelayedFrameCount(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VoDelayedFrameCountKind;

impl PropertyValueName for VoDelayedFrameCountKind {
    type Value = VoDelayedFrameCount;
    fn as_str() -> &'static str {
        "vo-delayed-frame-count"
    }
}
impl SetPropertyValue for VoDelayedFrameCount {
    type Kind = VoDelayedFrameCountKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PercentPos(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PercentPosKind;

impl PropertyValueName for PercentPosKind {
    type Value = PercentPos;
    fn as_str() -> &'static str {
        "percent-pos"
    }
}
impl SetPropertyValue for PercentPos {
    type Kind = PercentPosKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TimePos(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TimePosKind;

impl PropertyValueName for TimePosKind {
    type Value = TimePos;
    fn as_str() -> &'static str {
        "time-pos"
    }
}
impl SetPropertyValue for TimePos {
    type Kind = TimePosKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TimePosFull(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TimePosFullKind;

impl PropertyValueName for TimePosFullKind {
    type Value = TimePosFull;
    fn as_str() -> &'static str {
        "time-pos/full"
    }
}
impl SetPropertyValue for TimePosFull {
    type Kind = TimePosFullKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TimeStart(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TimeStartKind;

impl PropertyValueName for TimeStartKind {
    type Value = TimeStart;
    fn as_str() -> &'static str {
        "time-start"
    }
}
impl SetPropertyValue for TimeStart {
    type Kind = TimeStartKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TimeRemaining(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TimeRemainingKind;

impl PropertyValueName for TimeRemainingKind {
    type Value = TimeRemaining;
    fn as_str() -> &'static str {
        "time-remaining"
    }
}
impl SetPropertyValue for TimeRemaining {
    type Kind = TimeRemainingKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TimeRemainingFull(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TimeRemainingFullKind;

impl PropertyValueName for TimeRemainingFullKind {
    type Value = TimeRemainingFull;
    fn as_str() -> &'static str {
        "time-remaining/full"
    }
}
impl SetPropertyValue for TimeRemainingFull {
    type Kind = TimeRemainingFullKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AudioPts(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AudioPtsKind;

impl PropertyValueName for AudioPtsKind {
    type Value = AudioPts;
    fn as_str() -> &'static str {
        "audio-pts"
    }
}
impl SetPropertyValue for AudioPts {
    type Kind = AudioPtsKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AudioPtsFull(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AudioPtsFullKind;

impl PropertyValueName for AudioPtsFullKind {
    type Value = AudioPtsFull;
    fn as_str() -> &'static str {
        "audio-pts/full"
    }
}
impl SetPropertyValue for AudioPtsFull {
    type Kind = AudioPtsFullKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlaytimeRemaining(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaytimeRemainingKind;

impl PropertyValueName for PlaytimeRemainingKind {
    type Value = PlaytimeRemaining;
    fn as_str() -> &'static str {
        "playtime-remaining"
    }
}
impl SetPropertyValue for PlaytimeRemaining {
    type Kind = PlaytimeRemainingKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlaytimeRemainingFull(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaytimeRemainingFullKind;

impl PropertyValueName for PlaytimeRemainingFullKind {
    type Value = PlaytimeRemainingFull;
    fn as_str() -> &'static str {
        "playtime-remaining/full"
    }
}
impl SetPropertyValue for PlaytimeRemainingFull {
    type Kind = PlaytimeRemainingFullKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlaybackTime(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaybackTimeKind;

impl PropertyValueName for PlaybackTimeKind {
    type Value = PlaybackTime;
    fn as_str() -> &'static str {
        "playback-time"
    }
}
impl SetPropertyValue for PlaybackTime {
    type Kind = PlaybackTimeKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlaybackTimeFull(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaybackTimeFullKind;

impl PropertyValueName for PlaybackTimeFullKind {
    type Value = PlaybackTimeFull;
    fn as_str() -> &'static str {
        "playback-time/full"
    }
}
impl SetPropertyValue for PlaybackTimeFull {
    type Kind = PlaybackTimeFullKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RemainingFileLoops(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RemainingFileLoopsKind;

impl PropertyValueName for RemainingFileLoopsKind {
    type Value = RemainingFileLoops;
    fn as_str() -> &'static str {
        "remaining-file-loops"
    }
}
impl SetPropertyValue for RemainingFileLoops {
    type Kind = RemainingFileLoopsKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RemainingAbLoops(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RemainingAbLoopsKind;

impl PropertyValueName for RemainingAbLoopsKind {
    type Value = RemainingAbLoops;
    fn as_str() -> &'static str {
        "remaining-ab-loops"
    }
}
impl SetPropertyValue for RemainingAbLoops {
    type Kind = RemainingAbLoopsKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Chapter(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ChapterKind;

impl PropertyValueName for ChapterKind {
    type Value = Chapter;
    fn as_str() -> &'static str {
        "chapter"
    }
}
impl SetPropertyValue for Chapter {
    type Kind = ChapterKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Edition(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EditionKind;

impl PropertyValueName for EditionKind {
    type Value = Edition;
    fn as_str() -> &'static str {
        "edition"
    }
}
impl SetPropertyValue for Edition {
    type Kind = EditionKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CurrentEdition(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrentEditionKind;

impl PropertyValueName for CurrentEditionKind {
    type Value = CurrentEdition;
    fn as_str() -> &'static str {
        "current-edition"
    }
}
impl SetPropertyValue for CurrentEdition {
    type Kind = CurrentEditionKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Chapters(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ChaptersKind;

impl PropertyValueName for ChaptersKind {
    type Value = Chapters;
    fn as_str() -> &'static str {
        "chapters"
    }
}
impl SetPropertyValue for Chapters {
    type Kind = ChaptersKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Editions(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EditionsKind;

impl PropertyValueName for EditionsKind {
    type Value = Editions;
    fn as_str() -> &'static str {
        "editions"
    }
}
impl SetPropertyValue for Editions {
    type Kind = EditionsKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EditionList(pub fields::EditionList);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EditionListKind;

impl PropertyValueName for EditionListKind {
    type Value = EditionList;
    fn as_str() -> &'static str {
        "edition-list"
    }
}
impl SetPropertyValue for EditionList {
    type Kind = EditionListKind;
    type Primitive = fields::EditionList;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Metadata(pub fields::Metadata);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MetadataKind;

impl PropertyValueName for MetadataKind {
    type Value = Metadata;
    fn as_str() -> &'static str {
        "metadata"
    }
}
impl SetPropertyValue for Metadata {
    type Kind = MetadataKind;
    type Primitive = fields::Metadata;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FilteredMetadata(pub fields::Metadata);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FilteredMetadataKind;

impl PropertyValueName for FilteredMetadataKind {
    type Value = FilteredMetadata;
    fn as_str() -> &'static str {
        "filtered-metadata"
    }
}
impl SetPropertyValue for FilteredMetadata {
    type Kind = FilteredMetadataKind;
    type Primitive = fields::Metadata;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ChapterMetadata(pub fields::Metadata);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ChapterMetadataKind;

impl PropertyValueName for ChapterMetadataKind {
    type Value = ChapterMetadata;
    fn as_str() -> &'static str {
        "chapter-metadata"
    }
}
impl SetPropertyValue for ChapterMetadata {
    type Kind = ChapterMetadataKind;
    type Primitive = fields::Metadata;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VfMetadata(pub std::collections::HashMap<String, String>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VfMetadataKind;

impl PropertyValueName for VfMetadataKind {
    type Value = VfMetadata;
    fn as_str() -> &'static str {
        "vf-metadata"
    }
}
impl SetPropertyValue for VfMetadata {
    type Kind = VfMetadataKind;
    type Primitive = std::collections::HashMap<String, String>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AfMetadata(pub std::collections::HashMap<String, String>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AfMetadataKind;

impl PropertyValueName for AfMetadataKind {
    type Value = AfMetadata;
    fn as_str() -> &'static str {
        "af-metadata"
    }
}
impl SetPropertyValue for AfMetadata {
    type Kind = AfMetadataKind;
    type Primitive = std::collections::HashMap<String, String>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DeinterlaceActive(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DeinterlaceActiveKind;

impl PropertyValueName for DeinterlaceActiveKind {
    type Value = DeinterlaceActive;
    fn as_str() -> &'static str {
        "deinterlace-active"
    }
}
impl SetPropertyValue for DeinterlaceActive {
    type Kind = DeinterlaceActiveKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct IdleActive(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct IdleActiveKind;

impl PropertyValueName for IdleActiveKind {
    type Value = IdleActive;
    fn as_str() -> &'static str {
        "idle-active"
    }
}
impl SetPropertyValue for IdleActive {
    type Kind = IdleActiveKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CoreIdle(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CoreIdleKind;

impl PropertyValueName for CoreIdleKind {
    type Value = CoreIdle;
    fn as_str() -> &'static str {
        "core-idle"
    }
}
impl SetPropertyValue for CoreIdle {
    type Kind = CoreIdleKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CacheSpeed(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CacheSpeedKind;

impl PropertyValueName for CacheSpeedKind {
    type Value = CacheSpeed;
    fn as_str() -> &'static str {
        "cache-speed"
    }
}
impl SetPropertyValue for CacheSpeed {
    type Kind = CacheSpeedKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DemuxerCacheDuration(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DemuxerCacheDurationKind;

impl PropertyValueName for DemuxerCacheDurationKind {
    type Value = DemuxerCacheDuration;
    fn as_str() -> &'static str {
        "demuxer-cache-duration"
    }
}
impl SetPropertyValue for DemuxerCacheDuration {
    type Kind = DemuxerCacheDurationKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DemuxerCacheTime(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DemuxerCacheTimeKind;

impl PropertyValueName for DemuxerCacheTimeKind {
    type Value = DemuxerCacheTime;
    fn as_str() -> &'static str {
        "demuxer-cache-time"
    }
}
impl SetPropertyValue for DemuxerCacheTime {
    type Kind = DemuxerCacheTimeKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DemuxerCacheIdle(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DemuxerCacheIdleKind;

impl PropertyValueName for DemuxerCacheIdleKind {
    type Value = DemuxerCacheIdle;
    fn as_str() -> &'static str {
        "demuxer-cache-idle"
    }
}
impl SetPropertyValue for DemuxerCacheIdle {
    type Kind = DemuxerCacheIdleKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DemuxerCacheState(pub fields::DemuxerCacheState);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DemuxerCacheStateKind;

impl PropertyValueName for DemuxerCacheStateKind {
    type Value = DemuxerCacheState;
    fn as_str() -> &'static str {
        "demuxer-cache-state"
    }
}
impl SetPropertyValue for DemuxerCacheState {
    type Kind = DemuxerCacheStateKind;
    type Primitive = fields::DemuxerCacheState;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DemuxerViaNetwork(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DemuxerViaNetworkKind;

impl PropertyValueName for DemuxerViaNetworkKind {
    type Value = DemuxerViaNetwork;
    fn as_str() -> &'static str {
        "demuxer-via-network"
    }
}
impl SetPropertyValue for DemuxerViaNetwork {
    type Kind = DemuxerViaNetworkKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DemuxerStartTime(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DemuxerStartTimeKind;

impl PropertyValueName for DemuxerStartTimeKind {
    type Value = DemuxerStartTime;
    fn as_str() -> &'static str {
        "demuxer-start-time"
    }
}
impl SetPropertyValue for DemuxerStartTime {
    type Kind = DemuxerStartTimeKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PausedForCache(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PausedForCacheKind;

impl PropertyValueName for PausedForCacheKind {
    type Value = PausedForCache;
    fn as_str() -> &'static str {
        "paused-for-cache"
    }
}
impl SetPropertyValue for PausedForCache {
    type Kind = PausedForCacheKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CacheBufferingState(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CacheBufferingStateKind;

impl PropertyValueName for CacheBufferingStateKind {
    type Value = CacheBufferingState;
    fn as_str() -> &'static str {
        "cache-buffering-state"
    }
}
impl SetPropertyValue for CacheBufferingState {
    type Kind = CacheBufferingStateKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EofReached(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EofReachedKind;

impl PropertyValueName for EofReachedKind {
    type Value = EofReached;
    fn as_str() -> &'static str {
        "eof-reached"
    }
}
impl SetPropertyValue for EofReached {
    type Kind = EofReachedKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Seeking(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SeekingKind;

impl PropertyValueName for SeekingKind {
    type Value = Seeking;
    fn as_str() -> &'static str {
        "seeking"
    }
}
impl SetPropertyValue for Seeking {
    type Kind = SeekingKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MixerActive(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MixerActiveKind;

impl PropertyValueName for MixerActiveKind {
    type Value = MixerActive;
    fn as_str() -> &'static str {
        "mixer-active"
    }
}
impl SetPropertyValue for MixerActive {
    type Kind = MixerActiveKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AoVolume(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AoVolumeKind;

impl PropertyValueName for AoVolumeKind {
    type Value = AoVolume;
    fn as_str() -> &'static str {
        "ao-volume"
    }
}
impl SetPropertyValue for AoVolume {
    type Kind = AoVolumeKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AoMute(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AoMuteKind;

impl PropertyValueName for AoMuteKind {
    type Value = AoMute;
    fn as_str() -> &'static str {
        "ao-mute"
    }
}
impl SetPropertyValue for AoMute {
    type Kind = AoMuteKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AudioParams(pub fields::AudioParams);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AudioParamsKind;

impl PropertyValueName for AudioParamsKind {
    type Value = AudioParams;
    fn as_str() -> &'static str {
        "audio-params"
    }
}
impl SetPropertyValue for AudioParams {
    type Kind = AudioParamsKind;
    type Primitive = fields::AudioParams;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AudioOutParams(pub fields::AudioParams);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AudioOutParamsKind;

impl PropertyValueName for AudioOutParamsKind {
    type Value = AudioOutParams;
    fn as_str() -> &'static str {
        "audio-out-params"
    }
}
impl SetPropertyValue for AudioOutParams {
    type Kind = AudioOutParamsKind;
    type Primitive = fields::AudioParams;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Colormatrix(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ColormatrixKind;

impl PropertyValueName for ColormatrixKind {
    type Value = Colormatrix;
    fn as_str() -> &'static str {
        "colormatrix"
    }
}
impl SetPropertyValue for Colormatrix {
    type Kind = ColormatrixKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ColormatrixInputRange(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ColormatrixInputRangeKind;

impl PropertyValueName for ColormatrixInputRangeKind {
    type Value = ColormatrixInputRange;
    fn as_str() -> &'static str {
        "colormatrix-input-range"
    }
}
impl SetPropertyValue for ColormatrixInputRange {
    type Kind = ColormatrixInputRangeKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ColormatrixPrimaries(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ColormatrixPrimariesKind;

impl PropertyValueName for ColormatrixPrimariesKind {
    type Value = ColormatrixPrimaries;
    fn as_str() -> &'static str {
        "colormatrix-primaries"
    }
}
impl SetPropertyValue for ColormatrixPrimaries {
    type Kind = ColormatrixPrimariesKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Hwdec(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HwdecKind;

impl PropertyValueName for HwdecKind {
    type Value = Hwdec;
    fn as_str() -> &'static str {
        "hwdec"
    }
}
impl SetPropertyValue for Hwdec {
    type Kind = HwdecKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct HwdecCurrent(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HwdecCurrentKind;

impl PropertyValueName for HwdecCurrentKind {
    type Value = HwdecCurrent;
    fn as_str() -> &'static str {
        "hwdec-current"
    }
}
impl SetPropertyValue for HwdecCurrent {
    type Kind = HwdecCurrentKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct HwdecInterop(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HwdecInteropKind;

impl PropertyValueName for HwdecInteropKind {
    type Value = HwdecInterop;
    fn as_str() -> &'static str {
        "hwdec-interop"
    }
}
impl SetPropertyValue for HwdecInterop {
    type Kind = HwdecInteropKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Width(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WidthKind;

impl PropertyValueName for WidthKind {
    type Value = Width;
    fn as_str() -> &'static str {
        "width"
    }
}
impl SetPropertyValue for Width {
    type Kind = WidthKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Height(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HeightKind;

impl PropertyValueName for HeightKind {
    type Value = Height;
    fn as_str() -> &'static str {
        "height"
    }
}
impl SetPropertyValue for Height {
    type Kind = HeightKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VideoParams(pub fields::VideoParams);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VideoParamsKind;

impl PropertyValueName for VideoParamsKind {
    type Value = VideoParams;
    fn as_str() -> &'static str {
        "video-params"
    }
}
impl SetPropertyValue for VideoParams {
    type Kind = VideoParamsKind;
    type Primitive = fields::VideoParams;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VideoDecParams(pub fields::VideoParams);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VideoDecParamsKind;

impl PropertyValueName for VideoDecParamsKind {
    type Value = VideoDecParams;
    fn as_str() -> &'static str {
        "video-dec-params"
    }
}
impl SetPropertyValue for VideoDecParams {
    type Kind = VideoDecParamsKind;
    type Primitive = fields::VideoParams;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VideoOutParams(pub fields::VideoParams);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VideoOutParamsKind;

impl PropertyValueName for VideoOutParamsKind {
    type Value = VideoOutParams;
    fn as_str() -> &'static str {
        "video-out-params"
    }
}
impl SetPropertyValue for VideoOutParams {
    type Kind = VideoOutParamsKind;
    type Primitive = fields::VideoParams;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VideoTargetParams(pub fields::VideoParams);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VideoTargetParamsKind;

impl PropertyValueName for VideoTargetParamsKind {
    type Value = VideoTargetParams;
    fn as_str() -> &'static str {
        "video-target-params"
    }
}
impl SetPropertyValue for VideoTargetParams {
    type Kind = VideoTargetParamsKind;
    type Primitive = fields::VideoParams;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VideoFrameInfo(pub fields::VideoFrameInfo);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VideoFrameInfoKind;

impl PropertyValueName for VideoFrameInfoKind {
    type Value = VideoFrameInfo;
    fn as_str() -> &'static str {
        "video-frame-info"
    }
}
impl SetPropertyValue for VideoFrameInfo {
    type Kind = VideoFrameInfoKind;
    type Primitive = fields::VideoFrameInfo;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ContainerFps(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ContainerFpsKind;

impl PropertyValueName for ContainerFpsKind {
    type Value = ContainerFps;
    fn as_str() -> &'static str {
        "container-fps"
    }
}
impl SetPropertyValue for ContainerFps {
    type Kind = ContainerFpsKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EstimatedVfFps(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EstimatedVfFpsKind;

impl PropertyValueName for EstimatedVfFpsKind {
    type Value = EstimatedVfFps;
    fn as_str() -> &'static str {
        "estimated-vf-fps"
    }
}
impl SetPropertyValue for EstimatedVfFps {
    type Kind = EstimatedVfFpsKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CurrentWindowScale(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrentWindowScaleKind;

impl PropertyValueName for CurrentWindowScaleKind {
    type Value = CurrentWindowScale;
    fn as_str() -> &'static str {
        "current-window-scale"
    }
}
impl SetPropertyValue for CurrentWindowScale {
    type Kind = CurrentWindowScaleKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Focused(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FocusedKind;

impl PropertyValueName for FocusedKind {
    type Value = Focused;
    fn as_str() -> &'static str {
        "focused"
    }
}
impl SetPropertyValue for Focused {
    type Kind = FocusedKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AmbientLight(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AmbientLightKind;

impl PropertyValueName for AmbientLightKind {
    type Value = AmbientLight;
    fn as_str() -> &'static str {
        "ambient-light"
    }
}
impl SetPropertyValue for AmbientLight {
    type Kind = AmbientLightKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DisplayNames(pub Vec<String>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DisplayNamesKind;

impl PropertyValueName for DisplayNamesKind {
    type Value = DisplayNames;
    fn as_str() -> &'static str {
        "display-names"
    }
}
impl SetPropertyValue for DisplayNames {
    type Kind = DisplayNamesKind;
    type Primitive = Vec<String>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DisplayFps(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DisplayFpsKind;

impl PropertyValueName for DisplayFpsKind {
    type Value = DisplayFps;
    fn as_str() -> &'static str {
        "display-fps"
    }
}
impl SetPropertyValue for DisplayFps {
    type Kind = DisplayFpsKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EstimatedDisplayFps(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EstimatedDisplayFpsKind;

impl PropertyValueName for EstimatedDisplayFpsKind {
    type Value = EstimatedDisplayFps;
    fn as_str() -> &'static str {
        "estimated-display-fps"
    }
}
impl SetPropertyValue for EstimatedDisplayFps {
    type Kind = EstimatedDisplayFpsKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VsyncJitter(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VsyncJitterKind;

impl PropertyValueName for VsyncJitterKind {
    type Value = VsyncJitter;
    fn as_str() -> &'static str {
        "vsync-jitter"
    }
}
impl SetPropertyValue for VsyncJitter {
    type Kind = VsyncJitterKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DisplayWidth(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DisplayWidthKind;

impl PropertyValueName for DisplayWidthKind {
    type Value = DisplayWidth;
    fn as_str() -> &'static str {
        "display-width"
    }
}
impl SetPropertyValue for DisplayWidth {
    type Kind = DisplayWidthKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DisplayHeight(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DisplayHeightKind;

impl PropertyValueName for DisplayHeightKind {
    type Value = DisplayHeight;
    fn as_str() -> &'static str {
        "display-height"
    }
}
impl SetPropertyValue for DisplayHeight {
    type Kind = DisplayHeightKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DisplayHidpiScale(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DisplayHidpiScaleKind;

impl PropertyValueName for DisplayHidpiScaleKind {
    type Value = DisplayHidpiScale;
    fn as_str() -> &'static str {
        "display-hidpi-scale"
    }
}
impl SetPropertyValue for DisplayHidpiScale {
    type Kind = DisplayHidpiScaleKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct OsdWidth(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct OsdWidthKind;

impl PropertyValueName for OsdWidthKind {
    type Value = OsdWidth;
    fn as_str() -> &'static str {
        "osd-width"
    }
}
impl SetPropertyValue for OsdWidth {
    type Kind = OsdWidthKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct OsdHeight(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct OsdHeightKind;

impl PropertyValueName for OsdHeightKind {
    type Value = OsdHeight;
    fn as_str() -> &'static str {
        "osd-height"
    }
}
impl SetPropertyValue for OsdHeight {
    type Kind = OsdHeightKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct OsdPar(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct OsdParKind;

impl PropertyValueName for OsdParKind {
    type Value = OsdPar;
    fn as_str() -> &'static str {
        "osd-par"
    }
}
impl SetPropertyValue for OsdPar {
    type Kind = OsdParKind;
    type Primitive = f64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct OsdDimensions(pub fields::OsdDimensions);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct OsdDimensionsKind;

impl PropertyValueName for OsdDimensionsKind {
    type Value = OsdDimensions;
    fn as_str() -> &'static str {
        "osd-dimensions"
    }
}
impl SetPropertyValue for OsdDimensions {
    type Kind = OsdDimensionsKind;
    type Primitive = fields::OsdDimensions;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TermSize(pub Vec<i64>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TermSizeKind;

impl PropertyValueName for TermSizeKind {
    type Value = TermSize;
    fn as_str() -> &'static str {
        "term-size"
    }
}
impl SetPropertyValue for TermSize {
    type Kind = TermSizeKind;
    type Primitive = Vec<i64>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct WindowId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WindowIdKind;

impl PropertyValueName for WindowIdKind {
    type Value = WindowId;
    fn as_str() -> &'static str {
        "window-id"
    }
}
impl SetPropertyValue for WindowId {
    type Kind = WindowIdKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MousePos(pub fields::MousePos);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MousePosKind;

impl PropertyValueName for MousePosKind {
    type Value = MousePos;
    fn as_str() -> &'static str {
        "mouse-pos"
    }
}
impl SetPropertyValue for MousePos {
    type Kind = MousePosKind;
    type Primitive = fields::MousePos;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TouchPos(pub fields::TouchPos);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TouchPosKind;

impl PropertyValueName for TouchPosKind {
    type Value = TouchPos;
    fn as_str() -> &'static str {
        "touch-pos"
    }
}
impl SetPropertyValue for TouchPos {
    type Kind = TouchPosKind;
    type Primitive = fields::TouchPos;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SubAssExtradata(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SubAssExtradataKind;

impl PropertyValueName for SubAssExtradataKind {
    type Value = SubAssExtradata;
    fn as_str() -> &'static str {
        "sub-ass-extradata"
    }
}
impl SetPropertyValue for SubAssExtradata {
    type Kind = SubAssExtradataKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SubText(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SubTextKind;

impl PropertyValueName for SubTextKind {
    type Value = SubText;
    fn as_str() -> &'static str {
        "sub-text"
    }
}
impl SetPropertyValue for SubText {
    type Kind = SubTextKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SubTextAss(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SubTextAssKind;

impl PropertyValueName for SubTextAssKind {
    type Value = SubTextAss;
    fn as_str() -> &'static str {
        "sub-text/ass"
    }
}
impl SetPropertyValue for SubTextAss {
    type Kind = SubTextAssKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SubTextAssFull(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SubTextAssFullKind;

impl PropertyValueName for SubTextAssFullKind {
    type Value = SubTextAssFull;
    fn as_str() -> &'static str {
        "sub-text/ass-full"
    }
}
impl SetPropertyValue for SubTextAssFull {
    type Kind = SubTextAssFullKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SecondarySubText(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SecondarySubTextKind;

impl PropertyValueName for SecondarySubTextKind {
    type Value = SecondarySubText;
    fn as_str() -> &'static str {
        "secondary-sub-text"
    }
}
impl SetPropertyValue for SecondarySubText {
    type Kind = SecondarySubTextKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SubStart(pub Option<f64>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SubStartKind;

impl PropertyValueName for SubStartKind {
    type Value = SubStart;
    fn as_str() -> &'static str {
        "sub-start"
    }
}
impl SetPropertyValue for SubStart {
    type Kind = SubStartKind;
    type Primitive = Option<f64>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SubStartFull(pub Option<f64>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SubStartFullKind;

impl PropertyValueName for SubStartFullKind {
    type Value = SubStartFull;
    fn as_str() -> &'static str {
        "sub-start/full"
    }
}
impl SetPropertyValue for SubStartFull {
    type Kind = SubStartFullKind;
    type Primitive = Option<f64>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SecondarySubStart(pub Option<f64>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SecondarySubStartKind;

impl PropertyValueName for SecondarySubStartKind {
    type Value = SecondarySubStart;
    fn as_str() -> &'static str {
        "secondary-sub-start"
    }
}
impl SetPropertyValue for SecondarySubStart {
    type Kind = SecondarySubStartKind;
    type Primitive = Option<f64>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SubEnd(pub Option<f64>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SubEndKind;

impl PropertyValueName for SubEndKind {
    type Value = SubEnd;
    fn as_str() -> &'static str {
        "sub-end"
    }
}
impl SetPropertyValue for SubEnd {
    type Kind = SubEndKind;
    type Primitive = Option<f64>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SubEndFull(pub Option<f64>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SubEndFullKind;

impl PropertyValueName for SubEndFullKind {
    type Value = SubEndFull;
    fn as_str() -> &'static str {
        "sub-end/full"
    }
}
impl SetPropertyValue for SubEndFull {
    type Kind = SubEndFullKind;
    type Primitive = Option<f64>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SecondarySubEnd(pub Option<f64>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SecondarySubEndKind;

impl PropertyValueName for SecondarySubEndKind {
    type Value = SecondarySubEnd;
    fn as_str() -> &'static str {
        "secondary-sub-end"
    }
}
impl SetPropertyValue for SecondarySubEnd {
    type Kind = SecondarySubEndKind;
    type Primitive = Option<f64>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlaylistPos(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaylistPosKind;

impl PropertyValueName for PlaylistPosKind {
    type Value = PlaylistPos;
    fn as_str() -> &'static str {
        "playlist-pos"
    }
}
impl SetPropertyValue for PlaylistPos {
    type Kind = PlaylistPosKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlaylistPos1(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaylistPos1Kind;

impl PropertyValueName for PlaylistPos1Kind {
    type Value = PlaylistPos1;
    fn as_str() -> &'static str {
        "playlist-pos-1"
    }
}
impl SetPropertyValue for PlaylistPos1 {
    type Kind = PlaylistPos1Kind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlaylistCurrentPos(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaylistCurrentPosKind;

impl PropertyValueName for PlaylistCurrentPosKind {
    type Value = PlaylistCurrentPos;
    fn as_str() -> &'static str {
        "playlist-current-pos"
    }
}
impl SetPropertyValue for PlaylistCurrentPos {
    type Kind = PlaylistCurrentPosKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlaylistPlayingPos(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaylistPlayingPosKind;

impl PropertyValueName for PlaylistPlayingPosKind {
    type Value = PlaylistPlayingPos;
    fn as_str() -> &'static str {
        "playlist-playing-pos"
    }
}
impl SetPropertyValue for PlaylistPlayingPos {
    type Kind = PlaylistPlayingPosKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlaylistCount(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaylistCountKind;

impl PropertyValueName for PlaylistCountKind {
    type Value = PlaylistCount;
    fn as_str() -> &'static str {
        "playlist-count"
    }
}
impl SetPropertyValue for PlaylistCount {
    type Kind = PlaylistCountKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlaylistPath(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaylistPathKind;

impl PropertyValueName for PlaylistPathKind {
    type Value = PlaylistPath;
    fn as_str() -> &'static str {
        "playlist-path"
    }
}
impl SetPropertyValue for PlaylistPath {
    type Kind = PlaylistPathKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Playlist(pub fields::Playlist);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaylistKind;

impl PropertyValueName for PlaylistKind {
    type Value = Playlist;
    fn as_str() -> &'static str {
        "playlist"
    }
}
impl SetPropertyValue for Playlist {
    type Kind = PlaylistKind;
    type Primitive = fields::Playlist;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TrackList(pub fields::TrackList);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TrackListKind;

impl PropertyValueName for TrackListKind {
    type Value = TrackList;
    fn as_str() -> &'static str {
        "track-list"
    }
}
impl SetPropertyValue for TrackList {
    type Kind = TrackListKind;
    type Primitive = fields::TrackList;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CurrentTracksVideo(pub fields::Track);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrentTracksVideoKind;

impl PropertyValueName for CurrentTracksVideoKind {
    type Value = CurrentTracksVideo;
    fn as_str() -> &'static str {
        "current-tracks/video"
    }
}
impl SetPropertyValue for CurrentTracksVideo {
    type Kind = CurrentTracksVideoKind;
    type Primitive = fields::Track;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CurrentTracksAudio(pub fields::Track);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrentTracksAudioKind;

impl PropertyValueName for CurrentTracksAudioKind {
    type Value = CurrentTracksAudio;
    fn as_str() -> &'static str {
        "current-tracks/audio"
    }
}
impl SetPropertyValue for CurrentTracksAudio {
    type Kind = CurrentTracksAudioKind;
    type Primitive = fields::Track;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CurrentTracksSub(pub fields::Track);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrentTracksSubKind;

impl PropertyValueName for CurrentTracksSubKind {
    type Value = CurrentTracksSub;
    fn as_str() -> &'static str {
        "current-tracks/sub"
    }
}
impl SetPropertyValue for CurrentTracksSub {
    type Kind = CurrentTracksSubKind;
    type Primitive = fields::Track;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CurrentTracksSub2(pub fields::Track);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrentTracksSub2Kind;

impl PropertyValueName for CurrentTracksSub2Kind {
    type Value = CurrentTracksSub2;
    fn as_str() -> &'static str {
        "current-tracks/sub2"
    }
}
impl SetPropertyValue for CurrentTracksSub2 {
    type Kind = CurrentTracksSub2Kind;
    type Primitive = fields::Track;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ChapterList(pub fields::ChapterList);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ChapterListKind;

impl PropertyValueName for ChapterListKind {
    type Value = ChapterList;
    fn as_str() -> &'static str {
        "chapter-list"
    }
}
impl SetPropertyValue for ChapterList {
    type Kind = ChapterListKind;
    type Primitive = fields::ChapterList;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Af(pub Vec<fields::Filter>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AfKind;

impl PropertyValueName for AfKind {
    type Value = Af;
    fn as_str() -> &'static str {
        "af"
    }
}
impl SetPropertyValue for Af {
    type Kind = AfKind;
    type Primitive = Vec<fields::Filter>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Vf(pub Vec<fields::Filter>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VfKind;

impl PropertyValueName for VfKind {
    type Value = Vf;
    fn as_str() -> &'static str {
        "vf"
    }
}
impl SetPropertyValue for Vf {
    type Kind = VfKind;
    type Primitive = Vec<fields::Filter>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Seekable(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SeekableKind;

impl PropertyValueName for SeekableKind {
    type Value = Seekable;
    fn as_str() -> &'static str {
        "seekable"
    }
}
impl SetPropertyValue for Seekable {
    type Kind = SeekableKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PartiallySeekable(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PartiallySeekableKind;

impl PropertyValueName for PartiallySeekableKind {
    type Value = PartiallySeekable;
    fn as_str() -> &'static str {
        "partially-seekable"
    }
}
impl SetPropertyValue for PartiallySeekable {
    type Kind = PartiallySeekableKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlaybackAbort(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaybackAbortKind;

impl PropertyValueName for PlaybackAbortKind {
    type Value = PlaybackAbort;
    fn as_str() -> &'static str {
        "playback-abort"
    }
}
impl SetPropertyValue for PlaybackAbort {
    type Kind = PlaybackAbortKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CursorAutohide(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CursorAutohideKind;

impl PropertyValueName for CursorAutohideKind {
    type Value = CursorAutohide;
    fn as_str() -> &'static str {
        "cursor-autohide"
    }
}
impl SetPropertyValue for CursorAutohide {
    type Kind = CursorAutohideKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TermClipCc(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TermClipCcKind;

impl PropertyValueName for TermClipCcKind {
    type Value = TermClipCc;
    fn as_str() -> &'static str {
        "term-clip-cc"
    }
}
impl SetPropertyValue for TermClipCc {
    type Kind = TermClipCcKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct OsdSymCc(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct OsdSymCcKind;

impl PropertyValueName for OsdSymCcKind {
    type Value = OsdSymCc;
    fn as_str() -> &'static str {
        "osd-sym-cc"
    }
}
impl SetPropertyValue for OsdSymCc {
    type Kind = OsdSymCcKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct OsdAssCc(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct OsdAssCcKind;

impl PropertyValueName for OsdAssCcKind {
    type Value = OsdAssCc;
    fn as_str() -> &'static str {
        "osd-ass-cc"
    }
}
impl SetPropertyValue for OsdAssCc {
    type Kind = OsdAssCcKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VoConfigured(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VoConfiguredKind;

impl PropertyValueName for VoConfiguredKind {
    type Value = VoConfigured;
    fn as_str() -> &'static str {
        "vo-configured"
    }
}
impl SetPropertyValue for VoConfigured {
    type Kind = VoConfiguredKind;
    type Primitive = bool;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VoPasses(pub fields::VoPasses);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VoPassesKind;

impl PropertyValueName for VoPassesKind {
    type Value = VoPasses;
    fn as_str() -> &'static str {
        "vo-passes"
    }
}
impl SetPropertyValue for VoPasses {
    type Kind = VoPassesKind;
    type Primitive = fields::VoPasses;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PerfInfo(pub std::collections::HashMap<String, String>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PerfInfoKind;

impl PropertyValueName for PerfInfoKind {
    type Value = PerfInfo;
    fn as_str() -> &'static str {
        "perf-info"
    }
}
impl SetPropertyValue for PerfInfo {
    type Kind = PerfInfoKind;
    type Primitive = std::collections::HashMap<String, String>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VideoBitrate(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VideoBitrateKind;

impl PropertyValueName for VideoBitrateKind {
    type Value = VideoBitrate;
    fn as_str() -> &'static str {
        "video-bitrate"
    }
}
impl SetPropertyValue for VideoBitrate {
    type Kind = VideoBitrateKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AudioBitrate(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AudioBitrateKind;

impl PropertyValueName for AudioBitrateKind {
    type Value = AudioBitrate;
    fn as_str() -> &'static str {
        "audio-bitrate"
    }
}
impl SetPropertyValue for AudioBitrate {
    type Kind = AudioBitrateKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SubBitrate(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SubBitrateKind;

impl PropertyValueName for SubBitrateKind {
    type Value = SubBitrate;
    fn as_str() -> &'static str {
        "sub-bitrate"
    }
}
impl SetPropertyValue for SubBitrate {
    type Kind = SubBitrateKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AudioDeviceList(pub Vec<fields::AudioDevice>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AudioDeviceListKind;

impl PropertyValueName for AudioDeviceListKind {
    type Value = AudioDeviceList;
    fn as_str() -> &'static str {
        "audio-device-list"
    }
}
impl SetPropertyValue for AudioDeviceList {
    type Kind = AudioDeviceListKind;
    type Primitive = Vec<fields::AudioDevice>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AudioDevice(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AudioDeviceKind;

impl PropertyValueName for AudioDeviceKind {
    type Value = AudioDevice;
    fn as_str() -> &'static str {
        "audio-device"
    }
}
impl SetPropertyValue for AudioDevice {
    type Kind = AudioDeviceKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CurrentVo(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrentVoKind;

impl PropertyValueName for CurrentVoKind {
    type Value = CurrentVo;
    fn as_str() -> &'static str {
        "current-vo"
    }
}
impl SetPropertyValue for CurrentVo {
    type Kind = CurrentVoKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CurrentGpuContext(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrentGpuContextKind;

impl PropertyValueName for CurrentGpuContextKind {
    type Value = CurrentGpuContext;
    fn as_str() -> &'static str {
        "current-gpu-context"
    }
}
impl SetPropertyValue for CurrentGpuContext {
    type Kind = CurrentGpuContextKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CurrentAo(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrentAoKind;

impl PropertyValueName for CurrentAoKind {
    type Value = CurrentAo;
    fn as_str() -> &'static str {
        "current-ao"
    }
}
impl SetPropertyValue for CurrentAo {
    type Kind = CurrentAoKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UserData(pub std::collections::HashMap<String, String>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserDataKind;

impl PropertyValueName for UserDataKind {
    type Value = UserData;
    fn as_str() -> &'static str {
        "user-data"
    }
}
impl SetPropertyValue for UserData {
    type Kind = UserDataKind;
    type Primitive = std::collections::HashMap<String, String>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MenuData(pub Vec<fields::MenuItem>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MenuDataKind;

impl PropertyValueName for MenuDataKind {
    type Value = MenuData;
    fn as_str() -> &'static str {
        "menu-data"
    }
}
impl SetPropertyValue for MenuData {
    type Kind = MenuDataKind;
    type Primitive = Vec<fields::MenuItem>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct WorkingDirectory(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WorkingDirectoryKind;

impl PropertyValueName for WorkingDirectoryKind {
    type Value = WorkingDirectory;
    fn as_str() -> &'static str {
        "working-directory"
    }
}
impl SetPropertyValue for WorkingDirectory {
    type Kind = WorkingDirectoryKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CurrentWatchLaterDir(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrentWatchLaterDirKind;

impl PropertyValueName for CurrentWatchLaterDirKind {
    type Value = CurrentWatchLaterDir;
    fn as_str() -> &'static str {
        "current-watch-later-dir"
    }
}
impl SetPropertyValue for CurrentWatchLaterDir {
    type Kind = CurrentWatchLaterDirKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ProtocolList(pub Vec<String>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ProtocolListKind;

impl PropertyValueName for ProtocolListKind {
    type Value = ProtocolList;
    fn as_str() -> &'static str {
        "protocol-list"
    }
}
impl SetPropertyValue for ProtocolList {
    type Kind = ProtocolListKind;
    type Primitive = Vec<String>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DecoderList(pub Vec<fields::Decoder>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DecoderListKind;

impl PropertyValueName for DecoderListKind {
    type Value = DecoderList;
    fn as_str() -> &'static str {
        "decoder-list"
    }
}
impl SetPropertyValue for DecoderList {
    type Kind = DecoderListKind;
    type Primitive = Vec<fields::Decoder>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EncoderList(pub Vec<fields::Decoder>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EncoderListKind;

impl PropertyValueName for EncoderListKind {
    type Value = EncoderList;
    fn as_str() -> &'static str {
        "encoder-list"
    }
}
impl SetPropertyValue for EncoderList {
    type Kind = EncoderListKind;
    type Primitive = Vec<fields::Decoder>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DemuxerLavfList(pub Vec<String>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DemuxerLavfListKind;

impl PropertyValueName for DemuxerLavfListKind {
    type Value = DemuxerLavfList;
    fn as_str() -> &'static str {
        "demuxer-lavf-list"
    }
}
impl SetPropertyValue for DemuxerLavfList {
    type Kind = DemuxerLavfListKind;
    type Primitive = Vec<String>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct InputKeyList(pub Vec<String>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InputKeyListKind;

impl PropertyValueName for InputKeyListKind {
    type Value = InputKeyList;
    fn as_str() -> &'static str {
        "input-key-list"
    }
}
impl SetPropertyValue for InputKeyList {
    type Kind = InputKeyListKind;
    type Primitive = Vec<String>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MpvVersion(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MpvVersionKind;

impl PropertyValueName for MpvVersionKind {
    type Value = MpvVersion;
    fn as_str() -> &'static str {
        "mpv-version"
    }
}
impl SetPropertyValue for MpvVersion {
    type Kind = MpvVersionKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MpvConfiguration(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MpvConfigurationKind;

impl PropertyValueName for MpvConfigurationKind {
    type Value = MpvConfiguration;
    fn as_str() -> &'static str {
        "mpv-configuration"
    }
}
impl SetPropertyValue for MpvConfiguration {
    type Kind = MpvConfigurationKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FfmpegVersion(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FfmpegVersionKind;

impl PropertyValueName for FfmpegVersionKind {
    type Value = FfmpegVersion;
    fn as_str() -> &'static str {
        "ffmpeg-version"
    }
}
impl SetPropertyValue for FfmpegVersion {
    type Kind = FfmpegVersionKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LibassVersion(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LibassVersionKind;

impl PropertyValueName for LibassVersionKind {
    type Value = LibassVersion;
    fn as_str() -> &'static str {
        "libass-version"
    }
}
impl SetPropertyValue for LibassVersion {
    type Kind = LibassVersionKind;
    type Primitive = i64;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Platform(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlatformKind;

impl PropertyValueName for PlatformKind {
    type Value = Platform;
    fn as_str() -> &'static str {
        "platform"
    }
}
impl SetPropertyValue for Platform {
    type Kind = PlatformKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Options(pub std::collections::HashMap<String, String>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct OptionsKind;

impl PropertyValueName for OptionsKind {
    type Value = Options;
    fn as_str() -> &'static str {
        "options"
    }
}
impl SetPropertyValue for Options {
    type Kind = OptionsKind;
    type Primitive = std::collections::HashMap<String, String>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FileLocalOptions(pub std::collections::HashMap<String, String>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FileLocalOptionsKind;

impl PropertyValueName for FileLocalOptionsKind {
    type Value = FileLocalOptions;
    fn as_str() -> &'static str {
        "file-local-options"
    }
}
impl SetPropertyValue for FileLocalOptions {
    type Kind = FileLocalOptionsKind;
    type Primitive = std::collections::HashMap<String, String>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct OptionInfo(pub fields::OptionInfo);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct OptionInfoKind;

impl PropertyValueName for OptionInfoKind {
    type Value = OptionInfo;
    fn as_str() -> &'static str {
        "option-info"
    }
}
impl SetPropertyValue for OptionInfo {
    type Kind = OptionInfoKind;
    type Primitive = fields::OptionInfo;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PropertyList(pub Vec<String>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PropertyListKind;

impl PropertyValueName for PropertyListKind {
    type Value = PropertyList;
    fn as_str() -> &'static str {
        "property-list"
    }
}
impl SetPropertyValue for PropertyList {
    type Kind = PropertyListKind;
    type Primitive = Vec<String>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ProfileList(pub Vec<HashMap<String, String>>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ProfileListKind;

impl PropertyValueName for ProfileListKind {
    type Value = ProfileList;
    fn as_str() -> &'static str {
        "profile-list"
    }
}
impl SetPropertyValue for ProfileList {
    type Kind = ProfileListKind;
    type Primitive = Vec<HashMap<String, String>>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CommandList(pub Vec<HashMap<String, String>>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CommandListKind;

impl PropertyValueName for CommandListKind {
    type Value = CommandList;
    fn as_str() -> &'static str {
        "command-list"
    }
}
impl SetPropertyValue for CommandList {
    type Kind = CommandListKind;
    type Primitive = Vec<HashMap<String, String>>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct InputBindings(pub Vec<fields::InputBinding>);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InputBindingsKind;

impl PropertyValueName for InputBindingsKind {
    type Value = InputBindings;
    fn as_str() -> &'static str {
        "input-bindings"
    }
}
impl SetPropertyValue for InputBindings {
    type Kind = InputBindingsKind;
    type Primitive = Vec<fields::InputBinding>;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ClipboardText(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ClipboardTextKind;

impl PropertyValueName for ClipboardTextKind {
    type Value = ClipboardText;
    fn as_str() -> &'static str {
        "clipboard/text"
    }
}
impl SetPropertyValue for ClipboardText {
    type Kind = ClipboardTextKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ClipboardTextPrimary(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ClipboardTextPrimaryKind;

impl PropertyValueName for ClipboardTextPrimaryKind {
    type Value = ClipboardTextPrimary;
    fn as_str() -> &'static str {
        "clipboard/text-primary"
    }
}
impl SetPropertyValue for ClipboardTextPrimary {
    type Kind = ClipboardTextPrimaryKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CurrentClipboardBackend(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CurrentClipboardBackendKind;

impl PropertyValueName for CurrentClipboardBackendKind {
    type Value = CurrentClipboardBackend;
    fn as_str() -> &'static str {
        "current-clipboard-backend"
    }
}
impl SetPropertyValue for CurrentClipboardBackend {
    type Kind = CurrentClipboardBackendKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Clock(pub String);

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ClockKind;

impl PropertyValueName for ClockKind {
    type Value = Clock;
    fn as_str() -> &'static str {
        "clock"
    }
}
impl SetPropertyValue for Clock {
    type Kind = ClockKind;
    type Primitive = String;
    fn inner(&self) -> &Self::Primitive {
        &self.0
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum PropertyValueKind {
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "audio-speed-correction")]
    AudioSpeedCorrection,
    #[serde(rename = "video-speed-correction")]
    VideoSpeedCorrection,
    #[serde(rename = "display-sync-active")]
    DisplaySyncActive,
    #[serde(rename = "filename")]
    Filename,
    #[serde(rename = "filename/no-ext")]
    FilenameNoExt,
    #[serde(rename = "file-size")]
    FileSize,
    #[serde(rename = "estimated-frame-count")]
    EstimatedFrameCount,
    #[serde(rename = "estimated-frame-number")]
    EstimatedFrameNumber,
    #[serde(rename = "pid")]
    Pid,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "stream-open-filename")]
    StreamOpenFilename,
    #[serde(rename = "media-title")]
    MediaTitle,
    #[serde(rename = "file-format")]
    FileFormat,
    #[serde(rename = "current-demuxer")]
    CurrentDemuxer,
    #[serde(rename = "stream-path")]
    StreamPath,
    #[serde(rename = "stream-pos")]
    StreamPos,
    #[serde(rename = "stream-end")]
    StreamEnd,
    #[serde(rename = "duration")]
    Duration,
    #[serde(rename = "duration/full")]
    DurationFull,
    #[serde(rename = "avsync")]
    Avsync,
    #[serde(rename = "total-avsync-change")]
    TotalAvsyncChange,
    #[serde(rename = "decoder-frame-drop-count")]
    DecoderFrameDropCount,
    #[serde(rename = "frame-drop-count")]
    FrameDropCount,
    #[serde(rename = "mistimed-frame-count")]
    MistimedFrameCount,
    #[serde(rename = "vsync-ratio")]
    VsyncRatio,
    #[serde(rename = "vo-delayed-frame-count")]
    VoDelayedFrameCount,
    #[serde(rename = "percent-pos")]
    PercentPos,
    #[serde(rename = "time-pos")]
    TimePos,
    #[serde(rename = "time-pos/full")]
    TimePosFull,
    #[serde(rename = "time-start")]
    TimeStart,
    #[serde(rename = "time-remaining")]
    TimeRemaining,
    #[serde(rename = "time-remaining/full")]
    TimeRemainingFull,
    #[serde(rename = "audio-pts")]
    AudioPts,
    #[serde(rename = "audio-pts/full")]
    AudioPtsFull,
    #[serde(rename = "playtime-remaining")]
    PlaytimeRemaining,
    #[serde(rename = "playtime-remaining/full")]
    PlaytimeRemainingFull,
    #[serde(rename = "playback-time")]
    PlaybackTime,
    #[serde(rename = "playback-time/full")]
    PlaybackTimeFull,
    #[serde(rename = "remaining-file-loops")]
    RemainingFileLoops,
    #[serde(rename = "remaining-ab-loops")]
    RemainingAbLoops,
    #[serde(rename = "chapter")]
    Chapter,
    #[serde(rename = "edition")]
    Edition,
    #[serde(rename = "current-edition")]
    CurrentEdition,
    #[serde(rename = "chapters")]
    Chapters,
    #[serde(rename = "editions")]
    Editions,
    #[serde(rename = "edition-list")]
    EditionList,
    #[serde(rename = "metadata")]
    Metadata,
    #[serde(rename = "filtered-metadata")]
    FilteredMetadata,
    #[serde(rename = "chapter-metadata")]
    ChapterMetadata,
    #[serde(rename = "vf-metadata")]
    VfMetadata,
    #[serde(rename = "af-metadata")]
    AfMetadata,
    #[serde(rename = "deinterlace-active")]
    DeinterlaceActive,
    #[serde(rename = "idle-active")]
    IdleActive,
    #[serde(rename = "core-idle")]
    CoreIdle,
    #[serde(rename = "cache-speed")]
    CacheSpeed,
    #[serde(rename = "demuxer-cache-duration")]
    DemuxerCacheDuration,
    #[serde(rename = "demuxer-cache-time")]
    DemuxerCacheTime,
    #[serde(rename = "demuxer-cache-idle")]
    DemuxerCacheIdle,
    #[serde(rename = "demuxer-cache-state")]
    DemuxerCacheState,
    #[serde(rename = "demuxer-via-network")]
    DemuxerViaNetwork,
    #[serde(rename = "demuxer-start-time")]
    DemuxerStartTime,
    #[serde(rename = "paused-for-cache")]
    PausedForCache,
    #[serde(rename = "cache-buffering-state")]
    CacheBufferingState,
    #[serde(rename = "eof-reached")]
    EofReached,
    #[serde(rename = "seeking")]
    Seeking,
    #[serde(rename = "mixer-active")]
    MixerActive,
    #[serde(rename = "ao-volume")]
    AoVolume,
    #[serde(rename = "ao-mute")]
    AoMute,
    #[serde(rename = "audio-params")]
    AudioParams,
    #[serde(rename = "audio-out-params")]
    AudioOutParams,
    #[serde(rename = "colormatrix")]
    Colormatrix,
    #[serde(rename = "colormatrix-input-range")]
    ColormatrixInputRange,
    #[serde(rename = "colormatrix-primaries")]
    ColormatrixPrimaries,
    #[serde(rename = "hwdec")]
    Hwdec,
    #[serde(rename = "hwdec-current")]
    HwdecCurrent,
    #[serde(rename = "hwdec-interop")]
    HwdecInterop,
    #[serde(rename = "width")]
    Width,
    #[serde(rename = "height")]
    Height,
    #[serde(rename = "video-params")]
    VideoParams,
    #[serde(rename = "video-dec-params")]
    VideoDecParams,
    #[serde(rename = "video-out-params")]
    VideoOutParams,
    #[serde(rename = "video-target-params")]
    VideoTargetParams,
    #[serde(rename = "video-frame-info")]
    VideoFrameInfo,
    #[serde(rename = "container-fps")]
    ContainerFps,
    #[serde(rename = "estimated-vf-fps")]
    EstimatedVfFps,
    #[serde(rename = "current-window-scale")]
    CurrentWindowScale,
    #[serde(rename = "focused")]
    Focused,
    #[serde(rename = "ambient-light")]
    AmbientLight,
    #[serde(rename = "display-names")]
    DisplayNames,
    #[serde(rename = "display-fps")]
    DisplayFps,
    #[serde(rename = "estimated-display-fps")]
    EstimatedDisplayFps,
    #[serde(rename = "vsync-jitter")]
    VsyncJitter,
    #[serde(rename = "display-width")]
    DisplayWidth,
    #[serde(rename = "display-height")]
    DisplayHeight,
    #[serde(rename = "display-hidpi-scale")]
    DisplayHidpiScale,
    #[serde(rename = "osd-width")]
    OsdWidth,
    #[serde(rename = "osd-height")]
    OsdHeight,
    #[serde(rename = "osd-par")]
    OsdPar,
    #[serde(rename = "osd-dimensions")]
    OsdDimensions,
    #[serde(rename = "term-size")]
    TermSize,
    #[serde(rename = "window-id")]
    WindowId,
    #[serde(rename = "mouse-pos")]
    MousePos,
    #[serde(rename = "touch-pos")]
    TouchPos,
    #[serde(rename = "sub-ass-extradata")]
    SubAssExtradata,
    #[serde(rename = "sub-text")]
    SubText,
    #[serde(rename = "sub-text/ass")]
    SubTextAss,
    #[serde(rename = "sub-text/ass-full")]
    SubTextAssFull,
    #[serde(rename = "secondary-sub-text")]
    SecondarySubText,
    #[serde(rename = "sub-start")]
    SubStart,
    #[serde(rename = "sub-start/full")]
    SubStartFull,
    #[serde(rename = "secondary-sub-start")]
    SecondarySubStart,
    #[serde(rename = "sub-end")]
    SubEnd,
    #[serde(rename = "sub-end/full")]
    SubEndFull,
    #[serde(rename = "secondary-sub-end")]
    SecondarySubEnd,
    #[serde(rename = "playlist-pos")]
    PlaylistPos,
    #[serde(rename = "playlist-pos-1")]
    PlaylistPos1,
    #[serde(rename = "playlist-current-pos")]
    PlaylistCurrentPos,
    #[serde(rename = "playlist-playing-pos")]
    PlaylistPlayingPos,
    #[serde(rename = "playlist-count")]
    PlaylistCount,
    #[serde(rename = "playlist-path")]
    PlaylistPath,
    #[serde(rename = "playlist")]
    Playlist,
    #[serde(rename = "track-list")]
    TrackList,
    #[serde(rename = "current-tracks/video")]
    CurrentTracksVideo,
    #[serde(rename = "current-tracks/audio")]
    CurrentTracksAudio,
    #[serde(rename = "current-tracks/sub")]
    CurrentTracksSub,
    #[serde(rename = "current-tracks/sub2")]
    CurrentTracksSub2,
    #[serde(rename = "chapter-list")]
    ChapterList,
    #[serde(rename = "af")]
    Af,
    #[serde(rename = "vf")]
    Vf,
    #[serde(rename = "seekable")]
    Seekable,
    #[serde(rename = "partially-seekable")]
    PartiallySeekable,
    #[serde(rename = "playback-abort")]
    PlaybackAbort,
    #[serde(rename = "cursor-autohide")]
    CursorAutohide,
    #[serde(rename = "term-clip-cc")]
    TermClipCc,
    #[serde(rename = "osd-sym-cc")]
    OsdSymCc,
    #[serde(rename = "osd-ass-cc")]
    OsdAssCc,
    #[serde(rename = "vo-configured")]
    VoConfigured,
    #[serde(rename = "vo-passes")]
    VoPasses,
    #[serde(rename = "perf-info")]
    PerfInfo,
    #[serde(rename = "video-bitrate")]
    VideoBitrate,
    #[serde(rename = "audio-bitrate")]
    AudioBitrate,
    #[serde(rename = "sub-bitrate")]
    SubBitrate,
    #[serde(rename = "audio-device-list")]
    AudioDeviceList,
    #[serde(rename = "audio-device")]
    AudioDevice,
    #[serde(rename = "current-vo")]
    CurrentVo,
    #[serde(rename = "current-gpu-context")]
    CurrentGpuContext,
    #[serde(rename = "current-ao")]
    CurrentAo,
    #[serde(rename = "user-data")]
    UserData,
    #[serde(rename = "menu-data")]
    MenuData,
    #[serde(rename = "working-directory")]
    WorkingDirectory,
    #[serde(rename = "current-watch-later-dir")]
    CurrentWatchLaterDir,
    #[serde(rename = "protocol-list")]
    ProtocolList,
    #[serde(rename = "decoder-list")]
    DecoderList,
    #[serde(rename = "encoder-list")]
    EncoderList,
    #[serde(rename = "demuxer-lavf-list")]
    DemuxerLavfList,
    #[serde(rename = "input-key-list")]
    InputKeyList,
    #[serde(rename = "mpv-version")]
    MpvVersion,
    #[serde(rename = "mpv-configuration")]
    MpvConfiguration,
    #[serde(rename = "ffmpeg-version")]
    FfmpegVersion,
    #[serde(rename = "libass-version")]
    LibassVersion,
    #[serde(rename = "platform")]
    Platform,
    #[serde(rename = "options")]
    Options,
    #[serde(rename = "file-local-options")]
    FileLocalOptions,
    #[serde(rename = "option-info")]
    OptionInfo,
    #[serde(rename = "property-list")]
    PropertyList,
    #[serde(rename = "profile-list")]
    ProfileList,
    #[serde(rename = "command-list")]
    CommandList,
    #[serde(rename = "input-bindings")]
    InputBindings,
    #[serde(rename = "clipboard/text")]
    ClipboardText,
    #[serde(rename = "clipboard/text-primary")]
    ClipboardTextPrimary,
    #[serde(rename = "current-clipboard-backend")]
    CurrentClipboardBackend,
    #[serde(rename = "clock")]
    Clock,
}

impl PropertyValueKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Pause => "pause",
            Self::AudioSpeedCorrection => "audio-speed-correction",
            Self::VideoSpeedCorrection => "video-speed-correction",
            Self::DisplaySyncActive => "display-sync-active",
            Self::Filename => "filename",
            Self::FilenameNoExt => "filename/no-ext",
            Self::FileSize => "file-size",
            Self::EstimatedFrameCount => "estimated-frame-count",
            Self::EstimatedFrameNumber => "estimated-frame-number",
            Self::Pid => "pid",
            Self::Path => "path",
            Self::StreamOpenFilename => "stream-open-filename",
            Self::MediaTitle => "media-title",
            Self::FileFormat => "file-format",
            Self::CurrentDemuxer => "current-demuxer",
            Self::StreamPath => "stream-path",
            Self::StreamPos => "stream-pos",
            Self::StreamEnd => "stream-end",
            Self::Duration => "duration",
            Self::DurationFull => "duration/full",
            Self::Avsync => "avsync",
            Self::TotalAvsyncChange => "total-avsync-change",
            Self::DecoderFrameDropCount => "decoder-frame-drop-count",
            Self::FrameDropCount => "frame-drop-count",
            Self::MistimedFrameCount => "mistimed-frame-count",
            Self::VsyncRatio => "vsync-ratio",
            Self::VoDelayedFrameCount => "vo-delayed-frame-count",
            Self::PercentPos => "percent-pos",
            Self::TimePos => "time-pos",
            Self::TimePosFull => "time-pos/full",
            Self::TimeStart => "time-start",
            Self::TimeRemaining => "time-remaining",
            Self::TimeRemainingFull => "time-remaining/full",
            Self::AudioPts => "audio-pts",
            Self::AudioPtsFull => "audio-pts/full",
            Self::PlaytimeRemaining => "playtime-remaining",
            Self::PlaytimeRemainingFull => "playtime-remaining/full",
            Self::PlaybackTime => "playback-time",
            Self::PlaybackTimeFull => "playback-time/full",
            Self::RemainingFileLoops => "remaining-file-loops",
            Self::RemainingAbLoops => "remaining-ab-loops",
            Self::Chapter => "chapter",
            Self::Edition => "edition",
            Self::CurrentEdition => "current-edition",
            Self::Chapters => "chapters",
            Self::Editions => "editions",
            Self::EditionList => "edition-list",
            Self::Metadata => "metadata",
            Self::FilteredMetadata => "filtered-metadata",
            Self::ChapterMetadata => "chapter-metadata",
            Self::VfMetadata => "vf-metadata",
            Self::AfMetadata => "af-metadata",
            Self::DeinterlaceActive => "deinterlace-active",
            Self::IdleActive => "idle-active",
            Self::CoreIdle => "core-idle",
            Self::CacheSpeed => "cache-speed",
            Self::DemuxerCacheDuration => "demuxer-cache-duration",
            Self::DemuxerCacheTime => "demuxer-cache-time",
            Self::DemuxerCacheIdle => "demuxer-cache-idle",
            Self::DemuxerCacheState => "demuxer-cache-state",
            Self::DemuxerViaNetwork => "demuxer-via-network",
            Self::DemuxerStartTime => "demuxer-start-time",
            Self::PausedForCache => "paused-for-cache",
            Self::CacheBufferingState => "cache-buffering-state",
            Self::EofReached => "eof-reached",
            Self::Seeking => "seeking",
            Self::MixerActive => "mixer-active",
            Self::AoVolume => "ao-volume",
            Self::AoMute => "ao-mute",
            Self::AudioParams => "audio-params",
            Self::AudioOutParams => "audio-out-params",
            Self::Colormatrix => "colormatrix",
            Self::ColormatrixInputRange => "colormatrix-input-range",
            Self::ColormatrixPrimaries => "colormatrix-primaries",
            Self::Hwdec => "hwdec",
            Self::HwdecCurrent => "hwdec-current",
            Self::HwdecInterop => "hwdec-interop",
            Self::Width => "width",
            Self::Height => "height",
            Self::VideoParams => "video-params",
            Self::VideoDecParams => "video-dec-params",
            Self::VideoOutParams => "video-out-params",
            Self::VideoTargetParams => "video-target-params",
            Self::VideoFrameInfo => "video-frame-info",
            Self::ContainerFps => "container-fps",
            Self::EstimatedVfFps => "estimated-vf-fps",
            Self::CurrentWindowScale => "current-window-scale",
            Self::Focused => "focused",
            Self::AmbientLight => "ambient-light",
            Self::DisplayNames => "display-names",
            Self::DisplayFps => "display-fps",
            Self::EstimatedDisplayFps => "estimated-display-fps",
            Self::VsyncJitter => "vsync-jitter",
            Self::DisplayWidth => "display-width",
            Self::DisplayHeight => "display-height",
            Self::DisplayHidpiScale => "display-hidpi-scale",
            Self::OsdWidth => "osd-width",
            Self::OsdHeight => "osd-height",
            Self::OsdPar => "osd-par",
            Self::OsdDimensions => "osd-dimensions",
            Self::TermSize => "term-size",
            Self::WindowId => "window-id",
            Self::MousePos => "mouse-pos",
            Self::TouchPos => "touch-pos",
            Self::SubAssExtradata => "sub-ass-extradata",
            Self::SubText => "sub-text",
            Self::SubTextAss => "sub-text/ass",
            Self::SubTextAssFull => "sub-text/ass-full",
            Self::SecondarySubText => "secondary-sub-text",
            Self::SubStart => "sub-start",
            Self::SubStartFull => "sub-start/full",
            Self::SecondarySubStart => "secondary-sub-start",
            Self::SubEnd => "sub-end",
            Self::SubEndFull => "sub-end/full",
            Self::SecondarySubEnd => "secondary-sub-end",
            Self::PlaylistPos => "playlist-pos",
            Self::PlaylistPos1 => "playlist-pos-1",
            Self::PlaylistCurrentPos => "playlist-current-pos",
            Self::PlaylistPlayingPos => "playlist-playing-pos",
            Self::PlaylistCount => "playlist-count",
            Self::PlaylistPath => "playlist-path",
            Self::Playlist => "playlist",
            Self::TrackList => "track-list",
            Self::CurrentTracksVideo => "current-tracks/video",
            Self::CurrentTracksAudio => "current-tracks/audio",
            Self::CurrentTracksSub => "current-tracks/sub",
            Self::CurrentTracksSub2 => "current-tracks/sub2",
            Self::ChapterList => "chapter-list",
            Self::Af => "af",
            Self::Vf => "vf",
            Self::Seekable => "seekable",
            Self::PartiallySeekable => "partially-seekable",
            Self::PlaybackAbort => "playback-abort",
            Self::CursorAutohide => "cursor-autohide",
            Self::TermClipCc => "term-clip-cc",
            Self::OsdSymCc => "osd-sym-cc",
            Self::OsdAssCc => "osd-ass-cc",
            Self::VoConfigured => "vo-configured",
            Self::VoPasses => "vo-passes",
            Self::PerfInfo => "perf-info",
            Self::VideoBitrate => "video-bitrate",
            Self::AudioBitrate => "audio-bitrate",
            Self::SubBitrate => "sub-bitrate",
            Self::AudioDeviceList => "audio-device-list",
            Self::AudioDevice => "audio-device",
            Self::CurrentVo => "current-vo",
            Self::CurrentGpuContext => "current-gpu-context",
            Self::CurrentAo => "current-ao",
            Self::UserData => "user-data",
            Self::MenuData => "menu-data",
            Self::WorkingDirectory => "working-directory",
            Self::CurrentWatchLaterDir => "current-watch-later-dir",
            Self::ProtocolList => "protocol-list",
            Self::DecoderList => "decoder-list",
            Self::EncoderList => "encoder-list",
            Self::DemuxerLavfList => "demuxer-lavf-list",
            Self::InputKeyList => "input-key-list",
            Self::MpvVersion => "mpv-version",
            Self::MpvConfiguration => "mpv-configuration",
            Self::FfmpegVersion => "ffmpeg-version",
            Self::LibassVersion => "libass-version",
            Self::Platform => "platform",
            Self::Options => "options",
            Self::FileLocalOptions => "file-local-options",
            Self::OptionInfo => "option-info",
            Self::PropertyList => "property-list",
            Self::ProfileList => "profile-list",
            Self::CommandList => "command-list",
            Self::InputBindings => "input-bindings",
            Self::ClipboardText => "clipboard/text",
            Self::ClipboardTextPrimary => "clipboard/text-primary",
            Self::CurrentClipboardBackend => "current-clipboard-backend",
            Self::Clock => "clock",
        }
    }
}
