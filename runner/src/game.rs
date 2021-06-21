#![allow(unused)]

use std::time::Instant;

use bollard::{Docker, container::{self, LogOutput}, exec::{self, CreateExecOptions, StartExecOptions}, models::HostConfig};
use rand::seq::SliceRandom;
use tokio::io::AsyncWriteExt;
use futures_util::stream::StreamExt;

#[derive(Clone, Copy)]
enum Who {
    P1,
    P2,
}

#[derive(Clone, Copy)]
struct Card {
    rank: u32,
    suit: u32,
}

#[derive(Clone, Copy)]
struct Trick {
    first: Who,
    p1_card: Card,
    p2_card: Card,
}

#[derive(Clone, Copy)]
enum PartialTrick {
    First,
    Second(Card),
}

struct GameState {
    p1_hand: Vec<Card>,
    p2_hand: Vec<Card>,
    prev_tricks: Vec<Trick>,
    turn: Who,
    curr: PartialTrick,
}

const EOF: u8 = -1i8 as u8;

pub async fn play(binary_1: String, binary_2: String) {
    let mut state = GameState::random_starting_state();

    // connect to docker daemon socket
    let docker = Docker::connect_with_socket_defaults()
        .expect("Failed to connect to docker socket");

    // create the container
    let i = Instant::now();
    let p1_container = create_container(&docker, binary_1)
        .await;
    let duration = Instant::now().duration_since(i);
    println!("container creation: {:?}", duration);

    // execute binary
    println!("exec 1:");
    let stdin = String::from("zac");
    println!("  stdin={:?}", stdin);
    let i = Instant::now();
    let (stdout, stderr) = exec(&docker, &p1_container, stdin).await;
    let duration = Instant::now().duration_since(i);
    println!("  stdout={:?}, stderr={:?}", stdout, stderr);
    println!("  duration: {:?}", duration);

    // execute binary again
    println!("exec 2:");
    let stdin = String::from("jeff\n");
    println!("  stdin={:?}", stdin);
    let i = Instant::now();
    let (stdout, stderr) = exec(&docker, &p1_container, stdin).await;
    let duration = Instant::now().duration_since(i);
    println!("  stdout={:?}, stderr={:?}", stdout, stderr);
    println!("  duration: {:?}", duration);

    // teardown the container
    let i = Instant::now();
    teardown_container(&docker, &p1_container).await;
    let duration = Instant::now().duration_since(i);
    println!("Teardown: {:?}", duration);
}

async fn create_container(docker: &Docker, binary: String) -> String {
    let host_config = HostConfig {
        binds: Some(vec![format!("{}:/run:ro", binary)]),
        ..Default::default()
    };

    let container_config = container::Config {
        image: Some("busybox:glibc"),
        cmd: Some(vec!["/bin/sh", "-c", "adduser -DH -h / worker; sleep infinity"]),
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

async fn exec(docker: &Docker, container: &str, stdin: String) -> (String, String) {
    let exec_id = docker.create_exec(container, CreateExecOptions {
        privileged: Some(false),
        cmd:  Some(vec!["su", "-", "worker", "-c", "/run"]),
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

            while let Some(Ok(msg)) = output.next().await {
                match msg {
                    LogOutput::StdOut { message } => {
                        stdout.push_str(&*String::from_utf8_lossy(&message));
                    }
                    LogOutput::StdErr { message } => {
                        stderr.push_str(&*String::from_utf8_lossy(&message));
                    }
                    _ => {}
                }
            }

            (stdout, stderr)
        }
        _ => unreachable!("Attached to stdin, stdout, stderr"),
    }
}

async fn teardown_container(docker: &Docker, container: &str) {
    docker.kill_container::<String>(&container, None)
        .await
        .expect("Failed to kill container");
    
    docker.remove_container(&container, None)
        .await
        .expect("Failed to remove container");
}

impl GameState {
    fn random_starting_state() -> Self {
        let mut rng = rand::thread_rng();
        let mut cards = vec![];

        for rank in 1..5 {
            for suit in 0..4 {
                cards.push(Card { rank, suit });
            }
        }

        cards.shuffle(&mut rng);

        let (p1_hand, p2_hand) = cards.split_at(cards.len() / 2);
        let (p1_hand, p2_hand): (Vec<Card>, Vec<Card>) = (p1_hand.iter().copied().collect(), p2_hand.iter().copied().collect());

        let turn = if rand::random() { Who::P1 } else { Who::P2 };

        GameState {
            p1_hand,
            p2_hand,
            prev_tricks: vec![],
            turn,
            curr: PartialTrick::First,
        }
    }
}
