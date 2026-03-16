use anyhow::Result;
use fibonacci_lib::FibResult;
use zisk_sdk::{ZiskStdin, ElfBinary, ProofOpts, ProverClient, include_elf};

pub const ELF: ElfBinary = include_elf!("guest");

fn main() -> Result<()> {
    let n = 10u32;
    let stdin = ZiskStdin::new();
    stdin.write(&n);

    let client = ProverClient::builder().build().unwrap();
    let (pk, vkey) = client.setup(&ELF)?;

    // Execute
    println!("[1/3] Executing...");
    let result = client.execute(&pk, stdin.clone())?;
    let output: FibResult = result.get_public_values()?;
    println!("      Fibonacci({}) = {}", output.n, output.value);
    println!("      Cycles: {}", result.get_execution_steps());

    // Prove
    println!("[2/3] Generating proof...");
    let proof_opts = ProofOpts::default().minimal_memory();
    let proof_result = client.prove(&pk, stdin)
        .with_proof_options(proof_opts)
        .run()?;

    // Verify
    println!("[3/3] Verifying...");
    client.verify(proof_result.get_proof(), proof_result.get_publics(), &vkey)?;

    let proven: FibResult = proof_result.get_public_values()?;
    println!("      Proven: Fibonacci({}) = {}", proven.n, proven.value);
    println!("      Proof verified!");

    Ok(())
}
