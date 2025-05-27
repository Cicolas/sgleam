use std::{collections::HashMap, time::SystemTime};

use camino::{Utf8Path, Utf8PathBuf};
use gleam_core::{io::{memory::InMemoryFileSystem, BeamCompiler, CommandExecutor, Content, FileSystemReader, FileSystemWriter}, Error};

pub trait FileSystem: FileSystemReader + FileSystemWriter + CommandExecutor + BeamCompiler + Clone {
    fn new() -> Self;

    fn reset(&self);

    fn into_contents(self) -> HashMap<Utf8PathBuf, Content>;

    fn files(&self) -> Vec<Utf8PathBuf>;

    // #[cfg(test)]
    // fn set_modification_time(&self, path: &Utf8Path, time: SystemTime);

    fn try_set_modification_time(
        &self,
        path: &Utf8Path,
        time: SystemTime,
    ) -> Result<(), Error>;
}

impl FileSystem for InMemoryFileSystem {
    fn new() -> Self {
        InMemoryFileSystem::new()
    }

    fn reset(&self) {
        InMemoryFileSystem::reset(&self);
    }

    fn into_contents(self) -> HashMap<Utf8PathBuf, Content> {
        InMemoryFileSystem::into_contents(self)
    }

    fn files(&self) -> Vec<Utf8PathBuf> {
        InMemoryFileSystem::files(&self)
    }

    // #[cfg(test)]
    // fn set_modification_time(&self, path: &Utf8Path, time: SystemTime) {
    //     InMemoryFileSystem::set_modification_time(&self, path, time)
    // }

    fn try_set_modification_time(
        &self,
        path: &Utf8Path,
        time: SystemTime,
    ) -> Result<(), Error> {
        InMemoryFileSystem::try_set_modification_time(&self, path, time)
    }
}