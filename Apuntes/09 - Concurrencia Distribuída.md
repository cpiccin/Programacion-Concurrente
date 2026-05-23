# Concurrencia Distribuida

## Repaso de redes
Tipos de servicio:
- **Sin conexión:** los datos se envian al receptor y no hay control de flujo ni de errores. Envia el mensaje y se olvida.
- **Sin conexión con ACK:** por cada dato recibido, el receptor le avisa al emisor que recibió (ACK)
- **Con conexión:** hay control de flujo y control de errores. Hay tres fases:
    - establecimiento de la conexión
    - intercambio de datos
    - cierre de la conexión
 
### Modelo OSI
Modelo de capas de la ISO.

<img width="519" height="483" alt="image" src="https://github.com/user-attachments/assets/210b54e0-0219-4eef-a5ad-d5663e5d622e" />


- Capa fisica: su objetivo es la trasmision de bits a traves de un medio fisico, por ej a traves de un cable de red, a traves de wifi, etc.
- Capa de enlace: su objetivo es la comunicacion fiable de un bit a traves de un medio fisico. Un ejemplo de protolo de capa de enlace es el protocolo Ethernet. La unidad de mensaje de protocolo se denomina Frame.
- Capa de red: se encarga de realizar la comunicacion de los bits a traves de multiples redes, realiza la interconexion de multiples redes, ofrece distintos servicios como el de enrutamiento, para definir a donde se va a enviar el mensaje, y direccionamiento. La unidad de la capa es "paquete". Un ejemplo es el protocolo IP: Un mensaje de IP tiene un header y un body. No hay control de flujo ni de errores.
- Capa de transporte: su objetivo es realizar el envio confiable de los mensajes desde el emisor hacia el receptor. Por ejemplo el protocolo TCP, UDP, etc. La unidad de mensaje es un "segmento". Hay control de flujo y de errores. El receptor puede pedir un reenvio de mensajes por error.
- Capa de sesión: tiene como objetivo mantener estado entre multiples envios de mensajes en una misma comunicacion. Mantener el estado de esa comunicacion. Por ejemplo, el protocolo de SSH.
- Capa de presentacion: para uniformizar la representacion de los datos en los distintos hosts. No podria haber una comunicacion con distintas represetnaciones de los datos. Distintas arquitecturas representan de forma distintas formas a los enteros por ejemplo. Ejemplo estandar ASCII.
- Capa de aplicación: es donde se definen las reglas de negocio del usuario. Ejemplo protocolo de comuniacion web: HTTP que define reglas, procedimientos, que conecta un browser con un servidor web.


### Protocolos 
En la imagen de abajo se ve que aunque no se implemente todas las capas juntas de la ISO, en algun lugar alguna se implementa. 

<img width="595" height="390" alt="image" src="https://github.com/user-attachments/assets/441eb27d-82fc-49e9-b652-1e9dffad6bab" />

## Tipos de sockets
- **Stream sockets**: usan el protocolo TCP: entrega garantizada del flujo de bytes.
- **Datagram sockets**: usan el protocolo UDP: la entrega no está garantizada; servicio sin conexión.
- **Raw sockets**: permiten a las aplicaciones enviar paquetes IP.
- **Sequenced packet sockets**: similares a stream sockets, pero preservan los delimitadores de registro. Utilizan el protocolo SPP (Sequenced Packet Protocol).


# Algoritmos de elección de líder
Van a haber multiples computadoras conectadas a la red.

Un lider va a ser una computadora/programa/nodo que haga algo diferente al resto. Queremos un lider que lleve a cabo determinadas funciones, debe ser elegido por el resto y tambien no debe ser fijo, si se cae el lider se puede realizar una nueva eleccion y elegir el lider.

Se elige a traves de un algoritmo.

Aca se asume que lo votantes son "honestos", no hay tolerancia a fallas porque se trata de una rama clasica de los sistemas distribuidos. Asumimos que no se desobedece al lider

## Algoritmo Bully
Cuando un proceso `P` nota que el lider no responde, inicia el proceso de eleccion:
1. `P` envia el mensaje `ELECTION` a todos los procesos que tenga un id mayor al de el.
2. Si nadie responde `P` gana la eleccion y es el nuevo lider.
3. Si contesta alguien, éste proceso `Q` continua con el proceso y `P` finaliza.
4. El nuevo lider se anuncia al resto con un mensaje de tipo `COORDINATOR`

**Siempre gana el proceso con id mas grande**


## Algoritmo Ring
Necesitamos que los nodos/procesos esten organizados en un anillo lógico: uno nodo se comunica con el siguiente, y recibio del anterior. 
1. Los procesos estan ordenados logicamente: cada uno conoce a su sucesor.
2. Cuando algun proceso nota que el lider falló, le manda a su sucesor un mensaje `ELECTION` que contiene su propio id.
3. El que recibe ese mensaje, le agrega su id y lo reenvia al siguiente.
4. Cuando el proceso original recibe el mensaje, lo cambia a `COORDINATOR` y lo envia. El nuevo lider es el proceso con mayor id de la lista. La lista se mantiene para informar el nuevo anillo.
5. Cuando finaliza la circulacion del mensaje, se elimina del anillo.

