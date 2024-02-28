use crate::keybindings;
use std::sync::mpsc::Sender;
use std::thread;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

#[derive(Debug)]
pub enum Event {
    Mark(u16),
    Goto(u16),
}

pub fn start(sender: Sender<Event>) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        let goto_leaders_down = keybindings::GOTO_LEADER.iter().all(|&key| {
            (unsafe { GetAsyncKeyState(key.into()) } & 0b1000000000000000u16 as i16) != 0
        });
        let mark_leaders_down = keybindings::MARK_LEADER.iter().all(|&key| {
            (unsafe { GetAsyncKeyState(key.into()) } & 0b1000000000000000u16 as i16) != 0
        });

        if mark_leaders_down {
            for key in keybindings::ASSIGNABLE_KEYS {
                if (unsafe { GetAsyncKeyState(key.into()) } & 0b01) != 0 {
                    sender.send(Event::Mark(key)).unwrap();
                }
            }
        } else if goto_leaders_down {
            for key in keybindings::ASSIGNABLE_KEYS {
                if (unsafe { GetAsyncKeyState(key.into()) } & 0b01) != 0 {
                    sender.send(Event::Goto(key)).unwrap();
                }
            }
        }
        thread::sleep(std::time::Duration::from_millis(10));
    })
}
