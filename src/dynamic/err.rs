use std::error::Error;
use std::fmt;
use std::io;
use std::process;

use ::git2;

use super::library::LibraryError;

pub type Result<T> = ::std::result::Result<T, CompositerError>;

/// The enum `CompositerError` defines the possible errors
/// from constructor Compositer.
#[derive(Debug)]
pub enum CompositerError {
    /// The directory can't be moved.
    MvFail(io::Error),
    /// Can't remove the file.
    RmFile(io::Error),
    /// Can't remove the directory.
    RmDir(io::Error),
    /// Can't create the `git` sub-directory.
    MkDirGit(io::Error),
    /// Can't create the `lib` sub-directory.
    MkDirLib(io::Error),
    /// Can't read the `git` sub-directory.
    ReadDirGit(io::Error),
    /// Can't read the `lib` sub-directory.
    ReadDirLib(io::Error),
    /// Can't open the `lib` sub-directory.
    OpenDirLib(io::Error),
    /// Can't read the `manifest` Neko.toml file.
    ReadManifest(io::Error),
    /// Can't mount the dynamic library.
    Mount(LibraryError),
    /// Can't clone the repository.
    InstallClone(git2::Error),
    /// Can't update the repository.
    UpdateRepOpen(git2::Error),
    /// Can't found the origin from repository.
    UpdateRepOrigin(git2::Error),
    /// Can't fetch them repository.
    UpdateRepFetch(git2::Error),
    /// Can't found the branch from repository.
    UpdateRepBranch(git2::Error),
    /// Can't get the target identifiant from branch.
    UpdateRepBranchId,
    /// Can't found the object from target identifiant.
    UpdateRepObject(git2::Error),
    /// Can't reset the repository.
    UpdateRepReset(git2::Error),
    /// Can't run the command.
    BuildCommand(io::Error),
    /// The build haven't exited with success.
    BuildExit(process::ExitStatus),
    /// Can't found the $HOME environement variable.
    Home,
    /// Can't found the position.
    UnmountPosition,
    /// Can't remove the index.
    UnmountRemove,
    /// Can't parse the `manifest` Neko.toml file.
    ParseManifest,
    /// Can't parse a integer from the table.
    ParseInteger,
    /// The lib git haven't a valid format.
    InstallFormat,
    /// The dynamic library as already a repository.
    InstallExists,
}

impl fmt::Display for CompositerError {
  /// The function `fmt` formats the value using
  /// the given formatter.
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
       Ok(())
    }
}

impl Error for CompositerError {
  /// The function `description` returns a short description of
  /// the error.
  fn description(&self) -> &str {
      match *self {
        CompositerError::MvFail(_) => "The directory can't be moved.",
        CompositerError::RmFile(_) => "Can't remove the file.",
        CompositerError::RmDir(_) => "Can't remove the directory.",
        CompositerError::MkDirGit(_) => "Can't create the `git` sub-directory.",
        CompositerError::MkDirLib(_) => "Can't create the `Lib` sub-directory.",
        CompositerError::ReadDirGit(_) => "Can't read the `git` sub-directory.",
        CompositerError::ReadDirLib(_) => "Can't read the `Lib` sub-directory.",
        CompositerError::OpenDirLib(_) => "Can't open the `lib` sub-directory.",
        CompositerError::ReadManifest(_) => "Can't read the `manifest` Neko.toml\
                                             file.",
        CompositerError::Mount(_) => "Can't mount the dynamic library.",
        CompositerError::InstallClone(_) => "Can't clone the repository",
        CompositerError::UpdateRepOpen(_) =>"Can't update the repository.",
        CompositerError::UpdateRepOrigin(_) =>"Can't found the origin from\
                                                repository.",
        CompositerError::UpdateRepFetch(_) => "Can't fetch them repository.",
        CompositerError::UpdateRepBranch(_) =>"Can't found the branch from\
                                               repository.",
        CompositerError::UpdateRepBranchId => "Can't get the target\
                                                  identifiant from branch.",
        CompositerError::UpdateRepObject(_) => "Can't found the object from\
                                                target identifiant.",
        CompositerError::UpdateRepReset(_) => "Can't reset the repository.",
        CompositerError::BuildCommand(_) => "Can't run the command.",
        CompositerError::BuildExit(_) => "The build haven't exited with success.",
        CompositerError::Home => "Can't found the $HOME environement variable.",
        CompositerError::ParseManifest => "Can't parse the `manifest` Neko.toml\
                                           file.",
        CompositerError::ParseInteger => "Can't parse a integer from the table.",
        CompositerError::UnmountPosition => "Can't found the position.",
        CompositerError::UnmountRemove => "Can't remove the index.",
        CompositerError::InstallFormat => "The git link haven't a valid format",
        CompositerError::InstallExists => "The dynamic library as already a\
                                           repository.",
    }
  }

  /// The function `cause` returns the lower-level cause of
  /// this error if any.
  fn cause(&self) -> Option<&Error> {
      match *self {
        CompositerError::MvFail(ref why) => Some(why),
        CompositerError::RmFile(ref why) => Some(why),
        CompositerError::RmDir(ref why) => Some(why),
        CompositerError::MkDirGit(ref why) => Some(why),
        CompositerError::MkDirLib(ref why) => Some(why),
        CompositerError::ReadDirGit(ref why) => Some(why),
        CompositerError::ReadDirLib(ref why) => Some(why),
        CompositerError::OpenDirLib(ref why) => Some(why),
        CompositerError::ReadManifest(ref why) => Some(why),
        CompositerError::Mount(ref why) => Some(why),
        CompositerError::InstallClone(ref why) => Some(why),
        CompositerError::BuildCommand(ref why) => Some(why),
        _ => None,
    }
  }
}
