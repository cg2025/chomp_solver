use std::collections::{HashMap, HashSet};

/// print the board 
pub fn pretty_print_s(board: [u8; 5]) {
    println!("Board (s array) rows 1..5 (binary, 1 = eaten):");
    for (i, &row) in board.iter().enumerate() {
        println!("row {}: {:08b} (hex 0x{:02x})", i + 1, row, row);
    }
}

// Recursive function to generate all possible state vectors
fn get_states(start: u8, m: usize) -> Vec<Vec<u8>> {
    if m == 1 {
        vec![vec![start]]
    } else {
        let mut result = Vec::new();
        for i in 0..=start {
            for mut rest in get_states(start - i, m - 1) {
                let mut v = vec![start];
                v.append(&mut rest);
                result.push(v);
            }
        }
        result
    }
}

// Generate next states from a given state
fn get_next_states(state: &[u8]) -> Vec<Vec<u8>> {
    let mut next_states = Vec::new();
    for i in 0..5 {
        let old_tuple = &state[..i];
        for k in 1..=state[i] {
            let x = state[i] - k;
            let new_tuple: Vec<u8> = state[i..]
                .iter()
                .map(|&l| std::cmp::min(l, x))
                .collect();
            let mut next_state = old_tuple.to_vec();
            next_state.extend(new_tuple);
            next_states.push(next_state);
        }
    }
    next_states
}

fn main() {
    let mut state_space: Vec<Vec<u8>> = Vec::new();
    let mut qf: HashMap<Vec<u8>, (i32, i32)> = HashMap::new();

    // Initialize state space
    for s in 1..=8 {
        state_space.extend(get_states(s, 5));
    }

    state_space.sort(); 

    // Initialize Qf to (0,0)
    for state in &state_space {
        qf.insert(state.clone(), (0, 0));
    }

    // Base case
    qf.insert(vec![0, 0, 0, 0, 0], (-1, 1));

    // Compute Q values iteratively
    for state in &state_space {
        let next_states = get_next_states(state);
        let minimax: Vec<i32> = next_states
            .iter()
            .map(|s| qf.get(s).unwrap_or(&(0, 0)).1)
            .collect();
        let maximin: Vec<i32> = next_states
            .iter()
            .map(|s| qf.get(s).unwrap_or(&(0, 0)).0)
            .collect();
        let min_minimax = *minimax.iter().min().unwrap_or(&0);
        let max_maximin = *maximin.iter().max().unwrap_or(&0);
        qf.insert(state.clone(), (min_minimax, max_maximin));
    }

    // Collect losing states
    let losing_states: HashSet<Vec<u8>> = state_space
        .iter()
        .filter(|s| qf.get(*s).unwrap().1 == -1)
        .cloned()
        .collect();

    let board: [u8; 5] = [0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000];
    pretty_print_s(board);
    
    let state: Vec<u8> = board.iter().rev().map(|&row| (8 - row.count_ones()) as u8).collect();
    println!("State: {:?}", state);
    
    let next = get_next_states(&state);

    let winning: Vec<Vec<u8>> = next
        .iter() // iterate by reference: &Vec<u8>
        .filter(|s| qf.get(*s).map_or(false, |v| v.1 == -1)) // *s dereferences &Vec<u8> to Vec<u8>
        .cloned() // clone Vec<u8> for the result
        .collect();
 
    //for s in winning {
    //    println!("winning : {:?}", s);
    //}

    // Pick the first winning state (or any other selection)
    if let Some(target_state) = winning.first() {
        let target_state = target_state.clone(); // get ownership

        // Compute element-wise differences
        let differences: Vec<u8> = state
            .iter()
            .zip(target_state.iter())
            .map(|(&s, &p)| s - p )
            .collect();

        println!("Target state: {:?}", target_state);

        // Find the lowest non-zero index and the value at that index
        if let Some(idx) = differences.iter().position(|&d| d != 0) {
            println!("Best Move:({},{})", idx+1,differences[idx]);
        } else {
            println!("No differences found");
        }
    } else {
        println!("No winning states available");
    }
}

