use inotify::{Inotify, WatchMask};
use signal_hook::consts::signal::*;
use signal_hook::flag as signal_flag;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use systemd::daemon::booted;
use systemd::daemon::{self, STATE_READY, STATE_STOPPING};

fn main() {
    // check if systemd is running
    booted().expect("Systemd is not running");

    // Notify systemd that the service has started
    daemon::notify(true, [(STATE_READY, "1")].iter()).unwrap();

    // Create a flag to track termination requests
    let term = Arc::new(AtomicUsize::new(0));
    // Convert signal numbers to usize
    const SIGTERM_U: usize = SIGTERM as usize;
    const SIGINT_U: usize = SIGINT as usize;
    const SIGQUIT_U: usize = SIGQUIT as usize;
    // Register signal handlers for SIGTERM, SIGINT, and SIGQUIT
    signal_flag::register_usize(SIGTERM, Arc::clone(&term), SIGTERM_U).unwrap();
    signal_flag::register_usize(SIGINT, Arc::clone(&term), SIGINT_U).unwrap();
    signal_flag::register_usize(SIGQUIT, Arc::clone(&term), SIGQUIT_U).unwrap();

    // Initialize inotify
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    // creating buffer
    let mut buffer = [0u8; 1024];

    // Add watch for the directory and monitor modification events
    let mask = WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE;
    inotify
        .watches()
        .add("/home/rohan/Downloads", mask)
        .expect("Failed to add inotify watch");
    inotify
        .watches()
        .add("/home/rohan/Downloads/OSX-KVM", mask)
        .expect("Failed to add inotify watch");
    loop {
        match term.load(Ordering::Relaxed) {
            0 => {
                match inotify.read_events(&mut buffer) {
                    Ok(events) => {
                        for event in events {
                            // Handle file system events
                            println!(
                                "File modified: {:?} {:?} {:?} {:?}",
                                event.name, event.cookie, event.mask, event.wd
                            );

                            // write to a file the name and wd
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // No events available, continue
                    }
                    Err(e) => {
                        // An unexpected error occurred
                        panic!("Failed to read inotify events: {:?}", e);
                    }
                }
                // Sleep for a short duration to avoid busy-waiting
                std::thread::sleep(Duration::from_millis(100));
            }
            SIGTERM_U => {
                // Notify systemd that the service is stopping
                println!("SIGTERM");
                daemon::notify(false, [(STATE_STOPPING, "1")].iter()).unwrap();
                break;
            }

            SIGINT_U => {
                // Notify systemd that the service is stopping
                println!("SIGINT");
                daemon::notify(false, [(STATE_STOPPING, "1")].iter()).unwrap();
                break;
            }

            SIGQUIT_U => {
                // Notify systemd that the service is stopping
                println!("SIGQUIT");
                daemon::notify(false, [(STATE_STOPPING, "1")].iter()).unwrap();
                break;
            }

            _ => println!("default"),
        }
    }

    // Notify systemd that the service is stopping
    //daemon::notify(false, &[daemon::NotifyState::Stopping]).unwrap();
}
