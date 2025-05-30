use gleam_core::io::{
    memory::InMemoryFileSystem, BeamCompiler, CommandExecutor, FileSystemReader, FileSystemWriter,
};

pub trait IO: FileSystemReader + FileSystemWriter + CommandExecutor + BeamCompiler + Clone {}

impl IO for InMemoryFileSystem {}
