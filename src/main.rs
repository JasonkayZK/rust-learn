use memmap2::MmapOptions;
use object::{File, Object, ObjectSection};
use std::error::Error;
use std::fs::OpenOptions;
use std::{env, fs, ptr};

#[link_section = "run_count"]
#[used]
static mut RUN_COUNT: u32 = 0;

fn main() -> Result<(), Box<dyn Error>> {
    let run_count = unsafe { ptr::read_volatile(&RUN_COUNT) };
    println!("Previous run count: {}", run_count);

    change_run_count(run_count)
}

/// Change run count of the file
fn change_run_count(run_count: u32) -> Result<(), Box<dyn Error>> {
    // Find current executable file path
    let exe_path = env::current_exe()?;

    // New temp File
    let exe_temp_path = exe_path.with_extension("tmp");

    // Copy value to temp file
    let _ = fs::copy(&exe_path, &exe_temp_path)?;

    // Open temp file
    let temp_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&exe_temp_path)?;
    // Mmap temp file to memory
    let mut buf = unsafe { MmapOptions::new().map_mut(&temp_file) }?;
    // Parse file
    let temp_file = File::parse(&*buf)?;

    if let Some((offset, d_byte_size)) = get_file_section(&temp_file, "run_count") {
        // Section Found
        // Ensure is "RUN_COUNT"
        assert_eq!(d_byte_size, 4);

        // Overwrite the temp executable file
        let base = offset as usize;
        buf[base..(base + d_byte_size as usize)].copy_from_slice(&(run_count + 1).to_ne_bytes());

        // Change permissions for temp executable file
        let perms = fs::metadata(&exe_path)?.permissions();
        fs::set_permissions(&exe_temp_path, perms)?;

        // Override old file!
        fs::rename(&exe_temp_path, &exe_path)?;
    } else {
        // Section not found, remove temp executable file
        fs::remove_file(&exe_temp_path)?;
    }

    Ok(())
}

/// Find the specific section in executable file
///
/// Returns the (offset, byte-size[size of on-disk segment]) of the executable file
fn get_file_section(file: &File, sec_name: &str) -> Option<(u64, u64)> {
    for section in file.sections() {
        match section.name() {
            Ok(name) if name == sec_name => {
                return section.file_range();
            }
            _ => {}
        }
    }

    None
}
