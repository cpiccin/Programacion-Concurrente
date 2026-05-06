## Definiciones 

**Programa**: conjunto de datos, asignaciones e instrucciones de control de flujo que compilan a instrucciones de máquina, las cuales se ejecutan secuencialmente en un procesador y acceden a datos almacenados en memoria principal o memorias secundarias.

**Programa concurrente**: Un programa concurrente es un programa diseñado para que múltiples actividades o unidades de ejecución progresen durante el mismo intervalo de tiempo. Estas actividades pueden ejecutarse de forma intercalada en un solo procesador o simultáneamente en varios procesadores. La concurrencia permite estructurar programas que manejan múltiples tareas que avanzan de manera coordinada. Consiste en un conjunto finito de **procesos secuenciales**.

**Proceso**: están compuestos por un conjunto finito de **instrucciones atómicas**. Un proceso es una instancia de un programa en ejecución. Incluye el código del programa, su estado de ejecución, memoria asignada, recursos del sistema y el contexto necesario para que el sistema operativo lo gestione. Cada proceso tiene su propio espacio de direcciones y puede contener uno o más hilos de ejecución.

**Sistema paralelo**: sistema compuesto por varios programas que se ejecutan simultáneamente en procesadores distintos.

**Multitasking**: El multitasking es la capacidad de un sistema operativo para ejecutar múltiples procesos aparentemente al mismo tiempo. En sistemas con un solo procesador, esto se logra alternando rápidamente entre procesos mediante planificación del CPU. En sistemas con múltiples procesadores, algunos procesos pueden ejecutarse realmente en paralelo. El **scheduler** es quien coordina el acceso a los procesadores.

**Multithreading**: El multithreading es la capacidad de un proceso de tener múltiples hilos de ejecución dentro de su mismo espacio de memoria. Cada hilo ejecuta instrucciones de manera independiente pero comparte los recursos del proceso, como memoria y archivos abiertos.

**Sincronizacion**: coordinación temporal entre distintos procesos. Su objetivo es garantizar que el acceso a datos compartidos sea correcto y evitar race conditions, inconsistencias o interferencias.

**Comunicacion**: La comunicación es el mecanismo mediante el cual procesos o hilos intercambian información durante su ejecución.


## Ejecución del programa concurrente 
Resulta al ejecutar una secuencia de **instrucciones atómicas** que se obtiene de intercalar arbitrariamente las instrucciones atómicas de los procesos que lo componen. 

Una instruccion atomica se ejecuta de principio a fin sin interrumpirse, o no se ejecuta.


# Modelos de Concurrencia
* Estado mutable compartido
* Paralelismo fork-join: Es una forma de paralelismo. Un implementacion mas estructurada de programas, con una condicion, un programa se debe poder dividir en subtareas tal que cada subtarea sea independiente y no requiera de datos de las siguientes subtareas. O sea se separa y al final se hace un join.
* Canales / mensajes: se puede dividir al programa en entidades o partes que necesiten de datos que tienen otras entidades, se comunican entre sí a través de el envío de mensajes. No hay un estado mutable **compartido**. 
* Programación asincrónica: es una forma de programación en la que el programa está compuesto por tareas livianas, sencillas que cooperan entre sí en el uso de recursos y en el uso del CPU. Se usa cuando hay una gran cantidad de entrada y salida porque se pueden aprovechar esos tiempos muertos de entrada y salida para darle el uso del CPU a otra tarea. 
* Actores: la idea de mensajes se materializa en el modelo de actores donde cada actor no comparte su estado. Cada actor tiene una casilla de mensajes donde cada mensaje se procesa de manera secuencial. Cada actor modifica su estado interno a través de los mensajes que recibe. 

Hay conjuntos de operaciones que no pueden intercalarse porque pueden surgir inconsistencias con el estado o con los datos. Por eso hay que serializar el acceso al estado compartido. **Serializar** es para controlar el acceso a las variables compartidas

Los procesos se ejecutarán al mismo tiempo, pero habrá ciertos conjuntos de procedimientos tales que solo una ejecución de un procedimiento en cada se permite que suceda a la vez. Cualquier otro proceso que intente ejecutar cualquier procedimiento en el conjunto será obligado a esperar hasta que la primera ejecución haya terminado.

