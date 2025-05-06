use clap::Parser;
use core_affinity;
use std::thread;
use std::time::Duration;

/// A program that pegs specific CPU cores at a specified percentage usage
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The CPU core indices to peg (0-based, can specify multiple)
    #[arg(short, long, num_args = 1..)]
    core: Vec<usize>,

    /// Duration in seconds to run the CPU pegging
    #[arg(short, long, default_value_t = 60)]
    duration: u64,

    /// CPU usage percentage (1-100)
    #[arg(short, long, default_value_t = 100)]
    percentage: u8,
}

fn peg_core(core: usize, duration: u64, percentage: u8) {
    // Bind this thread to the specified core
    let core_ids = core_affinity::get_core_ids().expect("Failed to get core IDs");
    let core_id = core_ids.iter().find(|id| id.id == core);

    if core_id.is_none() {
        eprintln!("Unable to find core index {}", core);
        return;
    }

    let core_id = core_id.unwrap();

    core_affinity::set_for_current(*core_id);

    println!(
        "Pegging CPU core {} at {}% for {} seconds...",
        core_id.id, percentage, duration
    );

    // Calculate end time
    let end_time = std::time::Instant::now() + Duration::from_secs(duration);

    // Calculate sleep duration based on percentage
    let sleep_duration = Duration::from_micros((100 - percentage) as u64 * 100);
    let work_duration = Duration::from_micros(percentage as u64 * 100);

    // Tight loop that will peg the CPU at the specified percentage
    while std::time::Instant::now() < end_time {
        // Work phase
        let work_end = std::time::Instant::now() + work_duration;
        while std::time::Instant::now() < work_end {
            unsafe {
                std::ptr::read_volatile(&0);
            }
        }

        // Sleep phase
        thread::sleep(sleep_duration);
    }

    println!("Done pegging CPU core {}", core_id.id);
}

fn main() {
    let args = Args::parse();

    // Validate percentage
    if args.percentage > 100 {
        eprintln!("Error: Percentage must be between 1 and 100");
        std::process::exit(1);
    }

    // Get the available core IDs
    let core_ids = core_affinity::get_core_ids().expect("Failed to get core IDs");

    // Debug output to show available cores
    println!("Available CPU cores:");
    for core_id in &core_ids {
        println!("  Core ID: {}", core_id.id);
    }
    println!("Total cores detected: {}", core_ids.len());

    // Validate all requested core indices
    for &core in &args.core {
        if core > core_ids.len() {
            eprintln!(
                "Error: Core index {} is out of range. Available cores: 0-{}",
                core,
                core_ids.len()
            );
            std::process::exit(1);
        }
    }

    // Create a thread for each core
    let mut handles = vec![];
    for core in args.core {
        let duration = args.duration;
        let percentage = args.percentage;
        let handle = thread::spawn(move || {
            peg_core(core, duration, percentage);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}
