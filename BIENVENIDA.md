```
╔══════════════════════════════════════════════════════════════════════╗
║                                                                      ║
║                   🎮 MSX2 PROCESSOR - PAPIWEB 2026                   ║
║                                                                      ║
║              Post-Procesamiento Avanzado de Sprites Retro            ║
║                                                                      ║
║  ✨ Interpolación Bilineal (256x212 → 4K)                            ║
║  💡 Generación de Normal Maps (iluminación 3D)                       ║
║  🌟 Detección de Bordes (Sobel + Glow neón)                          ║
║  ⚡ Loop de procesamiento en tiempo real                              ║
║                                                                      ║
╚══════════════════════════════════════════════════════════════════════╝
```

# 👋 ¡Bienvenido!

Acabas de clonar **MSX2 Processor**, un procesador avanzado escrito en Rust que transforma gráficos retro de baja resolución (256×212) en imágenes modernas de 4K con efectos visuales sofisticados.

---

## 🚀 Empieza en 5 minutos

### Opción 1: Rápido (recomendado)
```bash
cargo run --example usage --release
```

### Opción 2: WebAssembly
```bash
rustup target add wasm32-unknown-unknown
npm install -g wasm-pack
wasm-pack build --release --target web
```

---

## 📖 ¿Dónde Ir?

### ⚡ Si tienes prisa (5 minutos)
→ Lee [QUICKSTART.md](QUICKSTART.md)

### 📘 Si quieres visión general (10 minutos)
→ Lee [README.md](README.md)

### 🔍 Si quieres entender TODO (2+ horas)
→ Lee el [INDICE_DOCUMENTACION.md](INDICE_DOCUMENTACION.md)

---

## 📁 Estructura del Proyecto

```
parseadorwebAsembler/
│
├── 📄 QUICKSTART.md                  ← COMIENZA AQUÍ
├── 📄 INDICE_DOCUMENTACION.md        ← Guía de documentos
│
├── 📚 Documentación Principal
│   ├── README.md                     Visión general
│   ├── TECNICO.md                    Detalles matemáticos
│   ├── DESARROLLO.md                 Guía para devs
│   ├── INTEGRACION_WASM.md           HTML5 Canvas + WASM
│   ├── DIAGRAMAS.md                  Visualizaciones
│   └── RESUMEN_IMPLEMENTACION.md     Qué se implementó
│
├── 💻 Código (Rust)
│   ├── src/lib.rs                    Implementación (368 líneas)
│   ├── examples/usage.rs             8 ejemplos prácticos
│   └── tests/integration_tests.rs    18+ tests
│
├── ⚙️ Configuración
│   ├── Cargo.toml                    Dependencias
│   └── .cargo/config.toml            Config build
│
└── 📦 Compilación
    └── pkg/                          (generado por wasm-pack)
```

---

## 🎯 Funcionalidades Principales

### 1️⃣ **Interpolación Bilineal**
Escala píxeles de 256×212 a 3840×2160 (4K) sin pixelado.

```rust
let upgraded = processor.bilinear_interpolation(&rgba);
// 256×212 → 3840×2160
```

### 2️⃣ **Normal Maps**
Genera vectores 3D para iluminación dinámica (linterna, sombras).

```rust
let normals = processor.generate_normal_map(&rgba);
// Vectores (X, Y, Z) para cada píxel
```

### 3️⃣ **Detección Sobel**
Identifica bordes para efectos neón/glow.

```rust
let edges = processor.detect_edges_sobel(&rgba);
let glow = processor.apply_neon_glow(&rgba, &edges, intensity);
```

### 4️⃣ **Todo Integrado**
Una función que lo hace todo.

```rust
let result = processor.process_with_post_effects(
    &data, true, true, true, 1.5
);
// rgba + normals + edges listos
```

---

## ✨ Características

- ✅ **Marca PAPIWEB incluida** en todo el código
- ✅ **Compilación WASM** lista para navegadores
- ✅ **Totalmente documentado** en español
- ✅ **18+ tests** de integración
- ✅ **8 ejemplos** prácticos
- ✅ **Optimizaciones** SIMD-friendly
- ✅ **Licencia MIT** - uso comercial permitido

---

## 📊 Rendimiento

