extern crate num_cpus;
use lazy_static::lazy_static;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

static NUM_OF_ELEMS: usize = 500000000;
lazy_static! {
    static ref THREAD_COUNT: usize = how_many_threads_to_use();
}
lazy_static! {
    static ref CHUNK_SIZE: usize = get_chunk_size();
}
lazy_static! {
    static ref BOUNDS: Vec<u64> = populate_bounds_vec();
}

fn populate_bounds_vec() -> Vec<u64> {
    let mut return_vec: Vec<u64> = vec![0; *THREAD_COUNT];
    for index in 0..*THREAD_COUNT {
        return_vec[index] = ((index * *CHUNK_SIZE) + 1) as u64; 
    }

    return_vec
}
fn vec_by_bounds_index(index: usize) -> Vec<u64> {
    (BOUNDS[index]..(BOUNDS[index] + (*CHUNK_SIZE as u64))).collect()
}

fn sum_mt() -> u64 {
    let (send_ch, recv_ch): (Sender<u64>, Receiver<u64>) = channel();
    for index in 0..BOUNDS.len() {
        let send_ch = send_ch.clone();
        
        thread::spawn(move || {
            let sumvec = vec_by_bounds_index(index);
            let sum = sumvec.iter().sum();
            send_ch.send(sum).unwrap();
        });
    }

    let mut the_sum: u64 = 0;
    for _ in 0..*THREAD_COUNT {
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
