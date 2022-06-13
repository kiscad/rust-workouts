use fswalk::*;
use std::io;
use std::path::Path;
use std::time::Instant;

fn main() -> io::Result<()> {
    // let mut entries: Vec<_> = fs::read_dir(r"D:\")?
    //     .map(|res| res.map(|e| e.path()))
    //     .collect::<Result<Vec<_>, io::Error>>()?;

    // // 不保证 `read_dir` 返回条目的顺序
    // // 如果需要可重复的排序，则应对条目进行显示排序
    // entries.sort();
    // for entry in entries {
    //     println!("{:?}", entry);
    // }

    let root_path = Path::new(r"D:/Documents/projects/rust/");
    let types = vec![
        "rs", "c", "cc", "cpp", "cxx", "h", "hpp", "hxx", "java", "py", "kt", "js", "lua",
    ];
    let now = Instant::now();
    let paths = find_files(root_path, &types)?;
    println!("total {} files found!", paths.len());
    let elasped_time = now.elapsed();
    println!("It took {:?} seconds.", elasped_time);

    Ok(())
}
