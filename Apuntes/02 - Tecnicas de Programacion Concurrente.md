# Modelo Fork Join
Es un estilo de **paralelizacion** donde el computo (task) es dividido en sub-computos menores (sub-tasks). Los resultados se unen (join) para construir la solucion al computo inicial.

La condicion es que la tarea se tiene que poder dividir en subtareas

Partir el computo se hace en general de forma **recursiva**:
- las sub-tasks son independientes ▶️ el computo se puede hacer en paralelo
- las sub-tasks se pueden crear en cualquier momento de la ejecucion de la tarea
- las tareas no deben bloquearse, excepto para esperar a las subtareas

<img width="546" height="388" alt="image" src="https://github.com/user-attachments/assets/8da8300d-415f-4aff-b9d9-77471cf16acf" />

* Este es un modelo sin race conditions. 
* Son deterministicos, los threads estan aislados: el programa produce el mismo resultado independientemente de las diferencias de velocidad de los threads.
* **Performance**: caso ideal $t_{secuencial} / N_{threads}$ pero puede variar por diferencias en el tamaño de las tareas y porque se tiene que realizar procesamiento para combinar los resultados individuales.
* Desventaja: las tareas deben ser aisladas.

## Work Stealing
Es un algoritmo usado para hacer scheduling de tareas entre threads.

Worker threads inactivos roban trabajo a threads ocupados para realizar un **balanceo de carga** porque el objetivo es que el tiempo real del algoritmo se parezca lo mas posible al caso ideal teorico.

- Cada thread tiene su propia cola de dos extremos (deque) donde almacena las tareas listas por ejecutar
- Cuando un thread termina la ejecucion de una tarea, pone las subtareas creadas al final de la cola.
- Luego, toma la siguiente tarea para ser ejecutada del final de la cola.
- **Si la cola esta vacia, y el thread no tiene mas trabajo hay que tratar de robar tareas del inicio de una cola de otro thread random**
- Ventajas:
  - Los worker threads se comunican solamente cuando lo necesitan ▶️ menor necesidad de sincronizacion
  - La implementacion de la deque agrega bajo overhead de sincronizacion

### De forma mas clara
Cada worker thread tiene su propia deque (cola de doble extremo):
- Cuando un thread genera subtareas (fork), las agrega al final (tail) de su propia cola.
- Ese mismo thread también consume tareas desde ese mismo extremo (LIFO), lo que mejora la localidad de memoria (cache-friendly).
- Si un thread se queda sin tareas:
  -   Elige otro thread al azar.
  -   Roba una tarea desde el inicio (head) de la deque del otro.

La diferencia clave es que:
- El dueño de la cola usa el final (rápido, sin sincronización pesada).
- El que roba usa el inicio (requiere sincronización, pero pasa poco).

Las tareas grandes tienden a quedarse cerca del inicio de la cola. Cuando un thread roba, suele agarrar una tarea relativamente “grande”, lo que le da suficiente trabajo para mantenerse ocupado sin tener que robar constantemente.



## Implementaciones 
### Standard Library

El algoritmo siguiente es fork join que no implementa work stealing.

```
fn process_files_in_parallel(filenames: Vec<String>) -> io::Result<()> {
    const NTHREADS: usize = 8;

    let worklists = split_vec_into_chunks(filenames, NTHREADS);
    let mut thread_handles = vec![];

    for worklist in worklists {
        thread_handles.push(
            std::thread::spawn(move || process_files(worklist))
        );
    }

    for handle in thread_handles {
        handle.join().unwrap()?;
    }

    Ok(())
}
```

### Rayon 
No existe el work stealing en la biblioteca estandar pero para ello hay una biblioteca externa: **Rayon** que implementa fork join de dos formas distintas
* Realizar dos tareas en paralelo: `let (v1, v2) = rayon::join(fn1, fn2);`  invoca a fn1 y fn2 y retorna una tupla con ambos resultados
* Realizar N tareas en paralelo: es utilizar Rayon sobre iteradores 
```
giant_vector.par_iter().for_each(|value| {
  do_thing_with_value(value);
});
```
El metodo `.par_iter()` crea un iterador _ParallelIterator_ similar a los iteradores de Rust. Rayon maneja los threads y distribuye el trabajo

1. `par_iter()`:
   - Convierte el vector en un **iterador paralelo**
   - Rayon decide cómo dividir los datos
2. Rayon:
   - Divide el vector en partes más chicas (**fork**)
   - Asigna esas partes a distintos threads
3. Mientras ejecuta:
   - Si un thread termina antes → **roba trabajo** de otro (hace **work stealing**)
4. `for_each`:
   - Aplica `do_thing_with_value` a cada elemento
   - No devuelve nada (como un `for_each` normal)

<img width="569" height="428" alt="image" src="https://github.com/user-attachments/assets/40b2f75b-8b9e-405f-ac7c-b95ea879ed50" />

Desde afuera, Rayon parece crear una tarea por elemento del vector.

Internamente, crea un worker thread por núcleo de CPU. Implementa: Work stealing.

Los métodos .reduce() y .reduce_with() se usan para combinar los resultados.

```
use rayon::prelude::*; // trae al scope los iteradores de Rayon

let s = ['a', 'b', 'c', 'd', 'e']
    .par_iter()
    .map(|c: &char| format!("{}", c))
    .reduce(|| String::new(),
        |mut a: String, b: String| {
            a.push_str(&b);
            a
        });

assert_eq!(s, "abcde");
```

## Crossbeam
Crossbeam es un crate de concurrencia muy utilizado, provee estructuras de datos y funciones para concurrencia y paralelismo. `crossbeam::scope` crea un nuevo entorno de thread que garantiza que los threads terminan antes de retornar el closure que se le pasa como argumento a esta función.

Todos los threads que no fueron manualmente esperados (join), son esperados antes de que nalice la invocación de la función. Si todos terminan exitosamente, se retorna **Ok**, si alguno ejecutó panic, se retorna **Err**

Es otra forma de hacer fork join y no lidiar con el ownership de rust porque recibo los elementos al principio y devuelvo todo al final.
