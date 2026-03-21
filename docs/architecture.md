# Architecture

## Workspace Layout

```text
.
├─ crates/
│  ├─ imu-core
│  ├─ imu-drivers
│  ├─ imu-firmware
│  └─ imu-platform-esp
├─ apps/
│  └─ esp32c3-board
└─ tools/
   └─ imu-viewer
```

## Layering

- `imu-core`
  - `no_std`
  - 定义 `ImuBus`、`ImuDriver`、`ImuId`、`WireFrame`、`DriverResources`
- `imu-drivers`
  - `no_std`
  - 放具体传感器寄存器驱动
- `imu-firmware`
  - `no_std`
  - 放 device topology、probe/runtime、transport 调度
- `imu-platform-esp`
  - `no_std`
  - 将 `imu-firmware` 落地到 `esp-hal`
- `esp32c3-board`
  - 最终板级应用入口与组装
- `imu-viewer`
  - `std`
  - 串口接收、协议解码、2D/3D 可视化

## Identity Model

- `system_id`
  - 一个设备源
- `bus_id`
  - 该设备源内部的一条逻辑总线
- `sensor_id`
  - 该设备源内部的一个 IMU 实例
- `ImuTargetId`
  - 驱动和总线层访问的逻辑目标设备

## Driver Model

- `ImuBus`
  - `apply_profile`
  - `read_reg`
  - `read_regs`
  - `write_reg`
  - `write_regs`
  - `delay_ms`
- `ImuDriver`
  - `kind`
  - `probe`
  - `reset`
  - `configure`
  - `read_raw`
  - `scale_profile`
  - `capabilities`

总线访问使用 `target`，不暴露独立 `ChipSelect` 抽象。

## BMI270

BMI270 被视为“资源型驱动”：

- 通用驱动生命周期保持一致
- 额外配置 blob 通过 `DriverResources` 注入
- 第三方源文件位于 `contrib/bmi270/`
- 当前 blob 生成逻辑位于 `crates/imu-platform-esp/build.rs`

## Protocol

正式链路：

- `postcard + COBS + CRC32`

调试链路：

- `NDJSON`

两种格式共享同一套 `WireFrame` 语义模型。
