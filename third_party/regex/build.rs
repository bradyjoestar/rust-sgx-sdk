use std::env;
use std::ffi::OsString;
use std::process::Command;

fn main() {
    let rustc = env::var_os("RUSTC").unwrap_or(OsString::from("rustc"));
    let output = Command::new(&rustc)
        .arg("--version")
        .output()
        .unwrap()
        .stdout;
    let _version = String::from_utf8(output).unwrap();

    // disabled avx2 | ssse3
    // enable_simd_optimizations(&version);
}

#[allow(unused)]
fn enable_simd_optimizations(version: &str) {
    // We don't activate SIMD optimizations these if we've explicitly disabled
    // them. Disabling auto optimizations is intended for use in tests, so that
    // we can reliably test fallback implementations.
    if env::var_os("CARGO_CFG_REGEX_DISABLE_AUTO_OPTIMIZATIONS").is_some() {
        return;
    }
    let parsed = match Version::parse(&version) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("failed to parse `rustc --version`: {}", err);
            return;
        }
    };
    let minimum = Version { major: 1, minor: 27, patch: 0 };
    if version.contains("nightly") || parsed >= minimum {
        println!("cargo:rustc-cfg=regex_runtime_teddy_ssse3");
        println!("cargo:rustc-cfg=regex_runtime_teddy_avx2");
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    #[allow(unused)]
    fn parse(mut s: &str) -> Result<Version, String> {
        if !s.starts_with("rustc ") {
            return Err(format!("unrecognized version string: {}", s));
        }
        s = &s["rustc ".len()..];

        let parts: Vec<&str> = s.split(".").collect();
        if parts.len() < 3 {
            return Err(format!("not enough version parts: {:?}", parts));
        }

        let mut num = String::new();
        for c in parts[0].chars() {
            if !c.is_digit(10) {
                break;
            }
            num.push(c);
        }
        let major = num.parse::<u32>().map_err(|e| e.to_string())?;

        num.clear();
        for c in parts[1].chars() {
            if !c.is_digit(10) {
                break;
            }
            num.push(c);
        }
        let minor = num.parse::<u32>().map_err(|e| e.to_string())?;

        num.clear();
        for c in parts[2].chars() {
            if !c.is_digit(10) {
                break;
            }
            num.push(c);
        }
        let patch = num.parse::<u32>().map_err(|e| e.to_string())?;

        Ok(Version { major, minor, patch })
    }
}
