use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::models::tasks::{Task, Summarizable, Status};

pub fn ejercicio_hilos() {
    let tareas = vec![
        Task::new(1, "Comprimir archivos".to_string()),
        Task::new(2, "Descargar actualización".to_string()),
        Task::new(3, "Analizar métricas".to_string()),
        Task::new(4, "Limpiar caché".to_string()),
        Task::new(5, "Generar reporte".to_string()),
    ];
    let tareas_compartidas = Arc::new(Mutex::new(tareas));
    let mut handles = vec![];

    for i in 0..3 {
        let tareas_clonadas = Arc::clone(&tareas_compartidas);

        let handle = thread::spawn(move || {
            loop {
                let mut tareas_guardadas = tareas_clonadas.lock().unwrap();
                let tarea_encontrada = tareas_guardadas.iter_mut().find(|t| match t.status {
                    Status::Pending => true,
                    _ => false,
                });

                let realizo_trabajo = match tarea_encontrada {
                    Some(tarea) => {
                        let _ = tarea.process();
                        println!("Hilo {}: Tomó Tarea {}. Trabajando...", i, tarea.id);

                        tarea.finish();
                        true
                    }
                    None => false,
                };

                drop(tareas_guardadas);

                if realizo_trabajo {
                    thread::sleep(Duration::from_millis(100));
                } else {
                    println!("Hilo {}: Nada pendiente. Bye.", i);
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    let cola_final = tareas_compartidas.lock().unwrap();
    println!("\nResumen final de tareas:");
    for t in cola_final.iter() {
        println!("{}", t.summary());
    }
}


