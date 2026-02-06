//! ╔════════════════════════════════════════════════════════════════╗
//! ║  PAPIWEB DESARROLLOS INFORMATICOS                              ║
//! ║  MSX2 Processor con Post-Procesamiento Avanzado                ║
//! ║  - Interpolación Bilineal (256x212 → 4K)                       ║
//! ║  - Generación de Normal Maps                                   ║
//! ║  - Detección de Bordes (Filtro Sobel) con Glow Neón            ║
//! ╚════════════════════════════════════════════════════════════════╝

use wasm_bindgen::prelude::*;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════
// MAPA DE MEMORIA DEL MSX2
// ═══════════════════════════════════════════════════════════════

#[wasm_bindgen]
pub struct MemoryMapSlot {
    address: u32,
    size: u32,
    name: String,
    region_type: String,
}

#[wasm_bindgen]
impl MemoryMapSlot {
    #[wasm_bindgen(constructor)]
    pub fn new(address: u32, size: u32, name: String, region_type: String) -> MemoryMapSlot {
        MemoryMapSlot {
            address,
            size,
            name,
            region_type,
        }
    }

    pub fn get_address(&self) -> u32 {
        self.address
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_region_type(&self) -> String {
        self.region_type.clone()
    }
}

// ═══════════════════════════════════════════════════════════════
// INFORMACIÓN DE CARGA
// ═══════════════════════════════════════════════════════════════

#[wasm_bindgen]
pub struct LoadInfo {
    load_address: u32,
    binary_size: u32,
    start_address: u32,
    end_address: u32,
    memory_slot: String,
}

#[wasm_bindgen]
impl LoadInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(
        load_address: u32,
        binary_size: u32,
        start_address: u32,
        end_address: u32,
        memory_slot: String,
    ) -> LoadInfo {
        LoadInfo {
            load_address,
            binary_size,
            start_address,
            end_address,
            memory_slot,
        }
    }

    pub fn get_load_address(&self) -> u32 {
        self.load_address
    }

    pub fn get_binary_size(&self) -> u32 {
        self.binary_size
    }

    pub fn get_start_address(&self) -> u32 {
        self.start_address
    }

    pub fn get_end_address(&self) -> u32 {
        self.end_address
    }

    pub fn get_memory_slot(&self) -> String {
        self.memory_slot.clone()
    }
}

// ═══════════════════════════════════════════════════════════════
// ESTRUCTURAS PRINCIPALES
// ═══════════════════════════════════════════════════════════════

#[wasm_bindgen]
pub struct MSX2Processor {
    palette: [[u8; 4]; 16],
    width: usize,
    height: usize,
    memory_map: HashMap<String, MemoryMapSlot>,
}

// ═══════════════════════════════════════════════════════════════
// IMPLEMENTACIÓN PRINCIPAL
// ═══════════════════════════════════════════════════════════════

#[wasm_bindgen]
impl MSX2Processor {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Self {
        let mut palette = [[0; 4]; 16];
        
        // Paleta MSX2 estándar completa
        palette[0] = [0, 0, 0, 255];           // Negro
        palette[1] = [255, 0, 0, 255];         // Rojo
        palette[2] = [0, 255, 0, 255];         // Verde
        palette[3] = [255, 255, 0, 255];       // Amarillo
        palette[4] = [0, 0, 255, 255];         // Azul
        palette[5] = [255, 0, 255, 255];       // Magenta
        palette[6] = [0, 255, 255, 255];       // Cyan
        palette[7] = [255, 255, 255, 255];     // Blanco
        palette[8] = [128, 0, 0, 255];         // Rojo oscuro
        palette[9] = [0, 128, 0, 255];         // Verde oscuro
        palette[10] = [128, 128, 0, 255];      // Marrón
        palette[11] = [0, 0, 128, 255];        // Azul oscuro
        palette[12] = [128, 0, 128, 255];      // Púrpura
        palette[13] = [0, 128, 128, 255];      // Teal
        palette[14] = [128, 128, 128, 255];    // Gris
        palette[15] = [200, 200, 200, 255];    // Gris claro
        
