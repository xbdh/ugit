
use crate::database::PathBuf;
use winnow::prelude::*;
use winnow::ascii::{digit1, hex_digit1, line_ending, space1, till_line_ending};
use winnow::combinator::{preceded, terminated, separated, seq, repeat, opt};
use winnow::error::ContextError;
use winnow::token::{take, take_till, take_while, literal, rest};
use winnow::Result as PResult;
use crate::database::author::Author;
use crate::tree_entry::{TreeEntryLine, TreeEntryMode};

pub fn parse_header(input: &mut &[u8]) -> PResult<(String, usize)> {
    let (obj_type, size) = seq!(
        take_till(1.., |b| b == b' ').map(|b: &[u8]| std::str::from_utf8(b).unwrap()),
        _: literal(b" "),
        digit1.map(|b: &[u8]| std::str::from_utf8(b).unwrap().parse::<usize>().unwrap()),
        _: literal(b"\0")
    ).parse_next(input)?;

    Ok((obj_type.to_string(), size))
}


pub fn parse_commit_content(input: &mut &[u8]) -> PResult<(String, Vec<String>, Author, Author, String)> {
    // 解析 tree
    let tree_id = preceded(
        literal(b"tree "),
        terminated(
            take(40usize).map(|b: &[u8]| String::from_utf8_lossy(b).into_owned()),
            line_ending
        )
    ).parse_next(input)?;

    // 解析 parents（可能有多个）
    let mut parents = Vec::new();
    parents = repeat(0.., preceded(
        literal(b"parent "),
        terminated(
            take(40usize).map(|b: &[u8]| String::from_utf8_lossy(b).into_owned()),
            line_ending
        )
    )).parse_next(input)?;

    // 解析 author
    let (author_name, author_email, author_time) = parse_person_line.parse_next(input)?;
    line_ending.parse_next(input)?;

    // 解析 committer
    let (committer_name, committer_email, committer_time) = parse_person_line.parse_next(input)?;
    line_ending.parse_next(input)?;

    // // 可能有 GPG 签名，跳过
    // while literal::<_, &[u8], _>(b"gpgsig").parse_peek(input).is_ok() {
    //     // 跳过整个 GPG 签名块
    //     till_line_ending.parse_next(input)?;
    //     line_ending.parse_next(input)?;
    //     while literal::<_, &[u8], _>(b" ").parse_peek(input).is_ok() {
    //         till_line_ending.parse_next(input)?;
    //         line_ending.parse_next(input)?;
    //     }
    // }

    // 跳过空行
    opt(line_ending).parse_next(input)?;

    // 剩余的都是 commit message
    let message = rest
        .map(|b: &[u8]| String::from_utf8_lossy(b).into_owned())
        .parse_next(input)?;

    // 构造 Author 对象
    // let author = Author{
    //     name: author_name,
    //     email: author_email,
    //     timestamp: parse_timestamp(&author_time),
    // };

    let author =Author::new(
        &author_name,
        &author_email,
        parse_timestamp(&author_time).parse().unwrap(),


    );
    let committer = Author::new(
        &committer_name,
        &committer_email,
        parse_timestamp(&committer_time).parse().unwrap(),
    );


    Ok((tree_id, parents, author, committer, message))
}

// 辅助函数：解析时间戳
pub fn parse_timestamp(time_str: &str) -> String {
    // 简单返回原始字符串，实际使用时可能需要更复杂的解析
    time_str.to_string()
}

// 解析 Commit 的作者/提交者行
pub fn parse_person_line(input: &mut &[u8]) -> PResult<(String, String, String)> {
    seq!(
        // 跳过 "author " 或 "committer "
        take_till(1.., |b| b == b' ').map(|_| ()),
        _: literal(b" "),
        // 解析名字和邮箱（直到时间戳）
        take_till(1.., |b| b == b'>')
            .map(|b: &[u8]| {
                let s = String::from_utf8_lossy(b);
                let parts: Vec<&str> = s.split(" <").collect();
                (parts[0].to_string(), parts.get(1).unwrap_or(&"").to_string())
            }),
        _: literal(b"> "),
        // 解析时间戳和时区
        till_line_ending
            .map(|b: &[u8]| String::from_utf8_lossy(b).into_owned())
    )
        .map(|(_, (name, email), timestamp)| (name, email, timestamp))
        .parse_next(input)
}




