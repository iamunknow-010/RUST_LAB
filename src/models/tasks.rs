#[derive(Debug)]
pub enum Status {
    Pending,
    Processing,
    Completed,
}

pub struct  Task {
    pub id: u32,
    pub content: String,  
    pub status: Status,
}

impl Task {

    pub fn new(id: u32, content: String) -> Self {
        Self {
            id,
            content,
            status: Status::Pending,
        }
    }

    pub fn process(&mut self) -> Result<String, String> {
        match self.status {
            Status::Processing => Err("Tarea en proceso".to_string()),
            Status::Completed => Err("Tarea ya completada".to_string()),
            Status::Pending => {
                self.status = Status::Processing;
                // Simular ttrabajo
                Ok(format!("Procesando: {}", self.content))
            }
        }
    }

    pub fn finish(&mut self) {
        // Solo podemos terminar si estamos procesando
        if let Status::Processing = self.status {
            self.status = Status::Completed;
        }
    }

}

pub trait Summarizable {
    fn summary(&self) -> String;
}

impl Summarizable for Task {
    fn summary(&self) -> String {
        format!("Tarea {}: [{:?}]", self.id, self.status)
    }
}