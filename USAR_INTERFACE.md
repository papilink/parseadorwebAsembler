# 🎮 MSX2 ROM Viewer - Guía de Uso

**© 2026 PAPIWEB DESARROLLOS INFORMATICOS**

## ⚡ Quick Start

### 1. Iniciar el Servidor

```bash
# Opción 1: Python (recomendado)
python3 server.py

# Opción 2: Node.js
npx http-server -p 8080 --cors

# Opción 3: Python (alternativa)
python -m http.server 8080

# Opción 4: Ruby
ruby -run -ehttpd . -p8080
```

### 2. Abrir en el Navegador

```
http://localhost:8080
```

## ✨ Características

### 📂 Carga de Archivos
- **Selector de archivos** - Click en el botón para cargar
- **Drag & Drop** - Arrastra directamente en el área
- **Formatos soportados**: .rom, .bin, .dat

### ⚡ Procesamiento WASM

#### Opción 1: RGBA Rápido
```
✉️ PROCESAR RGBA
├─ Conversión 4bpp → 32bpp
├─ Tiempo: < 1ms
└─ Resultado: 256×212 RGBA
```

#### Opción 2: Procesamiento Completo
```
🌟 PROCESAMIENTO COMPLETO
├─ Interpolación Bilineal (4K)
├─ Normal Maps (iluminación 3D)
├─ Detección Sobel (bordes)
└─ Glow Neón (intensidad configurable)
```

### ⚙️ Opciones Configurables

```javascript
// En la interfaz gráfica
☑ Interpolación Bilineal (4K)
☐ Normal Maps
☐ Detección Sobel
  Intensidad Glow: [========] 1.5
```

## 🖼️ Canvas Rendering

El resultado se renderiza directamente:

```javascript
// Flujo interno
ROM File (ArrayBuffer)
    ↓
MSX2Processor::transform_to_rgba()
    ↓
Uint8Array (RGBA)
    ↓
Canvas ImageData
    ↓
Renderizado visual
```

## 📊 Información en Tiempo Real

Después de procesar:
```
Información del Archivo:
├─ Nombre: archivo.rom
├─ Tamaño: 27.5 KB
└─ Tipo: application/octet-stream

Análisis:
├─ Bytes: 27,648
├─ Píxeles: 55,296
├─ Resolución: 256×212
└─ Tiempo: 0.45ms
```

## 🔧 Opciones Avanzadas

### Interpolación Bilineal
```
✓ Activada  → 4K (3840×2160)
✗ Desactivada → Original (256×212)
```

### Normal Maps
Genera vectores normales para:
- Iluminación dinámica
- Sombras en tiempo real
- Efectos 3D

### Detección Sobel
Identifica bordes para:
- Glow neón
- Efecto cyberpunk
- Realce de detalles

### Glow Intensity
```
Rango: 0.0 - 3.0
├─ 0.0 = Sin glow
├─ 1.0 = Moderado
├─ 1.5 = Recomendado
└─ 3.0 = Intenso
```

## 💾 Archivos Requeridos

```
parseadorwebAsembler/
├── index.html              (Interfaz web)
├── server.py               (Servidor HTTP)
├── pkg/
│   ├── msx2_processor.js   (Bindings WASM)
│   ├── msx2_processor.wasm (Binario compilado)
│   ├── package.json
│   └── README.md
└── target/
    └── release/
        └── libmsx2_processor.*
```

## 🎯 Workflow Típico

1. **Inicia servidor**
   ```bash
   python3 server.py
   ```

2. **Abre navegador**
   ```
   http://localhost:8080
   ```

3. **Carga ROM**
   - Click en "📂 Selecciona tu archivo"
   - O arrastra el archivo al área

4. **Configura opciones** (opcional)
   - Bilinear Interpolation
   - Normal Maps
   - Detección Sobel
   - Intensidad Glow

5. **Procesa**
   - Botón "⚡ PROCESAR RGBA" → Rápido
   - Botón "🌟 PROCESAMIENTO COMPLETO" → Completo

6. **Visualiza**
   - Canvas se actualiza automáticamente
   - Ver información en el panel lateral

