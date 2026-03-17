use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'r', long)]
    target_resistance: f64,
    #[arg(short = 'w', long)]
    target_wattage: f64,
    #[arg(short = 'i', long)]
    individual_wattage: f64,
    #[arg(short = 'f', long)]
    file_path: String,
}

struct BestResult {
    combo: Vec<f64>,
    error: f64,
    resistance: f64,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut inventory = load_resistors(&args.file_path)?;

    inventory.sort_by(|a, b| a.partial_cmp(b).expect("Invalid float"));
    inventory.dedup();

    let target_g = 1.0 / args.target_resistance;
    let v_squared = args.target_wattage * args.target_resistance;
    let min_resistors = (args.target_wattage / args.individual_wattage).ceil() as usize;

    let mut best: Option<BestResult> = None;

    println!("Target: {}Ω ({}S), Min Resistors: {}", args.target_resistance, target_g, min_resistors);

    for k in min_resistors..=12 {
        println!("Checking combinations of exactly {} resistors...", k);
        find_best_recursive(
            &inventory,
            target_g,
            v_squared,
            args.individual_wattage,
            k,
            0,
            0.0,
            &mut Vec::new(),
            &mut best,
        );

        if let Some(ref b) = best {
            if (b.error / args.target_resistance) < 0.00001 {
                println!("Perfect or near-perfect match found.");
                break;
            }
        }
    }

    if let Some(res) = best {
        println!("\n--- Final Result ---");
        println!("Resistance: {:.6} Ω", res.resistance);
        println!("Error:      {:.6} Ω ({:.4}%)", res.error, (res.error / args.target_resistance) * 100.0);

        let mut final_combo = res.combo.clone();
        final_combo.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut counts: HashMap<String, (usize, f64)> = HashMap::new();
        for &val in &final_combo {
            let key = format!("{}", val);
            let entry = counts.entry(key).or_insert((0, val));
            entry.0 += 1;
        }

        let mut sorted_counts: Vec<_> = counts.values().collect();
        sorted_counts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for (count, val) in sorted_counts {
            println!("  x{}  {} Ω", count, val);
        }
    }

    Ok(())
}

fn find_best_recursive(
    inventory: &[f64],
    target_g: f64,
    v_squared: f64,
    max_i_w: f64,
    remaining: usize,
    start_idx: usize,
    current_g: f64,
    current_combo: &mut Vec<f64>,
    best: &mut Option<BestResult>,
) {
    if remaining == 0 {
        let actual_r = 1.0 / current_g;
        let error = (actual_r - (1.0 / target_g)).abs();

        let mut power_safe = true;
        for &r in current_combo.iter() {
            if (v_squared / r) > max_i_w + 1e-9 {
                power_safe = false;
                break;
            }
        }

        if power_safe {
            if best.is_none() || error < best.as_ref().unwrap().error {
                *best = Some(BestResult {
                    combo: current_combo.clone(),
                    error,
                    resistance: actual_r,
                });
            }
        }
        return;
    }

    for i in start_idx..inventory.len() {
        let r_val = inventory[i];
        let g_val = 1.0 / r_val;
        let next_g = current_g + g_val;

        if next_g > target_g + 1e-12 {
            continue;
        }

        if next_g + (g_val * (remaining - 1) as f64) < target_g * 0.95 {
            break;
        }

        current_combo.push(r_val);
        find_best_recursive(
            inventory,
            target_g,
            v_squared,
            max_i_w,
            remaining - 1,
            i,
            next_g,
            current_combo,
            best,
        );
        current_combo.pop();
    }
}

fn load_resistors<P: AsRef<Path>>(path: P) -> io::Result<Vec<f64>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect())
}