Podemos marcar regiones de código que no pueden superponerse en la ejecución al mismo tiempo.


## Threads 
Los threads **comparten** los recursos generales del **proceso**, como el espacio de memoria. Los procesos son unidades de aislamiento porque son aislados unos de otros. 

Cada thread mantiene su propia información de estado (stack, PC, registros). Cada thread es una unidad de ejecución.


---

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


---

# Programación Asincrónica  

Si se crean muchos threads, puede surgir un problema de demanda de memoria ya que cada thread necesita su propio stack y recursos asociados

Los lenguajes modernos introducen el concepto de **continuacion**. Una **continuación** es una tarea que puede interrumpirse voluntariamente para ceder el uso de la CPU, porque esta no puede avanzar (por ejemplo, está esperando una operación de I/O).

Son las **tareas asincronicas** de Rust para intercalar tareas en un unico thread o en un pool de threads. En rust no tienen un stack ni contexto propio entonces:
- Mas mucho livianas que los threads.
- Mas rapidas de crear, mas eficiente de pasarle el control a ellas.
- Menor overhead de memoria.
- Como las tareas asincronas no son interrumpidas sino que ellas voluntariamente ceden el control, saben que info minima guardar para luego seguir con la continuacion, a diferencia de un thread que no sabe cuando va a ser interrumpido por lo que va a guardar todo su contexto.

El codigo asincronico es como el de threads pero se maneja diferente la forma en que se bloquean las operaciones (`async`/`await`).

Los puntos donde la tarea va a hacer el control son los puntos donde se va a poner en espera.

### Ejemplo
#### Version Sincronica
```
use std::{net, thread};
let listener = net::TcpListener::bind(address)?; // El bind() es bloqueante

for socket_result in listener.incoming() { // El incoming() es bloqueante
  let socket = socket_result?;
  let groups = chat_group_table.clone();
  thread::spawn(|| {
    log_error(serve(socket, groups));
  });
}
```
#### Version Asincronica
```
use async_std::{net, task};
let listener = net::TcpListener::bind(address).await?; // Punto de continuacion

let mut new_connections = listener.incoming();
while let Some(socket_result) = new_connections.next().await // Punto de continuacion
  let socket = socket_result?;
  let groups = chat_group_table.clone();
  task::spawn(async {
    log_error(serve(socket, groups).await); // Punto de continuacion
  });
}
}
```

## Futures
Rust introduce el trait `std::future::Future`.

- **Future** es un valor que representa un resultado que va a estar disponible en el futuro
- Representa una operación sobre la que se puede testear si se completó.
- El metodo `poll` nunca bloque e intenta avanzar la ejecución del Future:
  -   Si la operación se completó, retorna: `Poll::Ready(output)` (output es el resultado final de la operación).
  -    Si no se completó, retorna `Pending`
- No es que **yo** llamo a `poll`, el runtime async lo hace, `.await` usa poll internamente.

```
let f = suma_async();
```

`f` no es el resultado, es un Future. El resultado todavía no existe.

Se puede decir que es un modelo **piñata** de la programación asincrónica: lo único que se puede hacer con un future es golpearlo 😢 con poll hasta que caiga el valor.

### Performance
- La arquitectura async de Rust está diseñada para ser eficiente
- Se llama a poll solamente cuando vale la pena (algo debe retornar Ready, o progresar al objetivo).

---
 # Practica: Programacion Asincronica
 Cuando se usa async? 
 - Consulta a un servicio externo
 - Leer de un archivo
 - Servir requests HTTP
 - Cuando tenemos que hacer algo que no es tan costoso computacionalmente como para necesitar el thread completo.

## Macro join!
 `join!` hace que se haga `.await` concurrentemente

### Diferencia con `.await`?
Haciendo esto:
```
let r1 = tarea1().await;
let r2 = tarea2().await;
```
- ejecuta `tarea1` hasta que termina
- recién después empieza `tarea2`
- No es concurrente❗❗❗

