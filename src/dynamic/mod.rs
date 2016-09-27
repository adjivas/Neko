mod library;
pub mod err;


use ::git2;
pub use self::err::{CompositerError, Result};

use self::library::Library;
use std::collections::VecDeque;
use std::env;
use std::io;
use std::fs;
use std::ffi::OsStr;
use std::ops::BitAnd;
use std::path::PathBuf;
use std::process;
use std::usize;

/// The default capacity of heap.
const SPEC_CAPACITY: usize = 10;
/// The default priority of call.
const SPEC_PRIORITY: usize = usize::MAX / 2;
/// The default root path.
const SPEC_ROOT: &'static str = ".";
/// The default root name path.
const SPEC_ROOT_NAME: &'static str = ".neko";
/// The shared library extention.
#[cfg(any(target_os = "linux", target_os = "android"))]
const SPEC_LIBRARY_EXT: &'static str = "so";
#[cfg(any(target_os = "bitrig", target_os = "dragonfly",
          target_os = "freebsd", target_os = "ios", target_os = "macos",
          target_os = "netbsd", target_os = "openbsd"))]
const SPEC_LIBRARY_EXT: &'static str = "dylib";

/// The struct `Compositer` is a heap of a double tuple
/// of a dynamic libraries and a priority order.
#[derive(Debug)]
pub struct Compositer(VecDeque<Library>);

impl Compositer {

  /// The method `build` runs a Makefile from a directory.
  fn build(&self, directory: &PathBuf, root: &PathBuf) -> Result<()> {
    let librarypath: PathBuf = PathBuf::from(directory.file_name().unwrap_or_default());

    if directory.join("Makefile").exists() {
      match process::Command::new("make")
                             .current_dir(&directory)
                             .status() {
        Err(why) => Err(CompositerError::CallCommandFail(why)),
        Ok(status) if status.success() => {
          fs::create_dir_all(root.join("lib")).or_else(|why: io::Error|
            Err(CompositerError::MkdirFail(why))
          ).and(
            fs::rename(
              root.join("git").join(&librarypath).join(&librarypath)
                                                 .with_extension(SPEC_LIBRARY_EXT),
              root.join("lib").join(&librarypath).with_extension(SPEC_LIBRARY_EXT),
            ).or_else(|why: io::Error| 
              Err(CompositerError::MvFail(why))
            )
          )
        },
        Ok(status) => Err(CompositerError::CommandFail(
          status.code().unwrap_or_default())
        ),
      }
    } else {
      Err(CompositerError::NotMakeFound)
    }
  }

  /// The method `mount` adds a new library to the heap's compositer.
  pub fn mount<S: AsRef<OsStr>> (
    &mut self,
    libraryname: S,
    priority: Option<usize>
  ) -> Result<()> {
    let mut path: PathBuf = self.get_root()
      .join("lib")
      .join(libraryname.as_ref());

    if path.set_extension(SPEC_LIBRARY_EXT).bitand(path.exists()) {
      match Library::new(path, priority.unwrap_or(SPEC_PRIORITY)) {
        Err(why) => Err(CompositerError::NotMounted(why)),
        Ok(lib) => {
          lib.start();
          Ok(self.0.push_back(lib))
        }
      }
    } else {
      Err(CompositerError::LibraryNotFound)
    }
  }

  /// The method `mount_from_dir` adds a new library to the heap's compositer
  /// from a directory.
  pub fn mount_from_dir (
    &mut self,
    directory: &str,
    priority: Option<usize>
  ) -> Result<()> {
    let root: PathBuf = self.get_root();
    let path: PathBuf = PathBuf::from(directory);

    self.build(&path, &root)
        .and(self.mount(path.iter().last().unwrap_or_default(), priority))
  }

  /// The method `mount_from_git` adds a new library to the heap's compositer
  /// from a git repository.
  pub fn mount_from_git (
    &mut self,
    repository: &str,
    priority: Option<usize>
  ) -> Result<()> {
    let root: PathBuf = self.get_root();
    let source: PathBuf = root.join("git").join(repository.chars()
                              .skip(repository.rfind('/').unwrap_or_default())
                              .skip(1)
                              .take_while(|g| g.is_alphanumeric())
                              .collect::<String>());

    if source.exists() {
      git2::Repository::open(&source).ok().map(|repo: git2::Repository| {
        repo.find_remote("origin").map(|mut remote: git2::Remote| {
           remote.fetch(&["refs/heads/*:refs/heads/*"], None, None).is_ok()
                 .bitand(repo.find_branch("master", git2::BranchType::Local).ok()
                 .map(|branch: git2::Branch|
                    branch.get().target().map(|id: git2::Oid|
                      repo.find_object(id, None).ok().map(|object: git2::Object|
                        repo.reset(&object, git2::ResetType::Hard, None).ok()
                      ).is_some()
                    ).unwrap_or_default()
                  ).unwrap_or_default())
        }).ok().map(|_| Ok(()) ).unwrap_or(Err(CompositerError::NotGitPull))
      }).unwrap_or(Err(CompositerError::NotGitOpen))
    } else {
      git2::Repository::clone(repository, &source).map(|_: git2::Repository| Ok(()))
                       .unwrap_or(Err(CompositerError::NotGitClone))
    }.and(self.build(&source, &root)
     .and(self.mount(source.file_name().unwrap_or_default(), priority)))
  }

  /// The method `unmount` removes library from the queue.
  pub fn unmount(&mut self, libraryname: &str) -> Result<Library> {
    if let Some(Some(lib)) = self.0.iter().position(|lib| {
        lib.as_path_buf()
          .file_stem()
          .unwrap_or_default()
          .eq(libraryname)
    }).map(|index| self.0.remove(index)) {
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
      }
    }
  }

  /// The accessor method `root` returns the root directory where
  /// found the `git` and `lib` sub-directories.
  pub fn get_root(&self) -> PathBuf {
    env::home_dir()
        .unwrap_or(env::current_dir()
                       .unwrap_or(PathBuf::from(SPEC_ROOT)))
                                          .join(SPEC_ROOT_NAME)
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
  /// the libraries from the root directory.
  fn default() -> Compositer {
    let mut compositer: Compositer =
      Compositer(VecDeque::with_capacity(SPEC_CAPACITY));

    if let Some(mut paths) = fs::read_dir(
      compositer.get_root().join("lib")
    ).ok() {
      paths.all(|path| {
        path.ok().map(|entry| {
          entry.path().file_stem()
               .map(|lib| compositer.mount(lib, None).is_ok())
        });
        true
      });
    }
    compositer
  }
}
