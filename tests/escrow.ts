import { Escrow } from "./../target/types/escrow";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  clusterApiUrl,
  Commitment,
  Connection,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import {
  createAccount,
  createAssociatedTokenAccount,
  createMint,
  getAccount,
  mintToChecked,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { assert } from "chai";

describe("escrow", () => {
  const commitment: Commitment = "processed";
  const connection = new Connection("http://localhost:8899/", commitment);
  const wallet = NodeWallet.local();
  const options = anchor.Provider.defaultOptions();

  const provider = new anchor.Provider(connection, wallet, options);

  anchor.setProvider(provider);

  const program = anchor.workspace.Escrow as Program<Escrow>;

  let initializerTokenAccountA = null;
  let initializerTokenAccountB = null;
  let takerTokenAccountA = null;
  let takerTokenAccountB = null;
  let vault_account_pda = null;
  let vault_account_bump = null;
  let vault_authority_pda = null;

  const takerAmount = 1000;
  const initializerAmount = 500;

  const escrowAccount = anchor.web3.Keypair.generate();
  const payer = anchor.web3.Keypair.generate();
  const mintAuthority = anchor.web3.Keypair.generate();
  const initializerMainAccount = anchor.web3.Keypair.generate();
  const takerMainAccount = anchor.web3.Keypair.generate();

  it("initialize program state", async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, 1000000000)
    );

    await provider.send(
      (() => {
        const tx = new Transaction();
        tx.add(
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: initializerMainAccount.publicKey,
            lamports: 100000000,
          }),
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: takerMainAccount.publicKey,
            lamports: 100000000,
          })
        );
        return tx;
      })(),
      [payer]
    );

    let mintA = await createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      0,
      null,
      null,
      TOKEN_PROGRAM_ID
    );

    let mintB = await createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      0,
      null,
      null,
      TOKEN_PROGRAM_ID
    );

    initializerTokenAccountA = await createAssociatedTokenAccount(
      connection,
      payer,
      mintA,
      initializerMainAccount.publicKey
    );
    takerTokenAccountA = await createAssociatedTokenAccount(
      connection,
      payer,
      mintA,
      takerMainAccount.publicKey
    );

    initializerTokenAccountB = await createAssociatedTokenAccount(
      connection,
      payer,
      mintB,
      initializerMainAccount.publicKey
    );
    takerTokenAccountB = await createAssociatedTokenAccount(
      connection,
      payer,
      mintB,
      takerMainAccount.publicKey
    );

    await mintToChecked(
      connection,
      payer,
      mintA,
      initializerTokenAccountA,
      mintAuthority.publicKey,
      initializerAmount,
      8
    );

    await mintToChecked(
      connection,
      payer,
      mintB,
      initializerTokenAccountB,
      mintAuthority.publicKey,
      takerAmount,
      8
    );

    let _initializerTokenAccountA = await getAccount(
      connection,
      initializerTokenAccountA
    );
    let _takerTokenAccountB = await getAccount(connection, takerTokenAccountB);

    assert.ok(
      parseInt(_initializerTokenAccountA.amount.toString()) == initializerAmount
    );
    assert.ok(parseInt(_takerTokenAccountB.amount.toString()) == takerAmount);
  });
});
