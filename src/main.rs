mod cli;
mod summarize_likelihoods;

use clap::Parser;
use cli::{Cli, Command};
use cpg_likelihoods::{
    append_pl_columns, estimate_error_c2n_from_bedmethyl, filter_bedmethyl_by_state,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Command::ErrorC2n { input, estimation } => {
            let e = estimate_error_c2n_from_bedmethyl(
                input,
                estimation.min_cov,
                estimation.max_cov,
                estimation.min_ratio,
            )?;
            println!("{e:.10}");
        }
        Command::Likelihoods {
            input,
            out,
            e_c2n,
            estimation,
            n,
        } => {
            let e = match e_c2n {
                Some(e) => {
                    if !(0.0..=1.0).contains(&e) {
                        return Err("e_c2n must be between 0 and 1".into());
                    }
                    eprintln!("Using manual e_c2n = {e:.10}");
                    e
                }
                None => {
                    let e = estimate_error_c2n_from_bedmethyl(
                        &input,
                        estimation.min_cov,
                        estimation.max_cov,
                        estimation.min_ratio,
                    )?;
                    eprintln!("Estimated e_c2n = {e:.10}");
                    e
                }
            };
            append_pl_columns(&input, &out, e, n)?;
            eprintln!("Wrote: {}", out.display());
        }
        Command::SummarizeLikelihoods { input, min_gq } => {
            summarize_likelihoods::run(&input, min_gq)?
        }
        Command::FilterLikelihoods {
            input_bed,
            output_bed,
            state,
            min_gq,
            keep_pl,
        } => {
            filter_bedmethyl_by_state(&input_bed, &output_bed, state, min_gq, keep_pl)?;
        }
    }
    Ok(())
}
