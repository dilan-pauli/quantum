fn compute_overlap(left: &str, right: &str) -> usize {
    // Takes the minimum length of the two strings.
    let max_possible = left.len().min(right.len());

    for len in (1..=max_possible).rev() {
        if left.ends_with(&right[..len]) {
            return len;
        }
    }

    0
}

fn build_qubo_matrix(fragments: &[&str], penalty: f64) -> Vec<Vec<f64>> {
    let num_of_fragments = fragments.len();
    let num_of_vars = num_of_fragments * num_of_fragments;

    let mut qubo = vec![vec![0.0; num_of_vars]; num_of_vars];

    let index = |frag: usize, slot: usize| -> usize { frag * num_of_fragments + slot };

    for i in 0..num_of_fragments {
        for j in 0..num_of_fragments {
            if i == j {
                continue;
            } // Don't compute_overlap for same fragments

            let overlap = compute_overlap(fragments[i], fragments[j]) as f64;

            // If there is some overlap
            if overlap > 0.0 {
                // Find the matrix index for the score
                for t in 0..(num_of_fragments - 1) {
                    let u = index(i, t);
                    let v = index(j, t + 1);

                    // Place the score in the matrix (lower score is higher, so neg it)
                    qubo[u][v] -= overlap;
                }
            }
        }
    }

    // Now we have a martix of scores, (if the input vector was very large this would take forever)

    // Analese the qudo to apply penalty, every slot t must have 1 fragment
    for t in 0..num_of_fragments {
        for i in 0..num_of_fragments {
            let u = index(i, t);
            qubo[u][u] -= penalty;

            for j in (i + 1)..num_of_fragments {
                let v = index(j, t);
                qubo[u][v] += 2.0 * penalty;
            }
        }
    }

    // every fragment i must appear in exactly 1 slot
    for i in 0..num_of_fragments {
        for t in 0..num_of_fragments {
            let u = index(i, t);
            qubo[u][u] -= penalty;

            for s in (t + 1)..num_of_fragments {
                let v = index(i, s);
                qubo[u][v] += 2.0 * penalty;
            }
        }
    }

    qubo
}

/// Render a QUBO matrix as an ANSI-colored heatmap (dark terminal assumed).
///
/// Color key:
///   red   = positive coupling (penalty / repulsion)
///   blue  = negative coupling (overlap reward / attraction)
///   gray  = zero (no direct interaction)
///
/// Row/column `i` is binary variable `x_i` = fragment `i / n` placed in slot
/// `i % n`. Diagonal entries are linear biases; off-diagonal are quadratic
/// couplings — exactly the two terms of a QUBO, H = Σ a_i x_i + Σ b_ij x_i x_j.
fn print_qubo(name: &str, m: &[Vec<f64>]) {
    let max = m.iter().flatten().fold(1.0_f64, |acc, &v| acc.max(v.abs()));

    println!("\n{name}  ({0} x {0})", m.len());
    for row in m {
        for &v in row {
            if v == 0.0 {
                print!("\x1b[48;2;35;35;35m {:6.1} \x1b[0m", v);
                continue;
            }
            let t = v / max; // in [-1, 1]
            let (r, g, b) = if v > 0.0 {
                // positive -> red, intensity scales with magnitude
                (255.0, (1.0 - t) * 180.0, (1.0 - t) * 180.0)
            } else {
                // negative -> blue
                ((1.0 + t) * 180.0, (1.0 + t) * 180.0, 255.0)
            };
            print!(
                "\x1b[48;2;{};{};{}m {:6.1} \x1b[0m",
                r as u8, g as u8, b as u8, v
            );
        }
        println!();
    }
    println!(
        "  key: \x1b[31mred = + penalty (repulsion)\x1b[0m  \x1b[34mblue = - overlap reward (attraction)\x1b[0m  gray = 0"
    );
}

fn main() {
    let frag_a = "ATGCTT";
    let frag_b = "GCTTGA";
    let frag_c = "TTGAAA";

    let fragmets = vec![frag_a, frag_b, frag_c];
    let penalty = 50.0;

    let qubo = build_qubo_matrix(&fragmets, penalty);
    let size = qubo.len();

    println!("Fragments count: {}", fragmets.len());
    println!("Total Yes/No switches (Qubits needed): {}", size);
    println!("QUBO Matrix Dimensions: {} x {}", size, size);

    print_qubo("QUBO matrix", &qubo);

    // Check an example:
    // Fragment 0 in Slot 0 (Index 0) and Fragment 1 in Slot 1 (Index 4)
    println!("\nReward interaction between (Frag 0, Slot 0) and (Frag 1, Slot 1):");
    println!("qubo[0][4] = {} (Overlap reward)", qubo[0][4]);

    println!("\nConflict interaction if both Frag 0 and Frag 1 try to take Slot 0:");
    println!("qubo[0][3] = {} (Penalty for collision)", qubo[0][3]);

    //     "Overlap A -> B: {} letters",
    //     compute_overlap(frag_a, frag_b)
    // );
    // println!(
    //     "Overlap B -> C: {} letters",
    //     compute_overlap(frag_b, frag_c)
    // );
    // println!(
    //     "Overlap A -> C: {} letters",
    //     compute_overlap(frag_a, frag_c)
    // );
    // println!(
    //     "Overlap B -> A: {} letters",
    //     compute_overlap(frag_b, frag_a)
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const FRAG_A: &str = "ATGCTT";
    pub const FRAG_B: &str = "GCTTGA";
    pub const FRAG_C: &str = "TTGAAA";

    #[test]
    fn test_genomes() {
        assert_eq!(compute_overlap(FRAG_A, FRAG_B), 4);
        assert_eq!(compute_overlap(FRAG_B, FRAG_C), 4);
        assert_eq!(compute_overlap(FRAG_A, FRAG_C), 2);
        assert_eq!(compute_overlap(FRAG_B, FRAG_A), 1);
    }

    #[test]
    fn test_one() {
        assert_eq!(compute_overlap(FRAG_A, FRAG_B), 4);
    }
}
