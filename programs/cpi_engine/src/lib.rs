use anchor_lang::prelude::*;

declare_id!("HdD4VXi25uuvN9QnQXmAjRix9ELk7HmWEm9Xq5qdk9pE");

// --- LÓGICA DEL PROGRAMA ---

#[program]
pub mod cpi_engine {
    use super::*;

    /// Inicializa la cuenta de datos del motor.
    /// Establece las revoluciones por minuto (RPM) a 0.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let engine_account = &mut ctx.accounts.engine_account;
        engine_account.rpm = 0;
        
        msg!("Engine Account Initialized. RPM: 0");
        Ok(())
    }

    /// Actualiza el estado del motor.
    /// Esta función es el destino de la CPI.
    pub fn set_rpm(ctx: Context<SetRpm>, new_rpm: u64) -> Result<()> {
        let engine = &mut ctx.accounts.engine_account;
        engine.rpm = new_rpm;
        
        msg!("CPI Success: Engine RPM updated to {}", engine.rpm);
        Ok(())
    }
}

// --- VALIDACIÓN DE CUENTAS ---

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + EngineStats::INIT_SPACE // 8 bytes discriminador + Tamaño del struct
    )]
    pub engine_account: Account<'info, EngineStats>,

    #[account(mut)]
    pub user: Signer<'info>, // Pagador del alquiler (rent)

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetRpm<'info> {
    #[account(mut)] // La cuenta debe ser mutable para escribir el nuevo valor
    pub engine_account: Account<'info, EngineStats>,
}

// --- ESTADO ---

#[account]
#[derive(InitSpace)] // Calcula automáticamente el tamaño (u64 = 8 bytes)
pub struct EngineStats {
    pub rpm: u64,
}