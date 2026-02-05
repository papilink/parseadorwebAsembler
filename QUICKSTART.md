# ⚡ QUICKSTART - Comenzar en 5 minutos

**© 2026 PAPIWEB DESARROLLOS INFORMATICOS**

## 1️⃣ Instalación (1 minuto)

```bash
# Clonar repo
git clone https://github.com/papilink/parseadorwebAsembler.git
cd parseadorwebAsembler

# Ya está listo para compilar
```

## 2️⃣ Compilar (2 minutos)

### Opción A: Nativo (Escritorio)
```bash
cargo build --release
cargo run --example usage --release
```

**Output esperado:**
```
🎮 Procesamiento completado!
🕹️  CASO DE USO: Mejora de Juego Retro MSX2
...
```

### Opción B: WebAssembly (Web)
```bash
# Instalar herramientas (primera vez)
rustup target add wasm32-unknown-unknown
npm install -g wasm-pack

# Compilar WASM
wasm-pack build --release --target web

# Archivos listos en: pkg/
```

## 3️⃣ Usar en tu Código (1 minuto)

### Rust Nativo

```rust
use msx2_processor::MSX2Processor;

fn main() {
    // Crear procesador
    let processor = MSX2Processor::new(256, 212);
    
    // Datos MSX2 (4bpp)
    let sprite_binary = vec![0x12, 0x34, 0x56, 0x78];
    
    // Procesar con TODOS los efectos
    let result = processor.process_with_post_effects(
        &sprite_binary,
        true,   // Escala a 4K
        true,   // Normal maps
        true,   // Detección bordes
        1.5,    // Glow intensity
    );
    
    // Acceder resultados
    let rgba_4k = result.get_rgba();      // 3840×2160 RGBA
    let normals = result.get_normals();   // Vectores 3D
    let edges = result.get_edges();       // Bordes Sobel
    
    println!("✅ Procesado: {} KB", rgba_4k.len() / 1024);
}
```

### JavaScript + WebAssembly

```javascript
// En tu HTML
<script type="module">
  import init, { MSX2Processor } from './pkg/msx2_processor.js';
  
  async function main() {
    await init();
    
    // Crear procesador
    const proc = new MSX2Processor(256, 212);
    
    // Datos MSX2
    const spriteData = new Uint8Array([0x12, 0x34, ...]);
    
    // Procesar
    const result = proc.process_with_post_effects(
        spriteData,
        true, true, true, 1.5
    );
    
    // Mostrar en canvas
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    const imageData = ctx.createImageData(3840, 2160);
    imageData.data.set(result.get_rgba());
    ctx.putImageData(imageData, 0, 0);
  }
  
  main();
</script>
```

## 4️⃣ Ejecutar Tests (1 minuto)

```bash
# Todos los tests
cargo test --release

# Output: 18+ tests passed ✓
```

## 5️⃣ Servir en Navegador (Opcional)

```bash
# JavaScript + WASM
cd pkg
npm install
npm start

# O simple HTTP
python -m http.server 8000

# Abrir: http://localhost:8000
```

---

## 📚 Documentación Disponible

| Documento | Contenido | Tiempo |
|-----------|----------|---------|
| [README.md](README.md) | Overview general | 5 min |
| [TECNICO.md](TECNICO.md) | Fórmulas y detalles | 30 min |
| [DESARROLLO.md](DESARROLLO.md) | Guía dev completa | 20 min |
| [INTEGRACION_WASM.md](INTEGRACION_WASM.md) | HTML5 Canvas + WASM | 15 min |
| [DIAGRAMAS.md](DIAGRAMAS.md) | Flujos visuales | 10 min |
| [examples/usage.rs](examples/usage.rs) | 8 ejemplos code | 10 min |

---

## 🎯 Ejemplos Rápidos

### 1. Solo Escalado 4K (sin glow)
```rust
let result = processor.process_with_post_effects(
    &data, true,  // ← bilinear
    false,        // ← sin normals
    false,        // ← sin sobel
    0.0,          // ← sin glow
);
// Resultado: 3840×2160 pixel-perfect suave
```

### 2. Solo Normal Maps (iluminación 3D)
```rust
let result = processor.process_with_post_effects(
    &data, false, // ← sin bilinear
    true,         // ← normal maps ✓
    false, 0.0,
);
// Resultado: Vectores 3D para shaders
```

### 3. Solo Efecto Neón
```rust
let result = processor.process_with_post_effects(
    &data, true,  // ← 4K quality
    false,        // ← sin normals
    true,         // ← detectar bordes
    2.5,          // ← glow intenso
);
// Resultado: Glow cyberpunk intenso
```

### 4. Loop 30 FPS
```rust
for frame in 0..300 {
    let sprite_frame = load_frame(frame);
    
    let result = processor.process_with_post_effects(
        &sprite_frame,
        true, false,  // Sin normals (overhead)
        true, 1.0,
    );
    
    render(result.get_rgba());
    sleep(33ms);  // ~30 FPS
}
```

---

## ⚙️ Configuración Rápida

### Perfil "Calidad"
- Todo activado
- Mayor CPU pero máxima belleza

```rust
process_with_post_effects(&data, true, true, true, 2.0)
```

### Perfil "Rendimiento"
- Sin normal maps (overhead)
- Buena performance + visual

```rust
process_with_post_effects(&data, true, false, true, 1.0)
```

### Perfil "Ligero"
- Solo escalado
- Máximo FPS

```rust
process_with_post_effects(&data, true, false, false, 0.0)
```

---

## 🚀 Despliegue

### Build para Producción

```bash
# Release optimizado
cargo build --release

# WASM optimizado
wasm-pack build --release --target web
```

**Archivos generados:**
- `target/release/` (ejecutable)
- `pkg/` (WASM + JS + npm package)

### Subir a npm

```bash
cd pkg
npm publish
```

Luego el usuario puede instalar:
```bash
npm install @papiweb/msx2-processor
```

---

## ❓ FAQ Rápido

**P: ¿Necesito Node.js?**
R: No, solo para WASM. Rust nativo funciona solo.

**P: ¿Cuánto tarda en procesar?**
R: ~100-200ms por frame (bilinear + all effects)

**P: ¿Qué resolución sale?**
R: 3840×2160 (4K) cuando bilinear está activado

**P: ¿Puedo hacer 60 FPS?**
R: Sí, desactivando normals en el loop

**P: ¿Es compatible con WebGL?**
R: Sí, normal maps sirven en shaders GLSL

---

## 📞 Soporte

- Código: [GitHub Issues](https://github.com/papilink/parseadorwebAsembler/issues)
- Email: info@papiweb.dev
- Docs Completa: Ver carpeta raíz

---

**¡Listo para empezar!** 🚀

Próximo paso: Lee [README.md](README.md) o corre los [ejemplos](examples/usage.rs)

---

*© 2026 PAPIWEB DESARROLLOS INFORMATICOS*
*Procesamiento Advanced MSX2 → Modern Graphics*
