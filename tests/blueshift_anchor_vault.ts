import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BlueshiftAnchorVault } from "../target/types/blueshift_anchor_vault";
import * as fs from "fs";
import * as path from "path";

describe("blueshift_anchor_vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .blueshiftAnchorVault as Program<BlueshiftAnchorVault>;

  it("Is initialized!", async () => {
    // #region agent log
    // Debug: capture program id and provider details before initialize()
    // Hypothesis H1: Program ID used by the client does not match the deployed program.
    // Hypothesis H2: Tests are pointing at a cluster where the program is not deployed.
    // Hypothesis H3: anchor.workspace is resolving to a stale/cached program ID.
    // Hypothesis H4: Program is not deployed to the test validator at the expected address.
    // Hypothesis H5: anchor.workspace uses Anchor.toml program ID override instead of IDL address.
    const logPath = "/home/dbm/vault_challenge_final/.cursor/debug.log";
    const provider = anchor.getProvider();
    const logEntry = {
      sessionId: "debug-session",
      runId: "post-fix-workspace-resolution",
      hypothesisId: "H1,H2,H3,H4,H5",
      location: "tests/blueshift_anchor_vault.ts:14",
      message: "Before initialize() call - workspace resolution",
      data: {
        programIdFromWorkspace: program.programId.toBase58(),
        expectedProgramId: "8mDnY22HT7JHHbUmRDGSyCX6qp3CMZbokK7Z899zvsuN",
        idlAddressFromType: (BlueshiftAnchorVault as any).address,
        providerCluster: provider.connection.rpcEndpoint ?? "",
        walletPubkey: provider.publicKey?.toBase58()?.toString() ?? "null",
        programMatchesExpected: program.programId.toBase58() === "8mDnY22HT7JHHbUmRDGSyCX6qp3CMZbokK7Z899zvsuN",
      },
      timestamp: Date.now(),
    };
    try {
      fs.appendFileSync(logPath, JSON.stringify(logEntry) + "\n");
      console.log("DEBUG: Program ID from workspace:", program.programId.toBase58());
      console.log("DEBUG: Expected program ID:", "8mDnY22HT7JHHbUmRDGSyCX6qp3CMZbokK7Z899zvsuN");
      console.log("DEBUG: Program ID matches expected:", logEntry.data.programMatchesExpected);
      console.log("DEBUG: IDL address from type:", (BlueshiftAnchorVault as any).address);
    } catch (e) {
      console.log("DEBUG: Failed to write log:", e);
    }
    // #endregion agent log

    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
