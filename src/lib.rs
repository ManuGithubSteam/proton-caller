#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

/*!
# Proton Caller

This is the library which defines the Proton Caller API.

View [bin/proton-call.rs](bin/proton-call.rs) for more information about the `proton-call` executable.
*/

use std::io::{Error, ErrorKind, Result};

/// This is the latest version of Proton at the time of modification.
pub static PROTON_LATEST: &str = "6.3";

/// Main struct for PC, manages and contains all needed information to properly execute Proton.
pub struct Proton {
    steam: String,
    proton: String,
    executable: String,
    passed_args: Vec<String>,
    data: String,
    log: bool,
}

impl Proton {
    /// Creates a new Proton structure, with a Config and Args struct.
    pub fn new<T: ProtonConfig + ProtonArgs>(config: &T, args: &T) -> Proton {
        let common: String = config.get_common();

        let steam: String = config.get_steam();
        let data: String = config.get_data();
        let proton: String = args.get_proton(&common);
        let executable: String = args.get_executable();
        let passed_args: Vec<String> = args.get_extra_args();
        let log: bool = args.get_log();

        Proton {
            steam,
            proton,
            executable,
            passed_args,
            data,
            log,
        }
    }

    /// Checks the paths to Proton and the Executable to make sure they exist.
    ///
    /// # Errors
    ///
    /// Will fail if either requested Proton or executable don't exist.
    pub fn check(&self) -> Result<()> {
        use std::path::Path;

        if !Path::new(&self.proton).exists() {
            error!(ErrorKind::NotFound, "{} not found!", self.proton)?;
        }

        if !Path::new(&self.executable).exists() {
            error!(ErrorKind::NotFound, "{} not found!", self.executable)?;
        }

        Ok(())
    }

    /// Executes Proton using information in the struct, drops `self` at the end.
    ///
    /// # Errors
    ///
    /// * Will fail if spawning Proton fails, view Proton's output for information about fixing this.
    /// * Will fail if waiting for Proton child fails. Nothing can be done for this.
    pub fn run(self) -> Result<()> {
        use std::process::{Child, Command};

        let log = if self.log { "1" } else { "0" };

        println!("\n________Proton________");

        let mut child: Child = Command::new(self.proton)
            .arg("run")
            .arg(self.executable)
            .args(self.passed_args)
            .env("PROTON_LOG", log)
            .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", self.steam)
            .env("STEAM_COMPAT_DATA_PATH", self.data)
            .spawn()?;

        let exitcode = child.wait()?;
        println!("______________________\n");

        if !exitcode.success() {
            error!(ErrorKind::Other, "Proton exited with error")?;
        }

        Ok(())
    }
}

/// Trait used in the Proton struct to get information from the Config.
pub trait ProtonConfig {
    /// Return `steam` directory where Steam is installed.
    fn get_steam(&self) -> String;

    /// Return `common` directory where Proton versions are installed to.
    fn get_common(&self) -> String;

    /// Return `data` directory use during Proton's runtime.
    fn get_data(&self) -> String;
}

/// Trait used in the Proton struct to get argument information from Command line.
pub trait ProtonArgs {
    /// Return full path to the requested proton executable.
    fn get_proton(&self, common: &str) -> String;

    /// Return path to the Windows executable to run.
    fn get_executable(&self) -> String;

    /// Return any extra arguments to pass to the Windows executable, or empty vector if none.
    fn get_extra_args(&self) -> Vec<String>;

    /// Return value to use for the `PROTON_LOG` environment variable.
    fn get_log(&self) -> bool;
}

/// Macro to run the `error_here` function.
#[macro_export]
macro_rules! error {
    ($ek:expr, $fmt:expr) => { crate::error_here($ek, $fmt) };
    ($ek:expr, $fmt:literal, $($arg:expr),*) => { crate::error_here($ek, format!($fmt, $($arg),*)) }
}

/// Quick and dirt way to cause an error in a function.
///
/// # Errors
///
/// This function is mean to fail, mean to cause other functions to fail when needed.
pub fn error_here<R, T: ToString>(kind: ErrorKind, info: T) -> Result<R> {
    let error: String = info.to_string();
    drop(info);
    Err(Error::new(kind, error))
}
