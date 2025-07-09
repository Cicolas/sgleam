use gleam_core::io::{
    memory::InMemoryFileSystem, BeamCompiler, CommandExecutor, FileSystemReader, FileSystemWriter,
};

use crate::fs::ProjectIO;

pub trait IO: FileSystemReader + FileSystemWriter + CommandExecutor + BeamCompiler + Clone {}

impl IO for InMemoryFileSystem {}
impl IO for ProjectIO {}