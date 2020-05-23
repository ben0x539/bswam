use {std::{process, fs, error, env, path, thread}, fs_extra::dir};

fn main() -> Result<(), Box<dyn error::Error>> {
    let inner = toml::from_str::<
                toml::map::Map<String, toml::Value>
            >(&fs::read_to_string("Cargo.toml")?)?
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .map(|n| path::PathBuf::from(n))
        .ok_or_else(|| "welp")?;
    let inner_target = inner.join("target");

    if let Ok(parent) = env::var("BSWAM_PARENT").as_ref().map(|s|&s[..]) {
        let p = path::PathBuf::from(format!("/proc/{}/status", parent));
        while p.exists() {
            #[allow(deprecated)] thread::sleep_ms(100);
        }

        if inner_target.exists() {
            fs::remove_dir_all("target")?;
            dir::copy(&inner_target, ".", &dir::CopyOptions::new())?;
        }

        Ok(())
    } else {
        process::Command::new("touch").arg("build.rs").status()?;
        let status = fs::read_to_string("/proc/self/status")?;
        let parent = &status.lines().skip(6).next().ok_or("rip")?[6..];
        let cmdline = fs::read_to_string(format!("/proc/{}/cmdline", parent))?
            .replace('\0', " ");

        if path::Path::new("target").exists() && !inner_target.exists() {
            dir::copy("target", &inner, &dir::CopyOptions::new())?;
        }

        let mut args = vec!["--run".into(), cmdline, "-p".into()];
        let cfg: toml::map::Map<String, toml::Value> =
            toml::from_str(
                &fs::read_to_string(inner.join("Cargo.toml"))?)?;
        if let Some(toml::Value::Table(deps)) = cfg.get("sys-dependencies") {
            for (k, v) in deps {
                if v != &toml::Value::Boolean(false) {
                    args.push(k.clone());
                }
            }
        }

        let output = process::Command::new("nix-shell")
            .args(&args)
            .current_dir(&inner)
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

