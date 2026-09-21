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

fn main() {
    let frag_a = "ATGCTT";
    let frag_b = "GCTTGA";
    let frag_c = "TTGAAA";

    println!(
        "Overlap A -> B: {} letters",
        compute_overlap(frag_a, frag_b)
    );
    println!(
        "Overlap B -> C: {} letters",
        compute_overlap(frag_b, frag_c)
    );
    println!(
        "Overlap A -> C: {} letters",
        compute_overlap(frag_a, frag_c)
    );
    println!(
        "Overlap B -> A: {} letters",
        compute_overlap(frag_b, frag_a)
    );
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