Usando `join!` es concurrente porque:
 ```
let f1 = tarea1();
let f2 = tarea2();

let (r1, r2) = join!(f1, f2);
```
- empiezan ambas tareas
- las va ejecutando de forma intercalada
- espera a que las dos terminen
- devuelve una tupla con los resultados
- ambas avanzan “a la vez”, cuando una espera, la otra puede seguir

## Pin
Los tipos autogenerados de async que implementa Future guardan referencias a si mismos. Si fueran movidos (por ej por estar en el stack) sus referencias internas no se actualizarian.

Si se mueve, se perderia la autoreferencia. Por eso, no se permite que se mueva en memoria.

`Pin` garantiza que un valor no se va a mover en memoria.

- Todos los tipos implementan por depento el autotrait `Unpin`
- Si T es `!Unpin`, `Pin` evita que se mueva haciendo imposible llamar a metodos que requieran `&mut T` como `mem::swap`

El `poll` recibe un `Pin` de self, una variante de self que no es movible.

## Runtimes


---

# Correccion 

En programas concurrentes, la salida puede depender del escenario que resultó en la ejecución.

**Propiedades de la correccion**:
- Safety: debe ser verdadera siempre
- Liveness: debe volverse verdadera eventualmente

## Propiedades de correccion

### Propiedades tipo Safety
- **Exclusión mutua**: dos procesos no deben intercalar ciertas (sub)secuencias de instrucciones. Ejemplo: incremento de variable global.

- **Ausencia de deadlock**: un sistema que aún no finalizó debe poder continuar realizando su tarea, es decir, avanzar productivamente.

### Propiedades tipo Liveness
Tienen que ser **eventualmente** verdaderas.

- **Ausencia de starvation**: todo proceso que este listo para utilizar un recurso debe recibir dicho recurso eventualmente.

- **Fairness**: equidad o justicia. Un escenario es (débilmente) fair, si en algún estado en el escenario, una instrucción que está continuamente habilitada, eventualmente aparece en el escenario.

## Definicion del Problema
El codigo puede tener parte critica y parte no-critica. En cuanto a las propiedades de correccion:
- Exlusion mutua: no deben intercalarse la ejecucion de instrucciones de la seccion critica.
- Ausencia de deadlock: si dos procesos quieren entrar a una seccion critica, alguno debe poder entrar, sino hay un **deadlock**.
- Ausencia de starvation: si un proceso entra a la seccion critica, eventualmente debe poder salir de esa seccion (para dejar paso al resto).

La seccion critica debe progresar, o sea eventualmente salir de la seccion critica (**condicion de progreso**). La no-critica no necesita progresar, el proceso puede terminar o entrar en un loop infinito.

# Locks
Para proteger las regiones criticas existen locks, para crear/controlar esas secciones criticas. Permiten la exlusion mutua entre procesos. 

Se implementan mediante variables de tipo lock, que contienen el estado del mismo

## Locks en Unix
Locks son manipulaciones del file descriptor. Estan centrados en un archivo pero se pueden usar para sincronizar acceso a otros recursos.

En Unix son _advisory_ entonces los procesos pueden ignorarlos.

Tipos de locks:
- Locks de lectura o shared locks: mas de un proceso a la vez puede tener el lock
- Locks de escritura o exclusive locks: solo un proceso a la vez puede tener cualquier tipo de lock

Condiciones para tomar un lock:
- Para tomar un _shared (read) lock_, el proceso tiene que esperar a que se liberen todos los exclusive locks.
- Para poder tomar un _exclusive (write) lock_, el proceso debe esperar a que sean liberados todos los locks (de ambos tipos)

## Locks en Rust
En Rust un lock es un wrapper para un elemento, no es un elemento abstracto.

### Traits Send y Sync
- El trait _marker_ **Send** indica que el ownership del tipo que lo implementa puede ser transferido entre threads de forma segura. 

Los tipos compuestos que estan formados por tipos **Send** automaticamente son **Send**. Casi todos los tipos primitivos son **Send**, excepto los raw pointers.

- El trait _marker_ **Sync** indica que puedo referenciar de forma segura el tipo que lo implementa desde multiples threads.

Los tipos primitivos son **Sync** y los tipos compuesto que están formados por tipos **Sync** automáticamente son Sync.

