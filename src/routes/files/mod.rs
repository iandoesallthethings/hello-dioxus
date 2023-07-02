use crate::prelude::*;
mod file_row;
use file_row::*;

pub fn FilesPage(cx: Scope) -> Element {
    let paths = get_files();

    cx.render(rsx! {
        h1 { "files" },

        ul {
            for path in paths {
                FileRow{
                    path: path,
                }
            }
        }
    })
}

fn get_files() -> Vec<String> {
    let dir = "/Users/ian/DJ/musik";

    std::fs::read_dir(dir)
        .unwrap()
        .take(5)
        .filter_map(|result_dir_entry| {
            result_dir_entry
                .map(|dir_entry| dir_entry.path().to_string_lossy().into_owned())
                .ok()
        })
        .collect()
}
