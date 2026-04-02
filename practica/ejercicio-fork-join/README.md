## Ejercicio

### Consigna:
- Descargar de Kaggle o sitio similar un dataset de al menos 4 GB en formato de texto, CSV o JSONL.
- Realizar un procesamiento simple en el dataset, como un word count utilizando Ryon.
- Ejecutar con diferente cantidad de threads (1, 2, 4, 8).
- Comparar tiempos de ejecución y uso de memoria en cada caso.

Sacar los CSV del zip, tienen que estar en `no2_data/processed`.

Probar con `cargo run --release <cantidad de threads>`

Con los tres archivos anda mas rapido con `cantidad de threads = 3`.