import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiEngine } from "../target/types/cpi_engine";
import { CpiLever } from "../target/types/cpi_lever";
import { assert } from "chai";

describe("TFG: CPI Architecture (Engine & Lever)", () => {
  // Configuración del proveedor y conexión al nodo local
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Carga de los programas desde el Workspace
  const engineProgram = anchor.workspace.CpiEngine as Program<CpiEngine>;
  const leverProgram = anchor.workspace.CpiLever as Program<CpiLever>;

  // Generación de una nueva cuenta para el motor (Keypair)
  const engineAccount = anchor.web3.Keypair.generate();

  it("Paso 1: Inicialización del Motor (Estado Base)", async () => {
    // Llamada directa al programa Engine para crear la cuenta
    await engineProgram.methods
      .initialize()
      .accounts({
        engineAccount: engineAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([engineAccount])
      .rpc();

    // Verificación: Recuperamos el estado de la blockchain
    const accountState = await engineProgram.account.engineStats.fetch(
      engineAccount.publicKey
    );
    
    // Aserción: El motor debe estar parado (0 RPM)
    assert.ok(accountState.rpm.eqn(0), "Error: Las RPM iniciales no son 0");
    console.log("Motor inicializado correctamente.");
  });

  it("Paso 2: Ejecución de CPI (Lever -> Engine)", async () => {
    const targetRpm = new anchor.BN(5000);

    // Llamamos al programa LEVER (Intermediario)
    await leverProgram.methods
      .switchLever(targetRpm) 
      .accounts({
        engineAccount: engineAccount.publicKey, // Cuenta a modificar
        engineProgram: engineProgram.programId, // Programa destino
      })
      .rpc();

    console.log("Transacción CPI enviada.");

    // Verificación Final: Leemos el estado del ENGINE
    const accountState = await engineProgram.account.engineStats.fetch(
      engineAccount.publicKey
    );
    
    console.log("Estado final del Motor:", accountState.rpm.toString(), "RPM");
    
    // Aserción: El valor debe haber cambiado a 5000
    assert.ok(
      accountState.rpm.eq(targetRpm), 
      "FALLO: La CPI no actualizó el estado del motor"
    );
  });
});