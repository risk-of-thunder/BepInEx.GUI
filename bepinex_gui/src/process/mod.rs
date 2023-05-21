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
    shared::minwindef::DWORD,
    um::tlhelp32::{TH32CS_SNAPTHREAD, THREADENTRY32},
};

#[cfg(windows)]
pub(crate) fn for_each_thread(target_process_id: Pid, callback: impl Fn(HANDLE)) -> bool {
    use sysinfo::PidExt;
    use winapi::um::winnt::THREAD_SUSPEND_RESUME;

    unsafe {
        let sys = sysinfo::System::new_all();

        if let Some(_) = sys.process(target_process_id) {
            let thread_snapshot =
                winapi::um::tlhelp32::CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);

            let mut te32: THREADENTRY32 = Default::default();
            te32.dwSize = size_of::<THREADENTRY32>() as DWORD;
            let te32_ptr = std::mem::transmute(&te32);

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

                    if open_thread_handle == std::ptr::null_mut() {
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

        return false;
    }
}

#[cfg(not(windows))]
pub(crate) fn for_each_thread() {
    // todo
}

#[cfg(windows)]
pub(crate) fn resume(target_process_id: Pid) -> bool {
    for_each_thread(target_process_id, |thread_handle| unsafe {
        winapi::um::processthreadsapi::ResumeThread(thread_handle);
    })
}

#[cfg(not(windows))]
pub(crate) fn resume(target_process_id: Pid) -> bool {
    // todo
}

#[cfg(windows)]
pub(crate) fn suspend(target_process_id: Pid) -> bool {
    for_each_thread(target_process_id, |thread_handle| unsafe {
        winapi::um::processthreadsapi::SuspendThread(thread_handle);
    })
}

#[cfg(not(windows))]
pub(crate) fn suspend(target_process_id: Pid) -> bool {
    // todo
}

pub(crate) fn spawn_thread_is_process_dead(
    mut sys: sysinfo::System,
    target_process_id: Pid,
    should_check: Arc<AtomicBool>,
    out_true_when_process_is_dead: Arc<AtomicBool>,
) {
    thread::spawn(move || -> io::Result<()> {
        loop {
            if !sys.refresh_process(target_process_id) && should_check.load(Ordering::Relaxed) {
                break;
            }

            thread::sleep(time::Duration::from_millis(2000));
        }

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
