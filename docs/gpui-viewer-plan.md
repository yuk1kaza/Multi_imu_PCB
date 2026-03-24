# `imu-viewer-gpui` 计划

## Summary

保留现有 [imu-viewer](d:\Programs\rust\PCB_test\tools\imu-viewer) 作为稳定版，不替换。新增一个并行实验性工具：

- `tools/imu-viewer-gpui`

目标：

- 复用当前协议、串口、录制、回放、模式切换能力
- 用 `gpui` 重做 UI，重点验证 3D 视图性能是否优于现有 `egui/eframe`
- 功能保持与 `imu-viewer` 尽量一致，但优先级明确：
  - 先跑通串口接入和 3D
  - 再补齐 2D、录制、导出、回放

## Key Changes

### 1. 新增独立工具，不替换现有 viewer

新增：

- `tools/imu-viewer-gpui`

并保持：

- `tools/imu-viewer`
  - 继续作为当前可用稳定版
- `tools/imu-viewer-gpui`
  - 作为高性能 3D 实验版

这样做的原因：

- 现有 viewer 已可用，不应在性能实验阶段被破坏
- `gpui` 的工程整合风险高于普通 UI 改造，适合并行推进

### 2. 复用协议与数据层

`imu-viewer-gpui` 不重新定义协议，直接复用：

- `imu-core`
  - `WireFrame`
  - `SampleFrame`
  - `OrientationFrame`
  - `ImuDescriptor`
  - `ViewMode`
  - `Quaternion`
  - `default_scale_profile_for_kind`

串口输入模式保持一致：

- `Auto`
- `Json`
- `Binary`

Windows 下继续沿用当前已验证的串口策略：

- JSON 优先支持 PowerShell `System.IO.Ports.SerialPort` fallback
- Binary 继续使用 `serialport` crate
- 若后续 `gpui` 工具验证表明需要，也可以把 PowerShell fallback 抽成 `tools` 共享模块

### 3. `gpui` viewer 功能范围

第一阶段必须具备：

- 串口连接
- 输入模式切换
- IMU 列表
- `Raw 6-Axis / Quaternion` 模式切换
- 3D 视图
- 基本状态栏

第二阶段补齐：

- 2D 曲线
- 错误列表
- 录制
- JSONL 导出
- CSV 导出
- 回放

原因：

- 本次立项的核心动机是 3D 性能
- 所以 3D 路径优先级高于其他 UI 完整性

### 4. 3D 视图设计

`imu-viewer-gpui` 的 3D 目标不是“和旧 viewer 一样”，而是专门为性能优化：

- 选中 IMU 的 3D 视图作为主区域
- 使用 quaternion 驱动姿态
- 原始模式下可继续显示近似姿态预览，但强调 quaternion 模式为主要 3D 路径
- 3D 视图需要：
  - 连续更新
  - 平滑姿态变化
  - 降低 CPU/UI 主线程负担
  - 尽量避免每帧整页重排

建议策略：

- 2D 与状态面板低频更新
- 3D 视图高频更新
- 若需要，3D 单独做节流/插值，而不是依赖 UI 整体刷新频率

### 5. 代码结构建议

`tools/imu-viewer-gpui` 建议模块：

- `main`
  - 启动 `gpui` app
- `serial`
  - 串口接入与 PowerShell fallback
- `state`
  - topology / sample / orientation / replay 状态
- `views/sidebar`
  - IMU 列表
- `views/dashboard`
  - 2D 数据视图
- `views/preview_3d`
  - 3D 姿态视图
- `views/status`
  - 连接状态、错误、录制状态
- `replay`
  - 录制与回放

## Public Interfaces / Shared Types

保持复用，不新增协议分叉：

- `imu-core::WireFrame`
- `imu-core::SampleFrame`
- `imu-core::OrientationFrame`
- `imu-core::ImuDescriptor`
- `imu-core::ViewMode`
- `imu-core::Quaternion`

新增仅限 `imu-viewer-gpui` 内部状态类型，例如：

- `ViewerState`
- `ConnectionState`
- `RenderMode`
- `SelectedImuState`

不修改设备侧协议字段。

## Test Plan

### 构建

- `cargo check -p imu-viewer-gpui`
- 现有：
  - `cargo check -p imu-viewer`
  - `cargo check -p esp32c3-board --target riscv32imc-unknown-none-elf`
  继续保持通过

### 功能

- JSON 模式下可连接 `COM15`
- 能收到 `Hello / Topology / Sample / Orientation / Heartbeat`
- `Quaternion` 模式下 3D 视图正常更新
- `Raw 6-Axis` 模式下 2D 视图正常更新
- 选中 IMU 切换不会卡死或错乱

### 性能

- 与现有 `imu-viewer` 对比主观帧率
- 重点观察：
  - 3D 旋转流畅度
  - 拖动/交互卡顿
  - CPU 占用变化
- 至少在 quaternion 模式下，3D 体验应明显优于现有 viewer 才有保留价值

## Assumptions

- `imu-viewer` 保留，不做替换。
- `imu-viewer-gpui` 是性能实验版，初期允许功能不完全对齐，但协议必须一致。
- 本次的成功标准不是“把所有功能再做一遍”，而是先证明 `gpui` 在 3D 视图上值得引入。
- 若 `gpui` 在 Windows 串口/窗口集成上成本过高，则保留实验分支，不回写主 viewer。
