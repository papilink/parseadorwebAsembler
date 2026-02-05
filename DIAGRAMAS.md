# Diagrama de Flujo - Post-Procesamiento MSX2

## 🎬 Flujo Principal de Procesamiento

```mermaid
flowchart TD
    A["📦 Entrada: MSX2 Binary<br/>4bpp | 256×212 píxeles"] 
    
    B["🔄 RGBA Converter<br/>Mapear paleta 16 colores"]
    
    C{"¿Bilinear?"}
    C1["✨ Interpolación Bilineal<br/>256×212 → 3840×2160<br/>O(dst × src)"]
    C2["Mantener 256×212"]
    
    D{"¿Normals?"}
    D1["💡 Generate Normal Maps<br/>Luminancia → Vectores 3D<br/>O(w × h × 9)"]
    D2["Vacío"]
    
    E{"¿Edges?"}
    E1["🌟 Sobel Edge Detection<br/>Kernel 3×3 Gx, Gy<br/>O(w × h × 9)"]
    E2["Vacío"]
    
    F{"¿Glow?"}
    F1["🎆 Neon Glow<br/>Aplicar a bordes<br/>O(w × h × r²)"]
    F2["Sin glow"]
    
    G["📊 PostProcessResult<br/>rgba: Vec&lt;u8&gt;<br/>normals: Vec&lt;u8&gt;<br/>edges: Vec&lt;f32&gt;"]
    
    A --> B
    B --> C
    C -->|true| C1
    C -->|false| C2
    
    C1 --> D
    C2 --> D
    
    D -->|true| D1
    D -->|false| D2
    
    D1 --> E
    D2 --> E
    
    E -->|true| E1
    E -->|false| E2
    
    E1 --> F
    E2 --> F
    
    F -->|true| F1
    F -->|false| F2
    
    F1 --> G
    F2 --> G
```

---

## 📐 Interpolación Bilineal

```mermaid
graph LR
    A["Píxel Fuente<br/>256×212"] 
    B["Calcular Factores<br/>fx = frac(x)<br/>fy = frac(y)"]
    C["Leer 4 Vecinos<br/>P00, P10<br/>P01, P11"]
    D["Interpolación<br/>P0 = Lerp P00,P10<br/>P1 = Lerp P01,P11<br/>P = Lerp P0,P1"]
    E["Píxel Destino<br/>3840×2160"]
    
    A --> B --> C --> D --> E
    
    style A fill:#f9f,color:#000
    style E fill:#ff9,color:#000
    style D fill:#9ff,color:#000
```

---

## 💡 Generación Normal Maps

```mermaid
flowchart TD
    A["Píxel (x,y)"]
    B["Obtener Luminancia Vecinos<br/>Left, Right, Up, Down"]
    C["Calcular Derivadas<br/>dx = Right - Left<br/>dy = Down - Up"]
    D["Compute Normal<br/>N = (-dx, -dy, 1)<br/>Normalize N"]
    E["Codificar RGB<br/>X: -1..1 → 0..255<br/>Y: -1..1 → 0..255<br/>Z: -1..1 → 0..255"]
    F["Normal Map Píxel"]
    
    A --> B --> C --> D --> E --> F
    
    style F fill:#f9f,color:#000
```

---

## 🌟 Detección Sobel

```mermaid
flowchart TD
    A["Kernel 3×3<br/>Píxel Central"]
    B["Aplicar Gx<br/>[-1 0 +1]<br/>[-2 0 +2]<br/>[-1 0 +1]"]
    C["Aplicar Gy<br/>[-1 -2 -1]<br/>[ 0  0  0]<br/>[+1 +2 +1]"]
    D["Calcular Magnitud<br/>|G| = √(Gx² + Gy²)"]
    E["Edge Map Valor<br/>0..255"]
    
    A --> B
    A --> C
    B --> D
    C --> D
    D --> E
    
    style E fill:#9f9,color:#000
```

---

## 🎆 Neon Glow Application

```mermaid
flowchart TD
    A["Edge Map<br/>Magnitudes<br/>Sobel"]
    B{"Edge Value<br/>> threshold?"}
    B -->|No| C["Píxel sin cambios"]
    B -->|Si| D["Calcular Glow<br/>Radio: 3px<br/>Falloff: lineal"]
    D --> E["Expandir a Vecinos<br/>Multiplicar brillo<br/>Factor: intensity"]
    E --> F["Píxel + Glow<br/>Min(val+glow, 255)"]
    
    C --> G["Output RGBA<br/>con Neon Effect"]
    F --> G
    
    style G fill:#ff9,color:#000
```

---

## 🔄 Loop en Tiempo Real

