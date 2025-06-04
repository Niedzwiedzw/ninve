use {
    crate::{
        instance::MpvInstance,
        protocol::message::{
            api::SetPropertyCommand,
            low_level::{SetProperty, property::Pause},
        },
    },
    anyhow::{Context, Result},
    futures::{FutureExt, TryFutureExt},
    std::{future::ready, path::Path},
    tap::Pipe,
    test_log::test,
    tokio::time::{Duration, sleep},
    tracing::{info, info_span},
};

macro_rules! in_test_dir {
    ($file:literal) => {
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/tests/test_data/", $file))
    };
}

async fn sleep_1s() {
    sleep(Duration::from_secs(1)).await
}

#[test(tokio::test)]
async fn test_play_whole_video() -> Result<()> {
    let _s = info_span!("test_play_whole_video").entered();
    MpvInstance::new(in_test_dir!("test-video-1.mp4"))
        .map(|r| r.context("starting mpv"))
        .and_then(async |mut mpv| {
            mpv.process
                .event_bus
                .events
                .await_playback()
                .await
                .pipe(ready)
                .map(|r| r.context("awaiting for playback"))
                .map_ok(|_| mpv)
                .await
        })
        .inspect_ok(|_| info!("playback started"))
        .and_then(async |mut mpv| {
            let response = mpv
                .process
                .event_bus
                .commands
                .command(SetPropertyCommand::new(SetProperty(Pause(true))))
                .await
                .context("pausing")?;
            info!("response: {response:?}");
            sleep_1s().await;
            let response = mpv
                .process
                .event_bus
                .commands
                .command(SetPropertyCommand::new(SetProperty(Pause(false))))
                .await
                .context("unpausing")?;
            info!("response: {response:?}");
            sleep_1s().await;
            mpv.finish()
                .await
                .context("finishing the process")
                .map(|_| ())?;
            Ok(())
        })
        .await
        .context("testing basic functionality")
}
