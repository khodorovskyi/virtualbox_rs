use tokio::time::Duration;
use tokio::time::sleep;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use crate::Display;

use crate::framebuffer::{FramebufferEvent, FramebufferEventInternal};
use log::{debug, error, info, trace};
use tokio::signal;
use tokio::signal::unix::{signal, Signal, SignalKind};
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot::Receiver as ReceiverOneshot;

pub(super) fn handle_message(
    mut rx: Receiver<FramebufferEventInternal>,
    display: Display,
    framebuffer_id: String,
    max_fps: u32,
) {
    let display_clone = display.clone();
    let (stop_sender, receiver) = tokio::sync::oneshot::channel::<()>();

    tokio::spawn(handle_all_signals(
        display_clone,
        framebuffer_id.clone(),
        receiver,
    ));
    tokio::spawn(async move {
        let mut count = 0;
        let mut sender: Option<tokio::sync::oneshot::Sender<FramebufferEvent>> = None;
        let mut img_buffer: Option<FramebufferEvent> = None;
        let mut m_width: u32 = 0;
        let mut m_height: u32 = 0;
        let mut m_full_image: bool = false;
        let mut last_update: u64 = 0;
        while let Some(data) = rx.recv().await {
            match data {
                FramebufferEventInternal::GetImage(in_sender, full_image) => {
                    count += 1;
                    wait_if_needed(&mut last_update, max_fps).await;
                    m_full_image = full_image;
                    if full_image {
                        display.invalidate_and_update().unwrap();
                        img_buffer = None;
                        sender = Some(in_sender);
                        continue;
                    }
                    match img_buffer {
                        Some(img_buffer_event) => {
                            match in_sender.send(img_buffer_event) {
                                Ok(_) => {
                                    trace!("Image sent from buffer");
                                }
                                Err(err) => {
                                    error!("Error sending data from channel. Count: {}", count);
                                }
                            };
                        }
                        None => {
                            sender = Some(in_sender);
                            // display
                            //     .viewport_changed(0, 0, 0, m_width, m_height)
                            //     .unwrap();
                            
                        }
                    }
                    img_buffer = None;
                    debug!("GetImage event received");
                }
                FramebufferEventInternal::FramebufferImage(data) => {
                    if m_width == 0 && m_height == 0 {
                        m_width = data.width;
                        m_height = data.height;
                    }
                    if m_full_image && (data.width != data.original_width || data.height != data.original_height) {
                        img_buffer = None;
                        continue;
                    }
                  if let Some(_) = img_buffer {
                        error!("Image buffer is not empty, overwriting it. Count: {}", count);
                      m_full_image = true;
                      img_buffer = None;
                      display.invalidate_and_update().unwrap();
                      continue;
                    }
                    match sender {
                        Some(sender) => {
                            match sender.send(data.into()) {
                                Ok(_) => {}
                                Err(err) => {
                                    error!("Error sending data from channel. Count: {}", count);
                                    img_buffer = Some(err);
                                }
                            };
                        }
                        None => {
                            error!("Channel is None. Count: {}", count);
                            img_buffer = Some(data.into());
                        }
                    }
                    sender = None;
                }
                FramebufferEventInternal::ChangeResolution(width, height) => {
                    m_width = width;
                    m_height = height;
                    match sender {
                        Some(sender) => {
                            match sender.send(FramebufferEvent::ChangeResolution(width, height)) {
                                Ok(_) => {}
                                Err(err) => {
                                    img_buffer = Some(err);
                                }
                            };
                        }
                        None => {}
                    }
                    sender = None;
                }
            }
            last_update = get_now();
        }
        let _ = stop_sender.send(());
    });
}

fn supported_signals() -> Vec<SignalKind> {
    vec![
        SignalKind::terminate(), // SIGTERM
        SignalKind::interrupt(), // SIGINT (Ctrl+C)
        SignalKind::hangup(),    // SIGHUP
        SignalKind::quit(),      // SIGQUIT
    ]
}

pub async fn handle_all_signals(
    display: Display,
    framebuffer_id: String,
    mut receiver: ReceiverOneshot<()>,
) {
    let mut signals: Vec<Signal> = supported_signals()
        .into_iter()
        .filter_map(|kind| match signal(kind) {
            Ok(stream) => Some(stream),
            Err(e) => {
                eprintln!("Warning: Failed to listen for signal {:?}: {:?}", kind, e);
                None
            }
        })
        .collect();

    let ctrl_c = signal::ctrl_c();

    info!("Waiting for shutdown signals...");

    tokio::select! {
        _ = async {
            for signal in &mut signals {
                signal.recv().await; // Ожидаем сигнал
            }
        } => {
            info!("Received termination signal. Cleaning up...");
        }

        _ = ctrl_c => {
            info!("Received Ctrl+C (SIGINT). Cleaning up...");
        }
        _ = &mut receiver => {
            debug!("Received shutdown signal via channel. Cleaning up...");
        }

    }

    if let Err(err) = display.detach_framebuffer(0, framebuffer_id.as_str()) {
        error!("Error detaching framebuffer: {}", err);
    }

    debug!("Framebuffer detached successfully. Exiting...");
}

async fn wait_if_needed(last_update: &mut u64, max_fps: u32) {
    if max_fps == 0 {
        return;
    }

    let frame_duration = 1000 / (max_fps as u64);
    let now = get_now();
    let elapsed = now.saturating_sub(*last_update);

    if elapsed < frame_duration {
        let wait_time = frame_duration - elapsed;
        sleep(Duration::from_millis(wait_time)).await;
    }

    *last_update = get_now();
}
fn get_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
} 