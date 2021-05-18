# helltaker-solutions
> Helltaker solutions that I've written using Rust

## Motivation
* I was playing the new [Helltaker](https://store.steampowered.com/app/1289310/Helltaker/) chapter last Friday and I was struggling with it's Phase 3. I was about to search for the solution online when I realized that it's kinda interesting to code a program that finds the solution for this specific puzzle. I was coding on and off during the weekend but I wasn't able to "finish" this until today. When I say "finish", I just reduced the number of states that the program has to check (using brute force) from 4 ^ 35 to 4 ^ 18 (so basically, I still searched for the answer online for the first 17 moves then check if my program can find the answer for the next 18 moves). If my computation was right, you need hundreds or even thousands of years for the program to finish it's computation (using my AMD Ryzen 5 3600 CPU running at base clock). Reducing to 4 ^ 18 states (with some minor pruning) runs the program for about an hour or so. 
* So, why was I doing this even though my program won't find the solution in a decent amount of time? I dunno, I just feel like doing this that's why to somehow validate that my program is correct, I reduced the number of states.

## Prerequisites
* [Rust](https://www.rust-lang.org/) and Cargo

## Running the program
* `cargo run`

### Notes
* This is definitely not the most optimal solution, I just used a brute force solution with very minimal pruning.
* There's some code blocks that's kind of redundant but some of them are intentionally done for readability purposes.
* My program is not guaranteed to find the solution (given the full number of states) even if you have thousands of years cause it may encounter edge cases that I haven't encountered for the reduced number of states.
* Will I create a solution for the other chapters? Probably not. If I did, it'll probably be the same brute force algorithm, handling all the cases and a lot of debugging on the reduced number of states.