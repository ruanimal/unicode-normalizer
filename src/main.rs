use std::fs;
use std::io::Write;
use clap::Parser;
use walkdir;

extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;


const FORMS:[&str; 4] = ["NFC", "NFD", "NFKC", "NFKD"];

#[derive(Parser, Debug)]
#[command(name = "Unicode File Normalizer")]
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

fn rename_one(path: &String, log_fd: &mut fs::File, form: &String, dry_run: bool) {
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
        let msg = format!("SUCC\t{} -> {}", src.display(), dst.display());
        if dry_run {
            println!("dry_run:{}", msg);
            continue;
        }
        match fs::rename(&src, &dst) {
            Ok(_) => {
                println!("{}", msg);
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
    let mut fd = fs::File::create(&args.log).unwrap();
    for p in &args.path {
        rename_one(&p, &mut fd, &args.to_form, args.dry_run);
    }
    fd.sync_all().unwrap();
}

fn main() {
    let args: Args = Args::parse();
    rename(&args);
}
