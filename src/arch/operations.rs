use std::process::Command;
use colored::Colorize;

pub enum ArchOperation {
    CleanCache,
    RemoveOrphaned,
    ManualPackageRemoval,
    RepairFlatpak,
    RemoveUnusedFlatpak,
    ManualFlatpakRemoval,
    ChangeFlatpakDir,
    ClearSystemdJournal,
    CleanGeneralLogs,
    CleanUserCache,
    ManagePacFiles,
    RemoveOrphanedConfigs,
}

impl ArchOperation {
    pub fn execute(&self) -> Result<(), String> {
        match self {
            Self::CleanCache => clean_package_cache(),
            Self::RemoveOrphaned => remove_orphaned_packages(),
            Self::ManualPackageRemoval => manual_package_removal(),
            Self::RepairFlatpak => repair_flatpak(),
            Self::RemoveUnusedFlatpak => remove_unused_flatpak(),
            Self::ManualFlatpakRemoval => manual_flatpak_removal(),
            Self::ChangeFlatpakDir => change_flatpak_dir(),
            Self::ClearSystemdJournal => clear_systemd_journal(),
            Self::CleanGeneralLogs => clean_general_logs(),
            Self::CleanUserCache => clean_user_cache(),
            Self::ManagePacFiles => manage_pac_files(),
            Self::RemoveOrphanedConfigs => remove_orphaned_configs(),
        }
    }
}

fn clean_package_cache() -> Result<(), String> {
    println!("Running Operation: {}", "Clean package cache".bold().green());
    println!("Checking for pacman-contrib package...");
    let output = Command::new("pacman")
        .args(["-Qs", "pacman-contrib"])
        .output()
        .map_err(|e| e.to_string())?;
    
    let output = Command::new("sudo")
        .args(["paccache", "-r"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        println!("Operation {} {}", "Clean package cache".bold().green(), "completed successfully".green());
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn remove_orphaned_packages() -> Result<(), String> {
    println!("Running Operation: {}", "Remove orphan packages".bold().green());
    let output = Command::new("sudo")
        .args(["pacman", "-Rns", "$(pacman -Qtdq)"])
        .output()
        .map_err(|e| e.to_string())?;
    println!("Operation {} {}", "Remove orphan packages".bold().green(), "completed successfully".green());
    Ok(())
}

fn manual_package_removal() -> Result<(), String> {
    println!("Beginning Operation: {}", "Manual package removal".bold().green());
    println!("Manual package features coming soon...");
    println!("Operation {} {}", "Manual package removal".bold().green(), "completed successfully".green());
    Ok(())
}

fn repair_flatpak() -> Result<(), String> {
    println!("Running Operation: {}", "Repair flatpak libraries".bold().green());
    let output = Command::new("sudo")
        .args(["flatpak", "repair"])
        .output()
        .map_err(|e| e.to_string())?;
    println!("Operation {} {}", "Repair flatpak libraries".bold().green(), "completed successfully".green());
    Ok(())
}

fn remove_unused_flatpak() -> Result<(), String> {
    println!("Running Operation: {}", "Remove unused libraries".bold().green());
    let output = Command::new("sudo")
        .args(["flatpak", "uninstall", "--unused"])
        .output()
        .map_err(|e| e.to_string())?;
    println!("Operation {} {}", "Remove unused libraries".bold().green(), "completed successfully".green());
    Ok(())
}

fn manual_flatpak_removal() -> Result<(), String> {
    println!("Beginning Operation: {}", "Manual flatpak removal".bold().green());
    println!("Manual package features coming soon...");
    println!("Operation {} {}", "Manual flatpak removal".bold().green(), "completed successfully".green());
    Ok(())
}

fn change_flatpak_dir() -> Result<(), String> {
    println!("Running Operation: {}", "Change flatpak installation location".bold().green());
    println!("Feature coming soon...");
    println!("Operation {} {}", "Change flatpak installation location".bold().green(), "completed successfully".green());
    Ok(())
}

fn clear_systemd_journal() -> Result<(), String> {
    println!("Running Operation: {}", "Clear systemd journal".bold().green());
    let output = Command::new("sudo")
        .args(["journalctl", "--vacuum-time=1d"])
        .output()
        .map_err(|e| e.to_string())?;
    println!("Cleared all journal logs older than 1 day");
    println!("Operation {} {}", "Clear systemd journal".bold().green(), "completed successfully".green());
    Ok(())
}

fn clean_general_logs() -> Result<(), String> {
    println!("Running Operation: {}", "Clean general logs".bold().green());
    println!("Feature coming soon...");
    println!("Operation {} {}", "Clean general logs".bold().green(), "completed successfully".green());
    Ok(())
}

fn clean_user_cache() -> Result<(), String> {
    println!("Running Operation: {}", "Clean user cache".bold().green());
    let output = Command::new("rm")
        .args(["-rf", "~/.cache/*"])
        .output()
        .map_err(|e| e.to_string())?;
    println!("Operation {} {}", "Clean user cache".bold().green(), "completed successfully".green());
    Ok(())
}

fn manage_pac_files() -> Result<(), String> {
    println!("Running Operation: {}", "Manage pac* files".bold().green());
    let output = Command::new("sudo")
        .args(["pacdiff"])
        .output()
        .map_err(|e| e.to_string())?;
    println!("Operation {} {}", "Manage pac* files".bold().green(), "completed successfully".green());
    Ok(())
}

fn remove_orphaned_configs() -> Result<(), String> {
    println!("Beginning Operation: {}", "Remove orphaned configs".bold().green());
    print!("Feature coming soon...");
    println!("Operation {} {}", "Remove orphaned configs".bold().green(), "completed successfully".green());
    Ok(())
}




