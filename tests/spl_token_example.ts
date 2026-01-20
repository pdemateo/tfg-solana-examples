import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplTokenExample } from "../target/types/spl_token_example";
import { assert } from "chai";
import { 
  getAssociatedTokenAddress, 
  createAssociatedTokenAccount, 
  TOKEN_PROGRAM_ID 
} from "@solana/spl-token";

describe("TFG: Estándar SPL Token (Ciclo Completo)", () => {
  // Configuración del proveedor y conexión
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SplTokenExample as Program<SplTokenExample>;

  // --- VARIABLES DE PRUEBA ---
  
  // 1. La "Mint" (La moneda en sí)
  const mintKeypair = anchor.web3.Keypair.generate();
  
  // 2. Usuarios de prueba
  const wallet = provider.wallet as anchor.Wallet; // Nosotros (Admin/Authority)
  const alice = anchor.web3.Keypair.generate();    // Destinatario 1
  const bob = anchor.web3.Keypair.generate();      // Destinatario 2

  // Direcciones de las cuentas de tokens (se calcularán en el test)
  let aliceATA: anchor.web3.PublicKey;
  let bobATA: anchor.web3.PublicKey;

  it("Paso 1: Inicializar el Token (Mint Account)", async () => {
    const decimals = 9;

    // Llamamos a la instrucción create_mint
    await program.methods
      .createMint(decimals)
      .accounts({
        mint: mintKeypair.publicKey,
        payer: wallet.publicKey,
        authority: wallet.publicKey, // Nosotros controlamos la moneda
        tokenProgram: TOKEN_PROGRAM_ID,
        // systemProgram se infieren automáticamente
      })
      .signers([mintKeypair]) // Firmamos con la Mint porque es una cuenta nueva
      .rpc();

    console.log("Mint creada:", mintKeypair.publicKey.toBase58());
  });

  it("Setup Auxiliar: Crear Cuentas de Token (ATAs)", async () => {    
    // 1. Calculamos la dirección ATA para Alice y Bob
    aliceATA = await getAssociatedTokenAddress(
      mintKeypair.publicKey,
      alice.publicKey
    );
    bobATA = await getAssociatedTokenAddress(
      mintKeypair.publicKey,
      bob.publicKey
    );

    // 2. Creamos las cuentas realmente en la red (pagando nosotros el rent)
    // Usamos la librería @solana/spl-token directamente para este paso auxiliar
    await createAssociatedTokenAccount(
      provider.connection,
      wallet.payer,      // Paga la creación
      mintKeypair.publicKey,
      alice.publicKey    // Dueño de la cuenta
    );

    await createAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      mintKeypair.publicKey,
      bob.publicKey
    );

    console.log("ATAs creadas para Alice y Bob.");
  });

  it("Paso 2: Acuñar Tokens (MintTo)", async () => {
    const amountToMint = new anchor.BN(1000); // 1000 tokens (sin contar decimales)

    // Acuñamos tokens a la cuenta de Alice
    await program.methods
      .mintToken(amountToMint)
      .accounts({
        mint: mintKeypair.publicKey,
        destination: aliceATA,    // Destino: ATA de Alice
        authority: wallet.publicKey, // Autoridad: Nosotros
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    // Verificación: Consultamos el saldo de Alice
    const balance = await provider.connection.getTokenAccountBalance(aliceATA);
    console.log("Saldo de Alice tras acuñar:", balance.value.amount);
    
    assert.equal(balance.value.amount, "1000", "El saldo de Alice debería ser 1000");
  });

  it("Paso 3: Transferir Tokens (Alice -> Bob)", async () => {
    const transferAmount = new anchor.BN(500);

    // Alice transfiere 500 tokens a Bob
    await program.methods
      .transferToken(transferAmount)
      .accounts({
        from: aliceATA,
        to: bobATA,
        authority: alice.publicKey, // Alice debe firmar
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([alice]) // Inyectamos la firma de Alice
      .rpc();

    // Verificación final
    const aliceBalance = await provider.connection.getTokenAccountBalance(aliceATA);
    const bobBalance = await provider.connection.getTokenAccountBalance(bobATA);

    console.log("Saldo Final Alice:", aliceBalance.value.amount);
    console.log("Saldo Final Bob:", bobBalance.value.amount);

    assert.equal(aliceBalance.value.amount, "500", "Alice debería tener 500");
    assert.equal(bobBalance.value.amount, "500", "Bob debería tener 500");
  });
});