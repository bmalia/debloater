pub struct OperationDescription {
    pub title: &'static str,
    pub description: &'static str,
}

pub fn get_description(operation_name: &str) -> OperationDescription {
    match operation_name {
        "Clean cache" => OperationDescription {
            title: "Clean Package Cache",
            description: "By default, pacman keeps all packages ever installed on the system in a cache. This is useful for downgrading problematic packages, but can take up a lot of space. This operation removes all but the most recent three versions of each installed package from the cache.",
        },
        "Remove orphaned packages" => OperationDescription {
            title: "Remove Orphaned Packages",
            description: "Your system may have packages that were installed as dependencies for other packages, but are no longer needed. This operation removes these orphaned packages from the system.",
        },
        "Repair libraries" => OperationDescription {
            title: "Repair Flatpak Libraries",
            description: "Due to its sandboxed nature, Flatpak applications can sometimes have issues with shared libraries. This operation repairs the libraries used by Flatpak applications, trimming down on disk usage.",
        },
        "Remove unused libraries" => OperationDescription {
            title: "Remove Unused Flatpak Libraries",
            description: "Removes Flatpak runtimes and extensions that are no longer used by any installed applications.",
        },
        "Clear systemd journal" => OperationDescription {
            title: "Clear Systemd Journal",
            description: "Systemd, the system responsible for low-level system maintainence, keeps logs of system events in a journal. While useful for troubleshooting, these logs can take up a lot of space. This operation removes any log older than 2 days.",
        },
        "Clean user cache" => OperationDescription {
            title: "Clean User Cache",
            description: "Applications store cache data in your home folder, which quickly builds up, but are not required. This operation clears out ~/.cache, saving space.",
        },
        "pac* file management" => OperationDescription {
            title: "Manage Pacnew/Pacsave Files",
            description: "Helps manage .pacnew and .pacsave configuration files that were created during package updates.",
        },
        _ => OperationDescription {
            title: "No Description Available",
            description: "This operation has no detailed description available yet.",
        },
    }
} 
