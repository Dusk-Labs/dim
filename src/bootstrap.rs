use slog::error;
use slog::info;
use slog::Logger;
use std::error::Error;
use std::fs::File;
use std::fs::Permissions;
use std::io::Cursor;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use tar::Archive;
use xz2::read::XzDecoder;
use zip::read::ZipArchive;

pub fn bootstrap(log: Logger) {
    std::fs::create_dir_all("utils/");

    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            bootstrap_windows(log);
        } else {
            bootstrap_nix(log);
        }
    }
}

#[cfg(target_os = "windows")]
fn bootstrap_windows(log: Logger) {
    const FFMPEG_RELEASE: &str = "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip";

    if Path::new("utils/ffmpeg.exe").is_file() && Path::new("utils/ffprobe.exe").is_file() {
        info!(log, "ffmpeg and ffprobe have been found.");
        return;
    }

    info!(log, "Could not find ffmpeg or ffprobe, bootstrapping.");
    let zip_stream = download(FFMPEG_RELEASE);

    let mut zip_file = match ZipArchive::new(zip_stream) {
        Ok(x) => x,
        Err(e) => {
            error!(log, "{:?}", e);
            exit(0)
        }
    };

    let zip_dir_name = zip_file.by_index(0).unwrap().name().to_string();

    let mut zip_path = PathBuf::from(zip_dir_name);
    zip_path.push("bin");

    let path_base = PathBuf::from("./utils");

    {
        let mut zip_path = zip_path.clone();
        zip_path.push("ffmpeg.exe");

        let mut ffmpeg = zip_file
            .by_name(zip_path.to_str().unwrap())
            .expect("Failed to find ffmpeg.exe");
        let mut ffmpeg_path = path_base.clone();

        ffmpeg_path.push("ffmpeg.exe");

        let mut ffmpeg_file = File::create(ffmpeg_path).expect("Couldnt create ffmpeg.exe");

        std::io::copy(&mut ffmpeg, &mut ffmpeg_file).expect("Failed to write data to ffmpeg.exe");
    }

    {
        let mut zip_path = zip_path.clone();
        zip_path.push("ffprobe.exe");

        let mut ffprobe = zip_file
            .by_name(zip_path.to_str().unwrap())
            .expect("Failed to find ffprobe.exe");

        let mut ffprobe_path = path_base.clone();
        ffprobe_path.push("ffprobe.exe");

        let mut ffprobe_file = File::create(ffprobe_path).expect("Couldnt create ffprobe.exe");

        std::io::copy(&mut ffprobe, &mut ffprobe_file)
            .expect("Failed to write data to ffprobe.exe");
    }
}

#[cfg(not(target_os = "windows"))]
fn bootstrap_nix(log: Logger) {
    const FFMPEG_RELEASE: &str =
        "https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz";

    if Path::new("utils/ffmpeg").is_file() && Path::new("utils/ffprobe").is_file() {
        info!(log, "found ffmpeg and ffprobe in utils");
        return;
    }

    info!(log, "Could not find ffmpeg or ffprobe, bootstrapping.");
    let stream = download(FFMPEG_RELEASE);

    let xz_stream = XzDecoder::new(stream);
    let mut archive = Archive::new(xz_stream);

    let entries = archive.entries().unwrap();

    for entry in entries {
        if let Ok(mut entry) = entry {
            let file = PathBuf::from(entry.path().unwrap());
            let file_name = file.file_name().unwrap();

            if ["ffmpeg", "ffprobe"].contains(&file_name.to_str().unwrap()) {
                let mut file_path = PathBuf::from("utils/");
                file_path.push(file_name);

                let mut file = File::create(&file_path).unwrap();
                std::io::copy(&mut entry, &mut file).expect("Failed to write data to utils/");

                let perms = Permissions::from_mode(0o755);
                std::fs::set_permissions(file_path, perms).expect("failed to set file perms");
            }
        }
    }
}

fn download(url: &str) -> Cursor<bytes::Bytes> {
    let resp = reqwest::blocking::get(url).unwrap();
    let fname = resp
        .url()
        .path_segments()
        .expect("couldnt grab path segs")
        .last()
        .expect("couldnt get fname");

    Cursor::new(resp.bytes().unwrap())
}