Rust provee locks y los mutex. Provee locks compartidos (de lectura) y locks exclusivos (de escritura) en el módulo: `std::sync::RwLock`. <br>
No se provee una política específica, sino que es dependiente del sistema operativo.

Se requiere que **T** sea **Send** para ser compartido entre threads y **Sync** para permitir acceso concurrente entre lectores.

```
use std::sync:::RwLock;
let lock = RwLock::new(5);
```

## Locks envenenados
Un lock queda en estado envenenado cuando un thread lo toma de forma exclusiva (write lock) y mientras tiene tomado el lock, ejecuta `panic!`. O sea que el thread no deberia hacer panic porque no se estaria manejando bien el error.

Las llamadas posteriores a `read()` y `write()` sobre el mismo lock, devolverán Error.

El lock tiene metodos para dejar de estar envenenado.

Esta bueno minimizar la cantidad de lineas en una seccion critica porque se reduce la posibilidad de que surja un panic.


---

# Sincronizacion

## Estados de ejecucion de un proceso (teorico)
<img width="315" height="163" alt="image" src="https://github.com/user-attachments/assets/371007d3-01d7-49e7-909f-87f7e68ab8a1" />

## Semaforos
Un semáforo es una estructura de sincronización para coordinar el acceso de múltiples procesos o hilos a recursos compartidos, evitando conflictos como race conditions. Funciona como un contador.

Es un **tipo de dato** compuesto por dos campos:
- Un entero no negativo llamado V, es el contador del semaforo, es de implementacion interna entonces no se lo puede consultar
- Un set de procesos llamado L

Se inicializa con un valor $$k \ge 0$$ y con el conjunto vacio $$\emptyset$$

Un semáforo es un mecanismo que regula el acceso a recursos compartidos mediante un contador y bloqueo/desbloqueo de procesos, garantizando sincronización entre ejecuciones concurrentes.

### Contador
- Si $$\text{Contador} > 0$$ el recurso esta disponible
- Si $$\text{Contador} \le 0$$ el recurso no esta disponible
- Si el valor es 0 o 1, se llaman **semaforos binarios** y se comportan igual que los locks de escritura (Mutex)

Se definen dos operaciones atomicas sobre un semaforo $$S$$:
- `wait(5)` llamada `p(5)`: si un proceso intenta usar el recurso:
  - Si V > 0, lo decrementa y continúa.  
  - Si V = 0, el proceso se bloquea y se agrega a la lista de espera.
- `signal(S)` llamada `v(S)`: si un proceso libera el recurso.  
  - Incrementa V.  
  - Si hay procesos esperando, despierta a uno.


---

# Redes de Petri
Sirven para modelar el comportamiento de un sistema de forma tal que podamos hacer un seguimiento y comprobar x propiedad.

## Red Ordinaria de Petri
Es un grafo dirigido bipartito (se puede hacer una particion en dos grupos tal que las aristas de un grupo no se vinculan con nodos del mismo grupo). Las redes de petri ordinarias estan formadas por $$PN=(T, P, A)$$.
- $$T$$ ... es un conjunto de nodos llamado transiciones: producen los cambios de estado del sistema
- P es un conjunto de nodos llamados lugares (places)
- A ... es un conjunto de arcos dirigidos, subconjunto de todas las posibles combinaciones

Ejemplo

<img width="368" height="115" alt="image" src="https://github.com/user-attachments/assets/bd9ad554-0aa9-4b8c-9d5a-91aab7aec3c9" />

<img width="455" height="91" alt="image" src="https://github.com/user-attachments/assets/7bbd5f19-12c9-43e2-8cb2-8c4b29e8537e" />

### Funcion de Marca
Se define pra cada estado del sistema como:
- $$M:P \rightarrow N \cup 0$$

En los places se van a ubicar tokens que van a estar circulando entre estos places. La funcion de marca de p1 indica cuantos tokens hay en cada place cuando tengo un token en p1.

Cuando el token esta en el lugar $$p_{1}$$, entonces $$M(p_{1})=1$$ y $$M(p_{2})=0$$. Por lo tanto M0=(1, 0)

### Funciones de Entrada y Salida
En las redes de petri se definen 

<img width="494" height="154" alt="image" src="https://github.com/user-attachments/assets/75aada13-6557-4fd1-87a7-6dceda37bede" />

