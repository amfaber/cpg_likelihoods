mod cli;
mod summarize_likelihoods;

use clap::Parser;
use cli::{Cli, Command};
use cpg_likelihoods::{
    append_pl_columns, estimate_error_c2n_from_bedmethyl, filter_bedmethyl_by_state,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Command::ErrorC2n(args) => {
            let e = estimate_error_c2n_from_bedmethyl(
                &args.input,
                args.estimation.min_cov,
                args.estimation.max_cov,
                args.estimation.min_ratio,
            )?;
            println!("{e:.10}");
        }
        Command::Likelihoods(args) => {
            let e = match args.e_c2n {
                Some(e) => {
                    if !(0.0..=1.0).contains(&e) {
                        return Err("e_c2n must be between 0 and 1".into());
                    }
                    eprintln!("Using manual e_c2n = {e:.10}");
                    e
                }
                None => {
                    let e = estimate_error_c2n_from_bedmethyl(
                        &args.input,
                        args.estimation.min_cov,
                        args.estimation.max_cov,
                        args.estimation.min_ratio,
                    )?;
                    eprintln!("Estimated e_c2n = {e:.10}");
                    e
                }
            };
            append_pl_columns(&args.input, &args.out, e, args.n)?;
            eprintln!("Wrote: {}", args.out.display());
        }
        Command::SummarizeLikelihoods(args) => {
            summarize_likelihoods::run(&args.input, args.min_gq)?
        }
        Command::FilterLikelihoods(args) => {
            filter_bedmethyl_by_state(
                &args.input_bed,
                &args.output_bed,
                args.state,
                args.min_gq,
                args.keep_pl,
            )?;
        }
    }
    Ok(())
}
