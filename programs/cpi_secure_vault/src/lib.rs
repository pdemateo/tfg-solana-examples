use anchor_lang::prelude::*;

declare_id!("4s6WGZESrg51X63AbYT94uxck9fSBuPM2EeUk5wbZcxV");

// --- LÓGICA DEL PROGRAMA ---

#[program]
pub mod cpi_secure_vault {
    use super::*;

    /// Inicializa la bóveda con un valor predeterminado.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vault.data_value = 0;
        msg!("Bóveda inicializada.");
        Ok(())
    }

    /// Modifica el contenido de la bóveda.
    /// Esta función falla si la cuenta 'vault_authority' no ha firmado.
    pub fn update_contents(ctx: Context<AccessVault>, new_value: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // Lógica de almacenamiento
        vault.data_value = new_value;
        
        msg!("Vault updated successfully. New value: {}", new_value);
        Ok(())
    }
}

// --- VALIDACIÓN DE CUENTAS ---

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        space = 8 + VaultData::INIT_SPACE, // Discriminador + tamaño del struct
        payer = user
    )]
    pub vault: Account<'info, VaultData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AccessVault<'info> {
    #[account(mut)]
    pub vault: Account<'info, VaultData>,

    // EL CANDADO: Tipo 'Signer'.
    // Si la CPI no inyecta la firma de la PDA, esto devuelve error de autorización.
    pub vault_authority: Signer<'info>, 
}

// --- ESTADO ---

#[account]
#[derive(InitSpace)] // Calcula automáticamente el tamaño (u64 = 8 bytes)
pub struct VaultData {
    pub data_value: u64,
}