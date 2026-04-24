# Redes de Petri
Sirven para modelar el comportamiento de un sistema de forma tal que podamos hacer un seguimiento y comprobar x propiedad.

## Red Ordinaria de Petri
Es un grafo dirigido bipartito (se puede hacer una particion en dos grupos tal que las aristas de un grupo no se vinculan con nodos del mismo grupo). Las redes de petri ordinarias estan formadas por $$PN=(T, P, A)$$.
- $$T$$ ... es un conjunto de nodos llamado transiciones: producen los cambios de estado del sistema
- P es un conjunto de nodos llamados lugares (places)
- A ... es un conjunto de arcos dirigidos, subconjunto de todas las posibles combinaciones

Ejemplo

<img width="368" height="115" alt="image" src="https://github.com/user-attachments/assets/bd9ad554-0aa9-4b8c-9d5a-91aab7aec3c9" />

<img width="455" height="91" alt="image" src="https://github.com/user-attachments/assets/7bbd5f19-12c9-43e2-8cb2-8c4b29e8537e" />

### Funcion de Marca
Se define pra cada estado del sistema como:
- $$M:P \rightarrow N \cup 0$$

En los places se van a ubicar tokens que van a estar circulando entre estos places. La funcion de marca de p1 indica cuantos tokens hay en cada place cuando tengo un token en p1.

Cuando el token esta en el lugar $$p_{1}$$, entonces $$M(p_{1})=1$$ y $$M(p_{2})=0$$. Por lo tanto M0=(1, 0)

### Funciones de Entrada y Salida
En las redes de petri se definen 

<img width="494" height="154" alt="image" src="https://github.com/user-attachments/assets/75aada13-6557-4fd1-87a7-6dceda37bede" />

O sea I(t) son los places de **entrada** a la transicion t. Y O(t) son los places de salida (a donde aw puede ir desde la transicion t) de la transicion t.

### Ejemplo 1
Tengo el sistema siguiente. El estado inicial es en p1 porque ahi esta el token inicialmente. Dado este estado se puede disparar la transicion t1, se puede consumir el token de p1 y se crea un token para p2 y p3.

Como hay un token en p3, se puede disparar la transicion t3 y colocarse un token en p5.

Tambien se puede disparar la transicion t2, se pone un token en p4 y se puede disparar la transicion t4 que saca el token de p4 y lo pone de nuevo en p2, o sino, si hay un token en p4, junto al de p5, se puede disparar la transicion t5, se deben cumplir las dos condiciones, tiene que consumir los dos, no alcanza con uno solo, asi el sistema vuelve al sistema inicial en p1.


<img width="477" height="395" alt="image" src="https://github.com/user-attachments/assets/526bb2aa-c533-4d77-afc9-a4e45adce334" />


### Grafo de alcance
Grafo dirigido.

<img width="379" height="273" alt="image" src="https://github.com/user-attachments/assets/bb3f1907-3094-405a-90c5-636aa23b1d66" />

### Ejemplo 2
Aca sucede que si del estado inicial en p1, se dispara t2 de una, nunca se va a poder ejecutar ningun otro estado porque no hay token en p2.
<img width="344" height="319" alt="image" src="https://github.com/user-attachments/assets/fa0e3a58-9f35-44b7-b4ca-2b38dc7b865d" />

Vemos en el grado de alcance que puede haber un loop infinito o tambien, si se ejecuta primero t2, muere ahi el flujo: hay un deadlock!!!!

<img width="704" height="447" alt="image" src="https://github.com/user-attachments/assets/ed729b47-61ec-40c6-ba49-42f628670042" />

### Algunas interpretaciones
Las redes de petri se usan para modelar sistemas de lo que sea.

Ejemplos:

<img width="471" height="167" alt="image" src="https://github.com/user-attachments/assets/a19c4284-69d6-4875-81b9-28c742d3a63a" />


## Red general de Petri
Tienen mas informacion que las redes ordinarias, que son mas limitadas. Ahora cada arco tiene peso W positivo y una funcion de marca inicial M0, o sea que se define como es el estado inicial del sistema.

<img width="511" height="245" alt="image" src="https://github.com/user-attachments/assets/9b7b5601-0d1d-48f9-af96-ba83522ec65d" />

### Reglas Generales de Disparo de Transiciones

<img width="366" height="300" alt="image" src="https://github.com/user-attachments/assets/09aba07e-88e0-4f76-96d7-e2346eec1899" />

O sea... **Consume** 2 de p1, 1 de p2 y **genera** 2 en p3.

<img width="224" height="154" alt="image" src="https://github.com/user-attachments/assets/fbb1c873-f560-4518-8973-4b808b801aae" />

### Ejemplo 3
<img width="434" height="269" alt="image" src="https://github.com/user-attachments/assets/217d151d-b820-41ab-8159-272f41faac90" />

Puede suceder que `a-b` sea cero o que no sea cero, si es cero el sistema llega al resultado "indefinido". Si no es cero se puede realizar la division y devolver x.

### Ejemplo: Productor Consumidor con Bufer Infinito
<img width="659" height="438" alt="image" src="https://github.com/user-attachments/assets/30823553-823e-498b-8cab-0c4534478409" />

p1 dispara la transicion t1 que significa "producir" y coloca un token en p2 indicando que se produjo el elemento, t2 significa "colocar elemento en el buffer", p5 es el estado "elemento en el buffer", p1 vuelve al estado inicial "listo para producir". 

Para dispararse el estado t3 se necesita el token que salio de p1, cuando llega a p5 se consume ese token y el de p3 para disparar t3, t3 se lo pasa a p4 que dispara t4 y el token regresa a p3. 

p3 solo consume, p1 solo produce. Y si p1 produce mas rapido de lo que p3 consume, p5 puede llenarse infinitamente.

### Ejemplo: Productor Consumidor con Bufer Acotado
<img width="674" height="433" alt="image" src="https://github.com/user-attachments/assets/69b6f9de-e48b-4721-aa14-2684a11393ca" />

Quiero limitar al productor para que no consuma infinitamente como en el caso anterior.

Se agrega el estado p6 tiene `N` tokens, habran `N` lugares disponibles en el buffer. Para poder ejecutarse t2 tiene la condicion tener elemento producito y ademas tener espacio en el buffer. 

### Ejemplo: Cliente - Servidor
El cliente le hace una pregunta al servidor, este la procesa y le responde al cliente

<img width="786" height="465" alt="image" src="https://github.com/user-attachments/assets/a3ae0fd4-fd87-4611-be0c-b8cad0aeceb5" />

En p1 empieza el pedido del cliente, en p6 esta listo el servidor. La transiscion t2 envia el pedido y va al place p2 que es "esperando respuesta". El servidor solo hace algo cuando tiene un pedido del cliente. Cuando el servidor responde dispara la transicion t3, vuelve a estar listo para responder, y el cliente puede volver a preguntar.

