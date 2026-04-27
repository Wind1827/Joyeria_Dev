use anchor_lang::prelude::*;

declare_id!("6ur2SP628LkrgNBLpg1gavsQ95pFRcomKsBWsZwhgwEc");

#[program]
pub mod joyeria_program {
    use super::*;

    // CREATE (PDA)
    pub fn inicializar_joyeria(ctx: Context<CrearJoyeria>, nombre_tienda: String) -> Result<()> {
        let joyeria = &mut ctx.accounts.joyeria;
        joyeria.admin = ctx.accounts.admin.key();
        // Usamos .clone() para que nombre_tienda siga disponible para el msg!
        joyeria.nombre_tienda = nombre_tienda.clone(); 
        joyeria.inventario = Vec::new();
        
        msg!("Joyería '{}' inicializada.", nombre_tienda);
        Ok(())
    }

    // CREATE (Dato)
    pub fn registrar_joya(ctx: Context<GestionarInventario>, nombre: String, precio: u64) -> Result<()> {
        let joyeria = &mut ctx.accounts.joyeria;
        let nueva_joya = Joya { nombre, precio };
        joyeria.inventario.push(nueva_joya);
        msg!("Joya registrada exitosamente.");
        Ok(())
    }

    // UPDATE: Cambiar precio de una joya existente
    pub fn editar_precio(ctx: Context<GestionarInventario>, nombre: String, nuevo_precio: u64) -> Result<()> {
        let joyeria = &mut ctx.accounts.joyeria;
        // Buscamos la joya por nombre
        let index = joyeria.inventario.iter().position(|j| j.nombre == nombre);
        
        if let Some(i) = index {
            joyeria.inventario[i].precio = nuevo_precio;
            msg!("Precio de {} actualizado a {}", nombre, nuevo_precio);
            Ok(())
        } else {
            Err(error!(ErroresJoyeria::JoyaNoEncontrada))
        }
    }

    // DELETE: Eliminar joya
    pub fn eliminar_joya(ctx: Context<GestionarInventario>, nombre: String) -> Result<()> {
        let joyeria = &mut ctx.accounts.joyeria;
        let index = joyeria.inventario.iter().position(|j| j.nombre == nombre);
        
        if let Some(i) = index {
            joyeria.inventario.remove(i);
            msg!("Joya '{}' eliminada.", nombre);
            Ok(())
        } else {
            Err(error!(ErroresJoyeria::JoyaNoEncontrada))
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, Debug)]
pub struct Joya {
    #[max_len(40)]
    pub nombre: String,
    pub precio: u64,
}

#[account]
#[derive(InitSpace)]
pub struct JoyeriaAccount {
    pub admin: Pubkey,
    #[max_len(50)]
    pub nombre_tienda: String,
    #[max_len(10)] 
    pub inventario: Vec<Joya>,
}

#[derive(Accounts)]
pub struct CrearJoyeria<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + JoyeriaAccount::INIT_SPACE,
        seeds = [b"joyeria"], 
        bump
    )]
    pub joyeria: Account<'info, JoyeriaAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarInventario<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"joyeria"],
        bump,
        has_one = admin @ ErroresJoyeria::NoAutorizado
    )]
    pub joyeria: Account<'info, JoyeriaAccount>,
}

#[error_code]
pub enum ErroresJoyeria {
    #[msg("No tienes permiso para modificar esta joyería.")]
    NoAutorizado,
    #[msg("La joya no existe en el catálogo.")]
    JoyaNoEncontrada,
}
