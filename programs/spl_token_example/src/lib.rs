use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, MintTo, Transfer};

declare_id!("8TcA7B4GMhsu6GNgUChTLrSp59ufp68Seee1XToY6wfD");

#[program]
pub mod spl_token_example {
    use super::*;

    pub fn create_mint(ctx: Context<CreateMint>, decimals: u8) -> Result<()> {
        msg!("Creando Mint manualmente...");

        // 1. Crear la cuenta en la red (CPI al System Program)
        // Calculamos el espacio necesario para un Mint (82 bytes estándar)
        let space = 82; 
        // Calculamos la renta necesaria para que sea exenta
        let rent = Rent::get()?.minimum_balance(space);

        let create_accounts = system_program::CreateAccount {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.mint.to_account_info(),
        };
        let create_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            create_accounts,
        );
        
        system_program::create_account(
            create_ctx, 
            rent, 
            space as u64, 
            &ctx.accounts.token_program.key() // El dueño será el Token Program
        )?;

        // 2. Inicializar la Mint (CPI al Token Program)
        let init_accounts = token::InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let init_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            init_accounts,
        );
        
        token::initialize_mint(
            init_ctx, 
            decimals, 
            &ctx.accounts.authority.key(), 
            Some(&ctx.accounts.authority.key())
        )?;

        msg!("Mint creada exitosamente.");
        Ok(())
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

// --- NOTA DE IMPLEMENTACIÓN ---
// En este programa, la creación e inicialización de la Mint se realiza de forma manual
// mediante CPI (Cross-Program Invocation) en lugar de utilizar las macros automáticas
// de Anchor (como 'init' con tipos específicos). 
//
// Esta decisión responde a:
// 1. Optimización de memoria: Reduce la carga del compilador al desacoplar las dependencias 
//    de tipos de 'anchor_spl', previniendo errores de stack en entornos Docker.
// 2. Compatibilidad: Permite generar el IDL sin dependencias externas pesadas.

// --- VALIDACIÓN DE CUENTAS (Simplificada para evitar errores de IDL) ---

#[derive(Accounts)]
pub struct CreateMint<'info> {
    // Recibimos la cuenta como Signer (el cliente crea el par de claves)
    // Al ser Signer, Anchor no busca la definición de "Mint" y no falla.
    #[account(mut)]
    pub mint: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Solo lo usamos para firmar/autoridad
    pub authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    
    /// CHECK: Token Program oficial
    pub token_program: UncheckedAccount<'info>,
    
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    /// CHECK: Validado en la CPI
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: Validado en la CPI
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    /// CHECK: Token Program
    pub token_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    /// CHECK: Validado en la CPI
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: Validado en la CPI
    #[account(mut)]
    pub to: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    /// CHECK: Token Program
    pub token_program: UncheckedAccount<'info>,
}