O sea I(t) son los places de **entrada** a la transicion t. Y O(t) son los places de salida (a donde aw puede ir desde la transicion t) de la transicion t.

### Ejemplo 1
Tengo el sistema siguiente. El estado inicial es en p1 porque ahi esta el token inicialmente. Dado este estado se puede disparar la transicion t1, se puede consumir el token de p1 y se crea un token para p2 y p3.

Como hay un token en p3, se puede disparar la transicion t3 y colocarse un token en p5.

Tambien se puede disparar la transicion t2, se pone un token en p4 y se puede disparar la transicion t4 que saca el token de p4 y lo pone de nuevo en p2, o sino, si hay un token en p4, junto al de p5, se puede disparar la transicion t5, se deben cumplir las dos condiciones, tiene que consumir los dos, no alcanza con uno solo, asi el sistema vuelve al sistema inicial en p1.


<img width="477" height="395" alt="image" src="https://github.com/user-attachments/assets/526bb2aa-c533-4d77-afc9-a4e45adce334" />


### Grafo de alcance
Grafo dirigido.

<img width="379" height="273" alt="image" src="https://github.com/user-attachments/assets/bb3f1907-3094-405a-90c5-636aa23b1d66" />

### Ejemplo 2
Aca sucede que si del estado inicial en p1, se dispara t2 de una, nunca se va a poder ejecutar ningun otro estado porque no hay token en p2.
<img width="344" height="319" alt="image" src="https://github.com/user-attachments/assets/fa0e3a58-9f35-44b7-b4ca-2b38dc7b865d" />

Vemos en el grado de alcance que puede haber un loop infinito o tambien, si se ejecuta primero t2, muere ahi el flujo: hay un deadlock!!!!

<img width="704" height="447" alt="image" src="https://github.com/user-attachments/assets/ed729b47-61ec-40c6-ba49-42f628670042" />

### Algunas interpretaciones
Las redes de petri se usan para modelar sistemas de lo que sea.

Ejemplos:

<img width="471" height="167" alt="image" src="https://github.com/user-attachments/assets/a19c4284-69d6-4875-81b9-28c742d3a63a" />


## Red general de Petri
Tienen mas informacion que las redes ordinarias, que son mas limitadas. Ahora cada arco tiene peso W positivo y una funcion de marca inicial M0, o sea que se define como es el estado inicial del sistema.

<img width="511" height="245" alt="image" src="https://github.com/user-attachments/assets/9b7b5601-0d1d-48f9-af96-ba83522ec65d" />

### Reglas Generales de Disparo de Transiciones

<img width="366" height="300" alt="image" src="https://github.com/user-attachments/assets/09aba07e-88e0-4f76-96d7-e2346eec1899" />

O sea... **Consume** 2 de p1, 1 de p2 y **genera** 2 en p3.

<img width="224" height="154" alt="image" src="https://github.com/user-attachments/assets/fbb1c873-f560-4518-8973-4b808b801aae" />

### Ejemplo 3
<img width="434" height="269" alt="image" src="https://github.com/user-attachments/assets/217d151d-b820-41ab-8159-272f41faac90" />

Puede suceder que `a-b` sea cero o que no sea cero, si es cero el sistema llega al resultado "indefinido". Si no es cero se puede realizar la division y devolver x.

### Ejemplo: Productor Consumidor con Bufer Infinito
<img width="659" height="438" alt="image" src="https://github.com/user-attachments/assets/30823553-823e-498b-8cab-0c4534478409" />

p1 dispara la transicion t1 que significa "producir" y coloca un token en p2 indicando que se produjo el elemento, t2 significa "colocar elemento en el buffer", p5 es el estado "elemento en el buffer", p1 vuelve al estado inicial "listo para producir". 

Para dispararse el estado t3 se necesita el token que salio de p1, cuando llega a p5 se consume ese token y el de p3 para disparar t3, t3 se lo pasa a p4 que dispara t4 y el token regresa a p3. 

p3 solo consume, p1 solo produce. Y si p1 produce mas rapido de lo que p3 consume, p5 puede llenarse infinitamente.

