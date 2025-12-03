# Variables
DOCKER_IMAGE = solanafoundation/anchor:v0.32.1
MOUNT = -v $(shell pwd):/workdir
WORKDIR = -w /workdir

.PHONY: shell build test clean

# Entrar a la terminal de Docker
shell:
	docker run --rm -it $(MOUNT) $(WORKDIR) $(DOCKER_IMAGE) bash

# Construir desde fuera
build:
	docker run --rm $(MOUNT) $(WORKDIR) $(DOCKER_IMAGE) anchor build

# Testear (Genera wallet temporal + Ejecuta test)
test:
	docker run --rm $(MOUNT) $(WORKDIR) $(DOCKER_IMAGE) /bin/bash -c "solana-keygen new --no-bip39-passphrase && anchor test"

# Limpiar target
clean:
	docker run --rm $(MOUNT) $(WORKDIR) $(DOCKER_IMAGE) cargo clean