//! ╔═════════════════════════════════════════════════════════════════╗
//! ║         GUÍA DE INTEGRACIÓN: HTML5 CANVAS + WASM                ║
//! ║    © 2026 PAPIWEB DESARROLLOS INFORMATICOS                       ║
//! ╚═════════════════════════════════════════════════════════════════╝

/*

═══════════════════════════════════════════════════════════════════════════
PASO 1: COMPILAR A WEBASSEMBLY
═══════════════════════════════════════════════════════════════════════════

```bash
# Instalar wasm-pack si no lo tienes
npm install -g wasm-pack

# Compilar el proyecto de Rust a WASM
wasm-pack build --release --target web

# Esto genera la carpeta "pkg/" con:
# - msx2_processor.js (bindings JavaScript)
# - msx2_processor.wasm (binario WebAssembly)
```

═══════════════════════════════════════════════════════════════════════════
PASO 2: CREAR HTML CON CANVAS
═══════════════════════════════════════════════════════════════════════════

Archivo: index.html

```html
<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MSX2 Processor - PAPIWEB</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            background: #1a1a1a;
            color: #fff;
            display: flex;
            flex-direction: column;
            align-items: center;
            padding: 20px;
        }
        
        h1 {
            color: #00ff00;
            text-shadow: 0 0 10px #00ff00;
            margin-bottom: 20px;
        }
        
        .container {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 30px;
            max-width: 1200px;
        }
        
        .panel {
            background: #222;
            border: 2px solid #00ff00;
            border-radius: 8px;
            padding: 20px;
            box-shadow: 0 0 20px rgba(0, 255, 0, 0.3);
        }
        
        canvas {
            border: 1px solid #444;
            background: #000;
            width: 100%;
            height: auto;
            image-rendering: pixelated;
        }
        
        .controls {
            display: flex;
            flex-direction: column;
            gap: 15px;
        }
        
        label {
            display: flex;
            align-items: center;
            gap: 10px;
            cursor: pointer;
        }
        
        input[type="checkbox"] {
            width: 20px;
            height: 20px;
            cursor: pointer;
        }
        
        input[type="range"] {
            width: 100%;
        }
        
        button {
            background: #00ff00;
            color: #000;
            border: none;
            padding: 10px 20px;
            border-radius: 4px;
            cursor: pointer;
            font-weight: bold;
            transition: all 0.3s;
        }
        
        button:hover {
            background: #00cc00;
            box-shadow: 0 0 15px rgba(0, 255, 0, 0.5);
        }
        
        .status {
            background: #333;
            padding: 10px;
            border-radius: 4px;
            font-family: monospace;
            font-size: 12px;
            margin-top: 15px;
        }
        
        .stat {
            display: flex;
            justify-content: space-between;
            margin: 5px 0;
        }
    </style>
</head>
<body>
    <h1>🎮 MSX2 PROCESSOR - PAPIWEB</h1>
    <p>Post-procesamiento avanzado de sprites retro</p>
    
    <div class="container">
        <!-- Canvas para mostrar resultado -->
        <div class="panel">
            <h2>Resultado (4K)</h2>
            <canvas id="canvas" width="3840" height="2160"></canvas>
        </div>
        
        <!-- Controles -->
        <div class="panel">
            <h2>Controles</h2>
            <div class="controls">
                
                <label>
                    <input type="checkbox" id="cbBilinear" checked>
                    <span>Interpolación Bilineal (4K)</span>
                </label>
                
                <label>
                    <input type="checkbox" id="cbNormals" checked>
                    <span>Generar Normal Maps</span>
                </label>
                
                <label>
                    <input type="checkbox" id="cbEdges" checked>
                    <span>Detección Sobel</span>
                </label>
                
                <div>
                    <label for="sliderGlow">
                        Intensidad Glow: <span id="glowValue">1.5</span>
                    </label>
                    <input type="range" id="sliderGlow" min="0" max="3" 
                           step="0.1" value="1.5">
                </div>
                
                <button onclick="procesarSprite()">
                    ▶️ Procesar Frame
                </button>
                
                <button onclick="iniciarLoop()">
                    ▶️ Iniciar Loop
                </button>
                
                <button onclick="detenerLoop()">
                    ⏹️ Detener
                </button>
                
                <div class="status">
                    <div class="stat">
                        <span>Estado:</span>
                        <span id="estado">Listo</span>
                    </div>
                    <div class="stat">
                        <span>Frames:</span>
                        <span id="frames">0</span>
                    </div>
                    <div class="stat">
                        <span>FPS:</span>
                        <span id="fps">0</span>
                    </div>
                    <div class="stat">
                        <span>Tamaño RGBA:</span>
                        <span id="rgba_size">0 KB</span>
                    </div>
                    <div class="stat">
                        <span>Normal Maps:</span>
                        <span id="normals_size">0 KB</span>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <script type="module">
        // Importar módulo WASM
        import init, { MSX2Processor } from './pkg/msx2_processor.js';
        
        let processor = null;
        let loopId = null;
        let frameCount = 0;
        let lastTime = Date.now();
        
        // Inicializar WASM
        async function initWasm() {
            try {
                await init();
                processor = new MSX2Processor(256, 212);
                document.getElementById('estado').textContent = 'Listo';
                console.log('✅ WASM inicializado');
            } catch (err) {
                console.error('❌ Error inicializar WASM:', err);
                document.getElementById('estado').textContent = 'Error WASM';
            }
        }
        
        // Procesar un frame
        window.procesarSprite = function() {
            if (!processor) {
                alert('WASM no inicializado');
                return;
            }
            
            document.getElementById('estado').textContent = 'Procesando...';
            
            // Simular datos MSX2 (en producción vendrían de un binario)
            const spriteData = generarDatosAleatorios();
            
            const enableBilinear = document.getElementById('cbBilinear').checked;
            const enableNormals = document.getElementById('cbNormals').checked;
            const enableEdges = document.getElementById('cbEdges').checked;
            const glowIntensity = parseFloat(document.getElementById('sliderGlow').value);
            
            // Procesar
            const result = processor.process_with_post_effects(
                spriteData,
                enableBilinear,
                enableNormals,
                enableEdges,
                glowIntensity
            );
            
            // Mostrar en canvas
            const rgba = result.get_rgba();
            const canvas = document.getElementById('canvas');
            const ctx = canvas.getContext('2d', { willReadFrequently: true });
            
            if (enableBilinear) {
                // Si está escalado a 4K, ajustar canvas
                canvas.width = 3840;
                canvas.height = 2160;
            } else {
                canvas.width = 256;
                canvas.height = 212;
            }
            
            const imageData = ctx.createImageData(canvas.width, canvas.height);
            imageData.data.set(rgba);
            ctx.putImageData(imageData, 0, 0);
            
            // Estadísticas
            document.getElementById('rgba_size').textContent = 
                (rgba.length / 1024).toFixed(2) + ' KB';
            document.getElementById('normals_size').textContent = 
                (result.get_normals().length / 1024).toFixed(2) + ' KB';
            
            document.getElementById('estado').textContent = 'Completado';
        };
        
        // Loop en tiempo real
        window.iniciarLoop = function() {
            if (loopId) return; // Ya corriendo
            
            if (!processor) {
                alert('WASM no inicializado');
                return;
            }
            
            document.getElementById('estado').textContent = 'Loop...';
            frameCount = 0;
            lastTime = Date.now();
            
            loopId = setInterval(() => {
                frameCount++;
                procesarSprite();
                
                // Actualizar FPS cada 10 frames
                if (frameCount % 10 === 0) {
                    const now = Date.now();
                    const fps = (10 * 1000) / (now - lastTime);
                    document.getElementById('fps').textContent = fps.toFixed(1);
                    lastTime = now;
                }
                
                document.getElementById('frames').textContent = frameCount;
            }, 33); // ~30 FPS
        };
        
        // Detener loop
        window.detenerLoop = function() {
            if (loopId) {
                clearInterval(loopId);
                loopId = null;
                document.getElementById('estado').textContent = 'Detenido';
            }
        };
        
        // Actualizar valor glow
        document.getElementById('sliderGlow').addEventListener('input', (e) => {
            document.getElementById('glowValue').textContent = e.target.value;
        });
        
        // Generar datos MSX2 simulados
        function generarDatosAleatorios() {
            const size = 256 * 212 / 2; // 4bpp
            const data = new Uint8Array(size);
            for (let i = 0; i < size; i++) {
                data[i] = Math.floor(Math.random() * 256);
            }
            return data;
        }
        
        // Iniciar
        initWasm();
    </script>
</body>
</html>
```

═══════════════════════════════════════════════════════════════════════════
PASO 3: SERVIR LOCALMENTE
═══════════════════════════════════════════════════════════════════════════

```bash
# Opción 1: Python
python -m http.server 8000

# Opción 2: Node.js
npx http-server

# Opción 3: Ruby
ruby -run -ehttpd . -p8000

# Luego abrir: http://localhost:8000
```

═══════════════════════════════════════════════════════════════════════════
INTEGRACIÓN CON TEXTURA EN SHADER 3D
═══════════════════════════════════════════════════════════════════════════

Vertex Shader:

```glsl
#version 300 es

in vec3 position;
in vec2 uv;

out vec2 vUV;

void main() {
    vUV = uv;
    gl_Position = vec4(position, 1.0);
}
```

Fragment Shader:

```glsl
#version 300 es
precision highp float;

uniform sampler2D u_Texture;
uniform sampler2D u_NormalMap;
uniform vec3 u_LightPos;
uniform vec3 u_ViewPos;

in vec2 vUV;
out vec4 FragColor;

void main() {
    // Color base
    vec3 color = texture(u_Texture, vUV).rgb;
    
    // Normal del mapa normal
    vec3 normal = normalize(texture(u_NormalMap, vUV).rgb * 2.0 - 1.0);
    
    // Iluminación simple
    vec3 lightDir = normalize(u_LightPos - vec3(vUV, 0.0));
    float diffuse = max(dot(normal, lightDir), 0.0);
    
    // Salida con sombras
    FragColor = vec4(color * diffuse, 1.0);
}
```

═══════════════════════════════════════════════════════════════════════════
USO DESDE JAVASCRIPT VANILA
═══════════════════════════════════════════════════════════════════════════

```javascript
import init, { MSX2Processor } from './pkg/msx2_processor.js';

async function main() {
    // Inicializar WASM
    await init();
    
    // Crear procesador
    const processor = new MSX2Processor(256, 212);
    
    // Cargar datos MSX2 (ej: fetch de archivo binario)
    const response = await fetch('sprite.bin');
    const arrayBuffer = await response.arrayBuffer();
    const spriteData = new Uint8Array(arrayBuffer);
    
    // Procesar con todos los efectos
    const result = processor.process_with_post_effects(
        spriteData,
        true,   // bilinear → 4K
        true,   // normal maps
        true,   // sobel edges
        1.5     // glow intensity
    );
    
    // Obtener resultados
    const rgba4K = result.get_rgba();
    const normals = result.get_normals();
    const edges = result.get_edges();
    
    // Renderizar en canvas
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    const imageData = ctx.createImageData(3840, 2160);
    imageData.data.set(rgba4K);
    ctx.putImageData(imageData, 0, 0);
    
    // Usar normal maps en WebGL
    uploadNormalsToGPU(normals);
}

main().catch(console.error);
```

═══════════════════════════════════════════════════════════════════════════
OPTIMIZACIONES PARA PRODUCCIÓN
═══════════════════════════════════════════════════════════════════════════

1. Workers (procesar en background)
2. Streaming (procesar frame por frame)
3. Caching (guardar resultados)
4. Lazy loading (cargar WASM bajo demanda)

```javascript
// Usar Web Worker para no bloquear interfaz
const worker = new Worker('processor.worker.js');

worker.postMessage({
    spriteData: spriteData,
    options: {
        bilinear: true,
        normals: true,
        edges: true,
        glow: 1.5
    }
});

worker.onmessage = (e) => {
    const { rgba, normals, edges } = e.data;
    renderizarResultado(rgba);
};
```

═══════════════════════════════════════════════════════════════════════════
TROUBLESHOOTING
═══════════════════════════════════════════════════════════════════════════

Problema: "Cannot find module: pkg/msx2_processor.js"
Solución: Ejecutar "wasm-pack build --release --target web"

Problema: Canvas en blanco
Solución: Verificar que spriteData tenga datos válidos

Problema: Bajo rendimiento
Solución: Desactivar normals, usar Perfil Ligero

Problema: CORS error
Solución: Servir desde servidor, no file://

═══════════════════════════════════════════════════════════════════════════

*/
