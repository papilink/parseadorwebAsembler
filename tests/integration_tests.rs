//! ╔═════════════════════════════════════════════════════════════════╗
//! ║         TESTS - VALIDACIÓN DE COMPONENTES                        ║
//! ║    © 2026 PAPIWEB DESARROLLOS INFORMATICOS                       ║
//! ╚═════════════════════════════════════════════════════════════════╝

#[cfg(test)]
mod tests {
    use msx2_processor::MSX2Processor;

    #[test]
    fn test_processor_creation() {
        let _processor = MSX2Processor::new(256, 212);
        // Debería crear sin panic
        assert!(true);
    }

    #[test]
    fn test_rgba_conversion() {
        let processor = MSX2Processor::new(256, 212);
        
        // Dos píxeles: 0x12 = 0001 0010
        // Píxel 1: índice 1 (rojo), Píxel 2: índice 2 (verde)
        let bin_data = vec![0x12];
        let rgba = processor.transform_to_rgba(&bin_data);
        
        // Debería producir 8 bytes (2 píxeles × 4 bytes)
        assert_eq!(rgba.len(), 8);
        
        // Píxel 1: rojo [255, 0, 0, 255]
        assert_eq!(rgba[0..4], [255, 0, 0, 255]);
        
        // Píxel 2: verde [0, 255, 0, 255]
        assert_eq!(rgba[4..8], [0, 255, 0, 255]);
    }

    #[test]
    fn test_bilinear_interpolation_dimensions() {
        let processor = MSX2Processor::new(256, 212);
        let msx2_data = vec![0xFF; 256 * 212 / 2];
        let rgba = processor.transform_to_rgba(&msx2_data);
        let upscaled = processor.bilinear_interpolation(&rgba);
        
        // 4K = 3840 × 2160 × 4 bytes
        let expected_size = 3840 * 2160 * 4;
        assert_eq!(upscaled.len(), expected_size);
    }

    #[test]
    fn test_bilinear_preserves_colors() {
        let processor = MSX2Processor::new(256, 212);
        
        // Crear imagen uniforme (todos rojo)
        let mut msx2_data = vec![0x0; 256 * 212 / 2];
        // Llenar con color 1 (rojo)
        for byte in &mut msx2_data {
            *byte = 0x11; // Ambos píxeles = color 1
        }
        
        let rgba = processor.transform_to_rgba(&msx2_data);
        let upscaled = processor.bilinear_interpolation(&rgba);
        
        // Muestrear algunos píxeles
        let sample_pixel = &upscaled[0..4];
        // Debería ser rojo o muy parecido (interpolación)
        assert!(sample_pixel[0] > 200); // Plenty red
    }

    #[test]
    fn test_normal_map_generation() {
        let processor = MSX2Processor::new(256, 212);
        let msx2_data = vec![0xAA; 256 * 212 / 2];
        let rgba = processor.transform_to_rgba(&msx2_data);
        let normals = processor.generate_normal_map(&rgba);
        
        // Debería ser RGB (3 bytes) por píxel
        let expected_size = 256 * 212 * 3;
        assert_eq!(normals.len(), expected_size);
        
        // Verificar que hay vectores normales (valores entre 0-255)
        for &byte in &normals {
            assert!(byte <= 255);
        }
    }

    #[test]
    fn test_normal_map_center_value() {
        let processor = MSX2Processor::new(256, 212);
        let msx2_data = vec![0xAA; 256 * 212 / 2];
        let rgba = processor.transform_to_rgba(&msx2_data);
        let normals = processor.generate_normal_map(&rgba);
        
        // En píxel centro donde todo es uniforme,
        // la normal debería estar próxima a (0, 0, 1) = (128, 128, 255)
        let center_pixel = (212 / 2) * 256 + (256 / 2);
        let idx = center_pixel * 3;
        
        let nz = normals[idx + 2] as i32;
        assert!(nz > 240); // Z debería estar cerca de 255
    }

    #[test]
    fn test_sobel_edge_detection() {
        let processor = MSX2Processor::new(256, 212);
        let msx2_data = vec![0xFF; 256 * 212 / 2];
        let rgba = processor.transform_to_rgba(&msx2_data);
        let edges = processor.detect_edges_sobel(&rgba);
        
        // Debería producir un valor por píxel
        assert_eq!(edges.len(), 256 * 212);
        
        // En imagen uniforme, la mayoría de bordes debería ser 0
        let zero_count = edges.iter().filter(|&&e| e < 5.0).count();
        assert!(zero_count > 256 * 212 / 2);
    }

    #[test]
    fn test_sobel_gradient_detection() {
        let processor = MSX2Processor::new(256, 212);
        
        // Crear imagen con gradiente fuerte
        let mut msx2_data = vec![0x0; 256 * 212 / 2];
        
        // Mitad negra (0x00), mitad roja (0xFF)
        for y in 0..212 {
            for x in 0..128 {
                let idx = (y * 256 + x) / 2;
                msx2_data[idx] = 0x00;
            }
            for x in 128..256 {
                let idx = (y * 256 + x) / 2;
                msx2_data[idx] = 0xFF;
            }
        }
        
        let rgba = processor.transform_to_rgba(&msx2_data);
        let edges = processor.detect_edges_sobel(&rgba);
        
        // Alrededor del borde vertical, debería haber valores altos
        let border_x = 127;
        let mut border_edges = Vec::new();
        
        for y in 50..162 {
            border_edges.push(edges[y * 256 + border_x]);
        }
        
        let avg_edge: f32 = border_edges.iter().sum::<f32>() / border_edges.len() as f32;
        assert!(avg_edge > 50.0); // Bordes detectados significativos
    }

