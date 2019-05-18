// Copyright (c) 2019 CtrlC developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::io;


/// Platform specific error type
pub type Error = io::Error;

/// Platform specific signal type
#[derive(Debug)]
#[repr(i32)]
pub enum Signal {
    SIGHUP,
    SIGINT,
    SIGQUIT,
    SIGILL,
    SIGTRAP,
    SIGABRT,
    SIGBUS,
    SIGFPE,
    SIGKILL,
    SIGUSR1,
    SIGSEGV,
    SIGUSR2,
    SIGPIPE,
    SIGALRM,
    SIGTERM,
    #[cfg(all(any(target_os = "android", target_os = "emscripten", target_os = "linux"),
              not(any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc64"))))]
    SIGSTKFLT,
    SIGCHLD,
    SIGCONT,
    SIGSTOP,
    SIGTSTP,
    SIGTTIN,
    SIGTTOU,
    SIGURG,
    SIGXCPU,
    SIGXFSZ,
    SIGVTALRM,
    SIGPROF,
    SIGWINCH,
    SIGIO,
    #[cfg(any(target_os = "android", target_os = "emscripten", target_os = "linux"))]
    SIGPWR,
    SIGSYS,
    #[cfg(not(any(target_os = "android", target_os = "emscripten", target_os = "linux")))]
    SIGEMT,
    #[cfg(not(any(target_os = "android", target_os = "emscripten", target_os = "linux")))]
    SIGINFO,
}


/// Register os signal handler.
///
/// Must be called before calling [`block_ctrl_c()`](fn.block_ctrl_c.html)
/// and should only be called once.
///
/// # Errors
/// Will return an error if a system error occurred.
///
#[inline]
pub unsafe fn init_os_handler() -> Result<(), Error> {
    unimplemented!()
}

/// Blocks until a Ctrl-C signal is received.
///
/// Must be called after calling [`init_os_handler()`](fn.init_os_handler.html).
///
/// # Errors
/// Will return an error if a system error occurred.
///
#[inline]
pub unsafe fn block_ctrl_c() -> Result<(), Error> {
    unimplemented!()
}
