// A.k.a #include  in the C++
extern crate winreg;

// Imports the need features. A.k.a using namespace std in the C++
use std::env::var;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use winreg::enums::*;
use winreg::RegKey;

// Windows Kits registry key
const WIN_KITS_KEY: &str = r"SOFTWARE\Microsoft\Windows Kits\Installed Roots";

// Custom error type
#[derive(Debug)]
struct BuildError {
    msg: String,
}

impl Error for BuildError {}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BuildError: {}", self.msg)
    }
}

// Get the Windows Kits directory from the registry
fn get_windows_kits_dir() -> Result<PathBuf, BuildError> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    match hklm.open_subkey(WIN_KITS_KEY) {
        Ok(key) => match key.get_value::<String, _>("KitsRoot10") {
            Ok(dir) => Ok(PathBuf::from(r#dir)),
            Err(_) => Err(BuildError {
                msg: format!("Can not get value: {}", WIN_KITS_KEY),
            }),
        },
        Err(_) => Err(BuildError {
            msg: format!("Can not open sub_key: {}", WIN_KITS_KEY),
        }),
    }
}

// Get the latest km directory from the Windows Kits directory
fn get_km_dir(windows_kits_dir: &PathBuf) -> Result<PathBuf, BuildError> {
    match Path::new(windows_kits_dir).join("lib").read_dir() {
        Ok(read_dir) => {
            match read_dir
                .filter_map(|dir| dir.ok())
                .map(|dir| dir.path())
                .filter(|dir| {
                    dir.components()
                        .last()
                        .and_then(|c| c.as_os_str().to_str())
                        .map(|c| c.starts_with("10.") && dir.join("km").is_dir())
                        .unwrap_or(false)
                })
                .max()
                .ok_or_else(|| format!("Can not find a valid km dir in `{:?}`", windows_kits_dir))
            {
                Ok(max_lib_dir) => Ok(max_lib_dir.join("km")),
                Err(msg) => Err(BuildError { msg }),
            }
        }
        Err(_) => Err(BuildError {
            msg: format!("Can not read dir: {:?}", windows_kits_dir),
        }),
    }
}

fn main() {
    let windows_kits_dir = get_windows_kits_dir().unwrap();
    let km_dir = get_km_dir(&windows_kits_dir).unwrap();
    let target = var("TARGET").unwrap();
    let arch = if target.contains("x86_64") {
        "x64"
    } else if target.contains("i686") {
        "x86"
    } else {
        panic!("Only support x64 and x86!");
    };
    let wdk_lib_dir = km_dir.join(arch);

    // link
    println!(
        "cargo:rustc-link-search=native={}",
        wdk_lib_dir.to_str().unwrap()
    );
}