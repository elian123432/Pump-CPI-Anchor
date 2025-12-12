import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorProgram } from "../target/types/anchor_program";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { PublicKey,Connection , type GetSlotConfig} from "@solana/web3.js";

const web3 = anchor.web3;

describe("anchor-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const connection = new Connection(
    "https://api.mainnet-beta.solana.com",
    "processed"
  );

  const creator = new PublicKey("THEPOOLCREATOR");
  const user_publickey = new PublicKey("YOURPUBKEY");
  // BLK8JB6HNdkmr3j4DmVdwXYB7dVZunwvNVgcTZyDSX7q

  // CHECK YOUR PUMP POOL that you want to trade2 
  const base_mint = new PublicKey("BASEMINT");
  const quote_mint = new PublicKey("QUOTEMINT");
  const pool_base_token_account = new PublicKey("POOLBASETOKENACCOUNT");
  const pool_quote_token_account = new PublicKey("POOLQUOTETOKENACCOUNT");
    
  // can be derived or created with spl-token create-account <MINT>
  const user_base_token_account = new PublicKey("YOURBASETOKENACCOUNT");
  const user_quote_token_account = new PublicKey("YOURQUOTETOKENACCOUNT");


  const program = anchor.workspace.AnchorProgram as Program<AnchorProgram>;
  const program_id = new PublicKey("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");
  const event_authority = web3.PublicKey.findProgramAddressSync([Buffer.from("__event_authority")], program_id);
  

  const protocol_fee_recepient = new PublicKey("9rPYyANsfQZw3DnDmKE3YCQF5E8oD89UXoHn9JFEhJUz");
  const protocol_fee_recepient_token_account = new PublicKey("Bvtgim23rfocUzxVX9j9QFxTbBnH8JZxnaGLCEkXvjKS");

  const  userVolumePDA = web3.PublicKey.findProgramAddressSync([Buffer.from("user_volume_accumulator"), user_publickey.toBuffer()], program_id);
  
  const global_configPDA = web3.PublicKey.findProgramAddressSync([Buffer.from("global_config")], program_id);

  let index = 0; // is usually 0 
  const indexBuffer = Buffer.alloc(2);
  indexBuffer.writeUInt16LE(index);

  const poolPDA = web3.PublicKey.findProgramAddressSync([Buffer.from("pool"), indexBuffer ,
      creator.toBuffer(),  base_mint.toBuffer(), quote_mint.toBuffer()], program_id
    );


  it("testing stateless cpi", async () => {
    // Add your test here.

    
    
    let base_amount_out = new anchor.BN(0);   // u64 and bigger need anchor.BN
    let max_quote_amount_in = new anchor.BN(0);

    const tx = await program.methods.buyPumpSwapExactOut(
          index, 
          creator,
          base_amount_out,
          max_quote_amount_in,
     
    ).accounts({
            // @ts-ignore
            pool: poolPDA[0],
            // @ts-ignore
            user: user,
            globalConfig:global_configPDA[0],

            baseMint: base_mint,
            quoteMint: quote_mint,

            userBaseTokenAccount: user_base_token_account,
            userQuoteTokenAccount: user_quote_token_account,

            poolBaseTokenAccount: pool_base_token_account,
            poolQuoteTokenAccount: pool_quote_token_account,
          
            protocolFeeRecipient: protocol_fee_recepient,
            protocolFeeRecipientTokenAccount: protocol_fee_recepient_token_account,

            //baseTokenProgram:  base_token_program,
            //quoteTokenProgram: quote_token_program,
            //systemProgram: system_program,
            //associatedTokenProgram: associated_token_program,

            //eventAuthority: event_authority,
            program: program_id,  // must be even if wants to error

            //coinCreatorVaultAta: coin_creator_vault_ata,
            //coinCreatorVaultAuthority: coin_creator_vault_authority,

            //globalVolumeAccumulator: global_volume_accumulator,
            userVolumeAccumulator: userVolumePDA[0],
            // @ts-ignore
            feeConfig: fee_config,
            // @ts-ignore
            feeProgram: fee_program,
    
        }).rpc();
    console.log("Your transaction signature", tx);
  });
});
