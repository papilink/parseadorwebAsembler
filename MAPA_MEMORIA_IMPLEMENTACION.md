# 🗺️ Implementación de Mapeo de Memoria Virtual del MSX2

## Resumen Ejecutivo

Se ha implementado un sistema completo de **mapeo de memoria virtual** para el emulador MSX2 web, permitiendo que los usuarios carguen binarios en direcciones específicas respetando la arquitectura de memoria del computador real.

---

## 📋 Cambios Implementados

### 1. **Modificaciones en Rust (`src/lib.rs`)**

#### Nuevas Estructuras:

```rust
// Estructura para representar un slot de memoria
#[wasm_bindgen]
pub struct MemoryMapSlot {
    address: u32,
    size: u32,
    name: String,
    region_type: String,
}

// Estructura para información de carga
#[wasm_bindgen]
pub struct LoadInfo {
    load_address: u32,
    binary_size: u32,
    start_address: u32,
    end_address: u32,
    memory_slot: String,
}
```

#### Nuevos Métodos en MSX2Processor:

- `get_memory_map()` - Retorna JSON con el mapa de memoria completo
- `find_memory_slot(address: u32)` - Busca el slot de memoria para una dirección
- `create_load_info(load_address: u32, binary_size: u32)` - Crea información de carga

#### Mapa de Memoria del MSX2 Implementado:

```
Dirección    Tamaño    Descripción         Tipo
─────────────────────────────────────────────────
0x0000-0x3FFF  16 KB   BIOS/ROM (Slot 0)   ROM
0x4000-0x7FFF  16 KB   Cartridge (Slot 1)  Cartridge
0x8000-0xBFFF  16 KB   RAM (Slot 2)        RAM
0xC000-0xFFFF  16 KB   RAM Principal       RAM
                       (Slot 3)
```

---

### 2. **Modificaciones en HTML (`index.html`)**

#### Nuevo Panel: Mapa de Memoria

Se agregó un nuevo panel interactivo que muestra:

1. **Visualización textual del mapa de memoria** con todas las regiones disponibles
2. **Selector de dirección de carga** con botones preestablecidos:
   - `0xC000` (Estándar - RAM Principal)
   - `0x4000` (Slot 1 - Cartridge)
   - `0x8000` (Slot 2 - RAM)
3. **Campo personalizado** para ingresa dirección hexadecimal
4. **Panel de información de carga** que muestra:
   - Dirección de carga actual
   - Tamaño del binario
   - Dirección final de carga
   - Slot de memoria donde se carga
5. **Visualización gráfica** (canvas) que muestra:
   - Todos los slots de memoria
   - La ubicación del binario en cada slot
   - Código de colores para diferentes tipos de memoria

---

### 3. **Funciones JavaScript Agregadas**

#### Variables Globales:
```javascript
let currentLoadAddress = 0xC000;  // Dirección por defecto
let memoryMap = {
    'slot0': { address: 0x0000, size: 0x4000, name: 'BIOS/ROM' },
    'slot1': { address: 0x4000, size: 0x4000, name: 'Cartridge' },
    'slot2': { address: 0x8000, size: 0x4000, name: 'RAM (Slot 2)' },
    'slot3': { address: 0xC000, size: 0x4000, name: 'RAM Principal' }
};
```

#### Nuevas Funciones:

| Función | Descripción |
|---------|-------------|
| `setMemoryAddress(address)` | Establece la dirección de carga predefinida |
| `applyCustomAddress()` | Aplica una dirección personalizada ingresada por el usuario |
| `findMemorySlot(address)` | Busca el slot de memoria para una dirección |
| `updateLoadInfo()` | Actualiza la información visual del panel de carga |
| `drawMemoryMap()` | Dibuja la visualización gráfica del mapa de memoria |

---

## 🎯 Características Principales

### ✅ Dirección de Carga Adaptable

Los usuarios pueden seleccionar la dirección de carga de varias formas:

```javascript
// Preestablecidas (botones)
window.setMemoryAddress(0xC000);  // Estándar
window.setMemoryAddress(0x4000);  // Cartridge
window.setMemoryAddress(0x8000);  // RAM Slot 2

// Personalizada
document.getElementById('customLoadAddress').value = '0x5000';
window.applyCustomAddress();
```

### ✅ Validación de Entrada

```javascript
- Solo acepta formato hexadecimal (0xC000 o C000)
- Valida rango 16-bit (0x0000 - 0xFFFF)
- Detecta cuando el binario sobrepasa los límites de memoria
```

### ✅ Visualización del Mapa de Memoria

El canvas muestra:
- **Slots de memoria** en diferentes colores (ROM, Cartridge, RAM)
- **Ubicación del binario** en verde destacado
- **Proporción relativa** dentro de cada slot
- **Leyenda** para interpretar los colores

---

## 📊 Flujo de Uso

1. **Usuario carga un ROM**
   ```
   archivo ROM → handleFileSelect() → displayFileInfo()
   ```

2. **Panel de memoria se muestra automáticamente**
   ```
   Se muestran todos los slots disponibles
   Se visualiza dónde se cargará el binario
   Se actualiza el canvas gráfico
   ```

3. **Usuario selecciona dirección (opcional)**
   ```
   Click en botón predefinido → setMemoryAddress()
   O ingresa custom → applyCustomAddress()
   ```

4. **La información se actualiza en tiempo real**
   ```
   updateLoadInfo() → drawMemoryMap()
   Se muestran dirección inicio/fin y slot asignado
   ```

---

## 🔧 Integración con Procesador WASM

El sistema está completamente integrado con el módulo WASM:

