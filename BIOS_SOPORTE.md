# 🎮 Soporte de BIOS MSX2+ - PAPIWEB Desarrollos Informáticos

**Fecha**: Febrero 9, 2026  
**Versión**: 1.0  
**Estado**: ✅ Implementado y Documentado

---

## 📋 Resumen Ejecutivo

Se ha implementado un **sistema completo de validación y carga de archivos BIOS MSX2+** en el procesador Rust/WASM. El sistema incluye:

✅ Validación de tamaños BIOS válidos  
✅ Detección automática de tipos de BIOS  
✅ Cálculo de checksums para integridad  
✅ Gestión de carga/descarga de BIOS  
✅ Métodos de consulta para el estado actual  

---

## 🏗️ Archivos BIOS MSX2 Requirenidos

| Archivo | Tamaño | Tipo | Descripción |
|---------|--------|------|-------------|
| `msxbios.rom` | 16 KB, 32 KB | ROM | BIOS principal MSX1/MSX2 |
| `msx2bios.rom` | 16 KB, 32 KB | ROM | Extensión BIOS MSX2 específica |
| `msx2ext.rom` | 16 KB | ROM | Extensión MSX2 adicional |
| `basic.rom` | 32 KB | ROM | Intérprete BASIC incorporado |
| `kanji.rom` | 16 KB, 32 KB | ROM | ROM Kanji para caracteres japoneses (opcional) |

### Tamaños Válidos

```
✅ 8 KB   (0x2000 bytes)
✅ 16 KB  (0x4000 bytes)
✅ 32 KB  (0x8000 bytes)
✅ 64 KB  (0x10000 bytes)
```

### Ubicación en Memoria

Todos los archivos BIOS se cargan en **Slot 0 (0x0000 - 0x3FFF)** del mapa de memoria MSX2:

```
Dirección     Tamaño    Descripción
──────────────────────────────────────
0x0000-0x3FFF  16 KB   BIOS/ROM (Slot 0)
```

---

## 🔧 Implementación en Rust

### Nueva Estructura: `BiosInfo`

```rust
#[wasm_bindgen]
pub struct BiosInfo {
    filename: String,      // Nombre del archivo cargado
    size: u32,             // Tamaño en bytes
    loaded: bool,          // Estado de carga
    bios_type: String,     // Tipo detectado (8KB, 16KB, etc.)
    checksum: String,      // Hash de integridad (formato hexadecimal)
}
```

**Métodos públicos**:
- `get_filename()` - Obtener nombre del archivo
- `get_size()` - Obtener tamaño en bytes
- `is_loaded()` - Verificar estado
- `get_bios_type()` - Obtener tipo detectado
- `get_checksum()` - Obtener checksum

### Nueva Estructura: `BiosValidator`

```rust
#[wasm_bindgen]
pub struct BiosValidator {
    valid_sizes: Vec<u32>,    // Tamaños aceptados
    valid_types: Vec<String>, // Tipos de BIOS válidos
}
```

**Métodos de validación**:

```rust
// Validar tamaño
validator.is_valid_size(0x4000)  // → true (16 KB válido)

// Validar tipo
validator.is_valid_type("msxbios")  // → true

// Detectar tipo por tamaño
validator.detect_bios_type(0x4000)  // → "Estándar (16KB)"

// Calcular checksum
let checksum = validator.calculate_checksum(&bios_data);
// → "A1B2C3D4" (suma hexadecimal de bytes)
```

---

## 📖 API de Carga de BIOS

### Método: `load_bios`

Carga un archivo BIOS en la memoria del procesador.

```rust
pub fn load_bios(
    &mut self,
    bios_data: &[u8],
    filename: &str,
    bios_type: &str
) -> String
```

**Parámetros**:
- `bios_data` - Buffer de datos del BIOS
- `filename` - Nombre del archivo (ej: "msxbios.rom")
- `bios_type` - Tipo esperado (se detecta automáticamente)

**Retorna**: Mensaje de confirmación o error

**Validaciones**:
- ✅ Verifica tamaño válido
- ✅ Calcula checksum automáticamente
- ✅ Detecta tipo de BIOS
- ✅ Almacena en memoria

**Ejemplo de uso**:
```rust
let processor = MSX2Processor::new(256, 212);
let bios_data = /* leer archivo msxbios.rom */
let result = processor.load_bios(&bios_data, "msxbios.rom", "msxbios");
// → "✅ BIOS cargado: msxbios.rom (16384 bytes) - Checksum: A1B2C3D4"
```

---

### Método: `get_current_bios_info`

Obtiene información del BIOS cargado actualmente en formato JSON.

```rust
pub fn get_current_bios_info(&self) -> String
```

**Retorna**: JSON con información completa

**Ejemplo**:
```json
{
  "filename": "msxbios.rom",
  "size": 16384,
  "type": "Estándar (16KB)",
  "checksum": "A1B2C3D4",
  "loaded": true
}
```

