use dialog::DialogBox;
use ksni;
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Sender, TryRecvError};
use std::thread;

// #[derive(Debug)]
struct CaffeineTray {
    activated: bool,
    tx: Sender<i32>,
}

trait Caffeine: ksni::Tray {
    fn notify(&self);
}

impl Caffeine for CaffeineTray {
    fn notify(&self) {
        let mut val = 0;
        if self.activated {
            val = 1;
        }
        self.tx.send(val).unwrap();
    }
}

impl ksni::Tray for CaffeineTray {
    fn icon_name(&self) -> String {
        if self.activated {
            "caffeine-cup-full"
        } else {
            "caffeine-cup-empty"
        }
        .into()
    }
    fn title(&self) -> String {
        "xscreensaver-caffeine-rs".into()
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            CheckmarkItem {
                label: "Activated".into(),
                checked: self.activated,
                activate: Box::new(|this: &mut Self| {
                    this.activated = !this.activated;
                    this.notify();
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Info".into(),
                icon_name: "dialog-information-symbolic".into(),
                activate: Box::new(|_| {
                    dialog::Message::new(
                        "XScreenSaver Caffeine RS 1.0.1\nManually control the desktop's idle state.\nhttps://github.com/darkfrank-it/xscreensaver-caffeine-rs",
                    )
                    .title("XScreenSaver Caffeine RS 1.0.1")
                    .show()
                    .expect("Could not display dialog box")
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
    fn secondary_activate(&mut self, _x: i32, _y: i32) {
        self.activated = !self.activated;

        self.notify();
    }
}

fn main() {
    let (mtx, mrx) = mpsc::channel();

    let service = ksni::TrayService::new(CaffeineTray {
        activated: false,
        tx: mtx,
    });
    // let handle = service.handle();
    service.spawn();

    thread::spawn(move || loop {
        let mut val = mrx.recv().unwrap();

        while val == 1 {
            std::thread::sleep(std::time::Duration::from_secs(50));
            // Prevent execution if screen was manually locked
            // $ xscreensaver-command -time
            // XScreenSaver 5.15: screen locked since Wed Sep 26 16:26:15 2012
            let output = Command::new("xscreensaver-command")
                .arg("-time")
                .stdout(Stdio::piped())
                .output()
                .expect("Time is bleeding!");
            // extract the raw bytes that we captured and interpret them as a string
            let stdout = String::from_utf8(output.stdout).expect("No stdout from output");

            if !stdout.contains(" locked ") {
                Command::new("xscreensaver-command")
                    .arg("-deactivate")
                    .arg("-quiet").output().unwrap();
            }

            match mrx.try_recv() {
                Ok(0) => {
                    val = 0;
                    break;
                }
                Ok(_) | Err(TryRecvError::Empty) => {
                    continue;
                }
                Err(TryRecvError::Disconnected) => {
                    val = 9;
                    break;
                }
            }
        }

        if val == 9 {
            break;
        }
    });

    // Run forever
    loop {
        std::thread::park();
    }
}
