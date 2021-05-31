extern crate clap;
extern crate command_macros;

use std::{
    error::Error as StdError,
    result::Result as StdResult,
};

use clap::{
    Clap,
    AppSettings
};
use command_macros::cmd;

#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "Daniel W. <daniel.wanner@pm.me>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct RunArgs {
    #[clap(subcommand)]
    pub subcmd: SubCommand
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(version = "0.1.0", about = "Builds the container image")]
    #[clap(setting = AppSettings::ColoredHelp)]
    BuildImage(BuildImageArgs),
    #[clap(version = "0.1.0", about = "Starts aloxd on the local machine")]
    #[clap(setting = AppSettings::ColoredHelp)]
    LocalUp
}

#[derive(Clap, Debug)]
struct BuildImageArgs {
    #[clap(short, long, default_value = "dev", about = "The tag to use")]
    pub tag: String,
    #[clap(long, about = "enable release mode")]
    pub release: bool
}

type Result<T> = StdResult<T, Box<dyn StdError>>;


trait ToString {
    fn to_string(self) -> String;
}

impl ToString for Vec<u8> {
    fn to_string(self) -> String {
        unsafe {
            let mut ret = String::from_utf8_unchecked(self);
            ret = String::from(ret.trim());
            ret
        }
    }
}

fn main() -> Result<()> {
    let run_args = RunArgs::parse();

    match run_args.subcmd {
        SubCommand::BuildImage(args) => subcmd_build_image(args)?,
        SubCommand::LocalUp => subcmd_local_up()?
    };

    Ok(())
}

fn subcmd_local_up() -> Result<()> {
    cmd!(cargo run ("--bin") aloxd).status()?;
    Ok(())
}

fn subcmd_build_image(args: BuildImageArgs) -> Result<()> {
    let release = args.release;
    let mode = if release {
        "release"
    } else {
        "debug"
    };
    let tag = if args.tag == "dev" {
        format!("dev-{}", mode)
    } else {
        args.tag
    };

    println!("Building project in {} mode...", mode);

    build_project(release)?;

    println!("Building runner image with tag \"{}\" in {} mode...", tag, mode);

    let id = cmd!(buildah from ("frolvlad/alpine-glibc")).output()?
        .stdout.to_string();

    println!("Working container id: {}", id);

    let target_dir = format!("target/{}", mode);
    let aloxd_path = format!("{}/aloxd", target_dir);
    let aloxctl_path = format!("{}/aloxctl", target_dir);
    cmd!(buildah copy (&id) (&aloxd_path) ("/bin/")).status()?;
    cmd!(buildah copy (&id) (&aloxctl_path) ("/bin/")).status()?;

    cmd!(buildah config ("-v")("/etc/alox") ("-v")("/var/alox") (&id)).status()?;
    cmd!(buildah config ("--cmd") ("/bin/aloxd") (&id)).status()?;
    cmd!(buildah config ("--stop-signal") ("SIGINT") (&id)).status()?;
    cmd!(buildah config ("-p")("80") ("-p")("443") (&id)).status()?;

    let tagged_name = format!("alox:{}", tag);
    cmd!(buildah commit (&id) (&tagged_name)).status()?;
    cmd!(buildah rm (&id)).output()?;

    println!("Finished building container image.");
    Ok(())
}

fn build_project(release: bool) -> Result<()> {
    if release {
        cmd!(cargo build ("-p") alox ("--release")).status()?;
    } else {
        cmd!(cargo build ("-p") alox).status()?;
    }
    Ok(())
}