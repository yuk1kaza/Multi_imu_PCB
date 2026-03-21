# IMU Workspace

当前仓库正在从单 crate 测试程序重构为多 crate workspace，当前实现主线聚焦 `ESP32-C3 + 多 IMU 采集 + host viewer`。

## 当前结构

```text
.
├─ crates/
│  ├─ imu-core
│  ├─ imu-drivers
│  ├─ imu-firmware
│  └─ imu-platform-esp
├─ apps/
│  └─ esp32c3-board
├─ tools/
│  └─ imu-viewer
├─ contrib/
│  └─ bmi270/
└─ docs/
```

## 主要成员

- `imu-core`
  - 通用领域模型、驱动 trait、协议类型
- `imu-drivers`
  - 具体 IMU 驱动实现
- `imu-firmware`
  - 平台无关的设备侧运行时逻辑
- `imu-platform-esp`
  - `esp-hal` 平台适配，当前只落地 `esp32c3`
- `esp32c3-board`
  - 当前可构建的板级应用入口
- `imu-viewer`
  - host 侧桌面可视化壳

## 构建

Host 侧检查：

```bash
cargo check -p imu-core -p imu-drivers -p imu-firmware -p imu-viewer
```

ESP32-C3 固件检查：

```bash
cargo check -p esp32c3-board --target riscv32imc-unknown-none-elf
```

## 文档

- [docs/README.md](d:\Programs\rust\PCB_test\docs\README.md)
- [docs/architecture.md](d:\Programs\rust\PCB_test\docs\architecture.md)
- [docs/refactor-plan.md](d:\Programs\rust\PCB_test\docs\refactor-plan.md)
- [docs/hardware.md](d:\Programs\rust\PCB_test\docs\hardware.md)
- [docs/project-guide.md](d:\Programs\rust\PCB_test\docs\project-guide.md)
- [docs/troubleshooting.md](d:\Programs\rust\PCB_test\docs\troubleshooting.md)

## 第三方资源

第三方导入资源统一放在 `contrib/`。

当前已整理：

- [contrib/bmi270/README.md](d:\Programs\rust\PCB_test\contrib\bmi270\README.md)
- [contrib/bmi270/bmi270_upstream.c](d:\Programs\rust\PCB_test\contrib\bmi270\bmi270_upstream.c)
