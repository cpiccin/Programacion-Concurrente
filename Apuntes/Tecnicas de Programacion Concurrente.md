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
