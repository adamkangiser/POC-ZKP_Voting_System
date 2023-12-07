# Secure Voting System

## Overview
This project is a demonstration of a secure voting system using Zero-Knowledge Proofs (ZKPs), implemented in Rust. It showcases the use of cryptographic primitives, specifically focusing on zero-knowledge proofs, vector commitments, and elliptic curve cryptography.

## Features
- Implementation of a cryptographic library in Rust.
- Development of ZK circuits for validating the integrity of votes without revealing voter identities.
- Use of elliptic curve cryptography for secure key management.

## Technologies Used
- Rust
- `bellman` for Zero-Knowledge Proofs
- `bls12_381` for Elliptic Curve Cryptography

## Project Structure
- `main.rs`: The entry point of the application.
- `zkp.rs`: Contains the implementation of the Zero-Knowledge Proof circuit.

## Running the Project
To run the project, use the following command:
```bash
cargo run
