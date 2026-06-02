# Transacciones

El sistema esta conformado por un conjunto de procesos independientes donde **cada uno puede fallar independientemente**.

Los errores en la comunicacion se manejan transparentemente por la capa de comunicacion.

Un storage estable:
- se implementa con discos
- la probabilidad de perder datos es muy chica

## Primitivas
- `BEGIN TRANSACTION`: marca el inicio de la transaccion
- `END TRANSACTION`: finalizar la transaccion e intentar hacer commit
- `ABORT TRANSACTION`: terminar de forma forzosa la transaccion y restaurar valores
- `READ`: leer datos de un archivo u otro objeto
- `WRITE`: escribir datos a un archivo u otro objeto

## Propiedades ACID
- **Atomica**: no puede ser dividida
- **Consistente**: la transaccion cumple con todas los invariantes del sistema
- **Aisladas o serializadas**: las transacciones concurrentes no interfieren con ellas mismas
- **Durables**: cuando se commitean los cambios, son permanentes (exceptuando transacciones anidadas)

## Como implementar transacciones

### Private Workspace
Al iniciar una transaccion, el proceso recibe una copia de todos los archivos a los que tiene acceso. **Hasta que hace commit, el proceso trabaja con esa copia**. Y al hacer commit, se persisten los cambios.

**Desventaja:** extremadamente costoso salvo por optimizaciones.

### Writeahead Log
Se mantiene una lista de los cambios aplicados sobre archivos modificados, primero se escribe la lista, despues se hace la moficacion. 

Al commitear la transaccion se lo registra en el log.

Se puede hacer rollback para deshacer los cambios en caso de abortar la transaccion, el log se lee de atras hacia adelante.

### Commit en dos fases
El coordinador es un proceso que ejecuta la transaccion. Es un algoritmo de dos fases:
1. Se prepara la coordinacion:
    - el coordinador escribe en el log el mensaje `prepare` y lo envia al resto de procesos.
    - los procesos que reciben el mensaje escriben `ready` en el log y se lo envian al coordinador
2. El coordinador tiene la confirmacion de que puede ejecutar los cambios entonces los lleva a cabo:
    - El coordinador hace los cambios y envia el mensaje `commit` a los procesos
    - Los que lo reciben, escriben `commit` en el log y envian `finished` al coordinador

## Control de concurrencia
Como se controla la ejecucion de multiples transacciones.

### Lockeo: two-phase locking 
- Fase de expansion: se toman todos los locks a usar
- Fase de contraccion: se liberan todos los locks sin tomar nuevos locks

Esto garantiza que las transacciones sean serializables pero no garantiza la ausencia de deadlocks 😿

**Strict two-phase locking:** la contraccion ocurre despues del commit.

### Concurrencia Optimista
Se trata de modificar de forma optimista, o sea suponiendo que no van a haber conflictos y si al commitear los hay, se aborta la transaccion.

- **Ventaja:** no hay deadlocks y se favorece el paralelismo.
- **Desventaja:** rehacer todo puede ser costoso en codiciones de alta carga

36:26
