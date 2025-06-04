#![allow(clippy::unit_arg)]

use {
    anyhow::{Context, Result, bail},
    clap::Parser,
    cli::Cli,
    crossterm::event::KeyEvent,
    futures::{FutureExt, StreamExt, TryFutureExt, TryStreamExt},
    mpv_cli::{
        instance::{MpvInstance, MpvProcess},
        ipc::event_bus::EventBus,
        protocol::{
            event::{MpvEvent, MpvEventKind, grouped_events::ClientInteractionEvent},
            message::{
                api::{GetPropertyCommand, ObservePropertyCommand, SetPropertyCommand},
                low_level::{GetProperty, ObserveProperty, SetProperty, property::*},
            },
        },
    },
    ratatui::{
        layout::{Constraint, Layout},
        style::{Color, Style, Stylize, palette::tailwind},
        text::Span,
        widgets::{Block, Gauge, Paragraph, Widget},
    },
    serde::{Deserialize, Serialize},
    std::{
        collections::BTreeMap,
        ffi::OsStr,
        fmt,
        future::ready,
        ops::{Div, Not},
        path::{Path, PathBuf},
        process::{ExitStatus, Stdio},
    },
    tap::{Pipe, Tap},
    tokio::task::LocalSet,
    tracing::{debug, info},
    tui_logger::TuiWidgetState,
};

#[allow(dead_code)]
fn in_runtime<T, F>(in_runtime: F) -> Result<T>
where
    T: 'static,
    F: AsyncFnOnce() -> Result<T> + 'static,
{
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("building runtime")?;

    runtime.block_on(async {
        let set = LocalSet::new();
        let output = set.spawn_local(in_runtime());
        set.await;
        output
            .map(|r| {
                r.context("task crashed")
                    .and_then(|r| r.context("runtime crashed"))
            })
            .await
    })
}

mod logging {
    use {
        std::str::FromStr,
        tracing::Level,
        tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt},
    };

    /// Sets up tracing with a level from the RUST_LOG env var and output to stderr
    ///
    /// Falls back to INFO level if RUST_LOG is not set or invalid
    pub fn setup_tracing() {
        // Get the log level from RUST_LOG env var, default to INFO
        let log_level = std::env::var("RUST_LOG")
            .ok()
            .and_then(|level| Level::from_str(&level).ok())
            .unwrap_or(Level::INFO);

        // Create a filter from the log level
        let filter = EnvFilter::builder()
            .with_default_directive(log_level.into())
            .from_env_lossy();

        // Set up the formatting layer
        // let fmt_layer = fmt::layer()
        //     .with_writer(std::io::stderr)
        //     .with_ansi(true) // Enable colors in terminal
        //     .with_target(true) // Include event targets
        //     .with_thread_ids(false) // Optional: disable thread IDs
        //     .with_thread_names(false); // Optional: disable thread names

        // Install the subscriber
        tracing_subscriber::registry()
            .with(filter)
            .with(tui_logger::TuiTracingSubscriberLayer)
            // .with(fmt_layer)
            .init();
    }
}

#[derive(serde::Serialize)]
struct Ninve {
    full: TimeRange,
    trimmed: TimeRange,
    pos: Time,
    loop_mode: bool,
    event_log: BTreeMap<MpvEventKind, MpvEvent>,
}

impl Ninve {}

mod cli {
    use {clap::Parser, std::path::PathBuf};

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    pub(crate) struct Cli {
        /// path to the file
        pub media_path: PathBuf,
        /// path to render
        pub output_path: Option<PathBuf>,
    }
}

async fn init_mpv(media_path: &Path) -> Result<MpvInstance> {
    MpvInstance::new(media_path)
        .map(|r| r.context("spawning mpv instance"))
        .and_then(async |mut mpv| {
            mpv.process
                .event_bus
                .events
                .await_playback()
                .await
                .context("awaiting playback")
                .map(|_| mpv)
        })
        // .and_then(async |mut mpv| {
        // })
        .await
}

enum Event {
    Crossterm(ratatui::crossterm::event::Event),
    Mpv(MpvEvent),
    Exit(ExitStatus),
}

static CRATE_NAME: &str = clap::crate_name!();
static CRATE_VERSION: &str = clap::crate_version!();

