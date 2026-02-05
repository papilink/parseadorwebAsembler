# 🚀 GUÍA DE DESARROLLO - MSX2 Processor

**© 2026 PAPIWEB DESARROLLOS INFORMATICOS**

## Configuración del Entorno

### 1. Instalar Rust (si no lo tienes)

```bash
# Descargar e instalar
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verificar instalación
rustc --version
cargo --version
```

### 2. Clonar el Repositorio

```bash
git clone https://github.com/papilink/parseadorwebAsembler.git
cd parseadorwebAsembler
```

### 3. Instalar Dependencias WASM

```bash
# Agregar target WASM
rustup target add wasm32-unknown-unknown

# Instalar wasm-pack
npm install -g wasm-pack
# O con cargo
cargo install wasm-pack
```

---

## 🏗️ Estructura del Proyecto

```
parseadorwebAsembler/
│
├── 📁 src/
│   └── lib.rs                      # Librería principal MSX2Processor
│       ├── MSX2Processor           # Struct principal
│       ├── NormalMap               # Datos de normal maps
│       ├── EdgeMap                 # Datos de bordes Sobel
│       └── PostProcessResult       # Resultado final
│
├── 📁 examples/
│   └── usage.rs                    # 8 ejemplos prácticos
│
├── 📁 tests/
│   └── integration_tests.rs        # 18+ tests de integración
│
├── 📁 .cargo/
│   └── config.toml                 # Configuración build
│
├── Cargo.toml                      # Manifest del proyecto
├── Cargo.lock                      # Lock de dependencias
│
├── 📄 README.md                    # Este archivo
├── 📄 TECNICO.md                   # Documentación técnica
├── 📄 INTEGRACION_WASM.md          # Guía WASM → HTML5
├── 📄 RESUMEN_IMPLEMENTACION.md    # Resumen de features
└── 📄 DESARROLLO.md                # Este archivo
```

---

## 🔨 Compilación

### Compilar para Native (Escritorio)

```bash
# Debug (más rápido de compilar, más lento al ejecutar)
cargo build

# Release (más lento de compilar, más rápido al ejecutar)
cargo build --release

# Ejecutable en: target/release/msx2_processor
```

### Compilar para WebAssembly

```bash
# Build WASM optimizado para web
wasm-pack build --release --target web

# Genera en: pkg/
# - msx2_processor.js       (bindings)
# - msx2_processor.wasm     (binario)
# - package.json            (para npm)
```

### Compilar Específicamente

```bash
# Solo para WASM 32 bits
cargo build --target wasm32-unknown-unknown --release

# Con features específicos
cargo build --release --features console_error_panic_hook

# Verificar sin compilar
cargo check
```

---

## 🧪 Testing

### Ejecutar Todos los Tests

```bash
# Debug
cargo test

# Release (más rápido)
cargo test --release

# Con output detallado
cargo test -- --nocapture

# Test específico
cargo test test_bilinear_interpolation_dimensions

# Tests en paralelo (default)
cargo test -- --test-threads=4

# Tests secuenciales
cargo test -- --test-threads=1
```

### Tests Disponibles

- `test_processor_creation` - Crear instancia
- `test_rgba_conversion` - Conversión MSX2 → RGBA
- `test_bilinear_interpolation_dimensions` - Dimensión correcta 4K
- `test_bilinear_preserves_colors` - Colores interpolados
- `test_normal_map_generation` - Genración normal maps
- `test_normal_map_center_value` - Valores de normal correctos
- `test_sobel_edge_detection` - Detección bordes
- `test_sobel_gradient_detection` - Detección gradientes
- `test_neon_glow_application` - Aplicación glow
- `test_process_with_all_effects` - Procesamiento completo
- `test_process_without_optional_effects` - Sin efectos opcionales
- `test_palette_loading` - Carga paleta correcta
- `test_glow_intensity_levels` - Niveles intensidad glow
- `test_multiple_frames_processing` - Loop múltiples frames
- + más...

---

## 📝 Ejecutar Ejemplos

### Ejemplo 1: Procesamiento Completo

```bash
cargo run --example usage --release
```

**Output esperado:**
```
╔════════════════════════════════════════╗
║   MSX2 PROCESSOR - EJEMPLOS             ║
║   © 2026 PAPIWEB DESARROLLOS            ║
╚════════════════════════════════════════╝

🎮 Procesamiento completado!
   • Imagen RGBA escalada: ... bytes
   • Normal Map: ... bytes
   • Edge Map: ... valores
```

### Ejemplos Incluidos

En `examples/usage.rs`:

1. **Full Processing** - Todos los efectos
2. **Bilinear Upscale** - Solo escalado 4K
3. **Normal Map** - Solo mapas normales
4. **Neon Glow** - Efectos visuales neón
5. **Realtime Loop** - 30 frames
6. **Profiles** - Perfiles predefinidos
7. **Advanced Composition** - Multi-layer
8. **Game Enhancement** - Caso real

---

## 🎨 Desarrollo y Debugging

### Habilitar Output Debug

```rust
// En src/lib.rs
eprintln!("DEBUG: variablé = {:?}", variable);
```

```bash
# Ejecutar con output debug
cargo run --example usage -- --nocapture 2>&1
```

