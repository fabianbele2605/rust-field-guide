# 🦀 Rust Field Guide

Ruta de aprendizaje práctico de Rust: 18 proyectos progresivos, cada uno documentado con conceptos, código explicado y lecciones aprendidas — desde `File Explorer CLI` hasta un mini kernel con multitasking, paging, syscalls y filesystem, pasando por networking, sistemas distribuidos, herramientas CLI y machine learning.

📖 **Guía completa:** [`docs/guia.md`](docs/guia.md)

---

## 📊 Progreso

| Campo | Carpeta | Proyectos | Estado |
|-------|---------|-----------|--------|
| ⚙️ System Programming | [`file_explorer/`](file_explorer/), [`process_manager/`](process_manager/), [`mini_shell/`](mini_shell/) | 1, 2, 3 | ✅ |
| 🖥️ Kernel Development | [`kernel_hello_world/`](kernel_hello_world/) | 4, 5, 6, 6.1 (Paging), 6.2 (Syscalls), 6.3 (Filesystem) | ✅ |
| 🌐 Networking | [`networking/`](networking/) | 7 (TCP Client), 8 (HTTP Server), 9 (Reverse Proxy) | ✅ |
| ☁️ Distributed Systems | [`distributed-systems/`](distributed-systems/) | 10 (KV Server), 11 (Distributed KV Store), 12 (Mini Distributed DB) | ✅ |
| 📦 CLI / Developer Tools | [`cli-tools/`](cli-tools/) | 13 (rgrep), 14 (Mini Git), 15 (Mini Compiler/Interpreter) | ✅ |
| 🤖 AI / Machine Learning | [`ia-ml/`](ia-ml/) | 16 (Tensor), 17 (Neural Network), 18 (Inference Engine) | ✅ |
| 🔐 Cybersecurity | `cybersecurity/` *(próximo)* | 19-21 | 🔜 |
| 📱 Embedded / IoT | — | 22-24 | 🔜 |
| 🎮 Game Development | — | 25-27 | 🔜 |
| 🌐 WebAssembly | — | 28-30 | 🔜 |

Cada proyecto tiene su propia documentación educativa en [`docs/`](docs/) (ej: [`docs/proyecto_18_inference_engine.md`](docs/proyecto_18_inference_engine.md)), con explicación de conceptos Rust, conceptos del dominio, código final comentado, bugs reales encontrados durante el desarrollo y su corrección, y checklist de comprensión.

---

## 🗂️ Estructura del repositorio

```
Rust_ruta/
├── docs/                    # Documentación educativa de cada proyecto + guía maestra
├── file_explorer/           # Proyecto 1
├── process_manager/         # Proyecto 2
├── mini_shell/               # Proyecto 3
├── kernel_hello_world/       # Proyectos 4-6, 6.1-6.3 (kernel bare-metal x86-64, QEMU)
├── networking/                # Proyectos 7-9
├── distributed-systems/       # Proyectos 10-12
├── cli-tools/                 # Proyectos 13-15
└── ia-ml/                     # Proyectos 16-18
```

---

## 🧠 Metodología

Cada proyecto sigue el ciclo: **Instrucción → Escritura → Ejecución → Error → Corrección → Repetición → Proyecto más complejo** (ver [`docs/tutor.md`](docs/tutor.md)). El código se escribe a mano, se ejecuta, y los errores reales encontrados durante el desarrollo se documentan como parte del aprendizaje — no se ocultan.

---

## 🖥️ Requisitos para ejecutar los proyectos de Kernel Development

Los proyectos en `kernel_hello_world/` son bare-metal (`no_std`) y requieren:
- Rust nightly (`rustup toolchain install nightly`)
- QEMU (`qemu-system-x86_64`)
- `cargo-bootimage`

El resto de proyectos son binarios Rust estándar, ejecutables con `cargo run` desde su propia carpeta.