7. **Exporta** (opcional)
   - Click derecho en canvas → Guardar imagen
   - O usa DevTools → Network → Descargar

## 💡 Ejemplos de Uso

### Caso 1: Verificar ROM Rápido
```bash
1. Cargar archivo
2. Click "PROCESAR RGBA"
3. Ver resultado en canvas
4. Tiempo: ~1ms
```

### Caso 2: Procesamiento Artístico
```bash
1. Cargar archivo
2. Activar todas las opciones:
   ☑ Interpolación Bilineal
   ☑ Normal Maps
   ☑ Detección Sobel
   Intensidad: 2.0
3. Click "PROCESAMIENTO COMPLETO"
4. Resultado: Glow neón artístico 4K
```

### Caso 3: Batch Processing
```javascript
// Desde console browser
// Cargar múltiples archivos programáticamente

const processor = new MSX2Processor(256, 212);
for (const file of files) {
    const rgba = processor.transform_to_rgba(fileData);
    exportImage(rgba, file.name);
}
```

## 🐛 Troubleshooting

### "WASM no inicializado"
**Solución:**
- Espera a que cargue (3-5 seg)
- Abre Console (F12) → Ver logs
- Recarga página

### "Error al leer archivo"
**Solución:**
- Verifica que sea archivo binario válido
- Intenta otro .rom/.bin
- Comprueba formato: hexdump -C archivo.rom

### Canvas en blanco después de procesar
**Solución:**
- Archivo puede estar corrupto
- Intenta con Interpolación desactivada
- Revisa console para errores JavaScript

### Servidor no inicia
**Solución:**
- Puerto 8080 en uso: `python3 server.py 8081`
- Permiso denegado: `sudo python3 server.py`
- Firewall: Permite conexión local

## 📱 Compatibilidad

### Navegadores
- ✅ Chrome/Edge 90+
- ✅ Firefox 87+
- ✅ Safari 14+
- ⚠️ IE/Edge Legacy: No soportado

### Sistemas Operativos
- ✅ Linux
- ✅ macOS
- ✅ Windows
- ✅ Cualquiera con Python 3

## 🔐 Privacidad

- ✅ Todo procesa **localmente**
- ✅ Los archivos NO se suben a servidor
- ✅ Los archivos NO se guardan
- ✅ Sin conexión a internet necesaria
- ✅ WASM corre en tu navegador

## 📚 Documentación Completa

```bash
# Ver docs técnica
cat ../TECNICO.md

# Ver guía WASM
cat ../INTEGRACION_WASM.md

# Ver API de Rust
cat ../src/lib.rs
```

## 🎓 Código JavaScript Ejemplo

```javascript
// Desde console del navegador

// 1. Cargar archivo manualmente
const file = new File(
    [new Uint8Array([0x12, 0x34, ...])],
    'test.rom'
);

// 2. Procesar
const processor = new MSX2Processor(256, 212);
const rgba = processor.transform_to_rgba(new Uint8Array(fileData));

// 3. Renderizar
const canvas = document.querySelector('canvas');
const ctx = canvas.getContext('2d');
const imageData = ctx.createImageData(256, 212);
imageData.data.set(rgba);
ctx.putImageData(imageData, 0, 0);
```

## 🚀 Próximos Pasos

1. **Mejoras sugeridas:**
   - Modo batch para múltiples archivos
   - Exportar a PNG/WebP
   - Histórico de procesados
   - Comparador antes/después

2. **Integración:**
   - Embeder en tu web
   - API HTTP para otros apps
   - Docker container

3. **Análisis:**
   - Estadísticas de píxeles
   - Histograma de colores
   - Detección automática resolución

## 📞 Contacto

**PAPIWEB DESARROLLOS INFORMATICOS**

- 🌐 Web: papiweb.dev
- 📧 Email: info@papiweb.dev
- 🐙 GitHub: github.com/papilink/parseadorwebAsembler

---

**© 2026 PAPIWEB DESARROLLOS INFORMATICOS**

*Transformando gráficos retro en arte digital moderno* 🎨

