mod commands;
mod doctor;
mod modules;
mod tui;

use clap::Parser;
use colored::*;

#[derive(Parser)]
#[command(
    name = "penkit",
    about = "(｡•̀ᴗ-)✧ penkit : lazy hacker's swiss knife",
    version = "0.1.0"
)]
struct Cli {
    // Jump straight to a module (recon|web|smb|sqli|shells|cve|hydra|post|crypto|msf)
    #[arg(short, long)]
    module: Option<String>,

    // Target host/IP (pre-fills prompts)
    #[arg(short, long)]
    target: Option<String>,

    // Output file for generated commands
    #[arg(short, long)]
    output: Option<String>,

    // Check which pentest tools are installed
    #[arg(long)]
    doctor: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.doctor {
        let report = doctor::run_check();
        doctor::print_report(&report);
        return Ok(());
    }

    print_banner();

    tui::run(cli.module, cli.target, cli.output).await?;

    Ok(())
}

fn print_banner() {
    println!(
        "{}",
        r#"
 ██████╗ ███████╗███╗   ██╗██╗  ██╗██╗████████╗
 ██╔══██╗██╔════╝████╗  ██║██║ ██╔╝██║╚══██╔══╝
 ██████╔╝█████╗  ██╔██╗ ██║█████╔╝ ██║   ██║   
 ██╔═══╝ ██╔══╝  ██║╚██╗██║██╔═██╗ ██║   ██║   
 ██║     ███████╗██║ ╚████║██║  ██╗██║   ██║   
 ╚═╝     ╚══════╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝   ╚═╝   
    "#
        .bright_magenta()
        .bold()
    );
    println!(
        " {} {}\n",
        "lazy hacker's swiss knife".bright_cyan(),
        "v0.1.0".dimmed()
    );
}
