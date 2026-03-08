use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

entrypoint!(procesar_instruccion);

fn procesar_instruccion(
    _program_id: &Pubkey,
    _cuentas: &[AccountInfo],
    _datos_instruccion: &[u8],
) -> ProgramResult {

    msg!("Sistema de Inventario de Clínica");

    msg!("Medicamento registrado:");
    msg!("Nombre: Paracetamol");
    msg!("Cantidad: 100");

    Ok(())
}