// 解析 Blob 对象
pub fn parse_blob_content(input: &mut &[u8]) -> PResult<String> {
    // Blob 的内容就是剩余的所有字节
    rest.map(|bytes: &[u8]| String::from_utf8_lossy(bytes).into_owned())
        .parse_next(input)
}

// 解析单个 Tree Entry
pub fn parse_tree_entry(input: &mut &[u8]) -> PResult<TreeEntryLine> {
    seq!(
        // 解析 mode (如 "100644" 或 "40000")
        take_till(1.., |b| b == b' ')
            .map(|b: &[u8]| String::from_utf8_lossy(b).into_owned()),
        _: literal(b" "),
        // 解析文件名（直到 null 字节）
        take_till(1.., |b| b == b'\0')
            .map(|b: &[u8]| PathBuf::from(String::from_utf8_lossy(b).into_owned())),
        _: literal(b"\0"),
        // 解析 20 字节的 SHA-1 hash
        take(20usize).map(|b: &[u8]| b.to_vec())
    )
        .map(|(mode, name, hash)|{
            let object_id = hex::encode(hash);
            let entry = TreeEntryLine::new(
                name,
                &object_id,
                match mode.as_str() {
                    "40000" => TreeEntryMode::Tree,
                    "100644" => TreeEntryMode::RegularFile,
                    "100755" => TreeEntryMode::ExecutableFile,
                    // "120000" => "120000", // Symbolic link
                    // "160000" => "160000", // Git link
                    _ => TreeEntryMode::RegularFile// 默认处理为普通文件
                },
            );
            entry
        })
        .parse_next(input)
}

// fn parse_tree_entries(size: usize) -> impl Parser<&[u8], Vec<Entry>, ContextError> {
//     move |input: &mut &[u8]| {
//         let mut entries = Vec::new();
//         let start_len = input.len();
//
//         // 继续解析直到达到指定大小
//         while start_len - input.len() < size {
//             match parse_tree_entry.parse_next(input) {
//                 Ok(entry) => entries.push(entry),
//                 Err(_) => break,
//             }
//         }
//
//         Ok(entries)
//     }
// }

// 解析整个 Tree 的所有 entries
// fn parse_tree_entries(size: usize) -> impl Parser<&[u8], Vec<RawTreeEntry>, ContextError> {
//     move |input: &mut &[u8]| {
//         let mut entries = Vec::new();
//         let start_len = input.len();
//
//         // 继续解析直到达到指定大小
//         while start_len - input.len() < size {
//             match parse_tree_entry.parse_next(input) {
//                 Ok(entry) => entries.push(entry),
//                 Err(_) => break,
//             }
//         }
//
//         Ok(entries)
//     }
// }

// 为了更高效，可以创建一个专门的解析器来处理 mode
pub fn parse_mode(input: &mut &[u8]) -> PResult<TreeEntryMode> {
    take_till(1.., |b| b == b' ')
        .map(|b: &[u8]| {
            let mode_str = String::from_utf8_lossy(b);
            match mode_str.as_ref() {
                "40000" => TreeEntryMode::Tree,
                "100644" =>TreeEntryMode::RegularFile,
                "100755" => TreeEntryMode::ExecutableFile,
                // "120000" => TreeEntryMode::SymbolicLink,
                // "160000" => TreeEntryMode::GitLink,
                _ => TreeEntryMode::Other(mode_str.into_owned()),
            }
        })
        .parse_next(input)
}

pub fn parse_tree_entry_typed(input: &mut &[u8]) -> PResult<TreeEntryLine> {
    seq!(
        parse_mode,
        _: literal(b" "),
        take_till(1.., |b| b == b'\0')
            .map(|b: &[u8]| PathBuf::from(String::from_utf8_lossy(b).into_owned())),
        _: literal(b"\0"),
        take(20usize).map(|b: &[u8]| hex::encode(b))
    )
        .map(|(mode, name, hash)| {
            TreeEntryLine::new(
                name,
                &hash,
                mode
                // match mode {
                //     TreeEntryMode::Tree => "40000",
                //     TreeEntryMode::RegularFile => "100644",
                //     TreeEntryMode::ExecutableFile => "100755",
                //     // TreeEntryMode::SymbolicLink => "120000",
                //     // TreeEntryMode::GitLink => "160000",
                //     TreeEntryMode::Other(s) =>"100644", // 默认处理为普通文件
                // },
            )
        })
        .parse_next(input)
}