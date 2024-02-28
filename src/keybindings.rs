#![cfg_attr(rustfmt, rustfmt_skip)]

use windows::Win32::UI::Input::KeyboardAndMouse::{
    VK_F13, VK_F14, VK_F15, VK_F16,
    VK_F17, VK_F18, VK_F19, VK_F20,
    VK_F21, VK_F22, VK_F23, VK_F24,
    VK_LMENU,
};

pub const MARK_LEADER: [u16; 1] = [VK_LMENU];
pub const GOTO_LEADER: [u16; 0] = [];

pub const ASSIGNABLE_KEYS: [u16; 12] = [
    VK_F13, VK_F14, VK_F15, VK_F16,
    VK_F17, VK_F18, VK_F19, VK_F20,
    VK_F21, VK_F22, VK_F23, VK_F24,
    ];
    
const PERSONAL_BROWSER: Option<&str> = Some("chrome");
const PERSONAL_CODE: Option<&str> = Some("code C:/Users/torstein.tenstad/repos/td-deckbuilder");
const WORK_BROWSER: Option<&str> = Some("chrome --profile-directory=Default");
const WORK_NOR16X_PRS: Option<&str> = Some("chrome https://bitbucket.org/norsonicas/nor16x/pull-requests --new-window --profile-directory=Default");
const WORK_MCU_PRS: Option<&str> = Some("chrome https://bitbucket.org/norsonicas/mcu-common/pull-requests --new-window --profile-directory=Default");
const WORK_CODE_NOR16X: Option<&str> = Some("code C:/Users/torstein.tenstad/repos/nor16x");
const WORK_CODE_NOR16X_WSL: Option<&str> = Some("wsl code /home/torstein/repos/nor16x");
const WORK_KEIL: Option<&str> = Some("UV4");

pub const DEFAULT_MARKS: [Option<&str>; 12] = [
    WORK_BROWSER, WORK_NOR16X_PRS,  WORK_MCU_PRS,         PERSONAL_BROWSER,
    WORK_KEIL,    WORK_CODE_NOR16X, WORK_CODE_NOR16X_WSL, PERSONAL_CODE,
    None,         None,             None,                 None,
];

pub fn get_default_mark(key: u16) -> Option<&'static str> {
    ASSIGNABLE_KEYS
                    .iter()
                    .enumerate()
                    .find(|(_, x)| **x == key)
                    .and_then(|(i, _)| DEFAULT_MARKS.get(i)).as_ref().and_then(|x| **x)
}
