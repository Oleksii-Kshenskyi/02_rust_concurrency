extern crate num_cpus;
use lazy_static::lazy_static;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

static NUM_OF_ELEMS: usize = 500000000;
lazy_static! {
    static ref V: Vec<u64> = (1..=(NUM_OF_ELEMS as u64)).collect();
}

fn sum_mt() -> u64 {
    let chunk_size = get_chunk_size();
    let (send_ch, recv_ch): (Sender<u64>, Receiver<u64>) = channel();

    for chunk in V.chunks(chunk_size) {
        let send_ch = send_ch.clone();
        
        thread::spawn(move || {
            let sum = chunk.iter().sum();
            send_ch.send(sum).unwrap();
        });
    }

    let mut the_sum: u64 = 0;
    for _ in 0..16 {
        let local_sum = recv_ch.recv().unwrap();
        the_sum += local_sum;
    }

    the_sum
}

fn how_many_threads_to_use() -> usize {
    num_cpus::get()
}
fn get_chunk_size() -> usize {
    NUM_OF_ELEMS / how_many_threads_to_use()
}

fn main() {
    println!("Multithreaded sum is => {}!", sum_mt());
}
