use std::fs::read_dir;

use anyhow::Result;
use itertools::Itertools;

use crate::csv::models::CycleFile;

pub fn get_file_list() -> Result<Vec<CycleFile>> {
    let files: Vec<CycleFile> = read_dir("./cycles")?
        .map_ok(|x| x.path())
        .map_ok(|x| CycleFile {
            path: x.clone(),
            list_label: x.as_os_str().to_string_lossy().to_string(),
        })
        .sorted_by(|a, b| a.iter().cmp(b.iter()))
        .try_collect()?;

    if !files.is_empty() {
        return Ok(files);
    }

    vec![CycleFile::create_new_file()].into_iter().try_collect()
}
