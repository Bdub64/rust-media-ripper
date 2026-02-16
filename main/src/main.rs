use std::fs;
use std::collections::HashSet;
use std::ffi::OsString;

fn main() {

    let mut persistent_dirs: HashSet<OsString> = HashSet::new();
    for dir in get_all_subs() {
        print!("{dir:?}");
        persistent_dirs.insert(dir);
    }

    // while(true){

    // }

}

fn get_all_subs() -> Vec<OsString> {
    let mut dirs: Vec<OsString> = Vec::new();

    for entry in fs::read_dir("/dev/").unwrap(){


        
        dirs.push(entry.unwrap().file_name());
    }
    return dirs;
}
