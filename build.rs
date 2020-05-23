use {std::{process, fs, error, env, path, thread}, fs_extra::dir};

fn main() -> Result<(), Box<dyn error::Error>> {
    if let Ok(parent) = env::var("BSWAM_PARENT").as_ref().map(|s|&s[..]) {
        let p = path::PathBuf::from(format!("/proc/{}/status", parent));
        while p.exists() {
            #[allow(deprecated)] thread::sleep_ms(100);
        }

        if path::Path::new("bswam/target").exists() {
            fs::remove_dir_all("target")?;
            dir::copy("bswam/target", ".", &dir::CopyOptions::new())?;
        }

        Ok(())
    } else {
        process::Command::new("touch").arg("build.rs").status()?;
        let status = fs::read_to_string("/proc/self/status")?;
        let parent = &status.lines().skip(6).next().ok_or("rip")?[6..];
        let cmdline = fs::read_to_string(format!("/proc/{}/cmdline", parent))?
            .replace('\0', " ");

        if path::Path::new("target").exists() && !path::Path::new("bswam/target").exists() {
            dir::copy("target", "bswam", &dir::CopyOptions::new())?;
        }

        let output = process::Command::new("nix-shell")
            .args(&["-p", "pkgconfig", "openssl", "--run", &cmdline])
            .current_dir("bswam")
            .stdin(fs::File::open(format!("/proc/{}/fd/0", parent))?)
            .stdout(fs::File::create(format!("/proc/{}/fd/1", parent))?)
            .stderr(fs::File::create(format!("/proc/{}/fd/2", parent))?)
            .output()?;

        println!("cargo:rustc-env=BSWAM_STATUS={}", output.status.code().unwrap_or(-1));

        process::Command::new(env::args().next().ok_or("ugh")?)
            .env("BSWAM_PARENT", &parent)
            .stdin(fs::File::open(format!("/proc/{}/fd/0", parent))?)
            .stdout(fs::File::create(format!("/proc/{}/fd/1", parent))?)
            .stderr(fs::File::create(format!("/proc/{}/fd/2", parent))?)
            .spawn()?;
        Ok(())
    }
}
