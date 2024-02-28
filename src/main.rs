use std::collections::HashMap;

use windows::Win32::Foundation::HWND;

mod event_listener;
mod exec_cmd;
mod keybindings;
pub mod window;

fn main() -> windows::core::Result<()> {
    let (sender, receiver) = std::sync::mpsc::channel();
    let thread_handle = event_listener::start(sender);

    let mut marked: HashMap<u16, HWND> = HashMap::new();

    while let Ok(event) = receiver.recv() {
        match event {
            event_listener::Event::Mark(key) => {
                let hwnd = window::current();
                marked.insert(key, hwnd);
                println!("Mark {} to {}", window::name(hwnd), key)
            }
            event_listener::Event::Goto(key) => {
                if let Some(&hwnd) = marked.get(&key) {
                    if window::exists(hwnd) {
                        window::set_foreground(hwnd);
                        println!("Goto {}", window::name(hwnd));
                        continue;
                    }
                }
                if let Some(default_mark) = keybindings::get_default_mark(key)
                {
                    match exec_cmd::exec_cmd(default_mark) {
                        Ok(_) => {
                            std::thread::sleep(std::time::Duration::from_millis(1000));
                            let hwnd = window::current();
                            marked.insert(key, hwnd);
                            println!("Mark {} to {}", window::name(hwnd), key)
                        }
                        Err(e) => {
                            println!("Error executing default mark {}: {}", default_mark, e)
                        }
                    };
                } else {
                    println!("Nothing marked to {}", key)
                }
            }
        }
    }

    thread_handle.join().unwrap();

    Ok(())
}
