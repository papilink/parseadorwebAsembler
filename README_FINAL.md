# 🎮 MSX2 ROM Viewer - WebAssembly Processor

**© 2026 PAPIWEB DESARROLLOS INFORMATICOS**

> Transformando gráficos retro en arte digital moderno mediante procesamiento avanzado con WebAssembly

## 📌 Quick Start (1 minuto)

```bash
# 1. Inicia el servidor
python3 server.py

# 2. Abre en navegador
# http://localhost:8080

# 3. Prueba con la demo
# http://localhost:8080/demo.html
```

## 🎯 Características Principales

### ⚡ Procesamiento WASM
```
ROM (4bpp) → Conversión RGBA → Interpolación Bilineal → Normal Maps
                                      ↓
                            Sobel Edge Detection
                                      ↓
                            Neon Glow Effect
                                      ↓
                            Canvas Rendering
```

### 🖼️ Capacidades

| Función | Descripción | Parámetros |
|---------|-------------|-----------|
| **RGBA Conversion** | Convierte binario 4bpp a RGBA 32bpp | Input: ROMData |
| **Bilinear Interpolation** | Upscaling a 4K (3840×2160) | Width, Height |
| **Normal Map Generation** | Calcula vectores normales para iluminación | Source: Grayscale |
| **Sobel Edge Detection** | Detecta bordes con kernel 3×3 | Source: RGBA |
| **Neon Glow** | Aplica efecto glow a bordes detectados | Intensity: 0.0-3.0 |

### 🔧 Opciones Configurables

```javascript
// En la interfaz gráfica
☑ Interpolación Bilineal      // 256×212 → 3840×2160
☑ Normal Maps                 // Para iluminación 3D
☑ Detección Sobel             // Extrae bordes
  Intensidad Glow: [████] 1.5 // Rango: 0.0 - 3.0
```

## 📂 Estructura del Proyecto

```
parseadorwebAsembler/
├── 📄 index.html          ← Interfaz principal (para ROM reales)
├── 🎨 demo.html           ← Demo interactiva (sin ROM necesario)
├── 🐍 server.py           ← Servidor HTTP con CORS + WASM
│
├── 📦 pkg/                ← Binarios WASM compilados
│   ├── msx2_processor.js      (bindings JavaScript)
│   ├── msx2_processor.wasm    (34 KB compilado)
│   └── package.json
│
├── 🦀 src/lib.rs          ← Código Rust (360 líneas)
├── 📋 Cargo.toml          ← Manifest Rust
└── 🧪 tests/              ← 15+ tests integración
```

## 🚀 Instructiones de Uso

### Opción 1: Demo Interactiva (Sin ROM)

Perfecto para probar la interfaz sin archivos:

```bash
python3 server.py
# Abre: http://localhost:8080/demo.html
```

**Características:**
- ✓ Genera datos de prueba automáticamente
- ✓ Controles interactivos
- ✓ Demostración en tiempo real
- ✓ Sin necesidad de archivo ROM

### Opción 2: Procesar ROM Real

Con archivo ROM .rom/.bin genuine:

```bash
python3 server.py
# Abre: http://localhost:8080/
# Carga tu archivo ROM
```

**Flujo:**
1. Servidor HTTP inicia en puerto 8080
2. Abre `http://localhost:8080` en navegador
3. Drag & drop o selecciona archivo .rom
4. Click "PROCESAR RGBA" o "PROCESAMIENTO COMPLETO"
5. Resultado renderizado en canvas

## 💻 Requisitos

### Sistema Operativo
- ✅ Linux (recomendado)
- ✅ macOS
- ✅ Windows (WSL2 recomendado)

### Software Requerido
- **Python 3.6+** (para servidor)
- **Navegador moderno** con WebAssembly support:
  - Chrome 90+
  - Firefox 87+
  - Safari 14+
  - Edge 90+

### Sin necesidad de:
- ❌ Node.js (opcional)
- ❌ Pip packages (servidor puro Python)
- ❌ Interner (procesa localmente)

## ⚙️ Instalación Completa

### 1. Verificar Python

```bash
python3 --version
# Output: Python 3.10.x o superior
```

### 2. Clonar/Descargar proyecto

```bash
cd /ruta/del/proyecto
ls -la  # Verificar que existe index.html, server.py, etc
```

### 3. Iniciar servidor

