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

