use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

fn main() {
    let v: Vec<&str> = "Mary aa\nhad\na\nlittle\n\nlamb\n\0".split('\n').collect();
    for s in &v {
        println!("{}", s);
    }
    println!("{}", v.len())

    //SystemTime { tv_sec: 1706014908, tv_nsec: 926637927 }
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
                println!(
                    "{}: {}",
                    "-".repeat(level * 4) + &format!(" Level {}", level),
                    path.display()
                );
            }
        }
    }
    Ok(())
}
