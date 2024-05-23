use std::convert::TryInto;
use memmap::Mmap;
use memmap::MmapOptions;
use std::convert::TryFrom;
use std::fs::File;
use structopt::StructOpt;
use colored::Colorize;

mod requirements;
use requirements::Opt;
use requirements::Requirements;

fn map(skip: usize, len: usize) -> Result<Mmap, std::io::Error> {
    let mmap = unsafe {MmapOptions::new().offset(skip).len(len).map(&File::open("/dev/mem")?)? };
    Ok(mmap)
}

fn main() -> Result<(), String> {
    let reqs_res = Requirements::try_from(Opt::from_args());
    if let Err(_) = reqs_res {
        return Err(String::from("Failed to parse command line args"));
    };
    let reqs = reqs_res.unwrap();

    let map_res = map(reqs.addr, reqs.size);
    if let Err(_) = map_res {
        return Err(String::from("mmap failed"));
    }
    let mmap = map_res.unwrap();

    let row_count = ((reqs.size / 4) + (reqs.per_row - 1)) / reqs.per_row;

    for ii in 0..row_count {
        let row_start = reqs.addr + ii * 4 * reqs.per_row;
        let row_len = if (reqs.addr + reqs.size) - row_start >= 4 * reqs.per_row {
            reqs.per_row            
        } else {
            (reqs.size % reqs.per_row) / 4
        };

        print!("0x{:08x}:", row_start);
        for jj in 0..row_len {
            let start = ii * reqs.per_row + jj * 4;
            let entry = u32::from_ne_bytes(mmap[start..start + 4].try_into().unwrap());
            match entry {
                0xffff_ffff_u32 => { print!("{}", format!(" 0x{:08x}", entry).normal()); },
                0xdead_beef_u32 => { print!("{}", format!(" 0x{:08x}", entry).normal()); },
                _ => { print!("{}", format!(" 0x{:08x}", entry).red()); }
            }
        }
        println!();
    }

    Ok(())
}