### Ejemplo: Productor Consumidor con Bufer Acotado
<img width="674" height="433" alt="image" src="https://github.com/user-attachments/assets/69b6f9de-e48b-4721-aa14-2684a11393ca" />

Quiero limitar al productor para que no consuma infinitamente como en el caso anterior.

Se agrega el estado p6 tiene `N` tokens, habran `N` lugares disponibles en el buffer. Para poder ejecutarse t2 tiene la condicion tener elemento producito y ademas tener espacio en el buffer. 

### Ejemplo: Cliente - Servidor
El cliente le hace una pregunta al servidor, este la procesa y le responde al cliente

<img width="786" height="465" alt="image" src="https://github.com/user-attachments/assets/a3ae0fd4-fd87-4611-be0c-b8cad0aeceb5" />

En p1 empieza el pedido del cliente, en p6 esta listo el servidor. La transiscion t2 envia el pedido y va al place p2 que es "esperando respuesta". El servidor solo hace algo cuando tiene un pedido del cliente. Cuando el servidor responde dispara la transicion t3, vuelve a estar listo para responder, y el cliente puede volver a preguntar.



---

# Pasaje de Mensajes

Distintos tipos de comunicacion:
- **Comunicacion**
    - Sincronica
    - Asincronica - Buffer: se acumulan los mensajes, el receptor los procesa cuando pueda
- **Direccionamiento**: como se determina a quien dirigir un mensajes?
    - Simetrico: de un emisor a un receptor, 1 a 1
    - Asimetrico: de un emisor a multiples receptores, un servidor a usuarios
    - Sin direccionamiento (matcheo por estructura de mensaje): no enviamos mensajes en particular, multiples receptores acceden y deciden si le corresponde o no. Como ir a buscar algo al correo.
- **Flujo de datos**
    - Unidireccional: permite enviar mensajes durante toda la vida del canal, de un emisor al receptor
    - Bidireccional: en ambos sentidos de receptor a emisor y de emisor a receptor
 
## Canales: introduccion teorica
Conectan un emisor con un receptor. Tienen:
- Nombre
- Son tipados, se mandan mensajes de un tipo de dato
- Sincronicos o asincronicos
- Unidireccionales

### Productores y Consumidores
Se simplifica: el Productor produce el elemento `I`, se lo inserta en el canal `ch` y el consumidor extrae del canal el elemento y lo consume. Ocurre una sincronizacion entre el productor y el consumidor, se queda trabado cuando quiere extraer el elemento si es que no hay nada, y el productor se bloquea hasta que haya un lugar disponible en el canal. 

 Evolucion de los semaforos.
 
 <img width="880" height="473" alt="image" src="https://github.com/user-attachments/assets/145565a5-8fab-4172-994c-062fb5f1fa8d" />

### Filosofos Comensales
Se crea un vector de canales con 5 elementos (5 tenedores con 4 filosofos). Usamos los canales como herramienta de sincronismo, no me interesa que comunica. Avanza y puede comer cuando le dieron un tenedor. Sigue habiendo deadlock.

<img width="874" height="619" alt="image" src="https://github.com/user-attachments/assets/980abda0-ddd0-4372-9365-9deff149e00b" />

### Selective Input
Es una sintaxis permitida por los lenguajes que soportan canales.

Permite escuchar en varios canales **de forma bloqueante** y **desbloquearse** con el primero que recibe un mensaje.

### Remote Procedure Calls
**Permite invocar metodos de forma remota**: o sea le permite al cliente ejecutar funciones en un servidor localizado en otro procesador.
- Se requiere la implementacionm de _stubs_ en ambos lados. Un stub no es la implementacion real de la funcion sino que internamente llama al servidor realizando la invocacion y _marshall y unmarshall_ de los parametros, o sea una serializacion/deserializacion, se los convierte a un formato comun (por ej. pasar todo a big endian). Luego el servidor ejecuta el codigo.

## Implementaciones: Canales de Unix
Unix provee Pipes y FIFOs para conectar dos precesos independientes, **orientados a bytes**.
- los FIFOs tienen una representacion en el file system; unidireccionales; sincronicos
- los Pipes no se ven pero son una estructura interna del kernel

