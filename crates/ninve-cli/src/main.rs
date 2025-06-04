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
                api::{GetPropertyCommand, ObservePropertyCommand},
                low_level::{GetProperty, ObserveProperty, property::*},
            },
        },
    },
    ratatui::{
        layout::{Constraint, Layout, Rect},
        style::{Color, Style, Stylize, palette::tailwind},
        text::Span,
        widgets::{Block, Borders, Gauge, Paragraph, Widget},
    },
    ron::ser::{PrettyConfig, to_string_pretty},
    serde::{Deserialize, Serialize},
    std::{collections::BTreeMap, fmt, ops::Div, path::Path, process::ExitStatus},
    tap::{Pipe, Tap},
    tokio::task::LocalSet,
    tracing::{debug, info},
    tui_logger::{TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiLoggerWidget, TuiWidgetState},
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
        tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt},
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
    time_start: Time,
    time_end: Time,
    time_pos: Time,
    trim_left: Time,
    trim_right: Time,
    loop_mode: bool,
    event_log: BTreeMap<MpvEventKind, MpvEvent>,
}

impl Ninve {}

mod cli {
    use {clap::Parser, std::path::PathBuf};

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    pub(crate) struct Cli {
        /// path to the edited file
        pub media_path: PathBuf,
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
    fn precision(self) -> u32 {
        (self.0 * 100.) as u32
    }
}

impl Div for Time {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

fn main() -> Result<()> {
    logging::setup_tracing();
    let Cli { media_path } = Cli::parse();
    info!("starting up for {}", media_path.display());

    let tui_logger_state = TuiWidgetState::new();

    ratatui::init()
        .pipe(|mut terminal| {
            in_runtime(async move || -> Result<()> {
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
                    time_start: Time(0.),
                    time_end: Time(stream_len),
                    time_pos: Time(0.),
                    trim_left: Time(0.),
                    trim_right: Time(stream_len),
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
                    match event {
                        Event::Crossterm(event) => match event {
                            crossterm::event::Event::Key(KeyEvent { code, kind, .. }) => match code {
                                crossterm::event::KeyCode::Char('[') => {
                                    ninve.trim_left = ninve.time_pos;
                                }
                                crossterm::event::KeyCode::Char(']') => {
                                    ninve.trim_right = ninve.time_pos;
                                }
                                crossterm::event::KeyCode::Char('q') => {
                                    bail!("exited with [q]")
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
                            let kind = MpvEventKind::from(&mpv_event);
                            ninve.event_log.insert(kind, mpv_event.clone());
                            match mpv_event {
                                MpvEvent::ClientInteraction(ClientInteractionEvent::PropertyChange(ev)) => {
                                    if let Some(value) = ev.of_kind::<TimePosKind>() {
                                        value
                                            .context("could not parse value")
                                            .map(|TimePos(time_pos)| {
                                                ninve.time_pos = Time(time_pos);
                                            })?
                                    }
                                }
                                other => {
                                    debug!("[EVT] {other:?}");
                                }
                            }
                        }
                        Event::Exit(exit_status) => {
                            info!(?exit_status, "PROCESS FINISHED");
                            break;
                        }
                    }
                    terminal
                        .draw(|frame| {
                            ninve.pipe_ref(
                                |Ninve {
                                     time_start,
                                     time_end,
                                     time_pos,
                                     trim_left,
                                     trim_right,
                                     loop_mode,
                                     event_log,
                                 }| {
                                    let time_start = *time_start;
                                    let time_end = *time_end;
                                    let time_pos = *time_pos;
                                    let trim_left = *trim_left;
                                    let trim_right = *trim_right;
                                    let app = frame.area();
                                    // app
                                    {
                                        let [controls, progress_bar] = Layout::vertical([Constraint::Fill(1), Constraint::Length(4)]).areas(app);
                                        // progress bar
                                        {
                                            let [trimmed_left, center, trimmed_right] = Layout::horizontal([
                                                //
                                                Constraint::Ratio((trim_left - time_start).precision(), time_end.precision()),
                                                Constraint::Ratio((trim_right - trim_left).precision(), time_end.precision()),
                                                Constraint::Ratio((time_end - trim_right).precision(), time_end.precision()),
                                                //
                                            ])
                                            .areas(progress_bar);
                                            let mut gauge = |color: Color, rect: Rect, ratio| {
                                                Gauge::default()
                                                    .block(Block::bordered().title_bottom(Span::styled(format!("{time_pos}/{time_end}"), Style::new().bold())))
                                                    .style(color)
                                                    .ratio(ratio)
                                                    .render(rect, frame.buffer_mut());
                                            };
                                            gauge(tailwind::GRAY.c300, trimmed_left, 1.);
                                            gauge(
                                                match loop_mode {
                                                    true => tailwind::GREEN.c600,
                                                    false => tailwind::GRAY.c300,
                                                },
                                                center,
                                                ((time_pos - trim_left) / (trim_right - trim_left))
                                                    .min(1.)
                                                    .max(0.),
                                            );
                                            gauge(tailwind::GRAY.c300, trimmed_right, 1.);
                                        }
                                    }
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
                        })
                        .context("drawing to screen")?;
                }
                anyhow::Ok(())
            })
            .map(|_ninve| ())
        })
        .tap(|_| ratatui::restore())
}
