use std::fs;
use std::io::Write;
use walkdir;

extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;


pub fn normalize(form: &String, s: String) -> String {
    match form.as_str() {
        "NFC" => match unicode_normalization::is_nfc(s.as_str()) {
            true => s,
            false => s.nfc().collect::<String>()
        },
        "NFD" => match unicode_normalization::is_nfd(s.as_str()) {
            true => s,
            false => s.nfd().collect::<String>()
        },
        "NFKC" => match unicode_normalization::is_nfkc(s.as_str()) {
            true => s,
            false => s.nfkc().collect::<String>()
        },
        "NFKD" => match unicode_normalization::is_nfkd(s.as_str()) {
            true => s,
            false => s.nfkd().collect::<String>()
        },
        _ => panic!("Wrong form `{}`", form)
    }
}

pub fn rename_one(path: &String, log_fd: &mut fs::File, form: &String, dry_run: bool, today: &String) {
    for entry in walkdir::WalkDir::new(path).contents_first(true) {
        let entry = match entry {
            Ok(i) => i,
            Err(i) => {println!("WARN\t{:?}", i); continue;}
        };
        let filename = match entry.file_name().to_str() {
            Some(i) => i.to_string(),
            _ => {println!("SKIP\t{}", entry.path().display()); continue;}
        }; entry.file_name();

        let src = entry.path();
        let new_filename = normalize(form, filename.clone());
        if filename == new_filename {
            continue;
        }
        let dst = entry.path().parent().unwrap().join(new_filename);
        let msg = format!("{} -> {}", src.display(), dst.display());
        if dry_run {
            println!("DRY_RUN\t{}", msg);
            continue;
        }
        match fs::rename(&src, &dst) {
            Ok(_) => {
                println!("SUCC\t{}", msg);
                log_fd.write_all(format!("[{}]\t", today).as_bytes()).unwrap();
                log_fd.write_all(msg.as_bytes()).unwrap();
                log_fd.write_all(b"\n").unwrap();
                log_fd.sync_data().unwrap();
            },
            Err(i) => println!("FAIL\t{}\t{}", src.display(), i),
        };
    }
}
