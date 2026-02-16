use std::fs;
use std::collections::HashSet;
use std::ffi::OsString;
use std::{thread,time};
use std::process::Command;

// const s

fn main() {

    let mut persistent_dirs: HashSet<OsString> = HashSet::new();
    for dir in get_all_subs() {
        // print!("{dir:?}\n");
        persistent_dirs.insert(dir);
    }

    // let mut hb: Command;
    loop {
        thread::sleep(time::Duration::from_millis(60000));
        for dir in get_all_subs(){
            if !persistent_dirs.contains(&dir){
                print!("Call Handbrake Here\n");
            }
        }
        let _output = Command::new("flatpak").arg("run")
                    .arg("fr.handbrake.gdb")
                    .arg("--device=\"/dev/{dir:?}\"")
                    .arg("--preset=\"main\"")
                    .arg("-c").arg("--auto-start-queue")
                    .output().expect("failed to exectue process");
        // print!("Output: {0:?}\n", output.stdout);   
    }

}

fn get_all_subs() -> Vec<OsString> {
    let mut dirs: Vec<OsString> = Vec::new();

    for entry in fs::read_dir("/dev/").unwrap(){
        dirs.push(entry.unwrap().file_name());
    }
    return dirs;
}
