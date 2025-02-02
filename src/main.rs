//! Change `/sys/class/drm/card*/device/power_dpm_force_performance_level`.

use std::io;
use std::os::unix::ffi::OsStrExt;
use std::process::ExitCode;

/// Name of the binary, without possible insecure modifications.
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

/// Target performance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PerfLevel {
    /// Clocks are forced to the lowest power state.
    Low,
    /// Dynamically select the optimal power profile for current conditions in the driver.
    Auto,
    /// Clocks are forced to the highest power state.
    High,
}

impl PerfLevel {
    /// Textual value for writing to `power_dpm_force_performance_level`.
    pub const fn as_contents(self) -> &'static [u8] {
        match self {
            Self::Low => b"low",
            Self::Auto => b"auto",
            Self::High => b"high",
        }
    }
}

/// Parse performance level from command line.
#[cold]
fn parse_args() -> Option<PerfLevel> {
    let mut args = std::env::args_os().skip(1);
    match (args.next(), args.next()) {
        (Some(level), None) => match level.as_bytes() {
            b"low" => return Some(PerfLevel::Low),
            b"auto" => return Some(PerfLevel::Auto),
            b"high" => return Some(PerfLevel::High),
            arg => eprintln!("{PKG_NAME}: invalid performance level: {}", arg.escape_ascii()),
        },
        (None, _) => eprintln!("{PKG_NAME}: missing performance level"),
        (_, Some(arg)) => eprintln!("{PKG_NAME}: invalid argument: {}", arg.as_bytes().escape_ascii()),
    }

    eprintln!("Usage: {PKG_NAME} [low|auto|high]");
    None
}

/// Return the internal value or print IO errors.
fn ok<T>(result: io::Result<T>) -> Option<T> {
    #[cold]
    #[inline(never)]
    fn show_error(error: &io::Error) {
        eprintln!("{PKG_NAME}: {error}");
    }

    result.inspect_err(show_error).ok()
}

/// Binary entrypoint.
#[must_use]
pub fn main() -> ExitCode {
    let Some(perf_level) = parse_args() else {
        return ExitCode::FAILURE;
    };

    let Some(dir) = ok(std::fs::read_dir("/sys/class/drm/")) else {
        return ExitCode::FAILURE;
    };

    for entry in dir {
        if let Some(mut path) = ok(entry.map(|entry| entry.path())) {
            path.push("device/power_dpm_force_performance_level");
            if path.exists() {
                ok(std::fs::write(path, perf_level.as_contents()));
            }
        }
    }
    ExitCode::SUCCESS
}