Unix tambien provee colas de mensajes (Message queues), orientados a mensajes como unidades independientes. 
- Se pueden enviar mensajes con tipo de dato, permite estructuras mas complejas.
- Con cada read lee un solo mensaje.
- Tienen prioridad, colas con prioridad, si no hay prioridad es una cola sin mas.

## Implementaciones: Canales en Rust
Rust aplica la regla de ownership en los canales: 

> Do not communicate by sharing memory; instead, share memory by communicating

Cuando alguien coloca algo en el canal, esta entregando el ownership del elemento al receptor.

### Caracteristicas
- Un canal tiene dos extremos: un emisor y un receptor
- Una parte del codigo invoca metodos sobre el transmisor, con
- Una parte del codigo ejecuta los metodos de envio en el extremo de transmision, y otro de lectura, un extremo de recepcion.
- Se chequea el exceso de mensajes, si hay exceso, queda bloqueado.
- Multiples productores, un consumidor.
- Para crear multiples productores, se clona el extremo de envio




---

# Modelo de Actores

Es similar al modelo de pasaje de mensajes. 

## Actor
Encapsulan comportamiento y estado `->` Es la primitiva principal del modelo. Son livianos, se pueden crear miles (en lugar de threads).

**Esta compuesto por:**
- Dirección: adonde enviarle mensajes
- Casilla de correo (mailbox): un FIFO de los ultimos mensajes recibidos

Hay un actor supervisor que puede crear otros actores hijo.

<img width="569" height="401" alt="image" src="https://github.com/user-attachments/assets/0263ed98-265a-4421-a16e-79b17362527d" />

Caracteristicas:
- Son aislados entre si: no comparten memoria.
- El estado privado solo puede verse modificado a partir de procesar otros mensajes.
- Pueden manejar un mensaje a la vez. Como ventaja es que el manejo interno es trivial, no hay que pensar en mutex ni nada.
- En un _sistema distribuido_, la direccion del actor puede ser una direccion remota.

## Mensajes
Los actores se comunican entre ellos usando mensajes. Los actores los procesan de forma asincronica.
- Son estructuras simples **inmutables**.

## En Rust

### Actores

Se usa un crate (framework externo): **Actix**.

<img width="421" height="182" alt="image" src="https://github.com/user-attachments/assets/140d2fff-7b06-45e2-8778-16df52ebc2d7" />

Cómo crear un actor:

<img width="420" height="114" alt="image" src="https://github.com/user-attachments/assets/43db49fd-593d-4d9f-88b0-b41337d5a0e8" />

Luego de crear el Actor, hay que implementar metodos de ese struct para que pueda recibir y procesar mensajes, estos metodos son los **Handlers**: manejadores de mensajes recibidos.

Ciclo de vida de un actor:

<img width="426" height="220" alt="image" src="https://github.com/user-attachments/assets/474df4ef-49e1-4ef8-9771-f2b9ed0a00eb" />

### Mensajes
Un actor se comunica con otro enviando mensajes.

Todos los mensajes son tipados, deben implementar el trait _Message_.

<img width="556" height="149" alt="image" src="https://github.com/user-attachments/assets/b7ca13f0-1c8d-4339-ad77-710ef490281a" />

### Enviar mensajes
Cómo se envían mensajes de un actor a otro? Métodos disponibles:

<img width="437" height="189" alt="image" src="https://github.com/user-attachments/assets/4933168a-e559-4486-96d8-08b30eccc6c0" />

### Contexto
Los actores mantienen el contexto interno de ejecución, o estado. 

Permite al actor 
- determinar su dirección,
- cambiar los limites de la casilla de mensajes, o
- detenerse

Los mensajes primero llegan a la casilla y luego el contexto de ejecución llama al handler especifico de ese mensaje.

### Arbiter
Provee el contexto de ejecucion asincronica para los actores, funciones y futures.

Alojan el entorno donde se ejecuta el actor.

Realizan varias tareas:
- crear un nuevo thread del SO
- ejecutar un event loop
- crear tareas asincronicas en ese event loop


# Semaforos, Barreras y Monitores
A continuación, se definen y modelan las herramientas de sincronización solicitadas, detallando su funcionamiento interno, similitudes, diferencias y el manejo de condiciones críticas.

