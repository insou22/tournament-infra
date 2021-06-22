#![allow(unused)]

use crate::isolator::*;
use std::time::Instant;
use bollard::{Docker, container::{self, LogOutput}, exec::{self, CreateExecOptions, StartExecOptions}, models::{HostConfig, HostConfigCgroupnsModeEnum, ResourcesUlimits}};
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

pub async fn play(binary_1: String, binary_2: String) {
    let mut state = GameState::random_starting_state();

    // connect to docker daemon socket
    let docker = connect_docker();

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
    let result = exec_container_binary(&docker, &p1_container, stdin).await;
    let duration = Instant::now().duration_since(i);
    println!("  {:?}", result);
    println!("  duration: {:?}", duration);

    // execute binary again
    println!("exec 2:");
    let stdin = String::from("jeff\n");
    println!("  stdin={:?}", stdin);
    let i = Instant::now();
    let result = exec_container_binary(&docker, &p1_container, stdin).await;
    let duration = Instant::now().duration_since(i);
    println!("  {:?}", result);
    println!("  duration: {:?}", duration);

    // teardown the container
    let i = Instant::now();
    teardown_container(&docker, &p1_container).await;
    let duration = Instant::now().duration_since(i);
    println!("Teardown: {:?}", duration);

    // println!("Sleep:");
    // let container_id = create_container(&docker, "/home/zac/dev/tournament-infra/runner/sleep".to_string()).await;
    // println!("  {:?}", exec_container_binary(&docker, &container_id, String::new()).await);
    // teardown_container(&docker, &container_id).await;

    // println!("Loudmouth:");
    // let container_id = create_container(&docker, "/home/zac/dev/tournament-infra/runner/loudmouth".to_string()).await;
    // println!("  {:?}", exec_container_binary(&docker, &container_id, String::new()).await);
    // teardown_container(&docker, &container_id).await;

    println!("F-Bomb:");
    let container_id = create_container(&docker, "/home/zac/dev/tournament-infra/runner/f_bomb".to_string()).await;
    println!("  {:?}", exec_container_binary(&docker, &container_id, String::new()).await);
    teardown_container(&docker, &container_id).await;
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
