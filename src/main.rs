mod sampler;
// mod wav_io;
// mod dft;
// mod filters;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ss-audio-dsp", about = "EEE 2207: Signals & Systems Audio DSP")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Simulate the Nyquist Sampling Theorem
    Sample {
        #[arg(short, long, default_value_t = 2.0)]
        analog_freq: f64,
        #[arg(short, long, default_value_t = 10.0)]
        sample_rate: f64,
    },
    // Add these later:
    // Analyze { file: String }, 
    // Filter { file: String, cutoff: f64 },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Sample { analog_freq, sample_rate } => {
            sampler::run_simulation(*analog_freq, *sample_rate);
        }
    }
}