use {
    crate::{
        instance::MpvInstance,
        protocol::message::{
            api::SetPropertyCommand,
            low_level::{SetProperty, property::Pause},
        },
    },
    anyhow::{Context, Result},
    std::{path::Path, thread::sleep, time::Duration},
    tap::TapFallible,
    tracing::info,
};

macro_rules! in_test_dir {
    ($file:literal) => {
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/tests/test_data/", $file))
    };
}

fn sleep_1s() {
    sleep(Duration::from_secs(1))
}

#[test_log::test]
fn test_play_whole_video() -> Result<()> {
    info!("test_play_whole_video");
    MpvInstance::new(in_test_dir!("test-video-1.mp4"))
        .context("starting mpv")
        .and_then(|mpv| mpv.await_playback().context("awaiting for playback"))
        .tap_ok(|_| info!("playback started"))
        .and_then(|mut mpv| {
            Ok(())
                .and_then(|_| {
                    mpv.command(SetPropertyCommand::new(SetProperty(Pause(true))))
                        .context("pausing")
                })
                .and_then(|response| {
                    info!("response: {response:?}");
                    sleep_1s();
                    mpv.command(SetPropertyCommand::new(SetProperty(Pause(false))))
                        .context("unpausing")
                })
                .and_then(|response| {
                    info!("response: {response:?}");
                    sleep_1s();
                    mpv.kill().context("finishing the process").map(|_| ())
                })
        })
        .context("testing basic functionality")
}
