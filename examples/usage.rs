//! ╔═════════════════════════════════════════════════════════════════╗
//! ║              EJEMPLOS DE USO: POST-PROCESAMIENTO MSX2             ║
//! ║         © 2026 PAPIWEB DESARROLLOS INFORMATICOS                   ║
//! ╚═════════════════════════════════════════════════════════════════╝

use msx2_processor::MSX2Processor;

/// Ejemplo 1: Procesamiento completo con todos los efectos
pub fn example_full_processing() {
    let processor = MSX2Processor::new(256, 212);
    
    // Datos binarios MSX2 (4bpp)
    let msx2_binary = vec![0x12, 0x34, 0x56, 0x78]; // Ejemplo de datos
    
    // Procesar con todos los efectos activados
    let result = processor.process_with_post_effects(
        &msx2_binary,
        true,  // enable_bilinear (256x212 → 3840x2160)
        true,  // enable_normals (generación de normal maps)
        true,  // enable_edges (detección Sobel)
        1.5,   // glow_intensity (intensidad del neón)
    );
    
    println!("🎮 Procesamiento completado!");
    println!("   • Imagen RGBA escalada: {} bytes", result.get_rgba().len());
    println!("   • Normal Map: {} bytes", result.get_normals().len());
    println!("   • Edge Map: {} valores", result.get_edges().len());
}

/// Ejemplo 2: Solo interpolación bilineal (escalado 4K)
pub fn example_bilinear_upscale() {
    let processor = MSX2Processor::new(256, 212);
    let msx2_binary = vec![0xFF; 256 * 212 / 2];
    
    let rgba = processor.transform_to_rgba(&msx2_binary);
    let upscaled_4k = processor.bilinear_interpolation(&rgba);
    
    println!("📐 Interpolación Bilineal:");
    println!("   Original: 256x212 pixels");
    println!("   Escalada: 3840x2160 pixels (4K)");
    println!("   Bytes: {}", upscaled_4k.len());
}

/// Ejemplo 3: Generación de Normal Maps (para iluminación 3D)
pub fn example_normal_map_generation() {
    let processor = MSX2Processor::new(256, 212);
    let msx2_binary = vec![0xAA; 256 * 212 / 2];
    
    let rgba = processor.transform_to_rgba(&msx2_binary);
    let normal_map = processor.generate_normal_map(&rgba);
    
    println!("🔦 Normal Maps (para calcular sombras dinámicas):");
    println!("   Datos de altura (luminancia) procesados");
    println!("   Vectores normales calculados para cada píxel");
    println!("   Formato: RGB (X, Y, Z)");
}

/// Ejemplo 4: Detección de bordes con Sobel + Glow Neón
pub fn example_neon_effect() {
    let processor = MSX2Processor::new(256, 212);
    let msx2_binary = vec![0x48; 256 * 212 / 2];
    
    let rgba = processor.transform_to_rgba(&msx2_binary);
    let edges = processor.detect_edges_sobel(&rgba);
    
    // Aplicar efecto neón
    let neon_result = processor.apply_neon_glow(&rgba, &edges, 2.0);
    
    println!("✨ Efecto Neón/Glow:");
    println!("   Bordes detectados: {}", edges.len());
    println!("   Efecto glow aplicado con radio 3px");
    println!("   Intensidad: 2.0 (máximo: 3.0)");
}

/// Ejemplo 5: Loop de procesamiento en tiempo real
pub fn example_realtime_loop() {
    const SPRITE_WIDTH: u32 = 256;
    const SPRITE_HEIGHT: u32 = 212;
    const FRAME_RATE: f32 = 60.0;
    
    let processor = MSX2Processor::new(SPRITE_WIDTH, SPRITE_HEIGHT);
    
    println!("🎬 Loop de Procesamiento en Tiempo Real:");
    println!("   Resolución: {}x{}", SPRITE_WIDTH, SPRITE_HEIGHT);
    println!("   FPS: {}", FRAME_RATE as u32);
    println!("   Tiempo por frame: {:.2}ms", 1000.0 / FRAME_RATE);
    
    // Simular 30 frames
    for frame in 0..30 {
        // En un caso real, aquí vendrían datos del binario MSX2
        let msx2_frame_data = vec![(frame as u8) * 8; (SPRITE_WIDTH * SPRITE_HEIGHT / 2) as usize];
        
        // Procesar frame completo
        let result = processor.process_with_post_effects(
            &msx2_frame_data,
            true,   // escalado bilineal para 4K
            false,  // normal maps desactivados (overhead)
            true,   // detección de bordes
            1.0,    // glow intensity
        );
        
        if frame % 10 == 0 {
            println!("   [{:3}] Procesado: {} KB de datos", 
                frame, result.get_rgba().len() / 1024);
        }
    }
}

