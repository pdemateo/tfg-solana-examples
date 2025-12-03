use anchor_lang::prelude::*;

declare_id!("7XBBYkPXKXA7bMC3eJwgmYKmmjqmGrTs15FFmJPELWau");

// --- LÓGICA DEL PROGRAMA ---

#[program]
pub mod counter {
    use super::*;

    /// Inicializa una nueva cuenta de contador.
    pub fn initialize_counter(_ctx: Context<InitializeCounter>) -> Result<()> {
        msg!("Counter Account Created");
        Ok(())
    }

    /// Incrementa el valor actual del contador.
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        // Uso de checked_add para evitar desbordamientos (overflow protection)
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. New count: {}", counter.count);
        Ok(())
    } 
}

// --- VALIDACIÓN DE CUENTAS ---

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // Pagador de la transacción y alquiler (rent)

    #[account(
        init,
        space = 8 + Counter::INIT_SPACE, // Discriminador + Tamaño del struct
        payer = payer
    )]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)] // La cuenta sufrirá cambios de estado
    pub counter: Account<'info, Counter>,
}

// --- ESTADO ---

#[account]
#[derive(InitSpace)] // Calcula automáticamente el tamaño (u64 = 8 bytes)   
pub struct Counter {
    pub count: u64,
}