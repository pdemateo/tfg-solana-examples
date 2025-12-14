import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiSecureVault } from "../target/types/cpi_secure_vault";
import { CpiSecureProxy } from "../target/types/cpi_secure_proxy";
import { assert } from "chai";

describe("Sistema CPI Seguro (Integración)", () => {
  // Configurar el cliente (Provider)
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Cargamos los DOS programas
  const vaultProgram = anchor.workspace.CpiSecureVault as Program<CpiSecureVault>;
  const proxyProgram = anchor.workspace.CpiSecureProxy as Program<CpiSecureProxy>;

  // Generamos una cuenta nueva para la Bóveda
  const vaultAccount = anchor.web3.Keypair.generate();

  // Calculamos la PDA del Proxy (quien será la autoridad)
  // Semilla: "controller_auth" (definido en el código Rust del proxy)
  const [proxyPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("controller_auth")],
    proxyProgram.programId
  );

  it("1. Inicializar la Bóveda (Setup)", async () => {
    // Llamada directa al programa B para crear la cuenta
    await vaultProgram.methods
      .initialize()
      .accounts({
        vault: vaultAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([vaultAccount]) // Firmamos porque es una cuenta nueva
      .rpc();

    // Verificamos que empezó en 0
    const account = await vaultProgram.account.vaultData.fetch(vaultAccount.publicKey);
    console.log("Estado inicial Bóveda:", account.dataValue.toString());
    assert.ok(account.dataValue.eq(new anchor.BN(0)));
  });

  it("2. El Proxy modifica la Bóveda vía CPI", async () => {
    const newValue = new anchor.BN(42);

    // Llamamos al programa A (Proxy)
    await proxyProgram.methods
      .executeProxyUpdate(newValue) // Pasamos el 42
      .accounts({
        vault: vaultAccount.publicKey,       // La cuenta a modificar
        pdaAuthority: proxyPDA,              // La PDA que firmará virtualmente
        vaultProgram: vaultProgram.programId // ID del programa destino
      })
      // NOTA: NO pasamos la clave privada de la PDA (¡porque no existe!)
      // Anchor derivará la dirección y el programa inyectará la firma.
      .rpc();

    console.log("Proxy ejecutado. Verificando estado...");

    // 3. Verificamos en el programa B que el valor cambió
    const account = await vaultProgram.account.vaultData.fetch(vaultAccount.publicKey);
    console.log("Estado final Bóveda:", account.dataValue.toString());
    
    assert.ok(account.dataValue.eq(newValue));
  });
});