use crate::Result;
use std::{
    process::Command,
    thread,
    time::{Duration, Instant},
};

pub fn run_bounded(command: &mut Command, timeout: Duration) -> Result<()> {
    let mut child = command.spawn().map_err(|e| format!("Cannot start qmltestrunner: {e}. Install Qt declarative tools or set MIILO_QMLTESTRUNNER."))?;
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) if status.success() => return Ok(()),
            Ok(Some(status)) => return Err(format!("QML tests failed ({status})").into()),
            Ok(None) if start.elapsed() < timeout => thread::sleep(Duration::from_millis(20)),
            outcome => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(match outcome {
                    Err(error) => format!("Cannot wait for QML tests: {error}"),
                    _ => format!("QML tests timed out after {} ms", timeout.as_millis()),
                }
                .into());
            }
        }
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::{Command, Duration, run_bounded};
    #[test]
    fn child_failure_and_timeout_propagate() {
        assert!(
            run_bounded(
                Command::new("sh").args(["-c", "exit 3"]),
                Duration::from_secs(1)
            )
            .is_err()
        );
        assert!(run_bounded(Command::new("sleep").arg("10"), Duration::from_millis(30)).is_err());
        assert!(run_bounded(Command::new("true").arg(""), Duration::from_secs(1)).is_ok());
    }
}
