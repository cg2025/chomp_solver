Chomp Solver (Rust / Solana Blockchain)

This project implements a solver for the combinatorial game Chomp using Rust. It constructs the full reachable state space, evaluates positions using a minimax-style dynamic programming approach, and identifies optimal moves from any board state.

The solver is designed as a Solana-oriented system, focused on deterministic game logic in Rust. It runs off-chain for fast evaluation but is structured to be adaptable for on-chain deployment, game verification, and trustless competitive systems on Solana.

Game Overview

Chomp is a two-player grid-based game where players alternately select a cell, removing it and everything to its right and below. The objective is to avoid being forced into the losing state.

This Solver
1. Generates all valid game states up to a fixed board size
2. Builds a state transition graph
3. Computes win/loss values using backward induction
4. Determines optimal moves from any given position
