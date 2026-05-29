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


17:30
