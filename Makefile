# Variables
DOCKER_IMAGE = solanafoundation/anchor:v0.32.1
MOUNT = -v $(shell pwd):/workdir
WORKDIR = -w /workdir
USER_ID = $(shell id -u)
GROUP_ID = $(shell id -g)

.PHONY: shell build test clean own new

# ---------------------------------------------------------
# COMANDOS DE DESARROLLO
# ---------------------------------------------------------

# Entrar a la terminal de Docker (útil para explorar)
shell:
	docker run --rm -it $(MOUNT) $(WORKDIR) $(DOCKER_IMAGE) bash

# Construir los programas
build:
	docker run --rm $(MOUNT) $(WORKDIR) $(DOCKER_IMAGE) anchor build
	make own

# Ejecutar tests (Genera wallet temporal + Test + Devuelve permisos al acabar)
test:
	docker run --rm $(MOUNT) $(WORKDIR) $(DOCKER_IMAGE) /bin/bash -c "solana-keygen new --no-bip39-passphrase && anchor test"
	make own

# Crear un nuevo programa usando la versión de Anchor de Docker
# Uso: make new name=mi_programa
new:
	docker run --rm $(MOUNT) $(WORKDIR) $(DOCKER_IMAGE) anchor new $(name)
	make own

# ---------------------------------------------------------
# COMANDOS DE UTILIDAD
# ---------------------------------------------------------

# Recuperar la propiedad de los archivos (SOLUCIÓN AL PERMISSION DENIED)
own:
	sudo chown -R $(USER_ID):$(GROUP_ID) .

# Limpiar target (y recuperar permisos)
clean:
	docker run --rm $(MOUNT) $(WORKDIR) $(DOCKER_IMAGE) cargo clean
	make own