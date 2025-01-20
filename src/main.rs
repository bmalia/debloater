use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use colored::Colorize;

mod arch;
// mod debian;
// mod fedora;
mod operation_descriptions;

fn get_distribution() -> Option<String> {
    println!("{}", "Detecting distribution...".bold());
    let file = File::open("/etc/os-release").ok()?; // opens /etc/release
    let reader = BufReader::new(file);
    
    for line in reader.lines() { // Searches each line of the file for the 'ID=' tag
        if let Ok(line) = line {
            if (line.starts_with("ID=")) {
                return Some(line.replace("ID=", "").replace("\"", ""));
            }
        }
    }
    None
}

fn handle_debian() {
    println!("Handling Debian-based distribution");
    /* let mut tui = debian::tui::DebTui::new();
    if let Err(e) = tui.run() {
        eprintln!("Error running Debian TUI: {}", e); 
    } */
}

fn handle_arch() {
    let mut tui = arch::tui::ArchTui::new();
    if let Err(e) = tui.run() {
        eprintln!("Error running Arch TUI: {}", e);
    }
}

fn handle_fedora() {
    println!("Handling Fedora-based distribution");
    // fedora::init();
}

fn main() {
    let distro = get_distribution().unwrap_or_else(|| String::from("unknown")); // Grabs the distribution ID
    
    match distro.as_str() {
        "debian" | "ubuntu" | "linuxmint" | "pop" => handle_debian(), // Calls debian function for these distros
        "arch" | "manjaro" | "endeavouros" => handle_arch(), // Calls arch function for these distros
        "fedora" => handle_fedora(), // Calls fedora function for this distro TODO: do research and potentially add more distros here (eg. nobara)
        _ => {
            println!("Unsupported distribution: {}", distro);
            println!("If you know what your distribution is based on, enter it now:");
            let mut distro = String::new();
            io::stdin().read_line(&mut distro).expect("Failed to read line");
            match distro.trim() {
                "debian" | "ubuntu" | "linuxmint" | "pop" => handle_debian(), // Calls debian function for these distros
                "arch" | "manjaro" | "endeavouros" => handle_arch(), // Calls arch function for these distros
                "fedora" => handle_fedora(), // Calls fedora function for this distro TODO: do research and potentially add more distros here (eg. nobara)
                _ => println!("Unsupported distribution: {}", distro),
            }
        }
    }
}
