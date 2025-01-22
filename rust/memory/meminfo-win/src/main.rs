use kernel32;
use winapi;

use winapi::{
    DWORD, HANDLE, LPSYSTEM_INFO, LPVOID, MEMORY_BASIC_INFORMATION as MEM_INFO, PVOID, SIZE_T,
    SYSTEM_INFO,
};

fn main() {
    let this_pid: DWORD;
    let this_proc: HANDLE;
    let min_addr: LPVOID;
    let max_addr: LPVOID;
    let mut base_addr: PVOID;
    let mut proc_info: SYSTEM_INFO;
    let mut mem_info: MEM_INFO;

    const MEM_INFO_SIZE: usize = size_of::<MEM_INFO>();

    unsafe {
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    unsafe {
        this_pid = kernel32::GetCurrentProcessId();
        this_proc = kernel32::GetCurrentProcess();
        kernel32::GetSystemInfo(&mut proc_info as LPSYSTEM_INFO);
    };

    min_addr = proc_info.lpMinimumApplicationAddress;
    max_addr = proc_info.lpMaximumApplicationAddress;

    println!("{:?} @ {:p}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min: {:p}, max: {:p}", min_addr, max_addr);

    loop {
        let rc: SIZE_T = unsafe {
            kernel32::VirtualQueryEx(this_proc, base_addr, &mut mem_info, MEM_INFO_SIZE as SIZE_T)
        };

        if rc == 0 {
            break;
        }

        println!("{:#?}", mem_info);
        base_addr = ((base_addr as u64) + mem_info.RegionSize) as PVOID;
    }
}
