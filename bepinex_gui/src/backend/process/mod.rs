use core::time;
use std::io;
use std::mem::size_of;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;

use sysinfo::Pid;
use sysinfo::ProcessExt;
use sysinfo::SystemExt;
use winapi::um::winnt::HANDLE;
use winapi::{
    shared::{
        minwindef::{BOOL, DWORD, LPARAM},
        windef::HWND,
    },
    um::{
        processthreadsapi::GetCurrentProcessId,
        tlhelp32::{TH32CS_SNAPTHREAD, THREADENTRY32},
        winuser::{EnumWindows, GetWindowThreadProcessId, IsHungAppWindow},
    },
};

#[cfg(windows)]
pub fn for_each_thread(target_process_id: Pid, callback: impl Fn(HANDLE)) -> bool {
    use sysinfo::PidExt;
    use winapi::um::winnt::THREAD_SUSPEND_RESUME;

    unsafe {
        let sys = sysinfo::System::new_all();

        if sys.process(target_process_id).is_some() {
            let thread_snapshot =
                winapi::um::tlhelp32::CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);

            let mut te32: THREADENTRY32 = THREADENTRY32 {
                dwSize: size_of::<THREADENTRY32>() as DWORD,
                ..Default::default()
            };
            let te32_ptr = std::ptr::addr_of_mut!(te32);

            if winapi::um::tlhelp32::Thread32First(thread_snapshot, te32_ptr) == 0 {
                tracing::error!("Thread32First fail");
                winapi::um::handleapi::CloseHandle(thread_snapshot);
            }

            loop {
                if te32.th32OwnerProcessID == target_process_id.as_u32() {
                    let open_thread_handle = winapi::um::processthreadsapi::OpenThread(
                        THREAD_SUSPEND_RESUME,
                        false as i32,
                        te32.th32ThreadID,
                    );

                    if open_thread_handle.is_null() {
                        tracing::error!("OpenThread Failed");
                        break;
                    }

                    callback(open_thread_handle);

                    winapi::um::handleapi::CloseHandle(open_thread_handle);
                }

                if winapi::um::tlhelp32::Thread32Next(thread_snapshot, te32_ptr) == 0 {
                    break;
                }
            }

            winapi::um::handleapi::CloseHandle(thread_snapshot);

            return true;
        }

        false
    }
}

#[cfg(not(windows))]
pub fn for_each_thread() {
    // todo
}

#[cfg(windows)]
pub fn resume(target_process_id: Pid) -> bool {
    for_each_thread(target_process_id, |thread_handle| unsafe {
        winapi::um::processthreadsapi::ResumeThread(thread_handle);
    })
}

#[cfg(not(windows))]
pub fn resume(target_process_id: Pid) -> bool {
    // todo
}

#[cfg(windows)]
pub fn suspend(target_process_id: Pid) -> bool {
    for_each_thread(target_process_id, |thread_handle| unsafe {
        winapi::um::processthreadsapi::SuspendThread(thread_handle);
    })
}

#[cfg(not(windows))]
pub fn suspend(target_process_id: Pid) -> bool {
    // todo
}

pub fn spawn_thread_is_process_dead(
    target_process_id: Pid,
    should_check: Arc<AtomicBool>,
    out_true_when_process_is_dead: Arc<AtomicBool>,
) {
    thread::spawn(move || -> io::Result<()> {
        let mut sys = sysinfo::System::new_all();
        loop {
            if !sys.refresh_process(target_process_id) && should_check.load(Ordering::Relaxed) {
                break;
            }

            thread::sleep(time::Duration::from_millis(2000));
        }

        tracing::info!("Target process is dead, setting out_true_when_process_is_dead");
        out_true_when_process_is_dead.store(true, Ordering::Relaxed);

        Ok(())
    });
}

pub fn kill(target_process_id: Pid, callback: impl FnOnce()) {
    let sys = sysinfo::System::new_all();
    if let Some(proc) = sys.process(target_process_id) {
        proc.kill();
        callback();
    }
}

#[cfg(windows)]
pub fn spawn_thread_check_if_process_is_hung(callback: impl Fn() + std::marker::Send + 'static) {
    thread::spawn(move || -> io::Result<()> {
        unsafe {
            static mut CURRENT_PROCESS_ID: u32 = 0;
            CURRENT_PROCESS_ID = GetCurrentProcessId();

            static mut GOT_RESULT: bool = false;

            static mut WINDOW_HANDLE: HWND = 0 as _;

            GOT_RESULT = false;
            extern "system" fn enum_window(window: HWND, _: LPARAM) -> BOOL {
                unsafe {
                    if GOT_RESULT {
                        return true.into();
                    }

                    let mut proc_id: DWORD = 0 as DWORD;
                    let _ = GetWindowThreadProcessId(window, std::ptr::addr_of_mut!(proc_id));
                    if proc_id == CURRENT_PROCESS_ID {
                        WINDOW_HANDLE = window;
                    }

                    true.into()
                }
            }

            EnumWindows(Some(enum_window), 0 as LPARAM);

            let mut i = 0;
            loop {
                if IsHungAppWindow(WINDOW_HANDLE) == 1 {
                    if i == 3 {
                        callback();
                        tracing::info!("callback called!");
                        return Ok(());
                    }

                    i += 1;
                }

                thread::sleep(time::Duration::from_millis(1000));
            }
        }
    });
}

#[cfg(not(windows))]
pub(crate) fn spawn_thread_check_if_process_is_hung(
    target_process_id: Pid,
    should_check: Arc<AtomicBool>,
    out_true_when_process_is_dead: Arc<AtomicBool>,
) {
}
