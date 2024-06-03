#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_macros)]
#![allow(unused_parens)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

// fn main() {
//     let v: Vec<&str> = "Mary aa\nhad\na\nlittle\n\nlamb\n\0".split('\n').collect();
//     for s in &v {
//         println!("{}", s);
//     }
//     println!("{}", v.len())
//
//     //SystemTime { tv_sec: 1706014908, tv_nsec: 926637927 }
// }

fn main() {
    let flag: u16 = 3; // 假设 flag 为 2
    let file_length: u16 = 5; // 文件长度为 5

    // 写入数据到 16 位数字中
    // 将 flag 左移 12 位，使其位于前四位的第三和第四位置
    // 文件长度直接写入，因为它占据最低的 12 位
    let data: u16 = (flag << 12) | file_length;

    // 从 16 位数字中读取数据
    // 通过右移 12 位，我们可以得到 flag 的值
    let extracted_flag = (data >> 12) & 0b11; // 使用掩码 0b11 确保只获取两位

    // 通过应用掩码 0xFFF (即 12 位全为 1 的二进制数)，我们可以提取出文件长度的值
    let extracted_file_length = data & 0xFFF;

    println!("原始数据: flag = {}, file_length = {}", flag, file_length);
    println!("合并后的数据: {}", data);
    println!("提取的 flag: {}", extracted_flag);
    println!("提取的文件长度: {}", extracted_file_length);
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
