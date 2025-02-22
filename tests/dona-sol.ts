import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DonaSol } from "../target/types/dona_sol";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("dona-sol", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DonaSol as Program<DonaSol>;

  const adminAccount = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("admin")], program.programId)[0];

  it("Create Settings / Admin Account!", async () => {
    // Add your test here.
    const tx = await program.methods.initSettings()
    .accountsPartial({
      user: provider.publicKey,
      settings: adminAccount,
      systemProgram: SYSTEM_PROGRAM_ID,
    })
    .rpc();

    console.log("\nSettings account created");

    const settings = await program.account.settings.fetch(adminAccount);
    console.log("\nUser: ", provider.publicKey.toBase58())
    console.log("Settings account Admibn: ", settings.admin.toBase58());
    console.log("\nYour transaction signature: ", tx);
  });
});
