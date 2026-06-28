use tokio::sync::mpsc;

use crate::handler::SignalName;

pub async fn listen_signal(name: SignalName, tx: mpsc::UnboundedSender<SignalName>) {
    match name {
        SignalName::SigInt => loop {
            match tokio::signal::ctrl_c().await {
                Ok(()) => {
                    let _ = tx.send(SignalName::SigInt);
                }
                Err(e) => {
                    eprintln!("signal: failed to listen ctrl_c: {e}");
                    break;
                }
            }
        },
        #[cfg(windows)]
        SignalName::SigTerm => {
            let mut signal = match tokio::signal::windows::ctrl_close() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("signal: failed to register ctrl_close: {e}");
                    return;
                }
            };
            loop {
                signal.recv().await;
                let _ = tx.send(SignalName::SigTerm);
            }
        }
        #[cfg(windows)]
        SignalName::SigBreak => {
            let mut signal = match tokio::signal::windows::ctrl_break() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("signal: failed to register ctrl_break: {e}");
                    return;
                }
            };
            loop {
                signal.recv().await;
                let _ = tx.send(SignalName::SigBreak);
            }
        }
        #[cfg(unix)]
        SignalName::SigTerm => {
            let mut signal =
                match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("signal: failed to register SIGTERM: {e}");
                        return;
                    }
                };
            loop {
                signal.recv().await;
                let _ = tx.send(SignalName::SigTerm);
            }
        }
        #[cfg(unix)]
        SignalName::SigHup => {
            let mut signal =
                match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::hangup()) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("signal: failed to register SIGHUP: {e}");
                        return;
                    }
                };
            loop {
                signal.recv().await;
                let _ = tx.send(SignalName::SigHup);
            }
        }
        #[cfg(unix)]
        SignalName::SigQuit => {
            let mut signal =
                match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::quit()) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("signal: failed to register SIGQUIT: {e}");
                        return;
                    }
                };
            loop {
                signal.recv().await;
                let _ = tx.send(SignalName::SigQuit);
            }
        }
        #[cfg(not(unix))]
        SignalName::SigHup | SignalName::SigQuit => {
            eprintln!("signal: {} not supported on this platform", name.as_str());
        }
        #[cfg(not(windows))]
        SignalName::SigBreak => {
            eprintln!("signal: {} not supported on this platform", name.as_str());
        }
    }
}
