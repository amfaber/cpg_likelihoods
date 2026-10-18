use clap::{Args, Parser, Subcommand};
use cpg_likelihoods::FilterState;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "cpg-likelihoods",
    version,
    about = "Infer CpG states from bedMethyl counts"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Estimate the CpG-to-non-CpG error rate
    EstimateError {
        /// Input bedMethyl file (plain, gzip, or BGZF)
        input: PathBuf,
        #[command(flatten)]
        estimation: EstimationArgs,
    },
    /// Append Phred-scaled likelihoods as columns 19-21
    Likelihoods {
        /// Input bedMethyl file (plain, gzip, or BGZF)
        input: PathBuf,
        /// Output file (.bed, .bed.gz, or .bed.bgz)
        #[arg(long)]
        out: PathBuf,
        /// Supply an error rate instead of estimating it
        #[arg(long)]
        e_c2n: Option<f64>,
        #[command(flatten)]
        estimation: EstimationArgs,
        /// Common likelihood scale factor (does not affect PLs)
        #[arg(long, default_value_t = 2.0)]
        n: f64,
    },
    /// Summarize CpG states from a PL-annotated bedMethyl file
    Summarize {
        input: PathBuf,
        #[arg(long, default_value_t = 20)]
        min_gq: u32,
    },
    /// Filter a PL-annotated bedMethyl file by CpG state
    Filter {
        input: PathBuf,
        #[arg(long)]
        out: PathBuf,
        /// CpG_hom, CpG_het, non-CpG, or CpG_hom/het
        #[arg(long)]
        state: FilterState,
        #[arg(long, default_value_t = 20)]
        min_gq: u32,
        /// Retain the three PL columns in the output
        #[arg(long)]
        keep_pl: bool,
    },
}

#[derive(Debug, Args)]
pub struct EstimationArgs {
    /// Minimum coverage for error-rate estimation
    #[arg(long, default_value_t = 10)]
    pub min_cov: u64,
    /// Maximum coverage for error-rate estimation
    #[arg(long, default_value_t = 100)]
    pub max_cov: u64,
    /// Minimum cov/(cov+d) for error-rate estimation
    #[arg(long, default_value_t = 0.8)]
    pub min_ratio: f64,
}