/// Ejemplo 6: Config personalizada para diferentes tipos de sprites
pub fn example_sprite_profiles() {
    println!("⚙️  Perfiles de Procesamiento:");
    println!();
    
    println!("1️⃣  PERFIL CALIDAD (máxima calidad visual)");
    println!("   ✓ Interpolación Bilineal: SÍ (4K)");
    println!("   ✓ Normal Maps: SÍ");
    println!("   ✓ Detección Sobel: SÍ");
    println!("   ✓ Glow Neón: SÍ (intensidad: 2.0)");
    println!("   ⚠️  Alto uso de CPU/GPU");
    println!();
    
    println!("2️⃣  PERFIL RENDIMIENTO (equilibrio)");
    println!("   ✓ Interpolación Bilineal: SÍ (2K)");
    println!("   ✓ Normal Maps: NO");
    println!("   ✓ Detección Sobel: SÍ");
    println!("   ✓ Glow Neón: SÍ (intensidad: 1.0)");
    println!("   ⚠️  Uso moderado");
    println!();
    
    println!("3️⃣  PERFIL LIGERO (máximo rendimiento)");
    println!("   ✓ Interpolación Bilineal: SÍ (1080p)");
    println!("   ✓ Normal Maps: NO");
    println!("   ✓ Detección Sobel: NO");
    println!("   ✓ Glow Neón: NO");
    println!("   ⚠️  Uso mínimo");
}

/// Ejemplo 7: Composición con múltiples mapas
pub fn example_advanced_composition() {
    let processor = MSX2Processor::new(256, 212);
    
    // Datos de prueba
    let sprite_data = vec![0x55; 256 * 212 / 2];
    let rgba_base = processor.transform_to_rgba(&sprite_data);
    
    println!("🎨 Composición Avanzada (Multi-Layer):");
    println!();
    
    // Layer 1: Base escalada
    let layer1_scaled = processor.bilinear_interpolation(&rgba_base);
    println!("   Layer 1 (Base): 3840x2160px escalada");
    
    // Layer 2: Normal Map para iluminación
    let layer2_normals = processor.generate_normal_map(&rgba_base);
    println!("   Layer 2 (Normals): {} vectores para sombreado", 
             layer2_normals.len() / 3);
    
    // Layer 3: Bordes para glow
    let layer3_edges = processor.detect_edges_sobel(&rgba_base);
    println!("   Layer 3 (Edges): {} valores de magnitud Sobel", 
             layer3_edges.len());
    
    // Composición final
    let final_rgba = processor.apply_neon_glow(&layer1_scaled, &layer3_edges, 1.5);
    println!();
    println!("   ✨ Composición final: {} bytes (4K RGBA)", final_rgba.len());
}

/// Ejemplo 8: Caso de uso práctico - Juego retro mejorado
pub fn example_retro_game_enhancement() {
    println!("🕹️  CASO DE USO: Mejora de Juego Retro MSX2");
    println!();
    println!("Inicialmente:");
    println!("   • Resolución: 256x212 (16 colores)");
    println!("   • Sin sombras dinámicas");
    println!("   • Sin efectos de iluminación");
    println!();
    
    let processor = MSX2Processor::new(256, 212);
    
    println!("Con Post-Procesamiento:");
    println!("   ✨ Escala bilineal → 4K (3840x2160)");
    println!("      • Interpolación suave, sin pixelado");
    println!();
    
    println!("   💡 Normal Maps generados");
    println!("      • Posibilita linterna del ratón");
    println!("      • Sombras reales basadas en altura");
    println!("      • Efectos de bump mapping");
    println!();
    
    println!("   🌟 Detección de Bordes (Sobel)");
    println!("      • Glow neón en siluetas de sprites");
    println!("      • Efecto cyberpunk/neon retro gaming");
    println!("      • Resalta detalles del sprite original");
    println!();
    
    println!("Resultado Final:");
    println!("   • Juego retro con visual moderna");
    println!("   • Compatible con shaders 3D");
    println!("   • Efectos de iluminación dinámica");
    println!("   • Preserva lo nostálgico del original");
}

fn main() {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║   MSX2 PROCESSOR - EJEMPLOS DE POST-PROCESAMIENTO       ║");
    println!("║   © 2026 PAPIWEB DESARROLLOS INFORMATICOS              ║");
    println!("╚════════════════════════════════════════════════════════╝");
    println!();
    
    example_full_processing();
    println!();
    
    example_bilinear_upscale();
    println!();
    
    example_normal_map_generation();
    println!();
    
    example_neon_effect();
    println!();
    
    example_realtime_loop();
    println!();
    
    example_sprite_profiles();
    println!();
    
    example_advanced_composition();
    println!();
    
    example_retro_game_enhancement();
}
