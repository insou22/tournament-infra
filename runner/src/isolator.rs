use std::time::Duration;

use bollard::{Docker, container::{self, LogOutput}, exec::{self, CreateExecOptions, StartExecOptions}, models::{HostConfig, HostConfigCgroupnsModeEnum}};
use futures_util::StreamExt;
use tokio::{io::AsyncWriteExt, time::timeout};

const NANO_CPUS:    i64   = 1000 * 1000 * 1000; // 1 cpu
const MEMORY:       i64   = 256  * 1024 * 1024; // 256MiB
const PIDS:         i64   = 16;    // 16 procs
const NO_NETWORK:   bool  = true;  // no networking
const READ_ONLY_FS: bool  = true;  // no writing to fs
const PRIVILEGED:   bool  = false; // no privileged access to host
const TIMEOUT_MS:   u64   = 3000;  // 3s stdout cutoff timeout
const MAX_OUT_LEN:  usize = 1024;  // max output length = 1KiB

#[derive(Debug)]
pub struct ExecutionResult {
    stdout: String,
    stderr: String,
    timed_out: bool,
    output_too_long: bool,
}

pub fn connect_docker() -> Docker {
    Docker::connect_with_socket_defaults()
        .expect("Failed to connect to docker socket")
}

pub async fn create_container(docker: &Docker, binary_path: String) -> String {
    let host_config = HostConfig {
        binds:           Some(vec![format!("{}:/run:ro", binary_path)]),
        readonly_rootfs: Some(READ_ONLY_FS),
        ipc_mode:        Some(String::from("none")),
        cgroupns_mode:   Some(HostConfigCgroupnsModeEnum::PRIVATE),
        memory:          Some(MEMORY),
        nano_cpus:       Some(NANO_CPUS),
        pids_limit:      Some(PIDS),
        ..Default::default()
    };

    let container_config = container::Config {
        image: Some("busybox:glibc"),
        // keep the container alive until we're done
        cmd: Some(vec!["/bin/sh", "-c", "sleep infinity"]),
        network_disabled: Some(NO_NETWORK),
        host_config: Some(host_config),
        ..Default::default()
    };

    let container_id = docker.create_container::<&str, &str>(None, container_config)
        .await
        .expect("Failed to create glibc container")
        .id;

    docker.start_container::<String>(&container_id, None)
        .await
        .expect("Failed to start glibc container");

    container_id
}

pub async fn exec_container_binary(docker: &Docker, container_id: &str, stdin: String) -> ExecutionResult {
    let exec_id = docker.create_exec(container_id, CreateExecOptions {
        privileged: Some(PRIVILEGED),
        cmd:  Some(vec!["/run"]),
        user: Some("nobody"),
        attach_stdin:  Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        ..Default::default()
    }).await.expect("Failed to create run execution").id;

    let execution = docker.start_exec(&exec_id, Some(StartExecOptions {
        ..Default::default()
    })).await.expect("Failed to execute run");

    match execution {
        exec::StartExecResults::Attached { mut output, mut input } => {
            input.write_all(stdin.as_bytes())
                .await
                .expect("Failed to write stdin");
            
            input.flush()
                .await
                .expect("Failed to flush input");
            
            input.shutdown()
                .await
                .expect("Failed to close stream");

            let mut stdout = String::new();
            let mut stderr = String::new();
            let mut length = 0;
            let mut output_too_long = false;

            let output_collection = async {
                while let Some(Ok(msg)) = output.next().await {
                    match msg {
                        LogOutput::StdOut { message } => {
                            let message_len = message.len();

                            if length + message_len > MAX_OUT_LEN {
                                output_too_long = true;
                                stdout.push_str(&*String::from_utf8_lossy(&message[..(MAX_OUT_LEN - length)]));
                                break;
                            }

                            length += message_len;
                            stdout.push_str(&*String::from_utf8_lossy(&message));
                        }
                        LogOutput::StdErr { message } => {
                            let message_len = message.len();

                            if length + message_len > MAX_OUT_LEN {
                                output_too_long = true;
                                stderr.push_str(&*String::from_utf8_lossy(&message[..(MAX_OUT_LEN - length)]));
                                break;
                            }

                            length += message_len;
                            stderr.push_str(&*String::from_utf8_lossy(&message));
                        }
                        _ => {}
                    }
                }
            };

            let output_result = timeout(Duration::from_millis(TIMEOUT_MS), output_collection).await;

            ExecutionResult {
                stdout,
                stderr,
                timed_out: matches!(output_result, Err(_)),
                output_too_long,
            }
        }
        _ => unreachable!("Attached to stdin, stdout, stderr"),
    }
}

pub async fn teardown_container(docker: &Docker, container_id: &str) {
    docker.kill_container::<String>(&container_id, None)
        .await
        .expect("Failed to kill container");
    
    docker.remove_container(&container_id, None)
        .await
        .expect("Failed to remove container");
}

impl ExecutionResult {
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    pub fn timed_out(&self) -> bool {
        self.timed_out
    }

    pub fn output_too_long(&self) -> bool {
        self.output_too_long
    }
}
