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
Un lock queda en estado envenenado cuando un thread lo toma de forma exclusiva (write lock) y mientras tiene tomado el lock, ejecuta `panic!`.

Las llamadas posteriores a `read()` y `write()` sobre el mismo lock, devolverán Error.
