#!/usr/bin/env python3
"""
🎮 MSX2 ROM Viewer - Servidor HTTP de Desarrollo
© 2026 PAPIWEB DESARROLLOS INFORMATICOS

Servidor HTTP simple con soporte WASM y CORS para desarrollo local.
"""

import http.server
import socketserver
import os
import sys
import signal
from pathlib import Path

# Configuration
PORT = 8080
SCRIPT_DIR = Path(__file__).parent.absolute()

STARTUP_MESSAGE = """
╔════════════════════════════════════════════════════════════╗
║          🎮 MSX2 ROM VIEWER - Servidor Iniciado           ║
╚════════════════════════════════════════════════════════════╝

📍 Dirección:        http://127.0.0.1:{port}
📂 Directorio:       {workdir}
🌐 URL Principal:    http://127.0.0.1:{port}/
🎨 URL de Demo:      http://127.0.0.1:{port}/demo.html

📋 Archivos disponibles:
   ✓ index.html     - Interfaz principal (requiere ROM real)
   ✓ demo.html      - Demostración interactiva
   ✓ pkg/           - Binarios WASM compilados

⌨️  Controles:
   🔵 Ctrl+C        - Detener servidor
   🌀 Recarga        - F5 en el navegador

🔐 Seguridad WASM:
   ✓ CORS habilitado
   ✓ Cross-Origin headers configurados
   ✓ WASM binaries servidos correctamente

💡 Notas:
   • Los datos NO se suben a servidor
   • Todo procesa localmente en tu navegador
   • WASM requiere HTTPS en producción
   • Para parar: Presiona Ctrl+C

═══════════════════════════════════════════════════════════════
"""


class MSX2RequestHandler(http.server.SimpleHTTPRequestHandler):
    """Handler personalizado para servir archivos con headers WASM."""

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(SCRIPT_DIR), **kwargs)

    def end_headers(self):
        """Añade headers necesarios para WASM y CORS."""
        # CORS
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        
        # WASM/COEP
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        
        # Cache
        self.send_header('Cache-Control', 'no-cache, no-store, must-revalidate')
        
        # MIME types
        if self.path.endswith('.wasm'):
            self.send_header('Content-Type', 'application/wasm')
        elif self.path.endswith('.js'):
            self.send_header('Content-Type', 'application/javascript')
        elif self.path.endswith('.html'):
            self.send_header('Content-Type', 'text/html; charset=utf-8')
        
        super().end_headers()

    def log_message(self, format, *args):
        """Formato mejorado de logs."""
        if "GET" in format or "POST" in format:
            # Log peticiones HTTP
            status = args[1] if len(args) > 1 else "?"
            path = args[0] if args else "?"
            
            # Icon según status code
            if status == "200":
                status_icon = "✓"
            elif status == "404":
                status_icon = "✗"
            else:
                status_icon = "ℹ"
            
            print(f"  {status_icon} [{status}] {path}")


def run_server():
    """Inicia el servidor HTTP."""
    os.chdir(SCRIPT_DIR)
    
    # Mostrar banner
    print(STARTUP_MESSAGE.format(
        port=PORT,
        workdir=os.getcwd()
    ))
    
    # Verificar archivos necesarios
    # El wasm generado por wasm-bindgen suele llamarse "msx2_processor_bg.wasm".
    required_files = ['index.html', 'demo.html', 'pkg/msx2_processor_bg.wasm']
    print("📋 Verificando archivos...")
    for filepath in required_files:
        full_path = os.path.join(SCRIPT_DIR, filepath)
        if os.path.exists(full_path):
            size_kb = os.path.getsize(full_path) / 1024
            print(f"   ✓ {filepath:<35} ({size_kb:>8.2f} KB)")
        else:
            print(f"   ✗ {filepath:<35} (NOT FOUND)")
    print()
    
    # Crear server
    handler = MSX2RequestHandler
    server_address = ("", PORT)
    
    # Permitir reutilizar puerto
    socketserver.TCPServer.allow_reuse_address = True
    
    try:
        with socketserver.TCPServer(server_address, handler) as httpd:
            # Handler para Ctrl+C
            def signal_handler(sig, frame):
                print("\n\n⏹️  Servidor detenido.")
                print("👋 ¡Hasta luego!")
                sys.exit(0)
            
            signal.signal(signal.SIGINT, signal_handler)
            
            # Servir
            httpd.serve_forever()
            
    except OSError as e:
        if "Address already in use" in str(e):
            print(f"❌ Puerto {PORT} ya está en uso.")
            print(f"   Alternativas:")
            print(f"   • Cierra la aplicación que lo use")
            print(f"   • O intenta: python3 server.py 8081")
            sys.exit(1)
        else:
            raise


if __name__ == '__main__':
    # Obtener puerto de argumentos si se proporciona
    if len(sys.argv) > 1:
        try:
            PORT = int(sys.argv[1])
        except ValueError:
            print(f"❌ Puerto inválido: {sys.argv[1]}")
            sys.exit(1)
    
    run_server()

