a# Redes de Petri
Sirven para modelar el comportamiento de un sistema de forma tal que podamos hacer un seguimiento y comprobar x propiedad.

## Red Ordinaria de Petri
Es un grafo dirigido bipartito (se puede hacer una particion en dos grupos tal que las aristas de un grupo no se vinculan con nodos del mismo grupo) que cumple con $$PN=(T, P, A)$$.
- $$T$$ ... es un conjunto de nodos llamado transiciones
- P es un conjunto de nodos llamados lugares
- A ... es un conjunto de arcos, subconjunto de todas las posibles combinaciones

Ejemplo

<img width="368" height="115" alt="image" src="https://github.com/user-attachments/assets/bd9ad554-0aa9-4b8c-9d5a-91aab7aec3c9" />

<img width="455" height="91" alt="image" src="https://github.com/user-attachments/assets/7bbd5f19-12c9-43e2-8cb2-8c4b29e8537e" />

### Funcion de Marca
Se define como:
- $$M:P \rightarrow N \cup 0$$

Cuando el token esta en el lugar $$p_{1}$$, entonces $$M(p_{1})=1$$ y $$M(p_{2})=0$$. Por lo tanto M0=(1, 0)

### Funciones de Entrada y Salida
En las redes de petri se definen 

<img width="494" height="154" alt="image" src="https://github.com/user-attachments/assets/75aada13-6557-4fd1-87a7-6dceda37bede" />

### Ejemplo 1

Reveer la clase y explicar ejemplo

<img width="477" height="395" alt="image" src="https://github.com/user-attachments/assets/526bb2aa-c533-4d77-afc9-a4e45adce334" />


### Grafo de alcance



### Algunas interpretaciones
<img width="471" height="167" alt="image" src="https://github.com/user-attachments/assets/a19c4284-69d6-4875-81b9-28c742d3a63a" />


## Red general de Petri
Tienen mas informacion que las redes ordinarias, que son mas limitadas

<img width="511" height="245" alt="image" src="https://github.com/user-attachments/assets/9b7b5601-0d1d-48f9-af96-ba83522ec65d" />

### Reglas Generales de Disparo de Transiciones
<img width="366" height="300" alt="image" src="https://github.com/user-attachments/assets/09aba07e-88e0-4f76-96d7-e2346eec1899" />
