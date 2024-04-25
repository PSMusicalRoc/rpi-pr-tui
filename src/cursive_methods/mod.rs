use cursive::{
    traits::*, views::{Dialog, NamedView, SelectView}, Cursive
};
use std::{fs::{self, DirEntry}, path::{Path, PathBuf}};

fn populate_fileview(file_select: &mut SelectView<PathBuf>, directory: Option<PathBuf>) {
    file_select.clear();

    let cwd = match directory {
        Some(p) => p,
        None => Path::new("./").to_path_buf()
    };
    
    // get all paths in the directory
    let paths = fs::read_dir(cwd.clone()).unwrap();
    let mut files: Vec<DirEntry> = Vec::new();
    let mut directories: Vec<DirEntry> = Vec::new();

    for path in paths {
        let d = path.unwrap();
        if d.path().is_dir() {
            directories.push(d);
        } else if d.path().is_file() { //&& d.path().extension().is_some() && d.path().extension().unwrap() == ".rpipr" {
            files.push(d);
        }
    }

    files.sort_by_key(|f| f.path());
    directories.sort_by_key(|d| d.path());

    let dotdot = cwd.canonicalize().unwrap();
    if dotdot.parent().is_some() {
        file_select.add_item("..", dotdot.parent().unwrap().to_path_buf());
    }

    for directory in directories {
        file_select.add_item(format!(" {}", directory.file_name().to_str().unwrap()), directory.path());
    }
    
    for file in files {
        if file.path().extension().is_some() && file.path().extension().unwrap() == "rpipr" {
            file_select.add_item(format!(" {}", file.file_name().to_str().unwrap()), file.path());
        }
    }
}

/// Instantiates the Load File menu (specifically for .rpipr extensions)
/// as a layer over the current workspace. When a file is selected,
/// this function calls `callback` so that the user can do what they
/// want with the returned file path.
/// 
/// # Parameters
/// - `callback`: A `fn` reference or lambda that takes two parameters:
///   - A `&mut Cursive` object that will end up being the Cursive root
///   - A `PathBuf` containing the path to the selected file.
pub fn create_load_file_menu(callback: fn(&mut Cursive, PathBuf)) -> Dialog {
    let mut file_select: NamedView<SelectView<PathBuf>> = SelectView::new().h_align(cursive::align::HAlign::Left)
        .on_submit(move |s: &mut Cursive, val: &PathBuf| {
            if val.is_dir() {
                s.call_on_name("File Select", |select: &mut SelectView<PathBuf>| populate_fileview(select, Some(val.clone())));
            }
            if val.is_file() {
                // should be a .rpipr file, load it
                callback(s, val.clone());
            };
        })
        .with_name("File Select");
    file_select.call_on_name("File Select", |s: &mut SelectView<PathBuf>| {
            populate_fileview(s, None);
        }
    );

    let fs_window = Dialog::new()
        .title("Load PR File Object:")
        .content(file_select)
        .button("Cancel", |s| { s.pop_layer(); } );

    fs_window
}