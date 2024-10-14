use clap::Parser;
use goblin::{error, Object};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: String,
    #[arg(short, long)]
    recursive: bool,
    #[arg(short, long)]
    no_terminal_colors: bool,
    #[arg(short, long)]
    show_path: bool,
    #[arg(short, long)]
    disabled_only: bool,
    #[arg(long)]
    show_all_imports: bool,
    #[arg(long)]
    show_dangerous_imports: bool,
}

// e_lfanew = 0x3c
// dynamic base = 0x14E
const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: u16 = 0x0040;
const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: u16 = 0x0100;
const TERMINAL_COLOR_GREEN: &str = "\x1b[92m";
const TERMINAL_COLOR_RED: &str = "\x1b[91m";
const TERMINAL_COLOR_RESET: &str = "\x1b[0m";

fn parse_file(path: &PathBuf, cli: &Cli) -> error::Result<()> {
    let buffer = fs::read(path)?;

    match Object::parse(&buffer)? {
        Object::PE(pe) => {
            let dll_characteristics = &pe
                .header
                .optional_header
                .unwrap()
                .windows_fields
                .dll_characteristics;

            let has_aslr = (dll_characteristics & IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE) != 0;
            let has_dep = (dll_characteristics & IMAGE_DLLCHARACTERISTICS_NX_COMPAT) != 0;

            if cli.disabled_only && has_aslr && has_dep {
                return Ok(());
            }

            if cli.show_path {
                println!("{:?}", path);
            } else {
                println!("{:?}", path.file_name().unwrap());
            }

            if cli.show_all_imports || cli.show_dangerous_imports {
                for import in &pe.imports {
                    let is_dangerous = import.name.starts_with("ShellExecute")
                        || import.name.starts_with("CreateProcess");

                    if cli.show_dangerous_imports && !is_dangerous {
                        continue;
                    }

                    println!(
                        "    {}{}{} : {}",
                        if !cli.no_terminal_colors && is_dangerous {
                            TERMINAL_COLOR_RED
                        } else {
                            ""
                        },
                        import.name,
                        if !cli.no_terminal_colors && is_dangerous {
                            TERMINAL_COLOR_RESET
                        } else {
                            ""
                        },
                        import.dll
                    );
                }
            }

            println!(
                "    {}{}{} {}{}{} (0x{:X})",
                if !cli.no_terminal_colors {
                    if has_aslr {
                        TERMINAL_COLOR_GREEN
                    } else {
                        TERMINAL_COLOR_RED
                    }
                } else {
                    ""
                },
                if has_aslr {
                    "ASLR enabled"
                } else {
                    "ASLR disabled"
                },
                if !cli.no_terminal_colors {
                    TERMINAL_COLOR_RESET
                } else {
                    ""
                },
                if !cli.no_terminal_colors {
                    if has_dep {
                        TERMINAL_COLOR_GREEN
                    } else {
                        TERMINAL_COLOR_RED
                    }
                } else {
                    ""
                },
                if has_dep {
                    "DEP enabled"
                } else {
                    "DEP disabled"
                },
                if !cli.no_terminal_colors {
                    TERMINAL_COLOR_RESET
                } else {
                    ""
                },
                dll_characteristics
            );
            println!();
        }
        _ => {}
    }

    Ok(())
}

fn parse_dir(path: &Path, cli: &Cli) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Err(oops) = parse_file(&entry.path(), &cli) {
                        if cli.show_path {
                            println!("{:?}", path);
                        } else {
                            println!("{:?}", path.file_name().unwrap());
                        }

                        println!("{:#?}", oops);
                        println!();
                    }
                } else if cli.recursive && path.is_dir() {
                    parse_dir(&path, &cli);
                }
            }
        }
    }
}

fn main() -> error::Result<()> {
    let cli = Cli::parse();

    let path = Path::new(cli.path.as_str());
    if path.is_file() {
        if let Err(oops) = parse_file(&path.to_path_buf(), &cli) {
            if cli.show_path {
                println!("{:?}", path);
            } else {
                println!("{:?}", path.file_name().unwrap());
            }

            println!("{:#?}", oops);
            println!();
        }
    } else if path.is_dir() {
        parse_dir(path, &cli);
    } else {
        println!("Not found: {:#?}", path);
    }

    Ok(())
}
