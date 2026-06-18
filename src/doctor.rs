use std::process::Command;
use colored::*;

pub struct ToolCheck {
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub installed: bool,
}

fn check(name: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {} >/dev/null 2>&1", name))
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn run_check() -> Vec<ToolCheck> {
    vec![
        // Recon
        ToolCheck { name: "nmap", description: "Network mapper / port scanner", category: "Recon", installed: check("nmap") },
        ToolCheck { name: "masscan", description: "Async TCP port scanner", category: "Recon", installed: check("masscan") },
        ToolCheck { name: "tcpdump", description: "Packet capture analyzer", category: "Recon", installed: check("tcpdump") },
        // Web
        ToolCheck { name: "ffuf", description: "Fast web fuzzer", category: "Web", installed: check("ffuf") },
        ToolCheck { name: "gobuster", description: "Directory/file & DNS busting", category: "Web", installed: check("gobuster") },
        ToolCheck { name: "nikto", description: "Web vulnerability scanner", category: "Web", installed: check("nikto") },
        ToolCheck { name: "whatweb", description: "Web technology fingerprinting", category: "Web", installed: check("whatweb") },
        ToolCheck { name: "nuclei", description: "Fast template-based vulnerability scanner", category: "Web", installed: check("nuclei") },
        ToolCheck { name: "subfinder", description: "Passive subdomain discovery", category: "Web", installed: check("subfinder") },
        ToolCheck { name: "httpx", description: "Fast HTTP prober & tech detector", category: "Web", installed: check("httpx") },
        ToolCheck { name: "curl", description: "URL transfer tool", category: "Web", installed: check("curl") },
        // SMB
        ToolCheck { name: "enum4linux", description: "SMB enumeration (legacy)", category: "SMB", installed: check("enum4linux") },
        ToolCheck { name: "enum4linux-ng", description: "SMB enumeration (modern)", category: "SMB", installed: check("enum4linux-ng") },
        ToolCheck { name: "smbclient", description: "SMB client", category: "SMB", installed: check("smbclient") },
        ToolCheck { name: "crackmapexec", description: "SMB/WinRM/LDAP swiss knife", category: "SMB", installed: check("crackmapexec") },
        ToolCheck { name: "netexec", description: "Successor to crackmapexec", category: "SMB", installed: check("netexec") },
        // SQLi
        ToolCheck { name: "sqlmap", description: "Automated SQL injection", category: "SQLi", installed: check("sqlmap") },
        // Shells
        ToolCheck { name: "nc", description: "Netcat (GNU/OpenBSD)", category: "Shells", installed: check("nc") },
        ToolCheck { name: "socat", description: "Multipurpose relay (socket cat)", category: "Shells", installed: check("socat") },
        ToolCheck { name: "python3", description: "Python (for PTY upgrades)", category: "Shells", installed: check("python3") },
        ToolCheck { name: "msfvenom", description: "Metasploit payload generator", category: "Shells", installed: check("msfvenom") },
        // CVE
        ToolCheck { name: "searchsploit", description: "Exploit-DB local search", category: "CVE", installed: check("searchsploit") },
        ToolCheck { name: "wpscan", description: "WordPress vulnerability scanner", category: "CVE", installed: check("wpscan") },
        // Hydra
        ToolCheck { name: "hydra", description: "THC-Hydra brute-forcer", category: "Hydra", installed: check("hydra") },
        // Crypto
        ToolCheck { name: "hashcat", description: "GPU hash cracker", category: "Crypto", installed: check("hashcat") },
        ToolCheck { name: "john", description: "John the Ripper password cracker", category: "Crypto", installed: check("john") },
        ToolCheck { name: "openssl", description: "SSL/TLS toolkit", category: "Crypto", installed: check("openssl") },
        ToolCheck { name: "crunch", description: "Wordlist generator", category: "Crypto", installed: check("crunch") },
        // Post
        ToolCheck { name: "proxychains", description: "Proxy wrapper for tools", category: "Post", installed: check("proxychains") },
        ToolCheck { name: "proxychains4", description: "Proxychains-ng", category: "Post", installed: check("proxychains4") },
        ToolCheck { name: "chisel", description: "HTTP tunnel / pivoting", category: "Post", installed: check("chisel") },
        ToolCheck { name: "pspy64", description: "Process monitor (no root)", category: "Post", installed: check("pspy64") },
        // Dev
        ToolCheck { name: "go", description: "Go compiler (for building tools)", category: "Dev", installed: check("go") },
    ]
}

pub fn print_report(report: &[ToolCheck]) {
    println!("{}", "\n🩺 penkit doctor — tool availability check\n".bright_magenta().bold());

    let mut current_cat = "";
    for tool in report {
        if tool.category != current_cat {
            current_cat = tool.category;
            println!("\n{}", format!("【 {} 】", current_cat).bright_cyan().bold());
        }

        let status = if tool.installed {
            "✅".green()
        } else {
            "❌".red()
        };
        println!(
            "  {} {:<<18} {}",
            status,
            tool.name.bright_white(),
            tool.description.dimmed()
        );
    }

    let installed = report.iter().filter(|t| t.installed).count();
    let total = report.len();
    println!(
        "\n{}\n",
        format!("Summary: {}/{} tools installed", installed, total).bright_yellow()
    );

    let missing: Vec<_> = report.iter().filter(|t| !t.installed).map(|t| t.name).collect();
    if !missing.is_empty() {
        println!("{}", "Missing tools:".bright_red().bold());
        for name in missing {
            println!("  • {}", name.bright_red());
        }
        println!();
    }
}
