pub struct Producto {
    pub nombre: String,
    pub stock: i32,
}

impl Producto {

    pub fn new(nombre: String, stock: i32) -> Self {
        Self {
            nombre,
            stock,
        }
    }

    pub fn vender(&mut self, cantidad: i32) -> Result<String, String> {
        if cantidad >= self.stock {
            return Err(format!("Cantidad insuficiente del producto: {}", self.nombre))
        }
        self.stock = self.stock - cantidad;
        Ok(format!("Venta realizada con exito de {}", self.nombre))
    }
    
}