mod library;
pub mod err;

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::ops::BitAnd;
use std::usize;
use std::env;
use std::process;
use std::fs;

use self::library::Library;
pub use self::err::{CompositerError, Result};

use ::git2::Repository;

/// The default capacity of heap.
const SPEC_CAPACITY: usize = 10;
/// The default priority of call.
const SPEC_PRIORITY: usize = usize::MAX / 2;
/// The default source path.
const SPEC_SOURCE: &'static str = ".";
/// The default source name path.
const SPEC_SOURCE_NAME: &'static str = ".neko";

/// The struct `Compositer` is a heap of a double tuple
/// of a dynamic libraries and a priority order.
#[derive(Debug)]
pub struct Compositer(VecDeque<Library>);

impl Compositer {
    /// The accessor method `source` returns the source directory where
    /// found the `git` and `lib` sub-directories.
    pub fn source(&self) -> PathBuf {
        env::home_dir()
            .unwrap_or(env::current_dir().unwrap_or(PathBuf::from(SPEC_SOURCE)))
            .join(SPEC_SOURCE_NAME)
    }

    /// The method `build` runs a Makefile from a directory.
    fn build(&self, directory: &Path, source: &Path) -> Result<()> {
        if directory.join("Makefile").exists() {
            match process::Command::new("make")
                .env("SOURCE", source)
                .current_dir(directory)
                .status() {
                Ok(status) if status.success() => Ok(()),
                Ok(status) => {
                    Err(CompositerError::BadReturnCommand(status.code()
                        .unwrap_or_default()))
                }
                Err(why) => Err(CompositerError::BadCommand(why)),
            }
        } else {
            Err(CompositerError::NotMakeFound)
        }
    }

    /// The method `mount` adds a new library to the heap's compositer.
    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn mount<S: AsRef<OsStr>>(&mut self,
                                  libraryname: S,
                                  priority: Option<usize>)
                                  -> Result<()> {
        let mut path: PathBuf = self.source()
            .join("lib")
            .join(libraryname.as_ref());
        if path.set_extension("so").bitand(path.exists()) {
            match Library::new(path, priority.unwrap_or(SPEC_PRIORITY)) {
                Err(why) => Err(CompositerError::BadMount(why)),
                Ok(lib) => {
                    lib.start();
                    Ok(self.0.push_back(lib))
                }
            }
        } else {
            Err(CompositerError::BadPath)
        }
    }

    /// The method `mount` adds a new library to the heap's compositer.
    #[cfg(any(target_os = "bitrig", target_os = "dragonfly",
              target_os = "freebsd", target_os = "ios", target_os = "macos",
              target_os = "netbsd", target_os = "openbsd"))]
    pub fn mount<S: AsRef<OsStr>>(&mut self,
                                  libraryname: S,
                                  priority: Option<usize>)
                                  -> Result<()> {
        let path: PathBuf = self.source()
            .join("lib")
            .join(libraryname.as_ref());
        if path.set_extension("dylib").bitand(path.exists()) {
            match Library::new(&path, priority.unwrap_or(SPEC_PRIORITY)) {
                Err(why) => Err(CompositerError::BadMount(why)),
                Ok(lib) => Ok(self.0.push_back(lib)),
            }
        } else {
            Err(CompositerError::BadPath)
        }
    }

    /// The method `mount_from_dir` adds a new library to the heap's compositer
    /// from a directory.
    pub fn mount_from_dir(&mut self,
                          directory: &str,
                          priority: Option<usize>)
                          -> Result<()> {
        let source: &Path = &self.source();
        let path: &Path = Path::new(directory);

        self.build(path, &source)
            .and(self.mount(path.iter().last().unwrap_or_default(), priority))
    }

    /// The method `mount_from_git` adds a new library to the heap's compositer
    /// from a git repository.
    pub fn mount_from_git(&mut self,
                          repository: &str,
                          priority: Option<usize>)
                          -> Result<()> {
        let source: &Path = &self.source();
        let path: PathBuf = source.join("git")
            .join(repository.chars()
                .skip(repository.rfind('/')
                    .unwrap_or_default())
                .skip(1)
                .take_while(|g| g.is_alphanumeric())
                .collect::<String>());

        match Repository::clone(repository, &path) {
            Err(why) => Err(CompositerError::BadGitClone(why)),
            Ok(_) => {
                self.build(&path, source)
                    .and(self.mount(path.iter().last().unwrap_or_default(),
                                    priority))
            }
        }
    }

    /// The method `unmount` removes library from the queue.
    pub fn unmount(&mut self, libraryname: &str) -> Result<Library> {
        if let Some(Some(lib)) = self.0.iter().position(|lib| {
          lib.as_path_buf().file_stem().unwrap_or_default().eq(libraryname)
        }).map(|index|
          self.0.remove(index)
        ) {
            Ok(lib)
        } else {
            Err(CompositerError::NotUnmounted)
        }
    }

    /// The method `uninstall` removes library from the filesystem.
    pub fn uninstall(&mut self, libraryname: &str) -> Result<()> {
        match self.unmount(libraryname) {
            Err(why) => Err(why),
            Ok(lib) => {
                match fs::remove_file(lib.as_path_buf().as_os_str()) {
                    Err(why) => Err(CompositerError::NotUninstalled(why)),
                    Ok(()) => Ok(()),
                }
            },
        }
    }

    pub fn start(&self) {
        self.0.iter().all(|lib| {
            lib.start();
            true
        });
    }
}

/// A trait for giving a type a useful default value.
impl Default for Compositer {
    /// The constructor `default` returns a Compositer prepared with
    /// the libraries from the source directory.
    fn default() -> Compositer {
        let mut compositer: Compositer =
            Compositer(VecDeque::with_capacity(SPEC_CAPACITY));

        if let Some(mut paths) = fs::read_dir(compositer.source().join("lib"))
            .ok() {
            paths.all(|path| {
                if let Some(entry) = path.ok() {
                    if let Some(path) = entry.path().file_stem() {
                        compositer.mount(path, None).is_ok()
                    } else {
                        true
                    }
                } else {
                    true
                }
            });
        }
        compositer
    }
}
