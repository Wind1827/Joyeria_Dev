

### Configuración del Estado y Estructuras de Datos
```rust
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
```
Este bloque define la forma en que se almacena la información en la blockchain. La estructura Joya guarda las propiedades 
individuales de cada artículo, mientras que JoyeriaAccount funciona como el contenedor principal o base de datos de la tienda, almacenando la dirección del 
administrador, el nombre del negocio y un listado limitado a diez artículos para optimizar el espacio en cuenta.

---

### Inicialización de la Cuenta Principal
```rust
pub fn inicializar_joyeria(ctx: Context<CrearJoyeria>, nombre_tienda: String) -> Result<()> {
    let joyeria = &mut ctx.accounts.joyeria;
    joyeria.admin = ctx.accounts.admin.key();
    joyeria.nombre_tienda = nombre_tienda.clone(); 
    joyeria.inventario = Vec::new();
    msg!("Joyería '{}' inicializada.", nombre_tienda);
    Ok(())
}
```
Esta función se encarga de crear físicamente la cuenta de la joyería en Solana. Al ejecutarse, establece quién será el administrador de por vida de esa cuenta y prepara el inventario como una lista vacía lista para recibir datos, utilizando un identificador único basado en la semilla joyeria.

---

### Registro y Modificación de Productos
```rust
pub fn registrar_joya(ctx: Context<GestionarInventario>, nombre: String, precio: u64) -> Result<()> {
    let joyeria = &mut ctx.accounts.joyeria;
    let nueva_joya = Joya { nombre, precio };
    joyeria.inventario.push(nueva_joya);
    Ok(())
}

pub fn editar_precio(ctx: Context<GestionarInventario>, nombre: String, nuevo_precio: u64) -> Result<()> {
    let joyeria = &mut ctx.accounts.joyeria;
    let index = joyeria.inventario.iter().position(|j| j.nombre == nombre);
    if let Some(i) = index {
        joyeria.inventario[i].precio = nuevo_precio;
        Ok(())
    } else {
        Err(error!(ErroresJoyeria::JoyaNoEncontrada))
    }
}
```
Estas funciones permiten la interacción directa con el catálogo. La primera añade un nuevo elemento al 
final de la lista de inventario, mientras que la segunda recorre el vector buscando un nombre coincidente 
para actualizar su valor numérico, devolviendo un error si el producto buscado no existe en el registro actual.

---

### Lógica de Eliminación
```rust
pub fn eliminar_joya(ctx: Context<GestionarInventario>, nombre: String) -> Result<()> {
    let joyeria = &mut ctx.accounts.joyeria;
    let index = joyeria.inventario.iter().position(|j| j.nombre == nombre);
    if let Some(i) = index {
        joyeria.inventario.remove(i);
        Ok(())
    } else {
        Err(error!(ErroresJoyeria::JoyaNoEncontrada))
    }
}
```
La función de eliminación localiza la posición de una joya específica dentro del inventario mediante su nombre. 
Una vez encontrada, utiliza el método remove para extraerla del vector y compactar la lista, asegurando que el espacio en la cuenta se mantenga organizado y refleje solo los productos disponibles.

---

### Restricciones de Seguridad y Errores
```rust
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
```
Este bloque final define las reglas de acceso para todas las funciones de gestión. Mediante la instrucción has_one, el programa garantiza que solamente el 
administrador que inicializó la tienda originalmente tenga el permiso firmado para realizar cambios, bloqueando cualquier intento de manipulación por parte de usuarios no autorizados.
