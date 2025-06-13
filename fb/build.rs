use std::{
    path::Path, process::Command,
    fs::{ OpenOptions, read_to_string },
    io::{ self, Write, ErrorKind }
};

/**
 * Checks the command by simply executing `<command> --help`.
 *
 * Commands that do something by simple execution should not be checked here...
 */
fn check_cmd(cmd: &str) {
    println!("Testing command: {}", cmd);

    match Command::new(cmd)
        .arg("--help")
        .output() {
        Ok(_) => { println!("Command \"{}\" executed successfully!", cmd); }

        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("Not Found: The command {} was not found, please install it.", cmd);
                }

                ErrorKind::PermissionDenied => {
                    eprintln!("Permission Denied: Command {} lacked necessary permission.", cmd);
                }

                _ => { eprintln!("Error: {}", e); }
            }
        }
    }
}

/**
 * Creates rust source from the flatbuffer schema file.
 */
fn build_fbs(fbs_file: &str) {
    let p = format!("fbs/{}.fbs", fbs_file);

    let fbs_path = Path::new(p.as_str());

    if fbs_path.exists() {
        println!("Found fbs path.");

        let target = "./src/generated/";

        match fbs_path.to_str() {
            Some(path) => {
                match Command::new("flatc")
                    .args([ "-r", "-o", target, path ])
                    .output() {
                    Ok(output) => {
                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();

                        let mut content_exists = false;

                        // the content to write to the mod file.
                        let content = format!("\npub mod {}_generated;", fbs_file);

                        let mod_file_path = format!("{}mod.rs", target);

                        // Check if the content already exists in the mod file
                        match read_to_string(mod_file_path.as_str()) {
                            Ok(mod_file_content) => {
                                if mod_file_content.contains(content.as_str()) {
                                    println!("Generated module already exists!");
                                    content_exists = true;
                                }
                            }

                            Err(e) => { eprintln!("{}", e); }
                        }

                        // If the content does not exists, add it to the mod file!
                        if !content_exists {
                            let mod_file_write = OpenOptions::new()
                                .append(true)
                                .open(mod_file_path.as_str());

                            match mod_file_write {
                                Ok(mut mod_file) => {

                                    mod_file
                                        .write(content.as_bytes())
                                        .expect("Could not write to mod file");
                                }

                                Err(e) => { eprintln!("{}", e); }
                            }
                        }
                    }

                    Err(e) => {
                        eprintln!("Failed to build fbs file");

                        match e.kind() {
                            ErrorKind::NotFound => {
                                eprintln!("Not found: 'flatc' does not exist, please install it.");
                            }

                            _ => { }
                        }
                    }
                }
            }

            None => { println!("Check if fbs file {} exists.", p); }
        }
    }
}

fn main() {
    check_cmd("flatc");
    build_fbs("note");
    // build_fbs("retro");
}

