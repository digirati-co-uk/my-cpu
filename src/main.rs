use std::error::Error;
use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::ptr::NonNull;
use std::{ffi::CStr, process::Stdio};

use clap::Parser;
use regex::Regex;

#[unsafe(export_name = "__tls_get_addr@@GLIBC_2.3")]
pub fn glibc_tls_stub() { }

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Parser, Debug)]
#[command(term_width = 0)]
struct Args {
    /// The command arguments.
    #[arg(trailing_var_arg(true), allow_hyphen_values(true))]
    command: Vec<OsString>,

    /// Path to the fallback binary to be used if the CPU cannot be detected or there are no matches.
    #[arg(long)]
    fallback: PathBuf,

    /// 1 or more target pairs specified as `-t /path/to/binary:regex`
    #[arg(short = 't', long = "target", value_parser = parse_key_val::<PathBuf, Regex>)]
    targets: Vec<(PathBuf, Regex)>,
}

fn get_entrypoint(cpu: &str, targets: &[(PathBuf, Regex)]) -> Option<PathBuf> {
    for (path, regex) in targets {
        if regex.is_match(&cpu) {
            println!("Matched {regex}");
        }

        return Some(path.clone());
    }

    None
}

fn main() {
    let args = Args::parse();
    let cpu = unsafe {
        NonNull::new(llvm_sys::target_machine::LLVMGetHostCPUName())
            .map(|name| CStr::from_ptr(name.as_ptr()).to_string_lossy())
            .inspect(|v| println!("Detected CPU as {}", v))
    };

    let regex_strings = args
        .targets
        .iter()
        .map(|(_, regex)| regex.as_str().to_owned())
        .collect::<Vec<_>>()
        .join(", ");

    if regex_strings.is_empty() {
        println!("No regular expressions provided");
    } else {
        println!("Matching against {regex_strings}");
    }

    let entrypoint = cpu
        .and_then(|v| get_entrypoint(&v, &args.targets))
        .unwrap_or(args.fallback);

    let _ = std::process::Command::new(entrypoint)
        .args(args.command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .exec();
}
