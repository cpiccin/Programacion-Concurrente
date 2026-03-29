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

## Ejemplo
### Version Sincronica
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
### Version Asincronica
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
-   Si la operación se completó, retorna: Poll::Ready(output) (output es el resultado final de la operación).
-    Si no se completó, retorna Pending


```
let f = suma_async();
```

`f` no es el resultado, es un Future. El resultado todavía no existe.

Se puede decir que es un modelo **piñata** de la programación asincrónica: lo único que se puede hacer con un future es golpearlo 😢 con poll hasta que caiga el valor.

