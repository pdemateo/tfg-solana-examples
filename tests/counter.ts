import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { assert } from "chai";

describe("TFG: Pruebas de Integración para el Programa Contador", () => {
    // Configuración del proveedor de red local y wallet
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    // Carga del programa compilado utilizando el IDL generado
    const program = anchor.workspace.Counter as Program<Counter>;

    // Generación de un par de claves para la nueva cuenta de contador
    const counterAccount = anchor.web3.Keypair.generate();

    it("Debe inicializar el contador a 0", async () => {
        // Invocación RPC al método 'initialize_counter'
        await program.methods
            .initializeCounter()
            .accounts({
                counter: counterAccount.publicKey,
                payer: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([counterAccount]) // Firma requerida para la creación de la cuenta
            .rpc();
        
        // Fetch del estado en la blockchain para verificación
        const accountState = await program.account.counter.fetch(
            counterAccount.publicKey
        );
        
        // Aserción usando
        assert.ok(accountState.count.eqn(0), "El estado inicial debe ser 0");
    });

    it("Debe incrementar el contador", async () => {
        // Invocación RPC al método 'increment'
        await program.methods
            .increment()
            .accounts({
                counter: counterAccount.publicKey,
            })
            // Nota: No requiere signers adicionales, el provider firma por defecto
            .rpc();
        
        // Fetch del estado actualizado
        const accountState = await program.account.counter.fetch(
            counterAccount.publicKey
        );

        // Verificamos que el estado ha mutado correctamente
        assert.ok(accountState.count.eqn(1), "El contador debe ser 1 tras incremento");
    });
});