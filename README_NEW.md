# 🎮 MSX2 Processor - Post-Procesamiento Avanzado

**© 2026 PAPIWEB DESARROLLOS INFORMATICOS**

Procesador avanzado de sprites MSX2 con post-procesamiento de efectos visuales en tiempo real. Transforma gráficos retro de baja resolución (256×212) a 4K moderno con interpolación bilineal, generación de mapas normales y detección de bordes con efectos neón cyberpunk.

## 🌟 Características

### ✨ **Interpolación Bilineal (256x212 → 4K)**
Escala sprites MSX2 a resolución 4K verdadera (3840×2160 píxeles) con interpolación suave que elimina pixelado artificial manteniendo los detalles visuales.

### 💡 **Generación de Normal Maps**
Calcula vectores normales tridimensionales basados en luminancia de píxeles:
- Iluminación dinámica realista
- Linterna del ratón sobre sprites con sombras propias
- Compatibilidad con bump mapping y parallax mapping
- Efectos de profundidad visual avanzados

### 🌟 **Detección de Bordes (Filtro Sobel)**
Identifica y procesa bordes del sprite original con filtro Sobel:
- Glow neón en siluetas de sprites
- Efecto cyberpunk/retro gaming atractivo
- Resaltado automático de detalles
- Configuración de intensidad variable

### ⚡ **Procesamiento en Tiempo Real**
- Compilación a WebAssembly para navegadores
- Optimizaciones SIMD para máximo rendimiento
- Soporte para loop de renderizado 60 FPS
- Perfiles de rendimiento configurables

## 📦 Estructura del Proyecto

```
parseadorwebAsembler/
├── src/
│   └── lib.rs              # Librería principal (MSX2Processor)
├── examples/
│   └── usage.rs            # 8 ejemplos de uso práctico
├── tests/
│   └── integration_tests.rs # Suite completa de tests
├── Cargo.toml              # Configuración Rust
├── README.md               # Este archivo
└── TECNICO.md              # Documentación técnica detallada
```

## 🚀 Quick Start

### Instalación

```bash
# Clonar repositorio
git clone https://github.com/papilink/parseadorwebAsembler.git
cd parseadorwebAsembler

# Compilar para WASM (navegador)
wasm-pack build --release --target web

# Compilar librería nativa
cargo build --release

# Ejecutar tests
cargo test --release

# Ejecutar ejemplos
cargo run --example usage --release
```

### Uso Básico

```rust
use msx2_processor::MSX2Processor;

let processor = MSX2Processor::new(256, 212);

// Procesar datos MSX2 con todos los efectos
let result = processor.process_with_post_effects(
    &sprite_binary_data,
    true,   // Escala bilineal a 4K
    true,   // Generar normal maps
    true,   // Detectar bordes (Sobel)
    1.5,    // Intensidad glow neón
);

// Acceder a los resultados
let rgba_4k = result.get_rgba();        // Imagen en 4K
let normals = result.get_normals();     // Vectores normales
let edges = result.get_edges();         // Mapa de bordes
```

## 🎯 Métodos Principales

### Conversión Básica
```rust
pub fn transform_to_rgba(&self, bin_data: &[u8]) -> Vec<u8>
```
Convierte formato MSX2 4bpp a RGBA 32bpp estándar.

### Escalado
```rust
pub fn bilinear_interpolation(&self, rgba_data: &[u8]) -> Vec<u8>
```
Interpolación bilineal: 256×212 → 3840×2160 (4K).

### Iluminación 3D
```rust
pub fn generate_normal_map(&self, rgba_data: &[u8]) -> Vec<u8>
```
General normal maps para efectos de iluminación dinámica.

### Detección de Bordes
```rust
pub fn detect_edges_sobel(&self, rgba_data: &[u8]) -> Vec<f32>
```
Filtro Sobel para identificar bordes.

### Efectos Visuales
```rust
pub fn apply_neon_glow(&self, rgba_data: &[u8], edges: &[f32], intensity: f32) -> Vec<u8>
```
Aplica glow neón a bordes detectados.

### Procesamiento Completo (Recomendado)
```rust
pub fn process_with_post_effects(
    &self,
    bin_data: &[u8],
    enable_bilinear: bool,
    enable_normals: bool,
    enable_edges: bool,
    glow_intensity: f32,
) -> PostProcessResult
```