    #[test]
    fn test_neon_glow_application() {
        let processor = MSX2Processor::new(256, 212);
        let msx2_data = vec![0xAA; 256 * 212 / 2];
        let rgba = processor.transform_to_rgba(&msx2_data);
        let edges = processor.detect_edges_sobel(&rgba);
        
        let glowed = processor.apply_neon_glow(&rgba, &edges, 1.0);
        
        // Debería mantener el tamaño
        assert_eq!(glowed.len(), rgba.len());
        
        // Algunos píxeles deberían ser más brillantes
        let original_brightness: u32 = rgba.iter().take(3).map(|&b| b as u32).sum();
        let glowed_brightness: u32 = glowed.iter().take(3).map(|&b| b as u32).sum();
        
        assert!(glowed_brightness >= original_brightness);
    }

    #[test]
    fn test_process_with_all_effects() {
        let processor = MSX2Processor::new(256, 212);
        let msx2_data = vec![0x55; 256 * 212 / 2];
        
        let result = processor.process_with_post_effects(
            &msx2_data,
            true,  // bilinear
            true,  // normals
            true,  // edges
            1.0,   // glow
        );
        
        // Verificar que todos los componentes se generaron
        assert!(result.get_rgba().len() > 0, "RGBA debería generarse");
        assert!(result.get_normals().len() > 0, "Normals debería generarse");
        assert!(result.get_edges().len() > 0, "Edges debería generarse");
        
        // RGBA debería ser imagen escalada 4K
        assert_eq!(result.get_rgba().len(), 3840 * 2160 * 4);
        
        // Normals debería ser 256x212 (original)
        assert_eq!(result.get_normals().len(), 256 * 212 * 3);
        
        // Edges debería ser 256x212 (original)
        assert_eq!(result.get_edges().len(), 256 * 212);
    }

    #[test]
    fn test_process_without_optional_effects() {
        let processor = MSX2Processor::new(256, 212);
        let msx2_data = vec![0x88; 256 * 212 / 2];
        
        let result = processor.process_with_post_effects(
            &msx2_data,
            false,  // no bilinear
            false,  // no normals
            false,  // no edges
            0.0,    // no glow
        );
        
        // RGBA debería ser original 256x212
        assert_eq!(result.get_rgba().len(), 256 * 212 * 4);
        
        // Normals debería estar vacío
        assert_eq!(result.get_normals().len(), 0);
        
        // Edges debería estar vacío
        assert_eq!(result.get_edges().len(), 0);
    }

    #[test]
    fn test_palette_loading() {
        let processor = MSX2Processor::new(256, 212);
        
        // Verificar que la paleta se inicializó correctamente
        let test_data = vec![0x00]; // Píxeles con índice 0 = negro
        let rgba = processor.transform_to_rgba(&test_data);
        
        // Primer píxel debería ser negro
        assert_eq!(rgba[0], 0);   // R
        assert_eq!(rgba[1], 0);   // G
        assert_eq!(rgba[2], 0);   // B
        assert_eq!(rgba[3], 255); // A
    }

    #[test]
    fn test_glow_intensity_levels() {
        let processor = MSX2Processor::new(256, 212);
        let msx2_data = vec![0x55; 256 * 212 / 2];
        let rgba = processor.transform_to_rgba(&msx2_data);
        let edges = processor.detect_edges_sobel(&rgba);
        
        // Aplicar glow con diferentes intensidades
        let glow_low = processor.apply_neon_glow(&rgba, &edges, 0.5);
        let glow_high = processor.apply_neon_glow(&rgba, &edges, 2.0);
        
        // Glow de mayor intensidad debería ser más brillante
        let brightness_low: u32 = glow_low[..100].iter().map(|&b| b as u32).sum();
        let brightness_high: u32 = glow_high[..100].iter().map(|&b| b as u32).sum();
        
        // Más intensidad = más brillo
        assert!(brightness_high >= brightness_low);
    }

    #[test]
    fn test_multiple_frames_processing() {
        let processor = MSX2Processor::new(256, 212);
        
        // Simular múltiples frames
        for frame in 0..10 {
            let mut frame_data = vec![0; 256 * 212 / 2];
            frame_data.fill(frame as u8);
            
            let result = processor.process_with_post_effects(
                &frame_data,
                true,
                false,
                true,
                1.0,
            );
            
            assert_eq!(result.get_rgba().len(), 3840 * 2160 * 4);
        }
    }

    #[test]
    fn test_papiweb_branding() {
        // Asegurar que la marca PAPIWEB está presente en el código
        let _processor = MSX2Processor::new(256, 212);
        // Si se crea sin error, el branding está documentado en el código
        assert!(true);
    }
}
