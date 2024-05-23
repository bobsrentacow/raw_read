use std::convert::TryFrom;
use std::fmt;
use structopt::StructOpt;

//----
// Command Line Parsing

fn parse_autobase(src: &str) -> Result<usize, String> {
    if src.starts_with("0x") {
        match usize::from_str_radix(&src[2..], 16) {
            Ok(val) => return Ok(val),
            Err(_) => return Err(format!("Invalid integer string: {}", src))
        }
    } else if src.starts_with("0") {
        match usize::from_str_radix(&src[1..], 8) {
            Ok(val) => return Ok(val),
            Err(_) => return Err(format!("Invalid integer string: {}", src))
        }
    } else {
        match usize::from_str_radix(src, 10) {
            Ok(val) => return Ok(val),
            Err(_) => return Err(format!("Invalid integer string: {}", src))
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "raw_read",
    about = "Display hex dump of physical memory"
)]
pub struct Opt {
    /// Start address in bytes
    #[structopt(name = "start_addr", parse(try_from_str = parse_autobase))]
    pub start_addr_bytes: usize,

    /// Read size in bytes
    #[structopt(name = "size_bytes", parse(try_from_str = parse_autobase))]
    pub size_bytes: usize,

    /// Count of 32b values per row out displayed output
    #[structopt(name = "per_row", parse(try_from_str = parse_autobase))]
    pub per_row: usize,
}

//----
// Requirements
//   This should fully specify the problem we are trying to solve.

pub struct Requirements {
    pub addr: usize,
    pub size: usize,
    pub per_row: usize,
}

impl TryFrom<Opt> for Requirements {
    type Error = &'static str;

    fn try_from(opt: Opt) -> Result<Self, Self::Error> {
        if (opt.start_addr_bytes & 3) != 0{
            return Err("start_addr_bytes must be 32b aligned");
        }
        if (opt.size_bytes & 3) != 0{
            return Err("size_bytes must be 32b aligned");
        }
        if opt.per_row == 0 {
            return Err("per_row must be nonzero");
        }

        Ok(Requirements {
            addr: opt.start_addr_bytes,
            size: opt.size_bytes,
            per_row: opt.per_row,
        })
    }
}

impl fmt::Display for Requirements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "addr   : {}", self.addr)?;
        writeln!(f, "addr   : {}", self.size)?;
        writeln!(f, "addr   : {}", self.per_row)?;
        Ok(())
    }
}

//----
// Test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_requirements_from_opt_valid() -> Result<(), String> {
        let opt = Opt {
            start_addr_bytes: 0xfedc_ba98_7654_3210_usize,
            size_bytes: 0x0000_0000_0000_cdf0_usize,
            per_row: 16_usize,
        };

        if let Ok(_) = Requirements::try_from(opt) {
            Ok(())
        } else {
            Err(String::from("Requirements::try_from(opt) Should have succeeded"))
        }
    }

    #[test]
    fn test_requirements_from_opt_per_row_zero() -> Result<(), String> {
        let opt = Opt {
            start_addr_bytes: 0xfedc_ba98_7654_3210_usize,
            size_bytes: 0x0000_0000_0000_cdf0_usize,
            per_row: 0_usize,
        };

        if let Ok(_) = Requirements::try_from(opt) {
            Err(String::from("Requirements::try_from(opt) Should have failed"))
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_requirements_from_opt_misaligned_addr() -> Result<(), String> {
        let opt = Opt {
            start_addr_bytes: 0xfedc_ba98_7654_3211_usize,
            size_bytes: 0x0000_0000_0000_cdf0_usize,
            per_row: 16_usize,
        };

        if let Ok(_) = Requirements::try_from(opt) {
            Err(String::from("Requirements::try_from(opt) Should have failed"))
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_requirements_from_opt_misaligned_size() -> Result<(), String> {
        let opt = Opt {
            start_addr_bytes: 0xfedc_ba98_7654_3210_usize,
            size_bytes: 0x0000_0000_0000_cdf1_usize,
            per_row: 16_usize,
        };

        if let Ok(_) = Requirements::try_from(opt) {
            Err(String::from("Requirements::try_from(opt) Should have failed"))
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_requirements_from_opt_misaligned_addr_and_size() -> Result<(), String> {
        let opt = Opt {
            start_addr_bytes: 0xfedc_ba98_7654_3211_usize,
            size_bytes: 0x0000_0000_0000_cdf3_usize,
            per_row: 16_usize,
        };

        if let Ok(_) = Requirements::try_from(opt) {
            Err(String::from("Requirements::try_from(opt) Should have failed"))
        } else {
            Ok(())
        }
    }
}