#[derive(Debug, Clone, Copy, derive_more::From, derive_more::Add, derive_more::Sub, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Time(f64);

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_seconds = self.0;
        if total_seconds < 0.0 {
            return write!(f, "-{}", Time(total_seconds.abs()));
        }

        let hours = (total_seconds / 3600.0).floor() as u64;
        let minutes = ((total_seconds % 3600.0) / 60.0).floor() as u64;
        let seconds = (total_seconds % 60.0).floor() as u64;
        let milliseconds = ((total_seconds % 1.0) * 10000.0).round() as u64;

        // Remove leading zeros for milliseconds (e.g., 0080 -> 08, 0123 -> 123)
        write!(f, "{hours:02}:{minutes:02}:{seconds:02}.{milliseconds:04}",)
    }
}

impl Time {
    pub fn to_microseconds(&self) -> i64 {
        (self.0 * 1_000_000.0).round() as i64
    }
    pub fn to_seconds_string(&self) -> String {
        format!("{}", self.0)
    }
    fn precision(self) -> u32 {
        (self.0 * 1000.).max(0.) as u32
    }
    fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }
    fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }
}

impl Div for Time {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

#[derive(derive_more::Constructor, serde::Serialize, Clone, Copy)]
struct TimeRange {
    start: Time,
    end: Time,
}

impl std::fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.start, self.end)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Ratio(u32, u32);
impl Ratio {
    fn new(val: u32, compared_to: u32) -> Result<Self> {
        if val > compared_to {
            anyhow::bail!("{val} is greater than {compared_to}")
        }
        if compared_to == 0 {
            anyhow::bail!("compared_to cannot be 0 ")
        }
        Ok(Self(val, compared_to))
    }
}

impl From<Ratio> for f64 {
    fn from(val: Ratio) -> Self {
        val.pipe(|Ratio(min, max)| {
            (min as f64 / max as f64).tap(|out| {
                if !((0.)..=1.).contains(out) {
                    panic!("{out} for min:{min} and max:{max} is invalid")
                }
            })
        })
    }
}
impl From<Ratio> for Constraint {
    fn from(val: Ratio) -> Self {
        val.pipe(|Ratio(min, max)| Constraint::Ratio(min, max))
    }
}

impl TimeRange {
    fn length(&self) -> Time {
        self.end - self.start
    }

    fn contains(&self, time: Time) -> bool {
        self.start <= time && time <= self.end
    }
    #[allow(dead_code)]
    fn closest_edge(&self, to_time: Time) -> Time {
        [self.end, self.start]
            .into_iter()
            .min_by_key(|edge| ordered_float::OrderedFloat((edge.0 - to_time.0).abs()))
            .expect("time range is non empty")
    }

    fn ratio(&self, value: Time) -> Result<Ratio> {
        let value = Time(value.0.max(self.start.0).min(self.end.0));
        let max = (self.end - self.start).precision();
        Ratio::new((value - self.start).precision().min(max), max)
    }

    fn to_subrange(self, other: Self) -> Option<(Self, Self, Self)> {
        // Ensure ranges are valid (start <= end)
        if self.start.0 > other.start.0 || self.end.0 < other.end.0 {
            None
        } else {
            Some((
                Self {
                    start: self.start,
                    end: other.start,
                },
                Self {
                    start: other.start,
                    end: other.end,
                },
                Self {
                    start: other.end,
                    end: self.end,
                },
            ))
        }
    }
}

fn generate_filename(media_path: &Path, start: Time, end: Time) -> Result<PathBuf> {
    let normalize = |os_str: &OsStr| os_str.to_string_lossy().to_string();
    media_path
        .file_stem()
        .map(normalize)
        .zip(media_path.extension().map(normalize))
        .context("not a file")
        .map(|(stem, extension)| format!("{stem}--{start}-{end}.{extension}").replace(":", "-"))
        .map(PathBuf::from)
}

fn trim_video_with_ffmpeg(input: &Path, output: &Path, range: TimeRange) -> Result<PathBuf> {
    std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .args(["-ss", range.start.to_seconds_string().as_str()])
        .args(["-to", range.end.to_seconds_string().as_str()])
        .args(["-c", "copy"])
        .args(["-map", "0"])
        .arg(output)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .status()
        .context("command did not finish")
        .and_then(|status| if status.success() { Ok(()) } else { Err(anyhow::anyhow!("{status:?}")) })
        .map(|_| output.to_owned())
}