```rust
// En Rust - Disponible para futuras extensiones
let load_info = processor.create_load_info(0xC000, rom_data.len() as u32);
let slot = processor.find_memory_slot(0xC000);
```

---

## 🎨 Interfaz Visual

### Panel de Memoria (antes de cargar archivo)

```
🗺️ Mapa de Memoria MSX2
─────────────────────────────────
0x0000-0x3FFF    BIOS/ROM (Slot 0)
0x4000-0x7FFF    Cartridge (Slot 1)
0x8000-0xBFFF    RAM (Slot 2)
0xC000-0xFFFF    RAM Principal (Slot 3)

Dirección de Carga:
[0xC000 Estándar] [0x4000 Slot 1] [0x8000 Slot 2]
[Dirección personalizada: ______ Aplicar]
```

### Panel de Información de Carga (después de cargar archivo)

```
📍 Información de Carga
Dirección:      0xC000
Tamaño Binary:  512 bytes
Dirección Fin:  0xC200
Slot:          RAM Principal (0xC000)
```

### Visualización Gráfica

```
┌─────────────────────────────┐
│ 0x0000-0x3FFF BIOS/ROM     │  ← ROM (Gris)
├─────────────────────────────┤
│ 0x4000-0x7FFF Cartridge    │  ← Cartridge (Púrpura)
├─────────────────────────────┤
│ 0x8000-0xBFFF RAM Slot 2   │  ← RAM (Azul)
├─────┬───────────────────────┤
│ 0xC000 │███ Binario ███     │  ← Binario cargado en verde
│ RAM... │                     │
└─────┴───────────────────────┘
■ Verde = Binario cargado
```

---

## 🚀 Casos de Uso

### 1. **Cargar un programa en RAM Principal** (más común)
```
Dirección: 0xC000 (predeterminado)
Ideal para: Programas BASIC, Juegos compilados
```

### 2. **Cargar un cartridge**
```
Dirección: 0x4000
Ideal para: ROMs de cartridge, código de máquina
```

### 3. **Cargar en RAM alternativa**
```
Dirección: 0x8000
Ideal para: Datos auxiliares, buffers
```

### 4. **Cargar en dirección personalizada**
```
Dirección: 0x5200 (personalizada)
Ideal para: ROMs con encabezado, formatos especiales
```

---

## ⚠️ Validaciones y Advertencias

### Advertencia de Sobrepaso
Si el binario sobrepasa los límites de memoria:
```
⚠️ Sobrepasa memoria
Dirección Final: 0x1003B (fuera de rango)
```

### Validaciones Implementadas
- ✅ Rango válido de 16-bit
- ✅ Formato hexadecimal correcto
- ✅ Detección de sobrepaso de memoria
- ✅ Actualización dinámica al cambiar dirección

---

## 📝 Notas Técnicas

### Estructura interna del mapa de memoria:

```javascript
memoryMap = {
    'slot0': { 
        address: 0x0000, 
        size: 0x4000,
        name: 'BIOS/ROM',
        region_type: 'ROM'
    },
    // ... otros slots
}
```

### Parser de dirección hexadecimal:

```javascript
// Acepta ambos formatos:
'0xC000'  → parseInt('0xC000', 16) = 49152
'C000'    → parseInt('C000', 16) = 49152
```

### Cálculo de slot de memoria:

```javascript
// Busca el slot que contiene la dirección
for (let [key, slot] of Object.entries(memoryMap)) {
    if (address >= slot.address && 
        address < (slot.address + slot.size)) {
        return slot.name;
    }
}
```

---

## 🔄 Estado del Proyecto

| Componente | Estado | Notas |
|-----------|--------|-------|
| Rust (WASM) | ✅ Compilado | Sin errores, warnings menores |
| HTML Panel | ✅ Implementado | Totalmente funcional |
| JavaScript | ✅ Funcional | Validación de entrada incluida |
| Visualización | ✅ Canvas | Gráfico dinámico implementado |
| Integración | ✅ Completa | Funciona con WASM processor |

---

## 📦 Archivos Modificados

```
parseadorwebAsembler/
├── src/
│   └── lib.rs                    (↑ Nuevas estructuras y métodos)
├── index.html                     (↑ Nuevo panel de memoria)
├── pkg/
│   ├── msx2_processor.js          (↑ Regenerado automáticamente)
│   ├── msx2_processor.d.ts        (↑ Tipos TypeScript actualizados)
│   └── msx2_processor_bg.wasm     (↑ Binario WASM compilado)
└── MAPA_MEMORIA_IMPLEMENTACION.md (← Este archivo)
```

---

## 🎓 Próximas Mejoras Opcionales

### Funcionalidades futuras:
- [ ] Guardar/cargar presets de carga
- [ ] Histórico de direcciones usadas
- [ ] Validación de conflictos de memoria
- [ ] Estadísticas de ocupación
- [ ] Exportar mapa visual como imagen
- [ ] Soporte para múltiples binarios simultáneamente

---

## © 2026 PAPIWEB DESARROLLOS INFORMATICOS

**Versión**: 2.1  
**Fecha**: 6 de Febrero de 2026  
**Sistema Operativo**: Linux (Ubuntu 24.04 LTS)  
**Lenguajes**: Rust, WebAssembly, JavaScript, HTML5

---

## 📞 Soporte

Para reportes de problemas o sugerencias sobre el mapeo de memoria:
- Revisa la consola del navegador (F12)
- Verifica el formato de entrada hexadecimal
- Confirma que la dirección está dentro del rango 0x0000-0xFFFF

**Estado de Compilación**: ✅ Exitosa (6 Feb 2026 22:55)