### Profiling

```bash
# Instalar perf
sudo apt install linux-tools-common

# Compilar con símbolos de debug
cargo build --release

# Ejecutar con perf
perf record target/release/msx2-processor
perf report
```

### Memory Profiling

```bash
# Instalar valgrind
sudo apt install valgrind

# Ejecutar
valgrind --tool=memcheck \
         --leak-check=full \
         --show-leak-kinds=all \
         target/release/msx2-processor
```

---

## 📦 Publicar a crates.io

```bash
# 1. Registrarse en crates.io
# https://crates.io/me

# 2. Agregar token local
cargo login

# 3. Actualizar Cargo.toml con info
# - descripción
# - repositorio
# - licencia

# 4. Publicar
cargo publish --dry-run  # Probar
cargo publish             # Real

# 5. Verificar
# https://crates.io/crates/msx2-processor
```

---

## 🌐 Publicar a npm (WASM)

```bash
# 1. Actualizar package.json en pkg/
{
  "name": "@papiweb/msx2-processor",
  "version": "1.0.0",
  "license": "MIT"
}

# 2. Crear cuenta npm
npm adduser

# 3. Publicar
cd pkg
npm publish

# 4. Usar desde npm
npm install @papiweb/msx2-processor
```

---

## 🐛 Debugging WASM en Navegador

### Chrome DevTools

```javascript
// En chrome://devtools mientras ejecutas WASM

// Inspeccionar módulo
console.log(wasmModule);

// Inspeccionar memoria
const memory = new Uint8Array(wasmModule.memory.buffer);

// Breakdown de performance
console.time("process");
const result = processor.process_with_post_effects(...);
console.timeEnd("process");
```

### Firefox Developer Tools

```
1. Abre DevTools (F12)
2. Ir a "Debugger"
3. En sidebar: "sources" → "wasm"
4. Puedes ver disassembly del WASM
```

---

## 📊 Monitoring en Producción

### Implementar logging

```rust
// En lib.rs
#[wasm_bindgen]
pub fn process_with_telemetry(
    &self,
    bin_data: &[u8],
    enable_bilinear: bool,
    enable_normals: bool,
    enable_edges: bool,
    glow_intensity: f32,
) -> PostProcessResult {
    let start = web_sys::js_sys::Date::now();
    
    let result = self.process_with_post_effects(
        bin_data,
        enable_bilinear,
        enable_normals,
        enable_edges,
        glow_intensity,
    );
    
    let elapsed = web_sys::js_sys::Date::now() - start;
    web_sys::console::log_1(&format!("Tiempo: {}ms", elapsed).into());
    
    result
}
```

---

## 🔄 CI/CD (GitHub Actions)

Crear `.github/workflows/rust.yml`:

```yaml
name: Rust Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run tests
        run: cargo test --release
      
      - name: Build WASM
        run: |
          rustup target add wasm32-unknown-unknown
          npm install -g wasm-pack
          wasm-pack build --release --target web
```

---

## 📚 Recursos Útiles

### Documentación
- [The Rust Book](https://doc.rust-lang.org/book/)
- [WASM Bindgen](https://rustwasm.github.io/docs/wasm-bindgen/)
- [WebGL Spec](https://www.khronos.org/webgl/)

### Librerías Relacionadas
- `wasm-bindgen`: JavaScript ↔ Rust
- `web-sys`: Acceso a APIs web
- `rayon`: Paralelización
- `image`: Procesamiento de imágenes

### Ejemplos Online
- [WASM By Example](https://wasmbyexample.org/)
- [Rust WASM Games](https://rustwasm.org/game-of-life/introduction.html)

---

## 🚢 Deployment

### Build Release

```bash
# Compilar versión final
cargo build --release

# Crear WASM optimizado
wasm-pack build --release --target bundler
```

### Optimizaciones WASM

```bash
# Instalar herramientas
npm install -g wasm-opt binaryen

# Optimizar WASM
wasm-opt -Oz -o output.wasm pkg/msx2_processor_bg.wasm

# Comprimir con gzip
gzip -9 pkg/msx2_processor_bg.wasm
```

---

## 💡 Mejores Prácticas

### Rendimiento

```rust
// ✅ BIEN: Reutilizar buffers
let mut output = vec![0u8; capacity];
// ... reutilizar output

// ❌ MAL: Crear buffers innecesarios
for i in 0..1000 {
    let temp = vec![0u8; 1000]; // Evitar
}
```

### Seguridad

```rust
// ✅ BIEN: Validar límites
let idx = x.min(width - 1);

// ❌ MAL: Acceso sin validar
let idx = x; // Posible panic
```

### Testing

```rust
// ✅ BIEN: Assert con mensaje
assert_eq!(result.len(), 100, "Tamaño debe ser 100");

// ✅ BIEN: Descripción clara
#[test]
fn test_nombre_descriptivo() { ... }
```

---

## 📞 Soporte

**PAPIWEB DESARROLLOS INFORMATICOS**

Para problemas o preguntas:
- Issues en GitHub
- Pull requests bienvenidos
- Discusiones en Discussions

---

**Última actualización**: 2026
**Versión**: 1.0.0
**Licencia**: MIT