### 1. Semáforos
Un **semáforo** es un mecanismo de sincronización de alto nivel definido como un tipo de dato compuesto por un **contador entero no negativo (V)** y un **conjunto de procesos bloqueados (L)**.

*   **Comportamiento Interno:** Se basa en dos operaciones atómicas principales:
    *   **`wait(S)` (o `p(S)`):** Intenta ocupar un recurso. Si $V > 0$, resta 1 al contador y el proceso continúa; de lo contrario, el proceso se bloquea y se añade al conjunto $L$.
    *   **`signal(S)` (o `v(S)`):** Libera un recurso. Si el conjunto $L$ está vacío, suma 1 al contador; si hay procesos esperando, despierta a uno de ellos (usualmente de forma arbitraria) para que pase al estado "listo".
*   **Uso Correcto:** Se utilizan para controlar el acceso a recursos limitados o para señalización entre hilos. En Rust, es común el uso del patrón **RAII** mediante `access()`, que libera el semáforo automáticamente al salir del scope.

### 2. Barreras
Las **barreras** permiten coordinar un conjunto de hilos obligándolos a esperar en un punto determinado del código hasta que todos hayan llegado a dicho punto.

*   **Comportamiento Interno:** Poseen una operación fundamental de espera (`wait`). Internamente, mantienen un contador de cuántos hilos han alcanzado la barrera. Cuando el último hilo llega, todos los procesos bloqueados se liberan simultáneamente.
*   **Uso Correcto:** Son ideales para algoritmos que operan por **fases**, como simulaciones paso a paso o productos de matrices por bloques. En Rust, la operación `wait()` devuelve un resultado que permite identificar al **hilo líder** (el último en llegar).

### 3. Monitores
Un **monitor** es una estructura que garantiza la **exclusión mutua** automática y permite que los hilos esperen a que una condición específica se cumpla.

*   **Comportamiento Interno:** Se compone de variables internas protegidas, procedimientos (métodos) que acceden a ellas y un conjunto de **Variables de Condición (CV)**. Las CV no guardan valores, sino una cola FIFO de procesos. Sus operaciones son:
    *   **`waitC(cond)`:** Bloquea siempre al proceso, lo añade a la cola FIFO y libera el lock del monitor.
    *   **`signalC(cond)`:** Despierta al proceso al tope de la cola; si la cola está vacía, no tiene efecto.
*   **Uso Correcto:** Se recomienda para administrar recursos compartidos complejos donde el acceso debe estar **encapsulado**. El programador no necesita manejar manualmente el lock una vez dentro del monitor.

### Similitudes y Diferencias
*   **Encapsulamiento:** Los monitores encapsulan los datos y el protocolo de acceso, mientras que los semáforos no imponen control sobre cómo el proceso usa los datos una vez obtenido el permiso.
*   **Bloqueo de espera:** En un semáforo, `wait` solo bloquea si el contador es cero; en un monitor, `waitC` **siempre bloquea** al proceso.
*   **Efecto de la señal:** `signal` en un semáforo siempre tiene efecto (incrementa el contador o despierta a alguien); `signalC` en un monitor **no hace nada** si no hay hilos en la cola.
*   **Orden de despertar:** Los monitores usan colas **FIFO**, garantizando orden; los semáforos suelen despertar procesos de forma **arbitraria**.

### Spurious Wakeups (Despertares Espurios)
Un **spurious wakeup** ocurre cuando un hilo que está esperando en una variable de condición se despierta sin que ningún otro hilo haya llamado a `signal` o `notify`, o cuando se despierta pero la condición aún no es verdadera (por ejemplo, porque otro hilo tomó el recurso justo antes).

*   **Manejo Correcto:** Para evitar errores lógicos derivados de estos despertares, es obligatorio verificar la condición dentro de un bucle **`while`** en lugar de un `if`.
*   **Ejemplo de implementación:**
    ```rust
    // Uso correcto en monitores/condvars
    while !condicion_cumplida {
        condvar.wait(lock);
    }
    ```
    Esto garantiza que, si el hilo despierta "por error", vuelva a consultar la condición y se bloquee nuevamente si no es seguro proceder.