```mermaid
graph LR
    A["Frame N<br/>MSX2 Binary"]
    B["Process<br/>Post-Effects"]
    C["Render<br/>Canvas/WebGL"]
    D["Frame N+1"]
    
    A --> B --> C --> D
    
    loop Cada 33ms ~30FPS
    end
    
    style A fill:#9ff,color:#000
    style C fill:#f9f,color:#000
```

---

## 📊 Arquitectura de Datos

```mermaid
graph TD
    subgraph "Entrada"
    A["Binario MSX2<br/>4bpp<br/>27.5 KB<br/>256×212"]
    end
    
    subgraph "Procesamiento"
    B["RGBA<br/>256×212<br/>221 KB"]
    C["RGBA Escalado<br/>3840×2160<br/>31.6 MB"]
    D["Normal Maps<br/>256×212<br/>166.5 KB"]
    E["Edge Map<br/>256×212<br/>221 KB"]
    end
    
    subgraph "Salida"
    F["PostProcessResult<br/>rgba<br/>normals<br/>edges"]
    end
    
    A --> B
    B --> C
    B --> D
    B --> E
    C --> F
    D --> F
    E --> F
    
    style A fill:#ff9,color:#000
    style F fill:#9f9,color:#000
```

---

## 🎯 Pipeline de Shader WebGL

```mermaid
graph LR
    A["Texture Sprite<br/>RGBA"]
    B["Normal Map<br/>RGB"]
    C["Vertex Shader<br/>Transform"]
    D["Fragment Shader<br/>Iluminación"]
    E["Luz Dinámica<br/>Mouse Pos"]
    F["Output<br/>Canvas"]
    
    A --> D
    B --> D
    C --> D
    E --> D
    D --> F
    
    style D fill:#f9f,color:#000
    style F fill:#9f9,color:#000
```

---

## ⚡ Complejidad Computacional

```mermaid
graph TD
    A["MSX2 Conversion<br/>O(input size)<br/>~27.5 KB<br/>< 1ms"]
    
    B["Interpolación Bi<br/>O(dst_w × dst_h)<br/>3840 × 2160<br/>50-100ms"]
    
    C["Normal Maps<br/>O(w × h × 9)<br/>256 × 212 × 9<br/>10-20ms"]
    
    D["Sobel Edges<br/>O(w × h × 9)<br/>256 × 212 × 9<br/>10-20ms"]
    
    E["Neon Glow<br/>O(w × h × r²)<br/>radius=3<br/>30-50ms"]
    
    F["Total<br/>~100-200ms<br/>Compatible 60 FPS<br/>sin normal maps"]
    
    A --> F
    B --> F
    C --> F
    D --> F
    E --> F
    
    style F fill:#ff9,color:#000
```

---

## 🎨 Flujo Integración Web

```mermaid
graph TD
    A["ROM MSX2<br/>archivo .bin"]
    B["Compilar a WASM<br/>wasm-pack build"]
    C["Módulo JavaScript<br/>.js + .wasm"]
    D["HTML5 Canvas<br/>o WebGL"]
    E["Aplicación Web<br/>en navegador"]
    
    A --> B --> C --> D --> E
    
    style E fill:#9f9,color:#000
    style C fill:#f9f,color:#000
```

---

## 📈 Perfil de Memoria

```mermaid
graph LR
    A["Input<br/>27.5 KB"]
    B["RGBA Temp<br/>~221 KB"]
    C["Scaled RGBA<br/>31.6 MB<br/>(4K)"]
    D["Normals Temp<br/>~166.5 KB"]
    E["Edges Temp<br/>~221 KB"]
    F["Result<br/>31.6 MB"]
    
    style C fill:#ff9,color:#000
    style F fill:#9f9,color:#000
```

---

## 🔀 Rutas CondicionalES del Proceso

```mermaid
graph TD
    A["process_with_post_effects"]
    
    B{"enable_bilinear?"}
    B -->|true| B1["Interpolar 4K"]
    B -->|false| B2["Mantener original"]
    
    C{"enable_normals?"}
    C -->|true| C1["Generate normals"]
    C -->|false| C2["Empty vec"]
    
    D{"enable_edges?"}
    D -->|true| D1["Sobel detection"]
    D -->|false| D2["Empty vec"]
    
    E{"enable_glow?"}
    E -->|true| E1["Apply neon"]
    E -->|false| E2["Sin glow"]
    
    B1 --> C
    B2 --> C
    C1 --> D
    C2 --> D
    D1 --> E
    D2 --> E
    E1 --> F["PostProcessResult"]
    E2 --> F
    
    style F fill:#9f9,color:#000
```

---

*Diagramas generados con Mermaid.js*
*© 2026 PAPIWEB DESARROLLOS INFORMATICOS*