```bash
python3 server.py

# Output:
# ╔════════════════════════════════════════════════════════════╗
# ║          🎮 MSX2 ROM VIEWER - Servidor Iniciado           ║
# ╚════════════════════════════════════════════════════════════╝
# 
# 📍 Dirección:        http://127.0.0.1:8080
# 📋 Verificando archivos...
#    ✓ index.html                              (0.45 KB)
#    ✓ demo.html                               (28.35 KB)
#    ✓ pkg/msx2_processor.wasm                 (34.20 KB)
```

### 4. Abrir en navegador

```
http://localhost:8080
```

## 🎯 Casos de Uso

### Caso 1: Demo Rápida (1 minuto)

```bash
$ python3 server.py
# Espera iniciación
$ python3 -m webbrowser http://localhost:8080/demo.html
# Se abre automáticamente en navegador
```

Resultados inmediatos sin configuración.

### Caso 2: Procesar ROM MSX2

```bash
1. Inicia servidor:    python3 server.py
2. Abre navegador:     http://localhost:8080
3. Carga ROM:          Drag & drop archivo.rom
4. Configura:          Bilinear, Normals, Edges, Glow
5. Procesa:            Click botón correspondiente
6. Visualiza:          Resultado en canvas
```

### Caso 3: Análisis Binario

```bash
# En navegador console (F12):
const processor = new MSX2Processor(256, 212);
const data = new Uint8Array([...binary_data...]);
const rgba = processor.transform_to_rgba(data);
console.log('Pixeles:', rgba.length / 4);
```

## 🔍 Solución de Problemas

### "Puerto 8080 ya está en uso"
```bash
# Opción 1: Usar otro puerto
python3 server.py 8081
# http://localhost:8081

# Opción 2: Encontrar qué usa el puerto
lsof -i :8080
kill -9 <PID>
```

### "WASM no inicializa en navegador"
```bash
# 1. Abre console (F12)
# 2. Verifica errores CORS
# 3. Comprueba que server.py está corriendo
# 4. Recarga página (Ctrl+R o Cmd+R)
```

### "Canvas en blanco después de procesar"
```bash
# Causas probables:
# 1. ROM corrupto → Intenta otro archivo
# 2. WASM error → Revisa console del navegador
# 3. Parámetros incorrectos → Resetea opciones

# Solución:
# • Click "🔄 Limpiar"
# • Desactiva todas las opciones
# • Click "⚡ PROCESAR RGBA"
```

### "Error de CORS en console"
```
Si ves: Cross-Origin-Embedder-Policy

Soluciones:
1. Asegúrate que server.py está corriendo
2. Usa http:// no file://
3. Recarga la página completa
```

## 📊 Especificaciones Técnicas

### Arquitectura WASM

```
┌─────────────────────────┐
│   Browser JavaScript    │
│  (FileReader + Canvas)  │
└────────────┬────────────┘
             │
             ↓ (ArrayBuffer)
┌──────────────────────────────┐
│   WASM Module (Rust compiled) │
│   • MSX2Processor struct      │
│   • transform_to_rgba()       │
│   • bilinear_interpolation()  │
│   • generate_normal_map()     │
│   • detect_edges_sobel()      │
│   • apply_neon_glow()         │
└────────────┬─────────────────┘
             │
             ↓ (Uint8Array RGBA)
┌──────────────────────────────┐
│   Canvas ImageData API       │
│   → putImageData()           │
│   → Visual Output           │
└──────────────────────────────┘
```

### Rendimiento

| Operación | Tiempo | Entrada | Salida |
|-----------|--------|---------|--------|
| RGBA Conversion | ~0.5ms | 27.6 KB | 219.5 KB |
| Bilinear (4K) | ~15ms | 27.6 KB | 31.7 MB |
| Normal Map | ~8ms | RGBA | RGBA |
| Sobel Edge | ~10ms | RGBA | RGBA |
| Glow Effect | ~12ms | RGBA | RGBA |
| **Total** | **~45ms** | **-** | **-** |

### Compatibilidad de Formatos

| Formato | Soportado | Notas |
|---------|-----------|-------|
| .rom | ✓ | MSX2 ROM standard |
| .bin | ✓ | Binary image generic |
| .dat | ✓ | Data file genérico |
| .img | ✓ | Disk image |
| .dsk | ✗ | Disco formateado (futuro) |

## 🛠️ Desarrollo

### Compilar WASM desde fuente

