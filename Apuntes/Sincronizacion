# Sincronizacion

## Estados de ejecucion de un proceso (teorico)
<img width="315" height="163" alt="image" src="https://github.com/user-attachments/assets/371007d3-01d7-49e7-909f-87f7e68ab8a1" />

## Semaforos
Un semáforo es una estructura de sincronización para coordinar el acceso de múltiples procesos o hilos a recursos compartidos, evitando conflictos como race conditions. Funciona como un contador.

Es un **tipo de dato** compuesto por dos campos:
- Un entero no negativo llamado V, es el contador del semaforo, es de implementacion interna entonces no se lo puede consultar
- Un set de procesos llamado L

Se inicializa con un valor $$k \ge 0$$ y con el conjunto vacio $$\emptyset$$

Un semáforo es un mecanismo que regula el acceso a recursos compartidos mediante un contador y bloqueo/desbloqueo de procesos, garantizando sincronización entre ejecuciones concurrentes.

### Contador
- Si $$\text{Contador} > 0$$ el recurso esta disponible
- Si $$\text{Contador} \le 0$$ el recurso no esta disponible
- Si el valor es 0 o 1, se llaman **semaforos binarios** y se comportan igual que los locks de escritura (Mutex)

Se definen dos operaciones atomicas sobre un semaforo $$S$$:
- `wait(5)` llamada `p(5)`: si un proceso intenta usar el recurso:
  - Si V > 0, lo decrementa y continúa.  
  - Si V = 0, el proceso se bloquea y se agrega a la lista de espera.
- `signal(S)` llamada `v(S)`: si un proceso libera el recurso.  
  - Incrementa V.  
  - Si hay procesos esperando, despierta a uno.
