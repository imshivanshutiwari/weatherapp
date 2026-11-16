use std::{
    env,
    net::TcpStream,
    path::{Path, PathBuf},
    process::{Child, Command},
    thread,
    time::Duration,
};

fn is_server_running() -> bool {
    TcpStream::connect("127.0.0.1:8080").is_ok()
}

fn find_app_browser() -> Option<PathBuf> {
    let candidates = [
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
    ];

    for path in &candidates {
        let p = PathBuf::from(path);
        if p.exists() {
            return Some(p);
        }
    }
    None
}

fn find_rust_server() -> Option<PathBuf> {
    let current_exe = env::current_exe().unwrap_or_default();
    let parent = current_exe.parent().unwrap_or_else(|| Path::new("."));

    let candidates = [
        parent.join("rust_server.exe"),
        parent.join("bin").join("rust_server.exe"),
        PathBuf::from(r"C:\Users\shiva\Windy_App\bin\rust_server.exe"),
        PathBuf::from(r"C:\Users\shiva\Windy_App\rust_server\target\release\rust_server.exe"),
    ];

    for p in &candidates {
        if p.exists() {
            return Some(p.clone());
        }
    }
    None
}

fn main() {
    println!("=======================================================");
    println!("  Bhartiya Marut Kendra (भारतीय मारुत केंद्र)");
    println!("  Native Desktop Application Launcher");
    println!("=======================================================");

    let mut _server_proc: Option<Child> = None;

    // 1. Ensure backend Rust server is running
    if !is_server_running() {
        println!("[Launcher] Starting Bhartiya Marut Kendra Ultra-Fast Rust Server...");
        if let Some(server_exe) = find_rust_server() {
            match Command::new(&server_exe).spawn() {
                Ok(child) => {
                    _server_proc = Some(child);
                    println!("[Launcher] Rust server process spawned: {:?}", server_exe);
                }
                Err(e) => {
                    eprintln!("[Launcher] Error spawning rust server: {}", e);
                }
            }
        } else {
            eprintln!("[Launcher] Could not find rust_server.exe!");
        }

        // Wait up to 3 seconds for server to accept TCP connections
        for _ in 0..30 {
            thread::sleep(Duration::from_millis(100));
            if is_server_running() {
                break;
            }
        }
    }

    if is_server_running() {
        println!("[Launcher] Backend server verified running on http://127.0.0.1:8080/");
    } else {
        eprintln!("[Launcher] Warning: server not responding on port 8080 yet.");
    }

    // 2. Launch in dedicated standalone desktop application window
    if let Some(browser_exe) = find_app_browser() {
        println!("[Launcher] Launching native window via: {:?}", browser_exe);
        let status = Command::new(browser_exe)
            .args([
                "--app=http://localhost:8080/",
                "--window-size=1440,900",
                "--disable-features=TranslateUI",
            ])
            .status();

        match status {
            Ok(s) => println!("[Launcher] Desktop session closed with status: {}", s),
            Err(e) => eprintln!("[Launcher] Error launching desktop window: {}", e),
        }
    } else {
        eprintln!("[Launcher] No supported browser found to render desktop window.");
    }
}
