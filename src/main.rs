use std::process::Command;
use std::collections::HashMap;
use getopts::Options;
use std::env;
use std::path::{Path, PathBuf};
use glob::glob;
use std::fs::File;
use std::io::Write;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn copy_files(src: PathBuf, dst: &str) -> String{

    // ignore files
    let ignore_files = [
        ".git",
        ".gitignore",
        "README.md",
    ];

    let filename = src.file_name().unwrap().to_str().expect("REASON").to_string();

    // dst内に既にsrcと同名のファイルが存在する場合は確認する。
    let dst_path = Path::new(dst).join(&filename);
    if dst_path.exists() {
        println!("{} is already exists. Do you want to overwrite it? [y/N]", filename);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() != "y" {
            return String::from("");
        }
    }

    for ignore_file in ignore_files.iter() {
        if filename == ignore_file.to_string() {
            return String::from("");
        }
    }

    Command::new("cp")
        .args(["-r", src.to_str().unwrap(), dst])
        .output()
        .expect("failed to start `cp`");

    return filename;

    //println!("{}", String::from_utf8_lossy(&copy_output.stdout));
}

fn main() {
    let cwd = env::current_dir().unwrap();

    // 環境リスト
    let mut env_list = HashMap::new();
    env_list.insert(String::from("python"), "https://github.com/Yokohide0317/my-python-env.git");
    env_list.insert(String::from("r"), "https://github.com/Yokohide0317/my-r-env.git");

    // 引数の解析
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    //opts.optopt("o", "option", "Language", "p");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // 環境の選択
    let lang= args[1].clone();

    // langがenv_list内に存在するか確認。
    if !env_list.contains_key(&lang) {
        println!("'{}' is not supported.", lang);
        println!("Supported languages are below.");
        for (key, _) in &env_list {
            println!("- {}", key);
        }
        return;
    }

    println!("Selected: {}", lang);
    println!("Cloneing URL: {}", env_list.get(&lang).unwrap());

    // git clone
    let clone_path = Path::new(&cwd).join("tmpEnv");
    let clone_output = Command::new("git")
        .args(["clone", env_list.get(&lang).unwrap(), clone_path.to_str().unwrap()])
        .output()
        .expect("failed to start `git clone`");

    println!("{}", String::from_utf8_lossy(&clone_output.stdout));

    // コピーしたリポジトリ情報を書く
    let envinfo_path = Path::new(&cwd).join("MyEnvInfo.md");
    let mut file = File::create(envinfo_path).unwrap();
    file.write_all(String::from("# MyEnvInfo\n").as_bytes()).unwrap();
    file.write_all(format!("{}\n\n", env_list.get(&lang).unwrap().to_string()).as_bytes()).unwrap();
    file.write_all(String::from("## Files\n").as_bytes()).unwrap();


    // 中身をコピー
    let copy_contents = Path::new(&clone_path).join("*");
    for entry in glob(copy_contents.to_str().unwrap()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // MyEnvInfo.mdに記入。
                let filename = copy_files(path, cwd.to_str().unwrap());
                if filename != "" {
                    file.write_all(format!("- {}\n", filename).as_bytes()).unwrap();
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }


    // cloneしたディレクトリを削除
    let rm_output = Command::new("rm")
        .args(["-r", clone_path.to_str().unwrap()])
        .output()
        .expect("failed to remove tmpEnv");
    println!("{}", String::from_utf8_lossy(&rm_output.stdout));

}
