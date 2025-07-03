#[cfg(test)]
mod tests {
    use solana_client::rpc_client::RpcClient;
    use solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    };
    use solana_sdk::{
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
    };
    use std::str::FromStr;

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn submit_and_mint() {
        // Step 1: Create a Solana RPC client
        let rpc_client = RpcClient::new(RPC_URL.to_string());

        // Step 2: Load your signer keypair (your main Turbin3 wallet)
        let signer =
            read_keypair_file("Turbin3-wallet.json").expect("Couldn't find Turbin3-wallet.json");

        // Step 3: Define program and account public keys
        let turbin3_prereq_program =
            Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection =
            Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program =
            Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        
        // A new keypair for the NFT we're about to mint
        let mint = Keypair::new();

        // Step 4: Get the PDAs
        // Create a binding for the signer's public key to extend its lifetime
        let signer_pubkey = signer.pubkey();

        // PDA for your enrollment account
        let prereq_seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _prereq_bump) =
            Pubkey::find_program_address(prereq_seeds, &turbin3_prereq_program);

        // PDA for the collection authority
        let authority_seeds = &[b"collection", collection.as_ref()];
        let (authority_pda, _authority_bump) =
            Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);

        // Step 5: Prepare the instruction data (discriminator)
        // This is for the `submit_rs` instruction
        let instruction_data = vec![77, 124, 82, 163, 21, 133, 181, 206];

        // Step 6: Define the accounts metadata
        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new(prereq_pda, false),
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(collection, false),
            AccountMeta::new_readonly(authority_pda, false),
            AccountMeta::new_readonly(mpl_core_program, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ];

        // Step 7: Get the recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
        // Step 8: Build the instruction
        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data: instruction_data,
        };
        
        // Step 9: Create and sign the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );

        // Step 10: Send and confirm the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
