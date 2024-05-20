use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::{self, Write, BufWriter};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use progress_bar::ProgressBar;
use console_utils::{print_message, hide_cursor, show_cursor};

const BATCH_SIZE: usize = 10_000; // Increased batch size

fn main() -> io::Result<()> {
    // Hide the cursor at the start
    hide_cursor()?;

    // Get the input string and optional file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <input_string> [output_file]", args[0]);
        std::process::exit(1);
    }
    let input = &args[1];
    let output_file = if args.len() == 3 { &args[2] } else { "combinations.txt" };

    // Calculate the total number of combinations
    let total_combinations = 1 << input.len();
    let total_batches = (total_combinations + BATCH_SIZE - 1) / BATCH_SIZE;

    // Create atomic counters
    let counter_gen = Arc::new(AtomicUsize::new(0));
    let counter_write = Arc::new(AtomicUsize::new(0));

    // Print the start message for combination generation
    print_message("Generating letter combinations...")?;

    // Create and start the progress bar for combination generation
    let mut progress_bar_gen = ProgressBar::new(total_combinations)?;
    progress_bar_gen.update(0)?; // Initial display

    // Start a new thread for the progress counter for combination generation
    let counter_gen_clone = Arc::clone(&counter_gen);
    let total_combinations_clone = total_combinations;
    let handle_gen = thread::spawn(move || {
        loop {
            let processed_gen = counter_gen_clone.load(Ordering::Relaxed);
            if let Err(e) = progress_bar_gen.update(processed_gen) {
                eprintln!("Error updating progress bar: {}", e);
                return;
            }

            if processed_gen >= total_combinations_clone {
                if let Err(e) = progress_bar_gen.finish() {
                    eprintln!("Error finishing progress bar: {}", e);
                }
                if let Err(e) = print_message("Wrapping Up...") {
                    eprintln!("Error printing message: {}", e);
                }
                break;
            }
            thread::sleep(Duration::from_millis(100)); // Increase update frequency
        }
    });

    // Generate all combinations using parallel computation
    let combinations = generate_combinations(input, total_combinations, counter_gen);

    // Wait for the combination generation to finish
    handle_gen.join().expect("Thread panicked");

    // Print the completion message for combination generation
    print_message("\nCombining process is complete, will start creating text file.")?;
    print_message("Creating text file...")?;

    // Create and start the progress bar for file writing
    let total_progress_steps = total_combinations + total_batches; // Combinations + Flushes
    let mut progress_bar_write = ProgressBar::new(total_progress_steps)?;
    progress_bar_write.update(0)?; // Initial display

    // Start a new thread for the progress counter for file writing
    let counter_write_clone = Arc::clone(&counter_write);
    let total_progress_clone = total_progress_steps;
    let handle_write = thread::spawn(move || {
        loop {
            let processed_write = counter_write_clone.load(Ordering::Relaxed);
            if let Err(e) = progress_bar_write.update(processed_write) {
                eprintln!("Error updating progress bar: {}", e);
                return;
            }

            if processed_write >= total_progress_clone {
                if let Err(e) = progress_bar_write.finish() {
                    eprintln!("Error finishing progress bar: {}", e);
                }
                if let Err(e) = print_message("Wrapping Up...") {
                    eprintln!("Error printing message: {}", e);
                }
                break;
            }
            thread::sleep(Duration::from_millis(100)); // Increase update frequency
        }
    });

    // Create and write to the output file with progress tracking
    write_combinations_to_file(output_file, combinations, counter_write)?;
    
    // Wait for the file writing to finish
    handle_write.join().expect("Thread panicked");

    // Print the completion message for file writing
    print_message(&format!("\nFile creation is complete, {} is ready!", output_file))?;

    // Show the cursor at the end
    show_cursor()?;

    Ok(())
}

// Function to generate all combinations of uppercase and lowercase letters
fn generate_combinations(
    input: &str,
    total_combinations: usize,
    counter: Arc<AtomicUsize>,
) -> Vec<String> {
    let length = input.len();
    let mut results = Vec::with_capacity(total_combinations);

    // Start parallel computation
    (0..total_combinations)
        .into_par_iter()
        .map(|i| {
            let mut combination = String::with_capacity(length);
            for j in 0..length {
                let c = input.chars().nth(j).unwrap();
                if (i & (1 << j)) == 0 {
                    combination.push(c.to_ascii_lowercase());
                } else {
                    combination.push(c.to_ascii_uppercase());
                }
            }
            // Increment the counter
            counter.fetch_add(1, Ordering::Relaxed);
            combination
        })
        .collect_into_vec(&mut results);

    results
}

// Function to write combinations to a file with progress tracking
fn write_combinations_to_file(
    filename: &str,
    combinations: Vec<String>,
    counter: Arc<AtomicUsize>,
) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for batch in combinations.chunks(BATCH_SIZE) {
        for combination in batch {
            writeln!(writer, "{}", combination)?;
            // Increment the counter
            counter.fetch_add(1, Ordering::Relaxed);
        }
        writer.flush()?;
        // Increment the counter for flush
        counter.fetch_add(1, Ordering::Relaxed);
    }

    // Ensure final buffer flush
    writer.flush()?;
    // Increment the counter for the final flush
    counter.fetch_add(1, Ordering::Relaxed);

    Ok(())
}
