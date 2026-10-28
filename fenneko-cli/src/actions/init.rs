use common::{
    env::Args,
    fs::{self, CreateDirError, OpenWriteError},
    raw::fs::{CreateError, OpenError, WriteError},
};

use crate::{actions::Action, render::Renderable};

pub struct InitOut;

impl Renderable for InitOut {
    fn render_console(&self, f: &mut dyn core::fmt::Write) -> core::fmt::Result {
        write!(f, "Initialized Git repository")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InitErrorType {
    CreateDir(CreateDirError),
    Open(OpenError),
    Write(WriteError),
}

macro_rules! impl_from_initerr {
    ($($name:ident : $enum:ident),+ $(,)?) => {
        $(
            impl From<$name> for InitErrorType {
                fn from(value: $name) -> Self {
                    Self::$enum(value)
                }
            }
        )+
    };
}

impl_from_initerr!(
    CreateDirError: CreateDir,
    OpenError: Open,
    WriteError: Write,
);

impl From<OpenWriteError> for InitErrorType {
    fn from(value: OpenWriteError) -> Self {
        match value {
            OpenWriteError::FailedToOpen(e) => Self::Open(e),
            OpenWriteError::FailedToWrite(e) => Self::Write(e),
        }
    }
}

pub enum InitError {
    FsError(InitFsError),
    GitAlreadyInitialized,
}

pub struct InitFsError {
    typ: InitErrorType,
    object: &'static str,
}

impl Renderable for InitError {
    fn render_console(&self, f: &mut dyn core::fmt::Write) -> core::fmt::Result {
        let fs_err = match self {
            Self::FsError(e) => e,
            Self::GitAlreadyInitialized => {
                return writeln!(f, "Git repository already initialized!");
            },
        };

        writeln!(f, "Failed to init empty Git repository!")?;

        let obj = fs_err.object;
        match fs_err.typ {
            InitErrorType::CreateDir(e) => {
                writeln!(f, "Creation dir `{obj}` returned an error: {e}")
            },
            InitErrorType::Open(e) => writeln!(f, "Opening file `{obj}` returned an error: {e}"),
            InitErrorType::Write(e) => writeln!(f, "Writing file `{obj}` returned an error: {e}"),
        }
    }
}

fn map_into_init_err<T: Into<InitErrorType>>(e: T, path: &'static str) -> InitError {
    InitError::FsError(InitFsError {
        typ: e.into(),
        object: path,
    })
}

pub struct Init;

impl Action<InitOut, InitError> for Init {
    fn run(_args: Option<Args>) -> Result<InitOut, InitError> {
        // Root .git dir
        let ret = fs::create_dir(".git");
        match ret {
            Err(CreateError::AlreadyExists) => return Err(InitError::GitAlreadyInitialized),
            Err(e) => return Err(map_into_init_err(e, ".git")),
            Ok(()) => {},
        }

        // Head file
        fs::write(".git/HEAD", "ref: refs/heads/master\n")
            .map_err(|e| map_into_init_err(e, ".git/HEAD"))?;

        Ok(InitOut)
    }
}
