use std::fs;
use std::collections::HashSet;
use std::ffi::OsString;
use std::{thread,time};
use std::process::Command;
use std::env;


fn main() {
    let BASE_PATH: String = String::from(env::home_dir().unwrap().display().to_string() + "/Downloads/");
    print!("{}\n", BASE_PATH);
    let mut persistent_dirs: HashSet<OsString> = HashSet::new();
    for dir in get_all_subs(&BASE_PATH) {
        // print!("{dir:?}\n");
        persistent_dirs.insert(dir);
    }

    // let mut hb: Command;
    print!("Enter loop\n");
    loop {
        thread::sleep(time::Duration::from_millis(6000));
        print!("Checking for new files!\n");
        for dir in get_all_subs(&BASE_PATH){
            if !persistent_dirs.contains(&dir){
                let mut path = BASE_PATH.clone();

                path.push_str(&(dir.clone().into_string().unwrap()));

                print!("Calling Handbrake on {path:?} !\n");

                let output = Command::new("flatpak").arg("run")
                            .arg("fr.handbrake.ghb")
                            .arg(format!("--device=\"{path}\""))
                            .arg("-c").arg("--auto-start-queue")
                            .status().expect("failed to exectue process");

                persistent_dirs.insert(dir);
                
                print!("Output: {}\n", output);
            }
        }
    }

}

fn get_all_subs(path: &str) -> Vec<OsString> {

    


    let mut dirs: Vec<OsString> = Vec::new();

    for entry in fs::read_dir(path).unwrap(){
        dirs.push(entry.unwrap().file_name());
    }
    return dirs;
}