**Si no hay BIOS**:
```json
{
  "loaded": false,
  "message": "No hay BIOS cargado"
}
```

---

### Método: `has_bios_loaded`

Verifica si hay un BIOS cargado.

```rust
pub fn has_bios_loaded(&self) -> bool
```

**Retorna**: `true` si hay BIOS, `false` si no

---

### Método: `get_bios_data`

Obtiene los datos en bruto del BIOS cargado.

```rust
pub fn get_bios_data(&self) -> Vec<u8>
```

**Retorna**: Vector de bytes con los datos del BIOS

**Uso**: Para transmisión, almacenamiento o procesamiento

---

### Método: `unload_bios`

Descarga el BIOS actual de la memoria.

```rust
pub fn unload_bios(&mut self) -> String
```

**Retorna**: Mensaje de confirmación

**Ejemplo**:
```rust
let result = processor.unload_bios();
// → "✅ BIOS 'msxbios.rom' descargado correctamente"
```

---

### Método: `get_bios_description`

Obtiene una descripción completa y formateada del BIOS.

```rust
pub fn get_bios_description(&self) -> String
```

**Retorna**: Descripción multilinea con toda la información

**Ejemplo**:
```
📋 INFORMACIÓN BIOS
━━━━━━━━━━━━━━━━━━━━━━
Archivo: msxbios.rom
Tamaño: 16384 bytes
Tipo: Estándar (16KB)
Checksum: A1B2C3D4
Estado: ✅ CARGADO
Ubicación: Slot 0 (0x0000-0x3FFF)
```

---

### Método: `validate_bios_checksum`

Verifica que el checksum del BIOS cargado coincida con un valor esperado.

```rust
pub fn validate_bios_checksum(&self, expected_checksum: &str) -> bool
```

**Parámetros**:
- `expected_checksum` - Checksum esperado (formato hexadecimal)

**Retorna**: `true` si coincide, `false` si no

**Uso**: Validar integridad contra una base de datos de referencias

---

## 📊 Cambios en Estructura MSX2Processor

Se agregaron dos campos nuevos a la estructura principal:

```rust
pub struct MSX2Processor {
    palette: [[u8; 4]; 16],
    width: usize,
    height: usize,
    memory_map: HashMap<String, MemoryMapSlot>,
    bios_data: Vec<u8>,              // ← NUEVO
    current_bios: Option<BiosInfo>,  // ← NUEVO
}
```

---

## 🔄 Flujo de Trabajo Completo

```
┌─────────────────────────────────────────────────────┐
│ 1. Seleccionar archivo BIOS en interfaz HTML        │
└────────────────┬──────────────────────────────────────┘
                 │
┌────────────────▼──────────────────────────────────────┐
│ 2. JavaScript lee archivo y obtiene bytes             │
└────────────────┬──────────────────────────────────────┘
                 │
┌────────────────▼──────────────────────────────────────┐
│ 3. Llamar processor.load_bios(data, name, type)       │
└────────────────┬──────────────────────────────────────┘
                 │
┌────────────────▼──────────────────────────────────────┐
│ 4. Validar tamaño (8KB, 16KB, 32KB, 64KB)             │
└────────────────┬──────────────────────────────────────┘
                 │
         ┌───────┴───────┐
         │               │
    Válido            Inválido
         │               │
         │         ❌ Mostrar error
         │
┌────────▼──────────────────────────────────────────────┐
│ 5. Calcular checksum y detectar tipo                  │
└────────────────┬──────────────────────────────────────┘
                 │
┌────────────────▼──────────────────────────────────────┐
│ 6. Guardar datos en bios_data y info en current_bios  │
└────────────────┬──────────────────────────────────────┘
                 │
┌────────────────▼──────────────────────────────────────┐
│ 7. Retornar: "✅ BIOS cargado: ..."                   │
└────────────────┬──────────────────────────────────────┘
                 │
┌────────────────▼──────────────────────────────────────┐
│ 8. Mostrar información en interfaz HTML               │
└─────────────────────────────────────────────────────────┘
```

---

## 🎯 Casos de Uso

### Caso 1: Cargar BIOS Principal

```javascript
// JavaScript
const fileInput = document.getElementById('biosInput');
const file = fileInput.files[0];
const arrayBuffer = await file.arrayBuffer();
const biosData = new Uint8Array(arrayBuffer);

// Convertir a formato compatible
const rustArray = Array.from(biosData);

// Llamar función Rust
const result = processor.load_bios(
    rustArray,
    file.name,
    'msxbios'
);

console.log(result);
// → "✅ BIOS cargado: msxbios.rom (16384 bytes) - Checksum: A1B2C3D4"
```

### Caso 2: Verificar BIOS Cargado

```javascript
if (processor.has_bios_loaded()) {
    const info = processor.get_current_bios_info();
    console.log(JSON.parse(info));
    // {
    //   "filename": "msxbios.rom",
    //   "size": 16384,
    //   "type": "Estándar (16KB)",
    //   "checksum": "A1B2C3D4",
    //   "loaded": true
    // }
} else {
    console.log("No BIOS cargado");
}
```