fn main() -> Result<()> {
    logging::setup_tracing();
    let Cli { media_path, output_path } = Cli::parse();

    info!("starting up for {}", media_path.display());

    let _tui_logger_state = TuiWidgetState::new();

    ratatui::init()
        .pipe(|mut terminal| {
            in_runtime(async move || -> Result<(PathBuf, PathBuf, TimeRange)> {
                let mut mpv = init_mpv(&media_path).await.context("instantiating mpv")?;
                let MpvInstance {
                    process:
                        MpvProcess {
                            event_bus: EventBus { commands, events },
                            child,
                            ..
                        },
                } = &mut mpv;
                let Duration(stream_len) = commands
                    .command(GetPropertyCommand::new(GetProperty(DurationKind)))
                    .await
                    .context("reading current length")?
                    .data;
                let mut ninve = Ninve {
                    full: TimeRange {
                        start: Time(0.),
                        end: Time(stream_len),
                    },
                    trimmed: TimeRange {
                        start: Time(0.),
                        end: Time(stream_len),
                    },
                    pos: Time(0.),
                    loop_mode: false,
                    event_log: Default::default(),
                };
                commands
                    .command(ObservePropertyCommand::new(ObserveProperty(TimePosKind)))
                    .await
                    .context("observing stream position")?;

                let mut event_stream = [
                    events
                        .events()
                        .map(Event::Mpv)
                        .map(anyhow::Ok)
                        .fuse()
                        .boxed_local(),
                    ratatui::crossterm::event::EventStream::new()
                        .map(|e| e.context("crossterm error").map(Event::Crossterm))
                        .fuse()
                        .boxed_local(),
                    child
                        .wait()
                        .map(|c| c.context("Waiting for command to finish"))
                        .into_stream()
                        .fuse()
                        .map_ok(Event::Exit)
                        .boxed_local(),
                ]
                .pipe(futures::stream::iter)
                .flatten_unordered(3);

                while let Some(event) = event_stream.next().await.transpose().context("bad event")? {
                    let output_path = output_path
                        .clone()
                        .context("no output path provided")
                        .or_else(|reason| {
                            generate_filename(&media_path, ninve.trimmed.start, ninve.trimmed.end).with_context(|| format!("running because: {reason:?}"))
                        })
                        .context("evaluating output path")?;
                    let help_text = [
                        "QUIT:           <c> / <q>",
                        "TRIM LEFT:      <[>",
                        "TRIM RIGHT:     <]>",
                        "LOOP:           <]>",
                        "PAUSE/PLAY:     <SPC>",
                        "RENDER:         <R>",
                        "",
                        "use MPV controls for the rest",
                        "",
                        format!("file will be rendered to: {}", output_path.display()).as_str(),
                    ]
                    .join("\n");
                    match event {
                        Event::Crossterm(event) => match event {
                            crossterm::event::Event::Key(KeyEvent { code, .. }) => match code {
                                crossterm::event::KeyCode::Char(' ') => {
                                    commands
                                        .command(GetPropertyCommand::new(GetProperty(PauseKind)))
                                        .map_ok(|pause| pause.data.0)
                                        .await
                                        .pipe(ready)
                                        .and_then(|previous| commands.command(SetPropertyCommand::new(SetProperty(Pause(!previous)))))
                                        .await
                                        .context("toggling pause")?;
                                }
                                crossterm::event::KeyCode::Char('[') => {
                                    ninve.trimmed.start = ninve.pos.min(ninve.trimmed.end);
                                }
                                crossterm::event::KeyCode::Char(']') => {
                                    ninve.trimmed.end = ninve.pos.max(ninve.trimmed.start);
                                }
                                crossterm::event::KeyCode::Char(c @ 'q' | c @ 'c') => {
                                    bail!("exited with [{c}]")
                                }
                                crossterm::event::KeyCode::Char('l') => {
                                    info!("toggling loop mode");
                                    ninve.loop_mode = !ninve.loop_mode;
                                }
                                crossterm::event::KeyCode::Char('r') => {
                                    return Ok((media_path.clone(), output_path.clone(), ninve.trimmed));
                                }
                                key_event => {
                                    info!("[CROSSTERM] {key_event:?}");
                                }
                            },
                            event => {
                                info!("[CROSSTERM] {event:?}");
                            }
                        },
                        Event::Mpv(mpv_event) => {
                            ninve
                                .event_log
                                .insert(MpvEventKind::from(&mpv_event), mpv_event.clone());
                            match mpv_event {
                                MpvEvent::ClientInteraction(ClientInteractionEvent::PropertyChange(ev)) => {
                                    if let Some(TimePos(time_pos)) = ev
                                        .of_kind::<TimePosKind>()
                                        .transpose()
                                        .context("could not parse value")?
                                    {
                                        let mut time_pos = Time(time_pos);
                                        if ninve.loop_mode && ninve.trimmed.contains(time_pos).not() {
                                            time_pos = ninve.trimmed.start;
                                            commands
                                                .command(SetPropertyCommand::new(SetProperty(TimePos(time_pos.0))))
                                                .await
                                                .context("setting position due to looping")?;
                                        }
                                        ninve.pos = time_pos;
                                    }
                                }
                                other => {
                                    debug!("[EVT] {other:?}");
                                }
                            }
                        }
                        Event::Exit(exit_status) => {
                            info!(?exit_status, "PROCESS FINISHED");
                            anyhow::bail!("mpv closed");
                        }
                    }
                    terminal
                        .try_draw(|frame| {
                            ninve
                                .pipe_ref(
                                    |Ninve {
                                         loop_mode,
                                         event_log: _,
                                         full,
                                         trimmed,
                                         pos,
                                     }|
                                     -> Result<()> {
                                        let app = frame.area();
                                        // app
                                        {
                                            let [controls, progress_bar] = Layout::vertical([Constraint::Fill(1), Constraint::Length(4)]).areas(app);
                                            {
                                                // controls
                                                help_text
                                                    .pipe(Paragraph::new)
                                                    .block(Block::bordered().title_top(format!("{CRATE_NAME} v{CRATE_VERSION} ({})", media_path.display())))
                                                    .render(controls, frame.buffer_mut());
                                            }
                                            // progress bar
                                            {
                                                let (trim_left, active, trim_right) = full.to_subrange(*trimmed).expect("bad subrange");
                                                let [left, center, right] = Layout::horizontal([
                                                    //
                                                    full.ratio(trim_left.length()).map(Constraint::from)?,
                                                    full.ratio(active.length()).map(Constraint::from)?,
                                                    full.ratio(trim_right.length()).map(Constraint::from)?,
                                                    //
                                                ])
                                                .areas(progress_bar);
                                                {
                                                    // gauges
                                                    let gauge = |color: Color| Gauge::default().style(color);
                                                    if let Ok(ratio) = trim_left.ratio(*pos).map(f64::from) {
                                                        gauge(tailwind::GRAY.c700)
                                                            .ratio(ratio)
                                                            .block(Block::bordered())
                                                            .render(left, frame.buffer_mut());
                                                    }

                                                    if let Ok(ratio) = active.ratio(*pos).map(f64::from) {
                                                        gauge(match loop_mode {
                                                            true => tailwind::GREEN.c100,
                                                            false => tailwind::GRAY.c100,
                                                        })
                                                        .ratio(ratio)
                                                        .block(Block::bordered().title_bottom(Span::styled(
                                                            format!("{}", TimeRange { start: *pos, end: trimmed.end }),
                                                            Style::new().bold(),
                                                        )))
                                                        .render(center, frame.buffer_mut());
                                                    }

                                                    if let Ok(ratio) = trim_right.ratio(*pos).map(f64::from) {
                                                        gauge(tailwind::GRAY.c700)
                                                            .ratio(ratio)
                                                            .block(Block::bordered())
                                                            .render(right, frame.buffer_mut());
                                                    }
                                                }
                                            }
                                        }
                                        Ok(())
                                        // // logger
                                        // {
                                        //     TuiLoggerWidget::default()
                                        //         .style_error(Style::default().fg(Color::Red))
                                        //         .style_debug(Style::default().fg(Color::Green))
                                        //         .style_warn(Style::default().fg(Color::Yellow))
                                        //         .style_trace(Style::default().fg(Color::Magenta))
                                        //         .style_info(Style::default().fg(Color::Cyan))
                                        //         .output_separator(':')
                                        //         .output_timestamp(Some("%H:%M:%S".to_string()))
                                        //         .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
                                        //         .state(&tui_logger_state)
                                        //         .block(Block::default().title("Logs").borders(Borders::ALL))
                                        //         .render(logger, frame.buffer_mut());
                                        // }
                                    },
                                )
                                .map_err(std::io::Error::other)
                        })
                        .context("drawing to screen")?;
                }
                anyhow::bail!("channel closed");
            })
        })
        .tap(|_| ratatui::restore())
        .and_then(|(input, output, range)| trim_video_with_ffmpeg(&input, &output, range))
        .map(|output| println!("{}", output.display()))
}
