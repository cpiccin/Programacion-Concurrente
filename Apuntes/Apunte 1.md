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
* Canales / mensajes
* Programación asincrónica
* Actores

Hay conjuntos de operaciones que no pueden intercalarse porque pueden surgir inconsistencias con el estado o con los datos. Por eso hay que serializar el acceso al estado compartido. **Serializar** es para controlar el acceso a las variables compartidas

Los procesos se ejecutarán al mismo tiempo, pero habrá ciertos conjuntos de procedimientos tales que solo una ejecución de un procedimiento en cada se permite que suceda a la vez. Cualquier otro proceso que intente ejecutar cualquier procedimiento en el conjunto será obligado a esperar hasta que la primera ejecución haya terminado.

Podemos marcar regiones de código que no pueden superponerse en la ejecución al mismo tiempo.

