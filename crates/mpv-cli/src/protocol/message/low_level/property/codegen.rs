#[crabtime::function]
#[macro_export]
pub fn gen_property_values() {
    pub struct Definition {
        type_name: &'static str,
        ty: &'static str,
        property_name: &'static str,
    }

    pub const DEFINITIONS: &[Definition] = &[
        Definition {
            type_name: "Pause",
            ty: "bool",
            property_name: "pause",
        },
        Definition {
            type_name: "AudioSpeedCorrection",
            ty: "f64",
            property_name: "audio-speed-correction",
        },
        Definition {
            type_name: "VideoSpeedCorrection",
            ty: "f64",
            property_name: "video-speed-correction",
        },
        Definition {
            type_name: "DisplaySyncActive",
            ty: "bool",
            property_name: "display-sync-active",
        },
        Definition {
            type_name: "Filename",
            ty: "String",
            property_name: "filename",
        },
        Definition {
            type_name: "FilenameNoExt",
            ty: "String",
            property_name: "filename/no-ext",
        },
        Definition {
            type_name: "FileSize",
            ty: "i64",
            property_name: "file-size",
        },
        Definition {
            type_name: "EstimatedFrameCount",
            ty: "i64",
            property_name: "estimated-frame-count",
        },
        Definition {
            type_name: "EstimatedFrameNumber",
            ty: "i64",
            property_name: "estimated-frame-number",
        },
        Definition {
            type_name: "Pid",
            ty: "i64",
            property_name: "pid",
        },
        Definition {
            type_name: "Path",
            ty: "String",
            property_name: "path",
        },
        Definition {
            type_name: "StreamOpenFilename",
            ty: "String",
            property_name: "stream-open-filename",
        },
        Definition {
            type_name: "MediaTitle",
            ty: "String",
            property_name: "media-title",
        },
        Definition {
            type_name: "FileFormat",
            ty: "String",
            property_name: "file-format",
        },
        Definition {
            type_name: "CurrentDemuxer",
            ty: "String",
            property_name: "current-demuxer",
        },
        Definition {
            type_name: "StreamPath",
            ty: "String",
            property_name: "stream-path",
        },
        Definition {
            type_name: "StreamPos",
            ty: "i64",
            property_name: "stream-pos",
        },
        Definition {
            type_name: "StreamEnd",
            ty: "i64",
            property_name: "stream-end",
        },
        Definition {
            type_name: "Duration",
            ty: "f64",
            property_name: "duration",
        },
        Definition {
            type_name: "DurationFull",
            ty: "f64",
            property_name: "duration/full",
        },
        Definition {
            type_name: "Avsync",
            ty: "f64",
            property_name: "avsync",
        },
        Definition {
            type_name: "TotalAvsyncChange",
            ty: "f64",
            property_name: "total-avsync-change",
        },
        Definition {
            type_name: "DecoderFrameDropCount",
            ty: "i64",
            property_name: "decoder-frame-drop-count",
        },
        Definition {
            type_name: "FrameDropCount",
            ty: "i64",
            property_name: "frame-drop-count",
        },
        Definition {
            type_name: "MistimedFrameCount",
            ty: "i64",
            property_name: "mistimed-frame-count",
        },
        Definition {
            type_name: "VsyncRatio",
            ty: "f64",
            property_name: "vsync-ratio",
        },
        Definition {
            type_name: "VoDelayedFrameCount",
            ty: "i64",
            property_name: "vo-delayed-frame-count",
        },
        Definition {
            type_name: "PercentPos",
            ty: "i64",
            property_name: "percent-pos",
        },
        Definition {
            type_name: "TimePos",
            ty: "f64",
            property_name: "time-pos",
        },
        Definition {
            type_name: "TimePosFull",
            ty: "f64",
            property_name: "time-pos/full",
        },
        Definition {
            type_name: "TimeStart",
            ty: "f64",
            property_name: "time-start",
        },
        Definition {
            type_name: "TimeRemaining",
            ty: "f64",
            property_name: "time-remaining",
        },
        Definition {
            type_name: "TimeRemainingFull",
            ty: "f64",
            property_name: "time-remaining/full",
        },
        Definition {
            type_name: "AudioPts",
            ty: "f64",
            property_name: "audio-pts",
        },
        Definition {
            type_name: "AudioPtsFull",
            ty: "f64",
            property_name: "audio-pts/full",
        },
        Definition {
            type_name: "PlaytimeRemaining",
            ty: "f64",
            property_name: "playtime-remaining",
        },
        Definition {
            type_name: "PlaytimeRemainingFull",
            ty: "f64",
            property_name: "playtime-remaining/full",
        },
        Definition {
            type_name: "PlaybackTime",
            ty: "f64",
            property_name: "playback-time",
        },
        Definition {
            type_name: "PlaybackTimeFull",
            ty: "f64",
            property_name: "playback-time/full",
        },
        Definition {
            type_name: "RemainingFileLoops",
            ty: "i64",
            property_name: "remaining-file-loops",
        },
        Definition {
            type_name: "RemainingAbLoops",
            ty: "i64",
            property_name: "remaining-ab-loops",
        },
        Definition {
            type_name: "Chapter",
            ty: "i64",
            property_name: "chapter",
        },
        Definition {
            type_name: "Edition",
            ty: "i64",
            property_name: "edition",
        },
        Definition {
            type_name: "CurrentEdition",
            ty: "i64",
            property_name: "current-edition",
        },
        Definition {
            type_name: "Chapters",
            ty: "i64",
            property_name: "chapters",
        },
        Definition {
            type_name: "Editions",
            ty: "i64",
            property_name: "editions",
        },
        Definition {
            type_name: "EditionList",
            ty: "fields::EditionList",
            property_name: "edition-list",
        },
        Definition {
            type_name: "Metadata",
            ty: "fields::Metadata",
            property_name: "metadata",
        },
        Definition {
            type_name: "FilteredMetadata",
            ty: "fields::Metadata",
            property_name: "filtered-metadata",
        },
        Definition {
            type_name: "ChapterMetadata",
            ty: "fields::Metadata",
            property_name: "chapter-metadata",
        },
        Definition {
            type_name: "VfMetadata",
            ty: "std::collections::HashMap<String, String>",
            property_name: "vf-metadata",
        },
        Definition {
            type_name: "AfMetadata",
            ty: "std::collections::HashMap<String, String>",
            property_name: "af-metadata",
        },
        Definition {
            type_name: "DeinterlaceActive",
            ty: "bool",
            property_name: "deinterlace-active",
        },
        Definition {
            type_name: "IdleActive",
            ty: "bool",
            property_name: "idle-active",
        },
        Definition {
            type_name: "CoreIdle",
            ty: "bool",
            property_name: "core-idle",
        },
        Definition {
            type_name: "CacheSpeed",
            ty: "i64",
            property_name: "cache-speed",
        },
        Definition {
            type_name: "DemuxerCacheDuration",
            ty: "f64",
            property_name: "demuxer-cache-duration",
        },
        Definition {
            type_name: "DemuxerCacheTime",
            ty: "f64",
            property_name: "demuxer-cache-time",
        },
        Definition {
            type_name: "DemuxerCacheIdle",
            ty: "bool",
            property_name: "demuxer-cache-idle",
        },
        Definition {
            type_name: "DemuxerCacheState",
            ty: "fields::DemuxerCacheState",
            property_name: "demuxer-cache-state",
        },
        Definition {
            type_name: "DemuxerViaNetwork",
            ty: "bool",
            property_name: "demuxer-via-network",
        },
        Definition {
            type_name: "DemuxerStartTime",
            ty: "f64",
            property_name: "demuxer-start-time",
        },
        Definition {
            type_name: "PausedForCache",
            ty: "bool",
            property_name: "paused-for-cache",
        },
        Definition {
            type_name: "CacheBufferingState",
            ty: "i64",
            property_name: "cache-buffering-state",
        },
        Definition {
            type_name: "EofReached",
            ty: "bool",
            property_name: "eof-reached",
        },
        Definition {
            type_name: "Seeking",
            ty: "bool",
            property_name: "seeking",
        },
        Definition {
            type_name: "MixerActive",
            ty: "bool",
            property_name: "mixer-active",
        },
        Definition {
            type_name: "AoVolume",
            ty: "f64",
            property_name: "ao-volume",
        },
        Definition {
            type_name: "AoMute",
            ty: "bool",
            property_name: "ao-mute",
        },
        Definition {
            type_name: "AudioParams",
            ty: "fields::AudioParams",
            property_name: "audio-params",
        },
        Definition {
            type_name: "AudioOutParams",
            ty: "fields::AudioParams",
            property_name: "audio-out-params",
        },
        Definition {
            type_name: "Colormatrix",
            ty: "String",
            property_name: "colormatrix",
        },
        Definition {
            type_name: "ColormatrixInputRange",
            ty: "String",
            property_name: "colormatrix-input-range",
        },
        Definition {
            type_name: "ColormatrixPrimaries",
            ty: "String",
            property_name: "colormatrix-primaries",
        },
        Definition {
            type_name: "Hwdec",
            ty: "String",
            property_name: "hwdec",
        },
        Definition {
            type_name: "HwdecCurrent",
            ty: "String",
            property_name: "hwdec-current",
        },
        Definition {
            type_name: "HwdecInterop",
            ty: "String",
            property_name: "hwdec-interop",
        },
        Definition {
            type_name: "Width",
            ty: "i64",
            property_name: "width",
        },
        Definition {
            type_name: "Height",
            ty: "i64",
            property_name: "height",
        },
        Definition {
            type_name: "VideoParams",
            ty: "fields::VideoParams",
            property_name: "video-params",
        },
        Definition {
            type_name: "VideoDecParams",
            ty: "fields::VideoParams",
            property_name: "video-dec-params",
        },
        Definition {
            type_name: "VideoOutParams",
            ty: "fields::VideoParams",
            property_name: "video-out-params",
        },
        Definition {
            type_name: "VideoTargetParams",
            ty: "fields::VideoParams",
            property_name: "video-target-params",
        },
        Definition {
            type_name: "VideoFrameInfo",
            ty: "fields::VideoFrameInfo",
            property_name: "video-frame-info",
        },
        Definition {
            type_name: "ContainerFps",
            ty: "f64",
            property_name: "container-fps",
        },
        Definition {
            type_name: "EstimatedVfFps",
            ty: "f64",
            property_name: "estimated-vf-fps",
        },
        Definition {
            type_name: "CurrentWindowScale",
            ty: "f64",
            property_name: "current-window-scale",
        },
        Definition {
            type_name: "Focused",
            ty: "bool",
            property_name: "focused",
        },
        Definition {
            type_name: "AmbientLight",
            ty: "f64",
            property_name: "ambient-light",
        },
        Definition {
            type_name: "DisplayNames",
            ty: "Vec<String>",
            property_name: "display-names",
        },
        Definition {
            type_name: "DisplayFps",
            ty: "f64",
            property_name: "display-fps",
        },
        Definition {
            type_name: "EstimatedDisplayFps",
            ty: "f64",
            property_name: "estimated-display-fps",
        },
        Definition {
            type_name: "VsyncJitter",
            ty: "f64",
            property_name: "vsync-jitter",
        },
        Definition {
            type_name: "DisplayWidth",
            ty: "i64",
            property_name: "display-width",
        },
        Definition {
            type_name: "DisplayHeight",
            ty: "i64",
            property_name: "display-height",
        },
        Definition {
            type_name: "DisplayHidpiScale",
            ty: "f64",
            property_name: "display-hidpi-scale",
        },
        Definition {
            type_name: "OsdWidth",
            ty: "i64",
            property_name: "osd-width",
        },
        Definition {
            type_name: "OsdHeight",
            ty: "i64",
            property_name: "osd-height",
        },
        Definition {
            type_name: "OsdPar",
            ty: "f64",
            property_name: "osd-par",
        },
        Definition {
            type_name: "OsdDimensions",
            ty: "fields::OsdDimensions",
            property_name: "osd-dimensions",
        },
        Definition {
            type_name: "TermSize",
            ty: "Vec<i64>",
            property_name: "term-size",
        },
        Definition {
            type_name: "WindowId",
            ty: "i64",
            property_name: "window-id",
        },
        Definition {
            type_name: "MousePos",
            ty: "fields::MousePos",
            property_name: "mouse-pos",
        },
        Definition {
            type_name: "TouchPos",
            ty: "fields::TouchPos",
            property_name: "touch-pos",
        },
        Definition {
            type_name: "SubAssExtradata",
            ty: "String",
            property_name: "sub-ass-extradata",
        },
        Definition {
            type_name: "SubText",
            ty: "String",
            property_name: "sub-text",
        },
        Definition {
            type_name: "SubTextAss",
            ty: "String",
            property_name: "sub-text/ass",
        },
        Definition {
            type_name: "SubTextAssFull",
            ty: "String",
            property_name: "sub-text/ass-full",
        },
        Definition {
            type_name: "SecondarySubText",
            ty: "String",
            property_name: "secondary-sub-text",
        },
        Definition {
            type_name: "SubStart",
            ty: "Option<f64>",
            property_name: "sub-start",
        },
        Definition {
            type_name: "SubStartFull",
            ty: "Option<f64>",
            property_name: "sub-start/full",
        },
        Definition {
            type_name: "SecondarySubStart",
            ty: "Option<f64>",
            property_name: "secondary-sub-start",
        },
        Definition {
            type_name: "SubEnd",
            ty: "Option<f64>",
            property_name: "sub-end",
        },
        Definition {
            type_name: "SubEndFull",
            ty: "Option<f64>",
            property_name: "sub-end/full",
        },
        Definition {
            type_name: "SecondarySubEnd",
            ty: "Option<f64>",
            property_name: "secondary-sub-end",
        },
        Definition {
            type_name: "PlaylistPos",
            ty: "i64",
            property_name: "playlist-pos",
        },
        Definition {
            type_name: "PlaylistPos1",
            ty: "i64",
            property_name: "playlist-pos-1",
        },
        Definition {
            type_name: "PlaylistCurrentPos",
            ty: "i64",
            property_name: "playlist-current-pos",
        },
        Definition {
            type_name: "PlaylistPlayingPos",
            ty: "i64",
            property_name: "playlist-playing-pos",
        },
        Definition {
            type_name: "PlaylistCount",
            ty: "i64",
            property_name: "playlist-count",
        },
        Definition {
            type_name: "PlaylistPath",
            ty: "String",
            property_name: "playlist-path",
        },
        Definition {
            type_name: "Playlist",
            ty: "fields::Playlist",
            property_name: "playlist",
        },
        Definition {
            type_name: "TrackList",
            ty: "fields::TrackList",
            property_name: "track-list",
        },
        Definition {
            type_name: "CurrentTracksVideo",
            ty: "fields::Track",
            property_name: "current-tracks/video",
        },
        Definition {
            type_name: "CurrentTracksAudio",
            ty: "fields::Track",
            property_name: "current-tracks/audio",
        },
        Definition {
            type_name: "CurrentTracksSub",
            ty: "fields::Track",
            property_name: "current-tracks/sub",
        },
        Definition {
            type_name: "CurrentTracksSub2",
            ty: "fields::Track",
            property_name: "current-tracks/sub2",
        },
        Definition {
            type_name: "ChapterList",
            ty: "fields::ChapterList",
            property_name: "chapter-list",
        },
        Definition {
            type_name: "Af",
            ty: "Vec<fields::Filter>",
            property_name: "af",
        },
        Definition {
            type_name: "Vf",
            ty: "Vec<fields::Filter>",
            property_name: "vf",
        },
        Definition {
            type_name: "Seekable",
            ty: "bool",
            property_name: "seekable",
        },
        Definition {
            type_name: "PartiallySeekable",
            ty: "bool",
            property_name: "partially-seekable",
        },
        Definition {
            type_name: "PlaybackAbort",
            ty: "bool",
            property_name: "playback-abort",
        },
        Definition {
            type_name: "CursorAutohide",
            ty: "i64",
            property_name: "cursor-autohide",
        },
        Definition {
            type_name: "TermClipCc",
            ty: "String",
            property_name: "term-clip-cc",
        },
        Definition {
            type_name: "OsdSymCc",
            ty: "String",
            property_name: "osd-sym-cc",
        },
        Definition {
            type_name: "OsdAssCc",
            ty: "String",
            property_name: "osd-ass-cc",
        },
        Definition {
            type_name: "VoConfigured",
            ty: "bool",
            property_name: "vo-configured",
        },
        Definition {
            type_name: "VoPasses",
            ty: "fields::VoPasses",
            property_name: "vo-passes",
        },
        Definition {
            type_name: "PerfInfo",
            ty: "std::collections::HashMap<String, String>",
            property_name: "perf-info",
        },
        Definition {
            type_name: "VideoBitrate",
            ty: "i64",
            property_name: "video-bitrate",
        },
        Definition {
            type_name: "AudioBitrate",
            ty: "i64",
            property_name: "audio-bitrate",
        },
        Definition {
            type_name: "SubBitrate",
            ty: "i64",
            property_name: "sub-bitrate",
        },
        Definition {
            type_name: "AudioDeviceList",
            ty: "Vec<fields::AudioDevice>",
            property_name: "audio-device-list",
        },
        Definition {
            type_name: "AudioDevice",
            ty: "String",
            property_name: "audio-device",
        },
        Definition {
            type_name: "CurrentVo",
            ty: "String",
            property_name: "current-vo",
        },
        Definition {
            type_name: "CurrentGpuContext",
            ty: "String",
            property_name: "current-gpu-context",
        },
        Definition {
            type_name: "CurrentAo",
            ty: "String",
            property_name: "current-ao",
        },
        Definition {
            type_name: "UserData",
            ty: "std::collections::HashMap<String, String>",
            property_name: "user-data",
        },
        Definition {
            type_name: "MenuData",
            ty: "Vec<fields::MenuItem>",
            property_name: "menu-data",
        },
        Definition {
            type_name: "WorkingDirectory",
            ty: "String",
            property_name: "working-directory",
        },
        Definition {
            type_name: "CurrentWatchLaterDir",
            ty: "String",
            property_name: "current-watch-later-dir",
        },
        Definition {
            type_name: "ProtocolList",
            ty: "Vec<String>",
            property_name: "protocol-list",
        },
        Definition {
            type_name: "DecoderList",
            ty: "Vec<fields::Decoder>",
            property_name: "decoder-list",
        },
        Definition {
            type_name: "EncoderList",
            ty: "Vec<fields::Decoder>",
            property_name: "encoder-list",
        },
        Definition {
            type_name: "DemuxerLavfList",
            ty: "Vec<String>",
            property_name: "demuxer-lavf-list",
        },
        Definition {
            type_name: "InputKeyList",
            ty: "Vec<String>",
            property_name: "input-key-list",
        },
        Definition {
            type_name: "MpvVersion",
            ty: "String",
            property_name: "mpv-version",
        },
        Definition {
            type_name: "MpvConfiguration",
            ty: "String",
            property_name: "mpv-configuration",
        },
        Definition {
            type_name: "FfmpegVersion",
            ty: "String",
            property_name: "ffmpeg-version",
        },
        Definition {
            type_name: "LibassVersion",
            ty: "i64",
            property_name: "libass-version",
        },
        Definition {
            type_name: "Platform",
            ty: "String",
            property_name: "platform",
        },
        Definition {
            type_name: "Options",
            ty: "std::collections::HashMap<String, String>",
            property_name: "options",
        },
        Definition {
            type_name: "FileLocalOptions",
            ty: "std::collections::HashMap<String, String>",
            property_name: "file-local-options",
        },
        Definition {
            type_name: "OptionInfo",
            ty: "fields::OptionInfo",
            property_name: "option-info",
        },
        Definition {
            type_name: "PropertyList",
            ty: "Vec<String>",
            property_name: "property-list",
        },
        Definition {
            type_name: "ProfileList",
            ty: "Vec<HashMap<String, String>>",
            property_name: "profile-list",
        },
        Definition {
            type_name: "CommandList",
            ty: "Vec<HashMap<String, String>>",
            property_name: "command-list",
        },
        Definition {
            type_name: "InputBindings",
            ty: "Vec<fields::InputBinding>",
            property_name: "input-bindings",
        },
        Definition {
            type_name: "ClipboardText",
            ty: "String",
            property_name: "clipboard/text",
        },
        Definition {
            type_name: "ClipboardTextPrimary",
            ty: "String",
            property_name: "clipboard/text-primary",
        },
        Definition {
            type_name: "CurrentClipboardBackend",
            ty: "String",
            property_name: "current-clipboard-backend",
        },
        Definition {
            type_name: "Clock",
            ty: "String",
            property_name: "clock",
        },
    ];
    DEFINITIONS
        .into_iter()
        .for_each(|Definition { type_name, ty, property_name }| {
            let property_name = format!(r#""{property_name}""#);
            crabtime::output! {
                #[derive(Debug, Clone, serde::Deserialize)]
                pub struct {{type_name}}(pub {{ty}});

                #[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
                pub struct {{type_name}}Kind;

                impl PropertyValueName for {{type_name}}Kind {
                    type Value = {{type_name}};
                    fn as_str() -> &'static str {
                        {{property_name}}
                    }
                }
                impl SetPropertyValue for {{type_name}} {
                    type Kind = {{type_name}}Kind;
                    type Primitive = {{ty}};
                    fn inner(&self) -> &Self::Primitive {
                        &self.0
                    }
                }
            }
        });
    let variants = DEFINITIONS
        .iter()
        .map(|Definition { type_name, property_name, ty }| {
            format!(
                r#"
                    #[serde(rename = "{property_name}")]
                    {type_name}
                "#
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let variant_names = DEFINITIONS
        .iter()
        .map(|Definition { type_name, property_name, ty }| {
            format!(
                r#"
                    Self::{type_name} => "{property_name}"
                "#
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let variant_names = format!(
        r#"
            match self {{
                {variant_names}
            }}
        "#
    );

    crabtime::output! {
        #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
        pub enum PropertyValueKind {
            {{variants}}
        }

        impl PropertyValueKind {
            pub fn name(&self) -> &'static str {
                {{variant_names}}
            }
        }
    }
}