        // Crear mapa de memoria MSX2
        let mut memory_map = HashMap::new();
        memory_map.insert(
            "slot0".to_string(),
            MemoryMapSlot::new(0x0000, 0x4000, "BIOS/ROM".to_string(), "ROM".to_string()),
        );
        memory_map.insert(
            "slot1".to_string(),
            MemoryMapSlot::new(0x4000, 0x4000, "Cartridge".to_string(), "Cartridge".to_string()),
        );
        memory_map.insert(
            "slot2".to_string(),
            MemoryMapSlot::new(0x8000, 0x4000, "RAM (Slot 2)".to_string(), "RAM".to_string()),
        );
        memory_map.insert(
            "slot3".to_string(),
            MemoryMapSlot::new(0xC000, 0x4000, "RAM Principal".to_string(), "RAM".to_string()),
        );
        
        MSX2Processor {
            palette,
            width: width as usize,
            height: height as usize,
            memory_map,
        }
    }

    /// Obtener el mapa de memoria completo como JSON
    pub fn get_memory_map(&self) -> JsValue {
        let mut map_data = vec![];
        
        for (_, slot) in self.memory_map.iter() {
            map_data.push(format!(
                r#"{{"address":"0x{:04X}","size":"0x{:04X}","name":"{}","type":"{}"}}"#,
                slot.address, slot.size, slot.name, slot.region_type
            ));
        }
        
        JsValue::from_str(&format!("[{}]", map_data.join(",")))
    }

    /// Encontrar slot de memoria para una dirección dada
    pub fn find_memory_slot(&self, address: u32) -> String {
        for (_name, slot) in self.memory_map.iter() {
            if address >= slot.address && address < (slot.address + slot.size) {
                return format!("{} (0x{:04X})", slot.name, slot.address);
            }
        }
        "Desconocido".to_string()
    }

    /// Crear información de carga para una dirección específica
    pub fn create_load_info(&self, load_address: u32, binary_size: u32) -> LoadInfo {
        let end_address = load_address.saturating_add(binary_size);
        let memory_slot = self.find_memory_slot(load_address);
        
        LoadInfo::new(
            load_address,
            binary_size,
            load_address,
            end_address,
            memory_slot,
        )
    }

    /// Convierte MSX2 4bpp a RGBA 32bpp
    pub fn transform_to_rgba(&self, bin_data: &[u8]) -> Vec<u8> {
        let mut rgba_output = Vec::with_capacity(bin_data.len() * 8);

        for &byte in bin_data {
            let pixel1 = (byte >> 4) & 0x0F;
            let pixel2 = byte & 0x0F;

            rgba_output.extend_from_slice(&self.palette[pixel1 as usize]);
            rgba_output.extend_from_slice(&self.palette[pixel2 as usize]);
        }
        rgba_output
    }

    /// ════════════════════════════════════════════════════════
    /// INTERPOLACIÓN BILINEAL: 256x212 → 4K (3840x2160)
    /// ════════════════════════════════════════════════════════
    pub fn bilinear_interpolation(&self, rgba_data: &[u8]) -> Vec<u8> {
        let src_width = 256;
        let src_height = 212;
        let dst_width = 3840; // 4K Ultra HD
        let dst_height = 2160;

        let mut output = vec![0u8; dst_width * dst_height * 4];

        let scale_x = (src_width as f32) / (dst_width as f32);
        let scale_y = (src_height as f32) / (dst_height as f32);

        for y in 0..dst_height {
            for x in 0..dst_width {
                let src_x = (x as f32) * scale_x;
                let src_y = (y as f32) * scale_y;

                // Coordenadas base
                let x0 = src_x.floor() as usize;
                let y0 = src_y.floor() as usize;
                let x1 = (x0 + 1).min(src_width - 1);
                let y1 = (y0 + 1).min(src_height - 1);

                // Factores de interpolación
                let fx = src_x - src_x.floor();
                let fy = src_y - src_y.floor();

                // Interpolación bilineal para cada canal
                for c in 0..4 {
                    let p00 = rgba_data[(y0 * src_width + x0) * 4 + c] as f32;
                    let p10 = rgba_data[(y0 * src_width + x1) * 4 + c] as f32;
                    let p01 = rgba_data[(y1 * src_width + x0) * 4 + c] as f32;
                    let p11 = rgba_data[(y1 * src_width + x1) * 4 + c] as f32;

                    let p0 = p00 * (1.0 - fx) + p10 * fx;
                    let p1 = p01 * (1.0 - fx) + p11 * fx;
                    let pixel = (p0 * (1.0 - fy) + p1 * fy) as u8;

                    output[(y * dst_width + x) * 4 + c] = pixel;
                }
            }
        }

        output
    }

