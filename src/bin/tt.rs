use std::fs;
use std::path::Path;

fn main() {
    let root_path = "abc"; // 替换为你的目录路径
    visit_dirs(Path::new(root_path), 0).expect("Failed to visit directories");
}

fn visit_dirs(dir: &Path, level: usize) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // 递归进入子目录，层级加 1
                visit_dirs(&path, level + 1)?;
            } else {
                // 打印文件的完整路径和层级
                println!("{}: {}", "-".repeat(level * 4) + &format!(" Level {}", level), path.display());
            }
        }
    }
    Ok(())
}
