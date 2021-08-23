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
    pub fn new(
        steam: String,
        proton: String,
        executable: String,
        passed_args: Vec<String>,
        data: String,
        log: bool,
    ) -> Proton {
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

pub struct ProtonBuilder {
    steam: Option<String>,
    proton: Option<String>,
    executable: Option<String>,
    passed_args: Option<Vec<String>>,
    data: Option<String>,
    log: Option<bool>,
}

impl ProtonBuilder {
    pub fn new() -> Self {
        Self {
            steam: None,
            proton: None,
            executable: None,
            passed_args: None,
            data: None,
            log: None
        }
    }

    pub fn steam(mut self, steam: String) -> Self {
        self.steam = Some(steam);
        self
    }

    pub fn proton(mut self, proton: String) -> Self {
        self.proton = Some(proton);
        self
    }

    pub fn executable(mut self, exe: String) -> Self {
        self.executable = Some(exe);
        self
    }

    pub fn args(mut self, args: Vec<String>) -> Self {
        self.passed_args = Some(args);
        self
    }

    pub fn data(mut self, data: String) -> Self {
        self.data = Some(data);
        self
    }

    pub fn log(mut self, log: bool) -> Self {
        self.log = Some(log);
        self
    }

    pub fn build(self) -> Result<Proton> {
        macro_rules! error {
            ($val:literal) => (Error::new(ErrorKind::Other, format!("{} value is missing", $val)));
        }

        Ok(
            Proton {
                steam: self.steam.ok_or(error!("steam"))?,
                proton: self.proton.ok_or(error!("proton"))?,
                executable: self.executable.ok_or(error!("executable"))?,
                passed_args: self.passed_args.ok_or(error!("passed_args"))?,
                data: self.data.ok_or(error!("data"))?,
                log: self.log.ok_or(error!("log"))?,
            }
        )
    }
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