| Operación | Tiempo |
|-----------|--------|
| Conversión RGBA | < 1ms |
| Bilinear 4K | 50-100ms |
| Normal Maps | 10-20ms |
| Sobel Edges | 10-20ms |
| Neon Glow | 30-50ms |
| **Total** | **~100-200ms** |

Perfecto para **60 FPS** (desactivando normal maps en loop).

---

## 🎓 Ejemplos Incluidos

En `examples/usage.rs`:

1. Procesamiento completo
2. Bilinear upscale puro
3. Generación normal maps
4. Efecto neón
5. Loop en tiempo real
6. Perfiles predefinidos
7. Composición multi-capa
8. Caso práctico: juego mejorado

**Ejecutar:** `cargo run --example usage --release`

---

## 🧪 Tests

18+ tests de integración para validar:
- Conversión RGBA
- Interpolación bilineal
- Generación normal maps
- Detección Sobel
- Aplicación glow
- Procesamiento completo
- Múltiples frames

**Ejecutar:** `cargo test --release`

---

## 🌐 Usar en Web

### Manual Rápido

```html
<canvas id="canvas" width="3840" height="2160"></canvas>
<script type="module">
  import init, { MSX2Processor } from './pkg/msx2_processor.js';
  
  await init();
  const proc = new MSX2Processor(256, 212);
  const result = proc.process_with_post_effects(data, ...);
  
  // Mostrar en canvas
  ctx.putImageData(result.get_rgba(), 0, 0);
</script>
```

**Docs completa:** [INTEGRACION_WASM.md](INTEGRACION_WASM.md)

---

## 💡 Casos de Uso

- 🕹️ **Emuladores retro** con gráficos modernizados
- 🎬 **Upscaling** de sprites MSX2 a 4K
- 🌃 **Efectos cyberpunk** con glow neón
- 🎮 **Juegos retro** con iluminación dinámica
- 🎨 **Herramientas artísticas** de procesamiento

---

## 📞 Contacto

**PAPIWEB DESARROLLOS INFORMATICOS**

Procesamiento avanzado de sprites retro con tecnologías modernas.

- GitHub Issues: [Reportar problemas](https://github.com/papilink/parseadorwebAsembler/issues)
- Documentación: Ver [INDICE_DOCUMENTACION.md](INDICE_DOCUMENTACION.md)

---

## 🎓 Aprende Más

### Documentación Oficial
- [README.md](README.md) - Visión general completa
- [TECNICO.md](TECNICO.md) - Fórmulas matemáticas
- [DESARROLLO.md](DESARROLLO.md) - Guía para devs
- [INTEGRACION_WASM.md](INTEGRACION_WASM.md) - Web integration
- [DIAGRAMAS.md](DIAGRAMAS.md) - Visualizaciones

### Código Fuente
- [src/lib.rs](src/lib.rs) - Implementación principal
- [examples/usage.rs](examples/usage.rs) - Ejemplos
- [tests/integration_tests.rs](tests/integration_tests.rs) - Tests

---

## ⚡ Comandos Útiles

```bash
# Compilar
cargo build --release

# Ejecutar ejemplos
cargo run --example usage --release

# Tests
cargo test --release

# WASM
wasm-pack build --release --target web

# Documentación local
cargo doc --open

# Check sin compilar
cargo check
```

---

## 📋 Próximos Pasos

### 1️⃣ Lee QUICKSTART
```bash
cat QUICKSTART.md
```

### 2️⃣ Ejecuta los ejemplos
```bash
cargo run --example usage --release
```

### 3️⃣ Integra en tu proyecto
```rust
use msx2_processor::MSX2Processor;
let proc = MSX2Processor::new(256, 212);
```

### 4️⃣ Explora la documentación
```bash
ls -la *.md
```

---

## 📜 Licencia

MIT License - Libre para uso comercial y personal.

---

## 🎉 ¡Listo!

Has desbloqueado todo lo que necesitas para procesar sprites MSX2 con efectos modernos.

**Comienza por:** [QUICKSTART.md](QUICKSTART.md)  
**O ve a:** [INDICE_DOCUMENTACION.md](INDICE_DOCUMENTACION.md)

```
╔══════════════════════════════════════════════════════════════════════╗
║                                                                      ║
║                     ¡Bienvenido a PAPIWEB! 🚀                        ║
║                                                                      ║
║              Transformando gráficos retro en arte moderno            ║
║                                                                      ║
╚══════════════════════════════════════════════════════════════════════╝
```
