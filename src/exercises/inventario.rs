use crate::models::inventario::{Producto};


pub fn ejercicio_inventario() {
    let mut tienda = Producto::new(String::from("Pc"), 10);
    println!("Stock inical {}", tienda.stock);

    match tienda.vender(3) {
        Ok(mensaje) => println!("Se vendio {}", mensaje),
        Err(mensaje) => println!("No se pudo {}", mensaje),
    }

    println!("Stock en bodega: {}", tienda.stock);

    match tienda.vender(50) {
        Ok(mensaje) => println!("Se vendio {}", mensaje),
        Err(mensaje) => println!("No se pudo {}", mensaje),
    }

    println!("Stock en bodega: {}", tienda.stock);
}