use anchor_lang::prelude::*;
// Importamos los tipos del programa destino para tener seguridad de tipos
use cpi_secure_vault::program::CpiSecureVault;
use cpi_secure_vault::cpi::accounts::AccessVault;
use cpi_secure_vault::{self, cpi};

declare_id!("BfZQac4ivDdAC9jhvWFV5sdaioSRpmKK4U3ND6nCLYkG");

// --- LÓGICA DEL PROGRAMA ---

#[program]
pub mod cpi_secure_proxy {
    use super::*;

    pub fn execute_proxy_update(ctx: Context<ProxyUpdate>, value: u64) -> Result<()> {
        // 1. Definimos las semillas de la PDA (Identidad del Controlador)
        let seeds: &[&[u8]] = &[
            b"controller_auth",
            &[ctx.bumps.pda_authority]
        ];
        let signer_seeds = &[&seeds[..]];

        // 2. Preparamos las cuentas que exige la Bóveda
        let cpi_accounts = AccessVault {
            vault: ctx.accounts.vault.to_account_info(),
            vault_authority: ctx.accounts.pda_authority.to_account_info(), // Nuestra PDA
        };

        // 3. Creamos el contexto CPI inyectando la firma (new_with_signer)
        let cpi_program = ctx.accounts.vault_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program, 
            cpi_accounts, 
            signer_seeds // Aquí inyectamos los privilegios
        );

        // 4. Ejecutamos la acción en la Bóveda
        cpi_secure_vault::cpi::update_contents(cpi_ctx, value)?;

        Ok(())
    }
}

// --- VALIDACIÓN DE CUENTAS ---

#[derive(Accounts)]
pub struct ProxyUpdate<'info> {
    #[account(mut)]
    /// CHECK: Validado en el programa destino
    pub vault: AccountInfo<'info>,

    #[account(
        seeds = [b"controller_auth"], 
        bump
    )]
    /// CHECK: PDA propia que usaremos para firmar
    pub pda_authority: AccountInfo<'info>,

    // Referencia al programa de la Bóveda para poder invocarlo
    pub vault_program: Program<'info, CpiSecureVault>,
}
