use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM, PWSTR},
    UI::{
        Input::KeyboardAndMouse::{keybd_event, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, VK_MENU},
        WindowsAndMessaging::{
            EnumWindows, GetForegroundWindow, GetWindowTextW, IsIconic, IsWindowVisible,
            SetForegroundWindow, ShowWindow, SW_NORMAL,
        },
    },
};

pub fn list_visible() -> Vec<HWND> {
    let mut visible_windows: Vec<HWND> = Vec::new();
    unsafe {
        EnumWindows(
            Some(enum_window_callback),
            LPARAM(&mut visible_windows as *mut Vec<HWND> as isize),
        )
        .unwrap();
    }
    visible_windows
}

pub fn exists(hwnd: HWND) -> bool {
    unsafe { IsWindowVisible(hwnd).as_bool() }
}

pub fn set_foreground(hwnd: HWND) {
    unsafe {
        if (IsIconic(hwnd)).as_bool() {
            ShowWindow(hwnd, SW_NORMAL);
        } else {
            let bvk = VK_MENU.try_into().unwrap();
            keybd_event(bvk, 0x45, KEYEVENTF_EXTENDEDKEY, 0);
            keybd_event(bvk, 0x45, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
            SetForegroundWindow(hwnd);
        }
    }
}

pub fn name(hwnd: HWND) -> String {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(hwnd, PWSTR(text.as_mut_ptr()), text.len() as i32);
        String::from_utf16_lossy(&text[..len as usize]).to_string()
    }
}

pub fn current() -> HWND {
    unsafe { GetForegroundWindow() }
}

unsafe extern "system" fn enum_window_callback(window: HWND, l_param: LPARAM) -> BOOL {
    let visible_windows = &mut *(l_param.0 as *mut Vec<HWND>);

    let mut text: [u16; 512] = [0; 512];
    let len = GetWindowTextW(window, PWSTR(text.as_mut_ptr()), text.len() as i32);
    let text = String::from_utf16_lossy(&text[..len as usize]);
    let visible = IsWindowVisible(window).as_bool();

    if !text.is_empty() && visible {
        visible_windows.push(window);
    }

    true.into()
}
