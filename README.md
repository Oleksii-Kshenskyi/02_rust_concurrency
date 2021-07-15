# Rust Concurrency Test

This is a very small no-strings-attached learning project to understand more about Rust and concurrency. The idea of the project is to calculate the sum of a big array of integers. The array consists of integers of sequentially increasing values. In this project, the array consists of 500 mln. integers, from 1 to 500 000 000.

The machine this was tested on has Intel Core i9-10780 with 8 cores, and 16GB RAM, Windows 10 host + Arch Linux VM.

1. a_sum_singlethreaded: simple single-threaded summation in a loop: 
    - Debug, Win host: 26.5s
    - Debug, Arch VM: 22.5s
    - Release, Win host: 1.075s
    - Release, Arch VM: 1.180s
2. b_sum_multithreaded: multithreaded summation, array allocation in a single static block:
    - Debug, Win host: 16.5s
    - Debug, Arch VM: 12.5s
    - Release, Win host: 1.085s
    - Release, Arch VM: 1.105s
3. c_sum_mt_alloc: multithreaded summation, multithreaded allocation
    - Debug, Win host: 3.936s
    - Debug, Arch VM: 3.253s
    - Release, Win host: 0.899s
    - Release, Arch VM: 0.519s