### Caso 3: Validar Integridad

```javascript
// Referencia conocida de BIOS auténtico
const knownChecksum = "A1B2C3D4";

if (processor.validate_bios_checksum(knownChecksum)) {
    console.log("✅ BIOS verificado como auténtico");
} else {
    console.log("⚠️ Checksum no coincide - posible corrupción");
}
```

### Caso 4: Descargar BIOS

```javascript
const result = processor.unload_bios();
console.log(result);
// → "✅ BIOS 'msxbios.rom' descargado correctamente"

// Verificar
if (!processor.has_bios_loaded()) {
    console.log("Memoria BIOS liberada");
}
```

---

## ⚙️ Características de Seguridad

### 1. Validación de Tamaño

No se aceptan archivos BIOS de tamaños arbitrarios:
```rust
if !validator.is_valid_size(size) {
    return Err("Tamaño BIOS inválido");
}
```

### 2. Checksum Automático

Se calcula automáticamente para cada archivo cargado:
```rust
let checksum = validator.calculate_checksum(bios_data);
```

### 3. Límite de Memoria

El BIOS se reserva en Slot 0 (máximo 16KB):
```
Slot 0: 0x0000 - 0x3FFF (16 KB)
```

### 4. Validación de Integridad

Método para validar contra referencias conocidas:
```rust
pub fn validate_bios_checksum(&self, expected_checksum: &str) -> bool
```

---

## 📚 Integración con Sistema Existente

### Mapa de Memoria Actualizado

```
Dirección     Tamaño    Descripción         Tipo
─────────────────────────────────────────────────
0x0000-0x3FFF  16 KB   BIOS/ROM (Slot 0)   🔴 ROM
0x4000-0x7FFF  16 KB   Cartridge (Slot 1)  🟡 Cartridge
0x8000-0xBFFF  16 KB   RAM (Slot 2)        🔵 RAM
0xC000-0xFFFF  16 KB   RAM Principal       🔵 RAM
                       (Slot 3)
```

El BIOS ocupa exactamente Slot 0 del mapa de memoria existente.

---

## 🚀 Compilación y Pruebas

### Compilar Aplicación

```bash
# Compilación normal (Rust check)
cargo check

# Compilación completa (wasm-pack)
wasm-pack build --target web --release

# Resultado
pkg/msx2_processor.wasm      # Binary WASM
pkg/msx2_processor.js        # JavaScript bindings
pkg/msx2_processor.d.ts      # TypeScript types
```

### Verificar Funcionalidad

Los métodos BIOS se pueden probar usando las herramientas de desarrollo del navegador:

```javascript
// En consola del navegador
const processor = new MSX2Processor(256, 212);

// Cargar BIOS ficticio (ejemplo)
const biosData = new Uint8Array(16384);
processor.load_bios(biosData, "test.rom", "msxbios");

// Verificar carga
processor.has_bios_loaded();  // → true
processor.get_current_bios_info();  // → JSON con info
```

---

## ℹ️ Estado Actual del Proyecto

| Componente | Estado | Notas |
|-----------|--------|-------|
| Estructuras BIOS | ✅ Implementado | `BiosInfo`, `BiosValidator` |
| Carga de BIOS | ✅ Implementado | `load_bios()` con validación |
| Información BIOS | ✅ Implementado | Métodos de consulta |
| Descarga de BIOS | ✅ Implementado | `unload_bios()` |
| Checksum | ✅ Implementado | Cálculo automático |
| Validación | ✅ Implementado | Por tamaño e integridad |
| Integración HTML | ⏳ Pendiente | Interfaz visual en UI |
| Integración JavaScript | ⏳ Pendiente | Bindings y manejo de archivos |

---

## 📝 Próximos Pasos

1. **Integración en HTML** (`index.html`)
   - Panel de carga de archivos BIOS
   - Mostrar información de BIOS cargado
   - Validación en interfaz

2. **Integración en JavaScript**
   - Handler para input de archivos
   - Mostrar progreso de carga
   - Manejo de errores

3. **Base de Datos de Checksums**
   - Tabla de BIOS auténticos con checksums
   - Verificación automática de integridad
   - Advertencias de posible piratería

4. **Emulación Completa** (futuro)
   - Ejecutar código BIOS en Z80 simulado
   - Inicializar VDP con BIOS
   - Cargar aplicaciones BASIC

---

## 📞 Soporte

Para dudas sobre la implementación, consultar:
- Documentación técnica: `TECNICO.md`
- Mapa de memoria: `MAPA_MEMORIA_IMPLEMENTACION.md`
- VDP y CPU: `VDP_INICIALIZACION.md`

---

**© 2026 PAPIWEB DESARROLLOS INFORMATICOS**  
*Especialistas en emulación de sistemas retro y procesamiento avanzado de gráficos*
