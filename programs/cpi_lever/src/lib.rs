use anchor_lang::prelude::*;
// Importamos los tipos del programa destino para tener seguridad de tipos
use cpi_engine::cpi::accounts::SetRpm;
use cpi_engine::program::CpiEngine;

declare_id!("2XAFBGfTy3NXB33XAh26MQwSeLo1Ku8tErHvjrMdWxKN");

// --- LÓGICA DEL PROGRAMA ---

#[program]
pub mod cpi_lever {
    use super::*;

    /// Acciona la palanca para enviar una instrucción al Motor.
    /// Actúa como un proxy: recibe la orden y la delega mediante CPI.
    pub fn switch_lever(ctx: Context<SwitchLever>, new_rpm: u64) -> Result<()> {
        
        // 1. Referencia al programa destino (Engine)
        let cpi_program = ctx.accounts.engine_program.to_account_info();
        
        // 2. Configuración de las cuentas que necesita el destino
        let cpi_accounts = SetRpm {
            engine_account: ctx.accounts.engine_account.to_account_info(),
        };
        
        // 3. Creación del Contexto CPI
        // Usamos 'new' porque no necesitamos firmar con PDA
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // 4. Ejecución de la llamada Cross-Program
        cpi_engine::cpi::set_rpm(cpi_ctx, new_rpm)?;
        
        Ok(())
    }
}

// --- VALIDACIÓN DE CUENTAS ---

#[derive(Accounts)]
pub struct SwitchLever<'info> {
    /// CHECK: La validación de datos se delega al programa Engine.
    /// Solo verificamos que la cuenta sea mutable para poder pasarla.
    #[account(mut)]
    pub engine_account: AccountInfo<'info>,
    
    // Referencia al programa ejecutable para realizar la CPI
    pub engine_program: Program<'info, CpiEngine>,
}