use clap::Parser;
use set_ico::IcoReg;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    author,
    about,
    disable_version_flag = true,
    after_help(r#"set_ico -e json -i D:\json.ico -a D:\notepad++.exe"#),
    help_template = "\
{name} {version}
{author}
{about-with-newline}
\x1B[32m{usage-heading}\x1B[0m {usage}

\x1B[32m{all-args}\x1B[0m
{after-help}


"
)]
struct Cli {
    /// file extension e.g. json
    #[arg(short = 'e', long, long_help)]
    extension: String,
    /// ico file absolute path
    #[arg(short = 'i', long)]
    ico: PathBuf,
    /// app absolute path
    #[arg(short = 'a', long)]
    app: PathBuf,
    /// add `-r` will write in hkcr.  default write in hkcu
    #[arg(short = 'r', long, default_value = "false")]
    root: bool,
}

fn main() {
    let cli = Cli::parse();
    if !(cli.app.is_file() & cli.app.is_absolute() & cli.ico.is_file() & cli.ico.is_absolute()) {
        panic!("Please enter a valid absolute path")
    }
    let icoreg = IcoReg::new(
        cli.extension,
        cli.ico.to_string_lossy().into_owned(),
        cli.app.to_string_lossy().into_owned(),
        cli.root,
        cli.app.file_name().unwrap().to_str().unwrap().to_string(),
    );
    icoreg.set_file_extision_type();
    icoreg.set_file_extision();
    icoreg.set_default_open_app();
}
