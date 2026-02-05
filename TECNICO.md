# 🎮 MSX2 Processor - Post-Procesamiento Avanzado

**© 2026 PAPIWEB DESARROLLOS INFORMATICOS**

Procesador avanzado de sprites MSX2 con post-procesamiento en tiempo real. Transforma gráficos retro en versiones modernas con interpolación bilineal, generación de mapas normales y detección de bordes con efectos neón.

## 🌟 Características principales

### 1. **Interpolación Bilineal (256x212 → 4K)**
Escala sprites MSX2 originales (256x212 píxeles) a resolución 4K verdadera (3840x2160 píxeles) con interpolación suave que preserva los detalles sin pixelado artificial.

```rust
let processor = MSX2Processor::new(256, 212);
let upscaled_4k = processor.bilinear_interpolation(&rgba_data);
// 256x212 → 3840x2160 (interpolación suave)
```

### 2. **Generación de Normal Maps**
Calcula vectores normales basados en luminancia de píxeles. Los vectores de altura permiten:
- Iluminación dinámica realista
- Linterna del ratón sobre sprites con sombras propias
- Bump mapping y parallax mapping

```rust
let normals = processor.generate_normal_map(&rgba_data);
// Vectores normales (X, Y, Z) para cada píxel
```

### 3. **Detección de Bordes (Filtro Sobel)**
Identifica bordes del sprite original con el filtro Sobel. Permite aplicar efectos visuales sofisticados:
- Glow neón en siluetas
- Efecto cyberpunk/retro gaming
- Resaltado de detalles

```rust
let edges = processor.detect_edges_sobel(&rgba_data);
let neon = processor.apply_neon_glow(&rgba, &edges, intensity);
```

## 🏗️ Arquitectura

### Flujo de Procesamiento

```
MSX2 4bpp (binario)
        ↓
   [Transform RGBA]
        ↓
   ┌────┴────┬──────────┬─────────┐
   ↓         ↓          ↓         ↓
[Bilinear][Normals] [Sobel]  [Composición]
   ↓         ↓          ↓         ↓
   └────┬────┴──────────┴─────────┘
        ↓
   [Glow Neón]
        ↓
   PostProcessResult
   (RGBA + Normals + Edges)
```

### Estructuras de Datos

```rust
pub struct MSX2Processor {
    palette: [[u8; 4]; 16],  // Paleta 16 colores RGBA
    width: usize,             // 256 píxeles
    height: usize,            // 212 píxeles
}

pub struct PostProcessResult {
    pub rgba: Vec<u8>,        // Imagen final RGBA
    pub normals: Vec<u8>,     // Vectores normales (RGB)
    pub edges: Vec<f32>,      // Magnitud Sobel
}
```

## 🚀 Métodos Principales

### `transform_to_rgba(bin_data: &[u8]) -> Vec<u8>`
Convierte formato MSX2 4bpp a RGBA 32bpp.
- **Entrada**: Datos binarios MSX2 (dos píxeles de 4 bits por byte)
- **Salida**: Datos RGBA (4 bytes por píxel)

### `bilinear_interpolation(rgba_data: &[u8]) -> Vec<u8>`
Escala imagen con interpolación bilineal.
- **Entrada**: Imagen RGBA 256x212
- **Salida**: Imagen RGBA 3840x2160 (4K)
- **Fórmula**: $P(x,y) = P_{00}(1-f_x)(1-f_y) + P_{10}f_x(1-f_y) + P_{01}(1-f_x)f_y + P_{11}f_xf_y$

### `generate_normal_map(rgba_data: &[u8]) -> Vec<u8>`
Genera mapa de normales para iluminación 3D.
- **Entrada**: Imagen RGBA original
- **Salida**: Vectores normales RGB (128 = 0, 255 = +1, 0 = -1)
- **Fórmula**: Normal = normalize((-dx, -dy, 1))
  - $dx = (L_{right} - L_{left}) / 2$
  - $dy = (L_{down} - L_{up}) / 2$

### `detect_edges_sobel(rgba_data: &[u8]) -> Vec<f32>`
Detección de bordes con filtro Sobel.
- **Entrada**: Imagen RGBA
- **Salida**: Magnitud de gradientes para cada píxel
- **Fórmula**: $|G| = \sqrt{G_x^2 + G_y^2}$

Kernels Sobel:
```
Gx:             Gy:
[-1  0  +1]     [-1  -2  -1]
[-2  0  +2]     [ 0   0   0]
[-1  0  +1]     [+1  +2  +1]
```

### `apply_neon_glow(rgba: &[u8], edges: &[f32], intensity: f32) -> Vec<u8>`
Aplica efecto neón a bordes detectados.
- **Parámetros**:
  - `intensity`: Factor de brillo (0.0 - 3.0)
  - `glow_radius`: 3 píxeles
  - `glow_threshold`: 50.0
- **Efecto**: Difusión luminosa en bordes detectados

