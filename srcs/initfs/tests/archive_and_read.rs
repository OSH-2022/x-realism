#![feature(array_methods)]

use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};

#[path = "../tools/src/archive_common.rs"]
mod archive;

#[test]
fn archive_and_read() -> Result<()> {
    env_logger::init();

    let args = self::archive::Args {
        destination_path: Path::new("out.img"),
        source: Path::new("data"),
        max_size: self::archive::DEFAULT_MAX_SIZE,
    };
    self::archive::archive(&args).context("failed to archive")?;

    let data = std::fs::read(args.destination_path).context("failed to read new archive")?;
    let filesystem = initfs::InitFs::new(&data).context("failed to parse archive header")?;
    let root_inode = filesystem
        .get_inode(initfs::InitFs::ROOT_INODE)
        .ok_or_else(|| anyhow!("Failed to get root inode"))?;

    let dir = match root_inode.kind() {
        initfs::InodeKind::Dir(dir) => dir,
        _ => bail!("root inode was not a directory"),
    };

    for idx in 0..dir
        .entry_count()
        .context("failed to get inode entry count")?
    {
        let entry = dir
            .get_entry(idx)
            .context("failed to get entry for index")?
            .ok_or_else(|| anyhow!("no entry found"))?;

        if entry.name().context("failed to get entry name")? == b"file.txt".as_slice() {
            let inode = filesystem
                .get_inode(entry.inode())
                .context("failed to load file inode")?;

            let file = match inode.kind() {
                initfs::InodeKind::File(file) => file,
                _ => bail!("file.txt was a directory"),
            };
            let data = file.data().context("failed to get file.txt data")?;
            assert_eq!(
                data,
                std::fs::read("data/file.txt").context("failed to read the real file.txt")?
            );
        }
    }

    Ok(())
}
