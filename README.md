# Kivo Play

> **Play fast. Play together.**

Kivo Play es una plataforma open source de minijuegos multijugador enfocada en experiencias sociales rápidas, salas en tiempo real, juegos competitivos ligeros y una interfaz con identidad indie.

El proyecto combina un frontend web moderno con un backend desarrollado en Rust y lógica multijugador autoritativa desde el servidor.

> **Estado actual: Alpha**

---

## Descripción general

Kivo Play está siendo diseñado como una colección de pequeños juegos multijugador que puedan jugarse rápidamente con amigos, invitados, bots o jugadores públicos.

El objetivo a largo plazo es construir una plataforma flexible en la que cada juego pueda compartir la misma infraestructura multijugador, manteniendo sus propias reglas, estilo visual y sistemas de juego.

Actualmente el proyecto se centra en:

- Salas multijugador en tiempo real
- Códigos para salas privadas
- Nombres de jugador como invitado
- Administración del host
- Estados Ready / Not Ready
- Acciones de juego autoritativas desde el servidor
- Sistema para determinar el orden de los jugadores
- Sincronización mediante WebSockets
- Arquitectura modular para juegos
- Jugabilidad web ligera y responsive

---

## Funciones actuales del Alpha

La versión Alpha ya cuenta con una base multijugador funcional.

### Salas

Los jugadores pueden:

- Crear una sala
- Entrar a una sala mediante un código de seis caracteres
- Elegir entre visibilidad privada o pública
- Seleccionar un máximo de 1 a 4 jugadores
- Utilizar un nombre de invitado
- Salir de una sala
- Ver los jugadores conectados en tiempo real

Los códigos evitan caracteres visualmente ambiguos como:

```text
0 O 1 I L
```

---

### Lobby

Actualmente el lobby permite:

- Identificar al host
- Migrar automáticamente el host cuando este abandona la sala
- Cambiar entre Ready / Not Ready
- Actualizar jugadores en tiempo real
- Mostrar la capacidad de la sala
- Copiar el código de la sala
- Validar el inicio de una partida

Solamente el host puede iniciar la partida.

El servidor comprueba que:

- El jugador que intenta iniciar sea realmente el host
- Haya al menos dos jugadores conectados
- Todos los jugadores estén preparados
- La partida no haya comenzado anteriormente

---

## Sistema de orden de turnos

Kivo Play ya incluye su primera fase de gameplay controlada por el servidor.

Cuando comienza una partida, todos los jugadores reciben una cuenta regresiva sincronizada:

```text
3
2
1
¡GIRA!
```

Después, cada jugador recibe tres dados.

### Reglas de los dados

- Cada jugador posee tres dados.
- Cada dado solamente puede utilizarse una vez.
- El navegador no decide el resultado.
- El cliente únicamente solicita lanzar un dado.
- Rust genera el resultado real.
- Los resultados se sincronizan con todos mediante WebSockets.
- Los dados completados quedan bloqueados.
- El servidor calcula el total de cada jugador.
- Cuando todos terminan, el servidor genera el orden inicial.

Ejemplo de solicitud enviada por el cliente:

```json
{
  "type": "roll_turn_die",
  "die_index": 0
}
```

El servidor decide el resultado y distribuye el nuevo estado a todos los clientes conectados.

El total más alto recibe la primera posición.

La lógica completa de desempate está planificada para una siguiente iteración.

---

## Juegos planificados

Kivo Play está pensado como una plataforma para distintos minijuegos originales.

### Flux

Juego competitivo y rápido de cartas basado en familias dinámicas de cartas y efectos especiales.

Conceptos planificados:

- Plasma
- Tide
- Volt
- Terra

Algunas cartas especiales podrían incluir:

- Block
- Reverse
- Swap
- Shield
- Steal
- Pulse
- Overload
- Wild

Modos planificados:

- Classic
- Blitz
- Chaos
- Solo
- Bots

---

### SudoRush

Sudoku competitivo.

Modos planificados:

- Solo
- Sprint
- Versus
- Elimination

En los modos competitivos los jugadores podrán recibir el mismo tablero mientras que los rivales solamente observan su progreso y no sus respuestas.

---

### Stackfall

Juego arcade rápido de bloques descendentes.