### `process_with_post_effects(...) -> PostProcessResult`
Procesamiento completo orquestado.
- Convierte MSX2 → RGBA
- Aplica todos los efectos habilitados
- Retorna resultado con todos los mapas

## 📊 Perfiles de Rendimiento

| Perfil | Bilineal | Normals | Sobel | Glow | CPU | Uso Mem |
|--------|----------|---------|-------|------|-----|---------|
| **CALIDAD** | 4K ✓ | Sí ✓ | Sí ✓ | 2.0 ✓ | Alto | Alto |
| **MEDIO** | 2K ✓ | No | Sí ✓ | 1.0 ✓ | Medio | Medio |
| **LIGERO** | 1080p ✓ | No | No | No | Bajo | Bajo |

## 💡 Casos de Uso

### Juego Retro con Literna Dinámica
```rust
let processor = MSX2Processor::new(256, 212);
let result = processor.process_with_post_effects(
    &sprite_data,
    true,   // Escala 4K
    true,   // Normal maps (para sombras)
    true,   // Detección bordes
    1.5,    // Glow moderado
);

// Normal maps permiten calcular iluminación en shader:
// shadow = dot(normal, lightDirection)
```

### Efecto Cyberpunk Retro
```rust
let result = processor.process_with_post_effects(
    &sprite_data,
    true,   // Mantener claridad
    false,  // Sin normals
    true,   // Bordes vibrantes
    2.5,    // Glow neón intenso
);
```

### Preservar Pixelart Original
```rust
let result = processor.process_with_post_effects(
    &sprite_data,
    false,  // Sin upscale
    false,  // Sin normals
    false,  // Sin bordes
    0.0,    // Sin glow
);
```

## ⚙️ Especificaciones Técnicas

### Formato MSX2
- **Modo**: Screen 5
- **Resolución**: 256×192 (puede variar)
- **Profundidad**: 4 bits por píxel (16 colores)
- **Paleta**: 16 colores RGBA

### Formato de Salida
- **RGBA**: 32 bits (8 bits por canal)
- **Normal Map**: RGB (8 bits por canal, centrado en 128)
- **Edge Map**: 32 bits float (0.0 - 255.0)

### Complejidad Computacional
- **Bilinear**: O(src × dst) = O(256×212 × 3840×2160)
- **Normal Map**: O(width × height × 9) - kernel 3×3
- **Sobel**: O(width × height × 9) - kernel 3×3
- **Glow**: O(width × height × radius²)

## 🔧 Configuración

En `Cargo.toml`:
```toml
[dependencies]
wasm-bindgen = "0.2"

[profile.release]
opt-level = "z"      # Optimizar tamaño
lto = true           # Link-time optimization
codegen-units = 1    # Máxima optimización
```

## 📦 Instalación

```bash
# Compilar para WASM
wasm-pack build --release --target web

# Compilar librería nativa
cargo build --release

# Ejecutar ejemplos
cargo run --example usage
```

## 🎨 Integración con Shaders

### Shader de Iluminación (GLSL)
```glsl
// Usar normal map para calcular iluminación dinámica
vec3 normal = normalize(texture(u_NormalMap, uv).rgb * 2.0 - 1.0);
vec3 lightDir = normalize(u_LightPos - fragPos);
float diffuse = max(dot(normal, lightDir), 0.0);
gl_FragColor = vec4(color * diffuse, 1.0);
```

## 📈 Transformaciones Matemáticas

### Luminancia (Gamma BT.709)
$$L = 0.299 \times R + 0.587 \times G + 0.114 \times B$$

### Interpolación Bilineal
$$f(x,y) = \sum_{i=0}^{1} \sum_{j=0}^{1} f(x_i, y_j) \cdot B_i(x) \cdot B_j(y)$$

### Vector Normal (Bump Mapping)
$$\vec{N} = \text{normalize}((-\frac{\partial h}{\partial x}, -\frac{\partial h}{\partial y}, 1))$$

### Magnitud Sobel
$$|G| = \sqrt{G_x^2 + G_y^2} \quad \text{donde} \quad G_x = \sum S_x \cdot I, \quad G_y = \sum S_y \cdot I$$

## 🎯 Optimizaciones

- ✅ Vectorización SIMD en loops principales
- ✅ Caché-friendly memory layout
- ✅ Early termination en cálculos innecesarios
- ✅ Precalc de factors en interpolación
- ✅ Compilación LTO para WASM

## 📝 Ejemplos

Ver carpeta `examples/usage.rs` para:
1. Procesamiento completo
2. Upscale bilineal puro
3. Generación de normal maps
4. Efectos neón
5. Loop en tiempo real
6. Perfiles de rendimiento
7. Composición multi-capa
8. Caso práctico: juego mejorado

## 📞 Soporte

**PAPIWEB DESARROLLOS INFORMATICOS**
- Procesamiento de sprites MSX2
- Efectos visuales avanzados para retro gaming
- Integración con engines 3D modernos

---

*Creado en 2026 | Optimizado para WASM/WebGL*
