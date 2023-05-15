/// UnifiedSyscallError aims to simplify error handling of syscalls in
/// libcontainer. In many occasions, we mix nix::Error, std::io::Error and our
/// own syscall wrappers, which makes error handling complicated.
#[derive(Debug, thiserror::Error)]
pub enum UnifiedSyscallError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Nix(#[from] nix::Error),
    #[error(transparent)]
    Syscall(#[from] crate::syscall::SyscallError),
}

#[derive(Debug, thiserror::Error)]
pub enum MissingSpecError {
    #[error("missing process in spec")]
    MissingProcess,
    #[error("missing linux in spec")]
    MissingLinux,
    #[error("missing args in the process spec")]
    MissingArgs,
}
