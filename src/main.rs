use std::fs;
use std::io::Write;
use clap::Parser;
use walkdir;
use chrono::{Local};

extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;


const FORMS:[&str; 4] = ["NFC", "NFD", "NFKC", "NFKD"];

#[derive(Parser, Debug)]
#[command(name = "Unicode Filename Normalizer")]
#[command(author = "ruanlj <ruanlj@live.com>")]
#[command(about = format!("Unicode normalize filenames in folder to form one of [{}]", FORMS.join(", ")))]
#[command(long_about = None)]
struct Args {
    /// Normalize form
    #[arg(short, long, value_parser = FORMS)]
    to_form: String,

    /// Path to be convert
    path: Vec<String>,

    /// Log file path
    #[arg(short, long, default_value = "convert.log")]
    log: String,

    /// Dry run convert
    #[arg(long)]
    dry_run: bool,
}

fn normalize(form: &String, s: String) -> String {
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

fn rename_one(path: &String, log_fd: &mut fs::File, form: &String, dry_run: bool, today: &String) {
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

fn rename(args: &Args) {
    println!("Normalizing to {}, Paths: {:?}", args.to_form, args.path);
    let mut fd = fs::File::options().write(true).create(true).append(true).open(&args.log).unwrap();
    let today = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    for p in &args.path {
        rename_one(&p, &mut fd, &args.to_form, args.dry_run, &today);
    }
    fd.sync_all().unwrap();
}

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use super::*;
    use std::fs::File;
    // use std::io::{self, Write};
    use tempdir::TempDir;

    #[test]
    fn test_normalize() {
        let nfc = String::from("\u{3076}");
        let nfd = String::from("\u{3075}\u{3099}");
        let nfkc = String::from("\u{00c5}");
        let nfkd = String::from("A\u{030a}");
        println!("{:#} {:#} {:#} {:#}", nfc, nfd, nfkc, nfkd);
        assert_eq!(normalize(&String::from("NFD"), nfc.clone()), nfd.clone());
        assert_eq!(normalize(&String::from("NFC"), nfd.clone()), nfc.clone());
        assert_eq!(normalize(&String::from("NFKD"), nfkc.clone()), nfkd.clone());
        assert_eq!(normalize(&String::from("NFKC"), nfkd.clone()), nfkc.clone());
    }

    #[test]
    fn test_rename_one() {
        let tmp_dir = TempDir::new("unicode_normalizer").unwrap();
        let fs_root = tmp_dir.path().join("files");
        let log_file = tmp_dir.path().join("convert.log");
        fs::create_dir_all(fs_root.join("b")).unwrap();
        fs::create_dir_all(fs_root.join("a/aa")).unwrap();
        println!("test-dir: {:?}", fs_root);

        let nfc = String::from("1-\u{3076}.txt");
        let nfd = String::from("2-\u{3075}\u{3099}.txt");
        File::create(fs_root.join("a").join(&nfc)).unwrap();
        File::create(fs_root.join("b").join(&nfd)).unwrap();
        File::create(fs_root.join("a/aa").join(&nfc)).unwrap();
        File::create(fs_root.join("a/aa").join(&nfd)).unwrap();

        let mut log_fd = fs::File::options().write(true).create(true).append(true).open(&log_file).unwrap();
        let form = String::from("NFC");
        let today = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let path = fs_root.as_path().display().to_string();
        rename_one(&path, &mut log_fd, &form, false, &today);
        log_fd.sync_all().unwrap();

        let expected1 = format!("[{}]\t{} -> {}", today,
            fs_root.join("a/aa").join(&nfd).as_path().display(),
            fs_root.join("a/aa").join(&normalize(&form, nfd.clone())).as_path().display(),
        );
        let expected2 = format!("[{}]\t{} -> {}", today,
            fs_root.join("b").join(&nfd).as_path().display(),
            fs_root.join("b").join(&normalize(&form, nfd.clone())).as_path().display(),
        );
        let out_a = format!("{}\n{}\n", expected1, expected2);
        let out_b = fs::read_to_string(&log_file).unwrap();
        assert!(out_a.eq(&out_b));
        tmp_dir.close().unwrap();
    }
}

fn main() {
    let args: Args = Args::parse();
    rename(&args);
}
