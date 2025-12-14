# Smart Contracts en Solana

Este repositorio contiene la implementación práctica para el Trabajo de Fin de Grado (TFG) titulado **"Smart Contracts en Solana"**, presentado por **Pablo de Mateo**.

El código demuestra desde conceptos básicos hasta patrones avanzados de **Cross-Program Invocation (CPI)** con y sin gestión de privilegios (PDAs).

## Contenido del Repositorio

El proyecto se divide en módulos que ilustran diferentes conceptos de arquitectura en Solana:

### 1. Ejemplo Básico: Contador
* **`programs/counter`**:
    * Implementación robusta de un contador.

### 2. CPI Simple (Patrón Proxy)
Demostración de una llamada entre programas donde un programa delega la ejecución a otro sin comprobaciones de autoridad complejas.
* **`programs/cpi_engine` (Destino)**:
    * Gestiona el estado de un "motor" (RPM).
* **`programs/cpi_lever` (Origen)**:
    * Actúa como una "palanca". Recibe la instrucción del usuario y la reenvía al motor mediante `CpiContext::new`.

### 3. CPI Autenticada (Seguridad con PDAs)
Demostración avanzada donde el programa destino requiere una firma autorizada, la cual es generada por el proxy.
* **`programs/cpi_secure_vault` (Destino)**:
    * Una bóveda que solo permite modificaciones si una autoridad específica firma la transacción.
* **`programs/cpi_secure_proxy` (Origen)**:
    * Utiliza una **Program Derived Address (PDA)** para firmar la llamada CPI (`CpiContext::new_with_signer`).

---

## Entorno de Desarrollo (Docker + Makefile)

Para garantizar la reproducibilidad y evitar conflictos de versiones, este proyecto está diseñado para ejecutarse dentro de un contenedor Docker oficial de Anchor.

Se ha incluido un **`Makefile`** para simplificar la interacción con el contenedor.

### Requisitos
* Docker instalado y en ejecución.
* Make (generalmente preinstalado en Linux/Mac).

### Comandos Disponibles

No necesita instalar Rust ni Solana localmente. Usa los siguientes comandos desde la raíz del proyecto:

| Comando | Descripción |
| :--- | :--- |
| **`make build`** | Compila todos los programas (smart contracts) dentro del contenedor Docker. |
| **`make test`** | Genera una wallet temporal, inicia el validador local y ejecuta los tests de integración (TypeScript). |
| **`make shell`** | Abre una terminal interactiva dentro del contenedor para ejecutar comandos manuales (`anchor`, `solana-keygen`, etc.). |
| **`make clean`** | Elimina los archivos generados por la compilación (`target/`) para asegurar un build desde cero. |
| **`make own`** | **Utilidad:** Restaura los permisos de los archivos generados al usuario local (soluciona problemas de *Permission denied* creados por Docker). |

### Creación de nuevos programas
Si desea añadir un nuevo programa al workspace utilizando la versión de Anchor del contenedor:

```bash
make new name=<nombre_del_programa>