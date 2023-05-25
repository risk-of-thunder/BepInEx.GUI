use core::time;
use std::thread;
use sysinfo::Pid;
use winapi::um::winuser::{SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE};
use winapi::{
    shared::{
        minwindef::{BOOL, DWORD, LPARAM},
        windef::HWND,
    },
    um::{
        processthreadsapi::GetCurrentProcessId,
        winuser::{EnumWindows, GetWindowThreadProcessId, HWND_NOTOPMOST},
    },
};

#[cfg(windows)]
pub fn init(target_process_id: Pid) {
    use std::io;

    thread::spawn(move || -> io::Result<()> {
        loop {
            if is_current_process_in_front_of_target_process_window(target_process_id) {
                break;
            }

            thread::sleep(time::Duration::from_millis(500));
        }

        thread::spawn(|| -> io::Result<()> {
            thread::sleep(time::Duration::from_millis(500));

            set_topmost_current_process_window(false);

            Ok(())
        });

        Ok(())
    });
}

#[cfg(not(windows))]
pub fn init(&self, target_process_id: Pid) {}

#[cfg(windows)]
fn is_current_process_in_front_of_target_process_window(target_process_id_: Pid) -> bool {
    unsafe {
        static mut CURRENT_PROCESS_ID: u32 = 0;
        CURRENT_PROCESS_ID = GetCurrentProcessId() as u32;

        static mut TARGET_PROCESS_ID: u32 = 0;
        TARGET_PROCESS_ID = std::mem::transmute_copy(&target_process_id_);

        static mut GOT_CURRENT_PROC_WINDOW: bool = false;
        GOT_CURRENT_PROC_WINDOW = false;
        static mut GOT_TARGET_PROC_WINDOW: bool = false;
        GOT_TARGET_PROC_WINDOW = false;
        static mut GOT_RESULT: bool = false;

        GOT_RESULT = false;
        extern "system" fn enum_window(window: HWND, _: LPARAM) -> BOOL {
            unsafe {
                if GOT_RESULT {
                    return true.into();
                }

                let mut proc_id: DWORD = 0 as DWORD;
                let _ = GetWindowThreadProcessId(window, &mut proc_id as *mut DWORD);
                if proc_id == TARGET_PROCESS_ID {
                    let is_current_proc_in_front = GOT_CURRENT_PROC_WINDOW;
                    if is_current_proc_in_front {
                        GOT_RESULT = true;
                    } else {
                        GOT_TARGET_PROC_WINDOW = true;
                    }
                } else if proc_id == CURRENT_PROCESS_ID {
                    let is_target_proc_in_front = GOT_TARGET_PROC_WINDOW;
                    if is_target_proc_in_front {
                        set_topmost_current_process_window(true);
                        tracing::info!("Put bep gui window in front");
                        GOT_RESULT = true;
                    } else {
                        GOT_CURRENT_PROC_WINDOW = true;
                    }
                }

                true.into()
            }
        }

        EnumWindows(Some(enum_window), 0 as LPARAM);

        return GOT_CURRENT_PROC_WINDOW && GOT_RESULT;
    }
}
#[cfg(not(windows))]
fn is_current_process_in_front_of_target_process_window(&self, target_process_id: Pid) {}

fn set_topmost_current_process_window(set_topmost: bool) {
    unsafe {
        static mut CURRENT_PROCESS_ID: u32 = 0;
        CURRENT_PROCESS_ID = GetCurrentProcessId() as u32;

        static mut SET_TOPMOST: bool = false;
        SET_TOPMOST = set_topmost;

        extern "system" fn enum_window(window: HWND, _: LPARAM) -> BOOL {
            unsafe {
                let mut proc_id: DWORD = 0 as DWORD;
                let _ = GetWindowThreadProcessId(window, &mut proc_id as *mut DWORD);
                if proc_id == CURRENT_PROCESS_ID {
                    SetWindowPos(
                        window,
                        if SET_TOPMOST {
                            HWND_TOPMOST
                        } else {
                            HWND_NOTOPMOST
                        },
                        0,
                        0,
                        0,
                        0,
                        SWP_NOMOVE | SWP_NOSIZE,
                    );
                }

                true.into()
            }
        }

        EnumWindows(Some(enum_window), 0 as LPARAM);
    }
}