    /// ════════════════════════════════════════════════════════
    /// GENERACIÓN DE NORMAL MAPS
    /// Calcula vectores normales basados en luminancia (altura)
    /// ════════════════════════════════════════════════════════
    pub fn generate_normal_map(&self, rgba_data: &[u8]) -> Vec<u8> {
        let width = self.width;
        let height = self.height;
        let mut normals = vec![128u8; width * height * 3]; // RGB centrado en 128

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                // Obtener luminancia de píxeles vecinos
                let heightL = self.get_luminance(rgba_data, (x - 1) as i32, y as i32);
                let heightR = self.get_luminance(rgba_data, (x + 1) as i32, y as i32);
                let heightU = self.get_luminance(rgba_data, x as i32, (y - 1) as i32);
                let heightD = self.get_luminance(rgba_data, x as i32, (y + 1) as i32);

                // Calcular derivadas (Sobel simplificado para altura)
                let dx = (heightR - heightL) as f32 / 2.0;
                let dy = (heightD - heightU) as f32 / 2.0;

                // Vector normal en espacio tangente
                let normal = self.compute_normal(dx, dy, 1.0);

                // Guardar normal [0..255] → [-1..1]
                let idx = (y * width + x) * 3;
                normals[idx] = ((normal.0 + 1.0) * 127.5) as u8;     // X
                normals[idx + 1] = ((normal.1 + 1.0) * 127.5) as u8; // Y
                normals[idx + 2] = ((normal.2 + 1.0) * 127.5) as u8; // Z
            }
        }

        normals
    }

    /// ════════════════════════════════════════════════════════
    /// DETECCIÓN DE BORDES: FILTRO SOBEL
    /// Genera mapa de bordes para efectos glow/neón
    /// ════════════════════════════════════════════════════════
    pub fn detect_edges_sobel(&self, rgba_data: &[u8]) -> Vec<f32> {
        let width = self.width;
        let height = self.height;
        let mut edges = vec![0.0f32; width * height];

        // Kernels Sobel
        let sobel_x: [[i32; 3]; 3] = [
            [-1, 0, 1],
            [-2, 0, 2],
            [-1, 0, 1],
        ];

        let sobel_y: [[i32; 3]; 3] = [
            [-1, -2, -1],
            [ 0,  0,  0],
            [ 1,  2,  1],
        ];

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let mut gx = 0.0f32;
                let mut gy = 0.0f32;

                // Recorrer kernel 3x3
                for ky in -1..=1 {
                    for kx in -1..=1 {
                        let px = (x as i32 + kx) as usize;
                        let py = (y as i32 + ky) as usize;

                        let lum = self.get_luminance(rgba_data, px as i32, py as i32);
                        let kx_idx = (kx + 1) as usize;
                        let ky_idx = (ky + 1) as usize;

                        gx += (lum as f32) * (sobel_x[ky_idx][kx_idx] as f32);
                        gy += (lum as f32) * (sobel_y[ky_idx][kx_idx] as f32);
                    }
                }

                let magnitude = (gx * gx + gy * gy).sqrt();
                edges[y * width + x] = magnitude.min(255.0);
            }
        }

        edges
    }

    /// ════════════════════════════════════════════════════════
    /// APLICAR GLOW NEÓN A BORDES DETECTADOS
    /// ════════════════════════════════════════════════════════
    pub fn apply_neon_glow(
        &self,
        rgba_data: &[u8],
        edge_data: &[f32],
        intensity: f32,
    ) -> Vec<u8> {
        let width = self.width;
        let height = self.height;
        let mut result = rgba_data.to_vec();

        let glow_radius = 3;
        let glow_threshold = 50.0;

        for y in 0..height {
            for x in 0..width {
                let edge_value = edge_data[y * width + x];

                if edge_value > glow_threshold {
                    // Aplicar glow a píxeles cercanos
                    for gy in -glow_radius..=glow_radius {
                        for gx in -glow_radius..=glow_radius {
                            let ny = (y as i32 + gy).max(0).min((height - 1) as i32) as usize;
                            let nx = (x as i32 + gx).max(0).min((width - 1) as i32) as usize;

                            let dist = ((gx * gx + gy * gy) as f32).sqrt();
                            let falloff = (1.0 - dist / (glow_radius as f32)).max(0.0);
                            let glow_factor = (edge_value / 255.0) * falloff * intensity;

                            for c in 0..3 {
                                let idx = (ny * width + nx) * 4 + c;
                                let val = result[idx] as f32;
                                result[idx] = ((val + glow_factor * 100.0).min(255.0)) as u8;
                            }
                        }
                    }
                }
            }
        }

        result
    }

    /// ════════════════════════════════════════════════════════
    /// POST-PROCESAMIENTO COMPLETO (LOOP PRINCIPAL)
    /// ════════════════════════════════════════════════════════
    pub fn process_with_post_effects(
        &self,
        bin_data: &[u8],
        enable_bilinear: bool,
        enable_normals: bool,
        enable_edges: bool,
        glow_intensity: f32,
    ) -> PostProcessResult {
        // 1: Conversión inicial RGBA
        let rgba = self.transform_to_rgba(bin_data);

        // 2: Interpolación Bilineal (opcional)
        let scaled = if enable_bilinear {
            self.bilinear_interpolation(&rgba)
        } else {
            rgba.clone()
        };

        // 3: Generación de Normal Maps (opcional)
        let normals = if enable_normals {
            self.generate_normal_map(&rgba)
        } else {
            vec![]
        };

        // 4: Detección de Bordes con Sobel (opcional)
        let edges = if enable_edges {
            self.detect_edges_sobel(&rgba)
        } else {
            vec![]
        };

        // 5: Aplicar Glow Neón
        let final_rgba = if enable_edges {
            self.apply_neon_glow(&scaled, &edges, glow_intensity)
        } else {
            scaled
        };

        PostProcessResult::new(final_rgba, normals, edges)
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // FUNCIONES AUXILIARES
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    fn get_luminance(&self, rgba: &[u8], x: i32, y: i32) -> u8 {
        let x = x.max(0).min((self.width - 1) as i32) as usize;
        let y = y.max(0).min((self.height - 1) as i32) as usize;
        let idx = (y * self.width + x) * 4;

        // Luminancia: 0.299*R + 0.587*G + 0.114*B
        let r = rgba[idx] as f32;
        let g = rgba[idx + 1] as f32;
        let b = rgba[idx + 2] as f32;

        (0.299 * r + 0.587 * g + 0.114 * b) as u8
    }

    fn compute_normal(&self, dx: f32, dy: f32, strength: f32) -> (f32, f32, f32) {
        // Normal perturbado por altura (bump mapping)
        let x = -dx * strength;
        let y = -dy * strength;
        let z = 1.0;

        let len = (x * x + y * y + z * z).sqrt();
        (x / len, y / len, z / len)
    }
}

// ═══════════════════════════════════════════════════════════════
// ESTRUCTURA RESULTADO FINAL
// ═══════════════════════════════════════════════════════════════

#[wasm_bindgen]
pub struct PostProcessResult {
    rgba: Vec<u8>,
    normals: Vec<u8>,
    edges: Vec<f32>,
}

#[wasm_bindgen]
impl PostProcessResult {
    #[wasm_bindgen(constructor)]
    pub fn new(rgba: Vec<u8>, normals: Vec<u8>, edges: Vec<f32>) -> PostProcessResult {
        PostProcessResult { rgba, normals, edges }
    }

    pub fn get_rgba(&self) -> Vec<u8> {
        self.rgba.clone()
    }

    pub fn get_normals(&self) -> Vec<u8> {
        self.normals.clone()
    }

    pub fn get_edges(&self) -> Vec<f32> {
        self.edges.clone()
    }
}

// ═══════════════════════════════════════════════════════════════
// © 2026 PAPIWEB DESARROLLOS INFORMATICOS
// Procesamiento avanzado de sprites MSX2 con IA visual
// ═══════════════════════════════════════════════════════════════
