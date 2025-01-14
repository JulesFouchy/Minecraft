use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let res_dir = "res"; // Path to your resource directory
    let dest_dir = target_dir.join("res");

    if dest_dir.exists() {
        fs::remove_dir_all(&dest_dir).unwrap();
    }

    fs::create_dir_all(&dest_dir).unwrap();
    copy_dir(res_dir, &dest_dir).unwrap();
}

fn copy_dir(from: &str, to: &std::path::Path) -> std::io::Result<()> {
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = to.join(entry.file_name());
        if entry_path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir(&entry_path.to_string_lossy(), &dest_path)?;
        } else {
            fs::copy(entry_path, dest_path)?;
        }
    }
    Ok(())
}
