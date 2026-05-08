use std::{
    borrow::Cow,
    ffi::OsStr,
    path::Path,
    process::Command,
};

use walkdir::{DirEntry, WalkDir};

use crate::features::launcher::launch_item::LaunchItem;

const FILE_SEARCH_DEPTH: usize = 4;

const ASSUMED_FILE_SIZE: usize = 64;

const EXE_NAME_DENYLIST: [&str; 3] = [
    "UnityCrashHandler64.exe",
    "UnityCrashHandler32.exe",
    "notification_helper.exe",
];

pub struct LaunchItemRepository;

impl LaunchItemRepository {
    pub fn get_items() -> Vec<LaunchItem> {
        let search_root = Self::get_search_path();
        return Self::search_exe_files(search_root);
    }

    fn get_search_path<'a>() -> Cow<'a, Path> {
        #[cfg(feature = "use-current-dir")]
        return get_current_dir();

        #[cfg(feature = "use-executable-dir")]
        return Self::get_exe_dir().unwrap_or_else(Self::get_current_dir);
    }

    fn get_current_dir<'a>() -> Cow<'a, Path> {
        Cow::Borrowed(Path::new(r"."))
    }

    fn get_exe_dir<'a>() -> Option<Cow<'a, Path>> {
        let exe_path = std::env::current_exe().ok()?;
        let exe_dir = exe_path.parent()?;

        Some(Cow::Owned(exe_dir.into()))
    }

    fn search_exe_files(search_root: impl AsRef<Path>) -> Vec<LaunchItem> {
        let ext_exe = OsStr::new("exe");
        let current_exe = std::env::current_exe().ok();
        let exe_name_denylist = EXE_NAME_DENYLIST.map(OsStr::new);

        let mut collected: Vec<LaunchItem> = Vec::with_capacity(ASSUMED_FILE_SIZE);

        WalkDir::new(&search_root)
            .max_depth(FILE_SEARCH_DEPTH)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(move |entry| {
                if !entry.file_type().is_file() {
                    return false;
                }

                if let Some(ext) = entry.path().extension() {
                    if ext != ext_exe {
                        return false;
                    }
                } else {
                    return false;
                }

                if let Some(filename) = entry.path().file_name()
                    && exe_name_denylist.contains(&filename)
                {
                    return false;
                }

                if let Some(current_exe) = &current_exe
                    && entry.path() == current_exe
                {
                    return false;
                }

                true
            })
            .enumerate()
            .for_each(|(i, entry)| {
                collected.push(LaunchItem {
                    id: i,
                    name: Self::file_path_to_string(&search_root, &entry).to_string(),
                    path: entry.into_path(),
                });
            });

        collected
    }

    fn file_path_to_string<'a>(search_root: impl AsRef<Path>, entry: &'a DirEntry) -> Cow<'a, str> {
        let path = entry.path();

        path.strip_prefix(&search_root)
            .unwrap_or(path)
            .to_string_lossy()
    }

    pub fn launch_item(item: &LaunchItem) {
        Self::run_exe_file(&item.path);
    }

    fn run_exe_file(target: &Path) {
        let _ = Self::run_exe_file_inner(target);
    }

    #[allow(clippy::zombie_processes)]
    fn run_exe_file_inner(target: &Path) -> Option<()> {
        let path = target;
        let working_dir = path.parent()?;

        Command::new(path).current_dir(working_dir).spawn().ok()?;

        Some(())
    }
}
