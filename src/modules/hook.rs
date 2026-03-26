use rdev::{listen, EventType, Key};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};
use crate::modules::config::Config;

pub enum Command {
    Capture,
    ShowVault,
}

pub struct HookState {
    ctrl: bool,
    alt: bool,
    shift: bool,
    pub config: Config,
}

pub struct HookManager {
    tx: mpsc::Sender<Command>,
    state: Arc<Mutex<HookState>>,
}

impl HookManager {
    pub fn new(config: Config, tx: mpsc::Sender<Command>) -> Self {
        Self { 
            tx, 
            state: Arc::new(Mutex::new(HookState { 
                ctrl: false, 
                alt: false, 
                shift: false,
                config,
            }))
        }
    }

    pub fn start(self) {
        let tx = self.tx;
        let state = self.state;

        std::thread::spawn(move || {
            if let Err(error) = listen(move |event| {
                let mut s = match state.lock() {
                    Ok(guard) => guard,
                    Err(poisoned) => poisoned.into_inner(),
                };
                
                match event.event_type {
                    EventType::KeyPress(key) => {
                        match key {
                            Key::ControlLeft | Key::ControlRight => s.ctrl = true,
                            Key::Alt | Key::AltGr => s.alt = true,
                            Key::ShiftLeft | Key::ShiftRight => s.shift = true,
                            k if k == Key::Delete => { // We use Delete as a fixed trigger for now, but title is dynamic
                                if is_target_window(&s.config.target_titles) {
                                    let _ = tx.blocking_send(Command::Capture);
                                }
                            }
                            Key::KeyA => {
                                if s.ctrl && s.alt && s.shift {
                                    let _ = tx.blocking_send(Command::ShowVault);
                                }
                            }
                            _ => {}
                        }
                    }
                    EventType::KeyRelease(key) => {
                        match key {
                            Key::ControlLeft | Key::ControlRight => s.ctrl = false,
                            Key::Alt | Key::AltGr => s.alt = false,
                            Key::ShiftLeft | Key::ShiftRight => s.shift = false,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }) {
                eprintln!("Erreur lors du démarrage du crochet clavier : {:?}", error);
            }
        });
    }
}

fn is_target_window(targets: &[String]) -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 { return false; }

        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(hwnd, &mut text);
        if len == 0 { return false; }

        let title = String::from_utf16_lossy(&text[..len as usize]);
        for target in targets {
            if title.contains(target) {
                return true;
            }
        }
        false
    }
}
