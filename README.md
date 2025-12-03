## Ejecución con Docker (Recomendado)

Este proyecto ha sido desarrollado utilizando la imagen oficial de Anchor para garantizar la compatibilidad.

1. Asegúrate de tener Docker instalado y corriendo.
2. Desde la raíz del proyecto, ejecuta el entorno:
   ```bash
   # Ejemplo de comando
   docker run --rm -it -v $(pwd):/workdir -w /workdir solanafoundation/anchor:v0.32.1 bash