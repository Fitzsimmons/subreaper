extern crate libc;

use libc::{waitpid, c_int, prctl, PR_SET_CHILD_SUBREAPER, WEXITSTATUS};
use std::process::{Command, Stdio, Child};
use std::io::prelude::*;
use std::io;

struct GrandchildInfo {
    child_status: std::process::ExitStatus,
    grandchild_pid: i32
}

trait WaitForGrandchildPid {
    fn wait_for_grandchild_pid(&mut self) -> io::Result<GrandchildInfo>;
}

impl WaitForGrandchildPid for Child {
    fn wait_for_grandchild_pid(&mut self) -> io::Result<GrandchildInfo> {
        drop(self.stdin.take());
        let mut input = Vec::new();
        let out = self.stdout.take().expect("couldn't get child's stdout stream");
        for option_c in out.bytes() {
            let c = option_c.expect("couldn't read bytes from the child's stdout stream");
            if c == b'\n' {
                break;
            }
            input.push(c);
        }

        let pid_string = String::from_utf8(input).expect("couldn't convert the stream into a valid UTF-8 string");
        let grandchild_pid: i32 = pid_string.parse::<i32>().ok().expect("couldn't parse pid from child's output");

        let status = self.wait()?;

        Ok(GrandchildInfo{
            child_status: status,
            grandchild_pid: grandchild_pid
        })
    }
}

fn main() {
    unsafe { prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0); }

    let mut child = Command::new("ruby")
        .arg("layer2.rb")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    println!("waiting for child...");

    let output = child.wait_for_grandchild_pid().expect("failed to wait for child output");
    println!("Child exited with {}", output.child_status.code().expect("failed to get retval from child"));
    println!("waiting for grandchild pid to exit: {}", output.grandchild_pid);

    let mut status: c_int = 0;

    unsafe {
        waitpid(output.grandchild_pid, &mut status, 0);
        println!("{} exited with {}", output.grandchild_pid, WEXITSTATUS(status));
    }
}