## 📊 Especificaciones

| Aspecto | Valor |
|---------|-------|
| **Formato entrada** | MSX2 4bpp (Screen 5) |
| **Resolución original** | 256×212 píxeles |
| **Resolución máxima** | 3840×2160 (4K Ultra HD) |
| **Colores** | Paleta 16 colores RGBA |
| **Métodos de procesamiento** | 6 principales |
| **Compilación** | WebAssembly + Nativo |
| **Licencia** | MIT |

## 💼 Casos de Uso

### 🕹️ Mejora de Juegos Retro
```rust
// Juego MSX2 retro con visual moderna
let result = processor.process_with_post_effects(
    &rom_sprite,
    true,   // 4K
    true,   // Sombras dinámicas
    true,   // Glow neón
    1.5,
);
```

### 🎬 Emulación Avanzada
```rust
// Emulador MSX2 con gráficos mejorados
let result = processor.process_with_post_effects(
    &frame_data,
    true,   // Escala sin pixelado
    false,  // Sin overhead de normals
    true,   // Detalles visuales
    1.0,
);
```

### 🌃 Efecto Cyberpunk
```rust
// Retro gaming con estética cyberpunk
let result = processor.process_with_post_effects(
    &sprite,
    true,   // Alta res
    false,  // Minimalista
    true,   // Bordes vibrantes
    2.5,    // Glow muy intenso
);
```

## 🔬 Algoritmos Implementados

### Interpolación Bilineal
Escala suave de píxeles mediante interpolación de 4 vecinos.

### Luminancia (BT.709)
Cálculo perceptual de brillo: L = 0.299×R + 0.587×G + 0.114×B

### Filtro Sobel
Detección de bordes mediante gradientes direccionales con kernels 3×3.

### Vector Normal (Bump Mapping)
Cálculo de normales basado en différences de altura para iluminación 3D.

## 📈 Rendimiento

| Operación | Tiempo Aprox. |
|-----------|---------------|
| RGBA Conv. | < 1ms |
| Bilinear | 50-100ms |
| Normal Map | 10-20ms |
| Sobel Edge | 10-20ms |
| Neon Glow | 30-50ms |
| Total | ~100-200ms |

## 🧪 Testing

```bash
# Ejecutar todos los tests
cargo test --release

# Tests específicos
cargo test test_bilinear_interpolation_dimensions
cargo test test_normal_map_generation
cargo test test_sobel_edge_detection

# Con output detallado
cargo test -- --nocapture
```

**Cobertura**: 18+ tests de integración cubriendo todos los componentes principales.

## 📖 Documentación

- **[TECNICO.md](TECNICO.md)**: Documentación técnica detallada, API completa y fórmulas matemáticas
- **[examples/usage.rs](examples/usage.rs)**: 8 ejemplos prácticos de uso
- **[tests/integration_tests.rs](tests/integration_tests.rs)**: Suite de tests

## 🎨 Integración WebGL

```javascript
// Cargar WASM
const wasmModule = await import('./msx2_processor.js');

// Crear procesador
const processor = new wasmModule.MSX2Processor(256, 212);

// Procesar datos
const result = processor.process_with_post_effects(
    spriteData,
    true,   // bilinear
    true,   // normals
    true,   // edges
    1.5     // glow
);

// Usar en canvas/WebGL
const imageData = new ImageData(result.get_rgba(), 3840, 2160);
ctx.putImageData(imageData, 0, 0);
```

## 🏗️ Arquitectura

```
MSX2 Binary (4bpp)
        ↓
   [RGBA Converter]
        ↓
   ┌────┴────┬──────────┬─────────┐
   ↓         ↓          ↓         ↓
[Bilinear][Normals] [Sobel]  [Composición]
   ↓         ↓          ↓         ↓
3840×2160  Vectores  Magnitud   Final
   RGBA      3D        2D       RGBA
```

## 📞 Información

**PAPIWEB DESARROLLOS INFORMATICOS**

Procesamiento avanzado de gráficos retro con tecnologías modernas.

## 📄 Licencia

MIT License - Libre para uso comercial y personal.

---

**Creado en 2026** | Optimizado para WASM/WebGL | Listo para producción

Ver [TECNICO.md](TECNICO.md) para documentación API completa.
