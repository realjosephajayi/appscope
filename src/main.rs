use std::{env, fmt, fs, path::Path, process::Command};

#[derive(Debug, Clone, Copy)]
enum InstallSource {
    Apt,
    Dnf,
    Pacman,
    Flatpak,
    Snap,
    Binary,
}

impl fmt::Display for InstallSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Apt => "apt",
            Self::Dnf => "dnf/rpm",
            Self::Pacman => "pacman",
            Self::Flatpak => "flatpak",
            Self::Snap => "snap",
            Self::Binary => "binary",
        };
        write!(f, "{label}")
    }
}

fn command_exists(name: &str) -> bool {
    env::var_os("PATH")
        .map(|paths| env::split_paths(&paths).any(|path| path.join(name).is_file()))
        .unwrap_or(false)
}

fn command_output(command: &str, args: &[&str]) -> Option<String> {
    Command::new(command)
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn find_package(query: &str) -> Option<(InstallSource, String, String)> {
    let query_lower = query.to_lowercase();
    if command_exists("flatpak") {
        if let Some(lines) = command_output(
            "flatpak",
            &["list", "--app", "--columns=application,name,version"],
        ) {
            for line in lines.lines() {
                if line.to_lowercase().contains(&query_lower) {
                    let id = line.split_whitespace().next().unwrap_or(line);
                    return Some((
                        InstallSource::Flatpak,
                        line.to_owned(),
                        format!("flatpak uninstall {id}"),
                    ));
                }
            }
        }
    }
    if command_exists("snap") {
        if let Some(lines) = command_output("snap", &["list"]) {
            for line in lines.lines().skip(1) {
                if line.to_lowercase().contains(&query_lower) {
                    let package = line.split_whitespace().next().unwrap_or(line);
                    return Some((
                        InstallSource::Snap,
                        line.to_owned(),
                        format!("sudo snap remove {package}"),
                    ));
                }
            }
        }
    }
    for (manager, source, args) in [
        ("dpkg", InstallSource::Apt, vec!["-l"]),
        ("rpm", InstallSource::Dnf, vec!["-qa"]),
        ("pacman", InstallSource::Pacman, vec!["-Qq"]),
    ] {
        if command_exists(manager) {
            if let Some(lines) = command_output(manager, &args) {
                if let Some(line) = lines
                    .lines()
                    .find(|line| line.to_lowercase().contains(&query_lower))
                {
                    let package = line.split_whitespace().last().unwrap_or(line);
                    let removal = match source {
                        InstallSource::Apt => format!("sudo apt remove {package}"),
                        InstallSource::Dnf => format!("sudo dnf remove {package}"),
                        InstallSource::Pacman => format!("sudo pacman -Rns {package}"),
                        _ => unreachable!(),
                    };
                    return Some((source, package.to_owned(), removal));
                }
            }
        }
    }
    for directory in [
        "/usr/bin",
        "/usr/local/bin",
        "/opt",
        "/var/lib/flatpak/exports/bin",
    ] {
        if Path::new(directory).exists() {
            if let Ok(entries) = fs::read_dir(directory) {
                if let Some(entry) = entries.flatten().find(|entry| {
                    entry
                        .file_name()
                        .to_string_lossy()
                        .to_lowercase()
                        .contains(&query_lower)
                }) {
                    let path = entry.path().display().to_string();
                    return Some((
                        InstallSource::Binary,
                        entry.file_name().to_string_lossy().into_owned(),
                        format!("Inspect manually: {path}"),
                    ));
                }
            }
        }
    }
    None
}

fn print_help() {
    println!("appscope - inspect Linux applications safely\n\nUSAGE:\n    appscope scan <application>...\n    appscope help\n\nEXAMPLES:\n    appscope scan cursor\n    appscope scan \"Claude Desktop\" mt5\n\nThe scan never changes your system. Review any suggested command before running it.");
}

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("scan") => {
            let queries: Vec<_> = args.collect();
            if queries.is_empty() {
                eprintln!("error: provide at least one application name");
                std::process::exit(2);
            }
            for query in queries {
                match find_package(&query) {
                    Some((source, name, removal)) => println!("FOUND\n  Query: {query}\n  Match: {name}\n  Source: {source}\n  Suggested removal: {removal}\n"),
                    None => println!("NOT FOUND\n  Query: {query}\n  No matching package or common binary was detected.\n"),
                }
            }
        }
        _ => print_help(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_labels_are_stable() {
        assert_eq!(InstallSource::Flatpak.to_string(), "flatpak");
        assert_eq!(InstallSource::Apt.to_string(), "apt");
    }
}

// Supports apt, dnf/rpm, pacman, Flatpak, Snap, and binary installs.