Modos planificados:

- Endless
- Sprint
- Versus
- Battle

Phaser está considerado para la implementación de este juego.

---

### Diceforge

Juego de dados, puntuación y estrategia centrado en:

- Combinaciones
- Modificadores
- Bonificaciones
- Decisiones estratégicas
- Efectos similares a reliquias

Diceforge está pensado como uno de los juegos que más identidad original puede aportar a Kivo Play.

---

## Juegos futuros

### Crownshift

Juego estratégico inspirado en conceptos del ajedrez.

Posibles modos:

- Blitz
- Bullet
- Mini
- Puzzle Duel
- King Hunt

---

### Pocket Putt

Minigolf multijugador.

Posibles modos:

- Classic
- Speed Putt
- Knockout
- Trick Shot

Godot está considerado para las partes del juego que requieran una simulación física más avanzada.

---

## Tecnologías

### Frontend

- Astro
- Svelte
- TypeScript
- HTML
- CSS
- DOM
- SVG
- Canvas

Astro funciona como el framework principal del frontend.

Svelte se utiliza para componentes altamente interactivos y sistemas multijugador en tiempo real.

En el futuro podrían integrarse:

- PixiJS
- Phaser

---

### Backend

- Rust
- Axum
- Tokio
- Serde
- Serde JSON
- WebSockets
- tracing
- UUID
- rand

Rust es responsable de la autoridad de las partidas y la validación de las acciones multijugador.

---

## Arquitectura multijugador

Actualmente Kivo Play utiliza un servidor escrito en Rust con conexiones WebSocket.

El backend administra:

- Salas
- Jugadores
- Hosts
- Estados Ready
- Estado de cada sala
- Estado de las partidas
- Lanzamientos de dados
- Orden de los jugadores
- Sincronización multijugador

Actualmente las salas del Alpha se almacenan en memoria.

Reiniciar el backend elimina las salas activas.

---

## Autoridad del servidor

Kivo Play sigue una regla importante:

> El cliente solicita acciones. El servidor decide el resultado.

Por ejemplo, un jugador no envía:

```json
{
  "result": 6
}
```

En su lugar envía:

```json
{
  "type": "roll_turn_die",
  "die_index": 0
}
```

El servidor desarrollado en Rust valida la acción y genera el resultado.

Este modelo también se aplicará posteriormente a reglas donde la justicia del juego o la información oculta sean importantes.

---

## Roadmap de red

Actualmente la comunicación utiliza WebSockets.

Más adelante se contempla agregar:

- Tokens de reconexión
- Tiempo de gracia para reconectarse
- Detección de jugadores AFK
- Límites de tiempo por turno
- Explorador de salas públicas
- Bots
- WebRTC
- P2P
- Servidor de signaling
- TURN como respaldo
- Arquitectura híbrida entre servidor y P2P

Una futura implementación de reconexión podría permitir aproximadamente 30 segundos para regresar a una partida antes de eliminar o sustituir al jugador.

---

## Seguridad

Kivo Play está siendo diseñado teniendo en cuenta la seguridad multijugador.

Trabajo planificado:

- Validación del lado del servidor
- Rate limiting
- Validación de entradas
- Validación Unicode de nombres
- Tokens seguros de reconexión
- Protección de información oculta
- Protección contra abuso de salas
- Validación de acciones de partida

Los juegos que contengan información privada, como cartas ocultas, no deberían enviar datos innecesarios a otros clientes.

---

## Bots

Se planea incorporar bots en los juegos compatibles.

Posibles identidades:

- Nova — agresivo
- Echo — equilibrado
- Pixel — caótico
- Orbit — estratégico

Los bots también podrían utilizar pequeñas pausas antes de actuar para producir una sensación más natural.

---

## Estructura del proyecto

```text
Kivo-Play/
├── apps/
│   ├── web/
│   │   ├── public/
│   │   ├── src/
│   │   │   ├── components/
│   │   │   ├── games/
│   │   │   ├── layouts/
│   │   │   ├── lib/
│   │   │   ├── pages/
│   │   │   └── styles/
│   │   ├── astro.config.mjs
│   │   ├── package.json
│   │   └── tsconfig.json
│   │
│   └── server/
│       ├── src/
│       │   └── main.rs
│       ├── Cargo.toml
│       └── Cargo.lock
│
├── docs/
├── tests/
├── .github/
├── .gitignore
├── LICENSE
├── README.md
└── README.es.md
```