```bash
# Requisitos previos
rustup install stable
rustup target add wasm32-unknown-unknown
npm install -g wasm-pack

# Compilar
wasm-pack build --release --target web

# Resultado: pkg/ directory generado
```

### Ejecutar tests

```bash
cargo test --release

# Output:
# test test_rgba_conversion ... ok
# test test_bilinear_interpolation_dimensions ... ok
# ...
# result: ok. 15 passed; 0 failed
```

### Modificar interfaz

Edita `index.html` o `demo.html`:
- CSS personalizado (líneas 6-200)
- JavaScript handler (líneas 200+)
- Controles HTML (líneas 100+)

## 🔐 Seguridad & Privacidad

✅ **Completamente Seguro:**
- Todos los datos se procesan **localmente en tu navegador**
- NO se envía información a servidores
- NO se guardan datos en disco (en navegador)
- NO se requiere conexión a internet
- Código abierto y auditable

⚠️ **Notas de Producción:**
- WASM requiere HTTPS en producción (no HTTP)
- Configurar CORS apropiadamente en servidor
- Validar entrada de archivos para límites de tamaño
- Implementar timeout para procesamiento largo

## 📚 Documentación Adicional

| Archivo | Contenido |
|---------|----------|
| [USAR_INTERFACE.md](USAR_INTERFACE.md) | Guía de usuario detallada |
| [TECNICO.md](TECNICO.md) | Documentación técnica Rust |
| [INTEGRACION_WASM.md](INTEGRACION_WASM.md) | Integración JavaScript-WASM |
| [DESARROLLO.md](DESARROLLO.md) | Guía de desarrollo |
| [QUICKSTART.md](QUICKSTART.md) | Inicio rápido |

## 🎓 Ejemplos de Código

### JavaScript: Cargar ROM

```javascript
import init, { MSX2Processor } from './pkg/msx2_processor.js';

// Inicializar
await init();
const processor = new MSX2Processor(256, 212);

// Cargar archivo
const file = document.getElementById('fileInput').files[0];
const reader = new FileReader();

reader.onload = (e) => {
    const romData = new Uint8Array(e.target.result);
    const rgba = processor.transform_to_rgba(romData);
    
    // Renderizar
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    const imageData = ctx.createImageData(256, 212);
    imageData.data.set(rgba);
    ctx.putImageData(imageData, 0, 0);
};

reader.readAsArrayBuffer(file);
```

### Rust: Procesar datos

```rust
use msx2_processor::MSX2Processor;

fn main() {
    // Crear procesador
    let mut processor = MSX2Processor::new(256, 212);
    
    // Datos ROM simulados
    let rom_data = vec![0x12, 0x34, /*...*/];
    
    // Procesar
    let rgba = processor.transform_to_rgba(&rom_data);
    
    // Usar datos RGBA
    println!("Procesados {} bytes RGBA", rgba.len());
}
```

## 🤝 Contribuir

Reportar bugs y mejoras:

```bash
# 1. Documentar el problema
# 2. Proporcionar ROM de test
# 3. Incluir salida de console (F12)
# 4. Especificar navegador/SO
```

## 📞 Contacto

**PAPIWEB DESARROLLOS INFORMATICOS**

- 🌐 Web: https://papiweb.dev
- 📧 Email: info@papiweb.dev
- 🐙 GitHub: github.com/papilink/parseadorwebAsembler

## 📄 Licencia

© 2026 PAPIWEB DESARROLLOS INFORMATICOS. Todos los derechos reservados.

Uso permitido bajo licencia proprietaria para fines educativos y de desarrollo.

---

## 🌟 Próximas Características

- [ ] Exportar a PNG/WebP
- [ ] Batch processing de múltiples ROMs
- [ ] Histograma de colores
- [ ] Comparador antes/después
- [ ] API REST para integración
- [ ] Docker container
- [ ] Plugin para browsers alternativos

## 📈 Estadísticas del Proyecto

```
Líneas de código Rust:      360
Tests integración:           15
Documentación:            ~50 KB
Tamaño WASM compilado:    34 KB
Tiempo compilación:       ~0.5s
Soporta navegadores:        5+
```

---

**¡Gracias por usar MSX2 ROM Viewer! 🎮**

Transforma tus gráficos retro en arte digital moderno con procesamiento avanzado en WebAssembly.

*Hecho con ❤️ por PAPIWEB - 2026*
