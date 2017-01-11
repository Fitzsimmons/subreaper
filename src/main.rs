extern crate libc;

use libc::{waitpid, c_int, prctl, PR_SET_CHILD_SUBREAPER, WEXITSTATUS};
use std::process::{Command, Stdio, Child};
use std::io::prelude::*;
use std::io;

struct MyOutput {
    status: std::process::ExitStatus,
    grandchild_pid: i32
}

fn my_wait_with_output(ref mut child: Child) -> io::Result<MyOutput> {
    drop(child.stdin.take());
    let mut input = Vec::new();
    match child.stdout.take() {
        Some(out) => {
            for option_c in out.bytes() {
                let c = option_c.unwrap();
                if c == '\n' as u8 {
                    break;
                }
                input.push(c);
            }
        },
        None => {
            return Err(io::Error::new(io::ErrorKind::Other, "Couldn't read grandchild pid from process"));
        }
    }

    let pid_string = String::from_utf8(input).expect("Couldn't convert the stream into valid UTF-8");
    let grandchild_pid: i32 = pid_string.parse::<i32>().ok().expect("could not parse pid from child's output");

    let status = child.wait()?;

    Ok(MyOutput{
        status: status,
        grandchild_pid: grandchild_pid
    })
}

fn main() {
    unsafe { prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0); }

    let child = Command::new("ruby")
        .arg("layer2.rb")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    println!("waiting for child...");

    // let output = child.wait_with_output().expect("failed to wait for output");
    let output = my_wait_with_output(child).expect("failed to wait for output");
    println!("Child exited with {}", output.status.code().expect("failed to get retval from child"));
    println!("waiting for grandchild pid to exit: {}", output.grandchild_pid);

    let mut status: c_int = 0;

    unsafe {
        waitpid(output.grandchild_pid, &mut status, 0);
        println!("{} exited with {}", output.grandchild_pid, WEXITSTATUS(status));
    }
}
