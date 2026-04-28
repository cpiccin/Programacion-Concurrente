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


