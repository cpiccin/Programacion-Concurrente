# Programación Asincrónica  

Si se crean muchos threads, puede surgir un problema de demanda de memoria ya que cada thread necesita su propio stack y recursos asociados

Los lenguajes modernos introducen el concepto de **continuacion**. Una **continuación** es una tarea que puede interrumpirse voluntariamente para ceder el uso de la CPU, porque esta no puede avanzar (por ejemplo, está esperando una operación de I/O).

Son las **tareas asincronicas** de Rust para intercalar tareas en un unico thread o en un pool de threads. En rust no tienen un stack ni contexto propio entonces:
- Mas mucho livianas que los threads.
- Mas rapidas de crear, mas eficiente de pasarle el control a ellas.
- Menor overhead de memoria. 

El codigo asincronico es como el de threads pero se maneja diferente la forma en que se bloquean las operaciones.