La estructura seguirá evolucionando conforme los diferentes sistemas de juego se separen en módulos.

---

## Requisitos

### Frontend

- Node.js
- npm

### Backend

- Rust
- Cargo

---

## Ejecutar el proyecto

Clona el repositorio:

```bash
git clone https://github.com/alestooo/Kivo-Play.git
cd Kivo-Play
```

### Iniciar el backend

```bash
cd apps/server
cargo run
```

El servidor se ejecuta en:

```text
http://127.0.0.1:3000
```

Endpoint de estado:

```text
http://127.0.0.1:3000/health
```

WebSocket:

```text
ws://127.0.0.1:3000/ws
```

---

### Iniciar el frontend

Abre otra terminal:

```bash
cd apps/web
npm install
npm run dev
```

Normalmente el frontend de desarrollo estará disponible en:

```text
http://localhost:4321
```

---

## Nota para Windows PowerShell

Dependiendo de la política de ejecución de PowerShell, el archivo `npm.ps1` puede encontrarse bloqueado.

En ese caso pueden utilizarse los comandos:

```powershell
npm.cmd install
npm.cmd run dev
```

y:

```powershell
npx.cmd
```

---

## Comprobaciones durante el desarrollo

### Rust

```bash
cargo fmt
cargo check
```

### Frontend

```bash
npm run build
```

En Windows PowerShell, cuando sea necesario:

```powershell
npm.cmd run build
```

---

## Roadmap

### Núcleo multijugador

- [x] Servidor Rust
- [x] Axum
- [x] Conexiones WebSocket
- [x] Jugadores invitados
- [x] Creación de salas
- [x] Unión mediante código
- [x] Códigos de seis caracteres
- [x] Metadatos para salas privadas/públicas
- [x] Asignación de host
- [x] Migración básica del host
- [x] Ready / Not Ready
- [x] Start Game autoritativo
- [x] Cuenta regresiva sincronizada
- [x] Dados autoritativos desde Rust
- [x] Total para determinar orden
- [x] Ranking inicial
- [ ] Desempate con dados
- [ ] Mensaje explícito Leave Room
- [ ] Tokens de reconexión
- [ ] Tiempo de gracia para reconectarse
- [ ] Sistema AFK
- [ ] Temporizadores de turno
- [ ] Navegador de salas públicas
- [ ] Bots
- [ ] Ciclo completo de partida
- [ ] Play Again

### Juegos

- [ ] Flux
- [ ] SudoRush
- [ ] Stackfall
- [ ] Diceforge
- [ ] Crownshift
- [ ] Pocket Putt
- [ ] Party Mix

### Infraestructura

- [ ] Pruebas automatizadas
- [ ] Playwright E2E
- [ ] CI
- [ ] PostgreSQL
- [ ] Redis
- [ ] Docker
- [ ] Despliegue de producción

Estas tecnologías se introducirán únicamente cuando aporten una necesidad real al proyecto.

---

## Filosofía del proyecto

Kivo Play sigue algunos principios:

1. Mantener pequeña la primera versión.
2. Construir sistemas jugables antes que infraestructura innecesaria.
3. Utilizar autoridad del servidor cuando la justicia del juego lo requiera.
4. Evitar dependencias innecesarias.
5. Crear sistemas multijugador reutilizables.
6. Dar una identidad diferente a cada juego.
7. Priorizar interacciones rápidas y satisfactorias.
8. Agregar complejidad únicamente cuando el proyecto la necesite.

---

## Estado del proyecto

Kivo Play se encuentra actualmente en una etapa Alpha experimental.

Las APIs, protocolos, conceptos de juegos, estructura de carpetas e interfaces pueden cambiar frecuentemente mientras se desarrolla la base multijugador.

---

## Licencia

Kivo Play se distribuye bajo la licencia MIT.

Consulta el archivo `LICENSE` para más información.

---

## Autor

Creado y mantenido por **Alejandro Soto Víquez**.

GitHub: **@alestooo**

---

**Kivo Play — Play fast. Play together.**