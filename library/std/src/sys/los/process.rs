use crate::{
    ffi::{OsStr, OsString},
    fmt, io,
    num::NonZeroI32,
    path::Path,
    sys::{fs::File, pipe::AnonPipe, unsupported},
    sys_common::process::{CommandEnv, CommandEnvs},
};

pub use crate::ffi::OsString as EnvKey;

pub enum Stdio {
    Inherit,
    Null,
    MakePipe,
}

#[derive(Debug)]
pub struct Command {
    program: OsString,
    args: Vec<OsString>,
    env: CommandEnv,
}

#[derive(Debug)]
pub struct CommandArgs<'a> {
    iter: crate::slice::Iter<'a, OsString>,
}

#[derive(Debug, Clone, Copy)]
pub struct ExitCode(isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExitStatus(isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExitStatusError(isize);

pub struct Process;

pub struct StdioPipes {
    pub stdin: Option<AnonPipe>,
    pub stdout: Option<AnonPipe>,
    pub stderr: Option<AnonPipe>,
}

impl From<AnonPipe> for Stdio {
    fn from(_: AnonPipe) -> Stdio {
        Stdio::Null
    }
}

impl From<File> for Stdio {
    fn from(_: File) -> Stdio {
        Stdio::Null
    }
}

impl Command {
    pub fn new(program: &OsStr) -> Command {
        Command { program: program.to_os_string(), args: Vec::new(), env: Default::default() }
    }

    pub fn arg(&mut self, _arg: &OsStr) {}

    pub fn env_mut(&mut self) -> &mut CommandEnv {
        &mut self.env
    }

    pub fn cwd(&mut self, _dir: &OsStr) {}

    pub fn stdin(&mut self, _stdin: Stdio) {}

    pub fn stdout(&mut self, _stdout: Stdio) {}

    pub fn stderr(&mut self, _stderr: Stdio) {}

    pub fn get_program(&self) -> &OsStr {
        &self.program
    }

    pub fn get_args(&self) -> CommandArgs<'_> {
        let iter = self.args.iter();
        CommandArgs { iter }
    }

    pub fn get_envs(&self) -> CommandEnvs<'_> {
        self.env.iter()
    }

    pub fn get_current_dir(&self) -> Option<&Path> {
        None
    }

    pub fn spawn(
        &mut self,
        _default: Stdio,
        _needs_stdin: bool,
    ) -> io::Result<(Process, StdioPipes)> {
        unsupported()
    }
}

impl<'a> Iterator for CommandArgs<'a> {
    type Item = &'a OsStr;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|arg| arg.as_ref())
    }
}

impl<'a> ExactSizeIterator for CommandArgs<'a> {
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(0);
    pub const FAILURE: ExitCode = ExitCode(1);

    #[inline]
    pub fn as_i32(&self) -> i32 {
        self.0 as i32
    }
}

impl Process {
    pub fn kill(&mut self) -> io::Result<()> {
        unsupported()
    }

    pub fn id(&self) -> u32 {
        0
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        unsupported()
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        unsupported()
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 & 0x80000000 != 0 {
            write!(f, "exit code: {:#x}", self.0)
        } else {
            write!(f, "exit code: {}", self.0)
        }
    }
}

impl ExitStatus {
    pub fn exit_ok(&self) -> Result<(), ExitStatusError> {
        Ok(())
    }

    pub fn code(&self) -> Option<i32> {
        Some(self.0 as i32)
    }
}

impl Into<ExitStatus> for ExitStatusError {
    fn into(self) -> ExitStatus {
        ExitStatus(self.0.into())
    }
}

impl ExitStatusError {
    pub fn code(self) -> Option<NonZeroI32> {
        NonZeroI32::new(self.0 as i32)
    }
}
