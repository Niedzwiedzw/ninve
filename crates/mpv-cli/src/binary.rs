use {
    std::{path::Path, sync::OnceLock},
    tap::Pipe,
    tracing::instrument,
};

#[derive(thiserror::Error, Debug, Clone, Copy)]
#[error("Binary could not be extracted from PATH (try `which mpv`)")]
pub struct Error(#[source] which::Error);

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
pub struct MpvBinary<'a>(&'a Path);

impl<'a> MpvBinary<'a> {
    pub fn command(self) -> std::process::Command {
        std::process::Command::new(self.0)
    }
}

static MPV_BINARY: OnceLock<Result<MpvBinary<'static>>> = OnceLock::new();

fn init_mpv_binary() -> Result<MpvBinary<'static>> {
    which::which("mpv")
        .map_err(Error)
        .map(|path| -> &'static Path { path.leak::<'static>() })
        .map(MpvBinary)
}

#[instrument(ret, err, level = "TRACE")]
pub fn get_mpv_binary() -> Result<MpvBinary<'static>> {
    MPV_BINARY.get_or_init(init_mpv_binary).pipe(|r| *r)
}
