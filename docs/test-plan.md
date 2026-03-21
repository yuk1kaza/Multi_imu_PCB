# Test Plan

## Scope

当前测试面向以下主线：

- `apps/esp32c3-board`
- `tools/imu-viewer`
- `Json` transport
- `Binary` transport

当前已知例外：

- `slot3 / BMI270` 仍未完成，不阻塞其余 4 路 IMU 主线验收

## Build Checks

### Host-side

```bash
cargo check -p imu-core -p imu-drivers -p imu-firmware -p imu-viewer
```

预期结果：

- 命令退出码为 `0`
- 无编译错误

### Device-side JSON

```bash
cargo check -p esp32c3-board --target riscv32imc-unknown-none-elf
```

预期结果：

- 命令退出码为 `0`
- 默认 `json-transport` 模式可编译

### Device-side Binary

```bash
cargo check -p esp32c3-board --no-default-features --features binary-transport --target riscv32imc-unknown-none-elf
```

预期结果：

- 命令退出码为 `0`
- `binary-transport` 模式可编译

## JSON Hardware Validation

### Flash

```bash
espflash flash --chip esp32c3 --port COM15 target\\riscv32imc-unknown-none-elf\\debug\\esp32c3-board
```

预期结果：

- 烧录成功
- 芯片识别正常

### Serial Output

观察 `COM15` 上的输出，预期应出现：

- `Hello`
- `ProbeResult`
- `Topology`
- `Sample`
- `Heartbeat`

当前实测结论：

- `slot1` 正常
- `slot2` 正常
- `slot4` 已修复并识别为 `Qmi8658A`
- `slot5` 正常
- `slot3/BMI270` 未识别
- `active_imus = 4`

### Acceptance

JSON 模式通过的标准：

- viewer 可读取 topology
- 至少 4 路 IMU 稳定输出 sample
- `slot4` 识别为 `Qmi8658A`
- 输出为结构化 JSON 帧

## Viewer Validation

### Launch

```bash
cargo run -p imu-viewer
```

预期结果：

- GUI 正常启动
- 可选择串口和输入模式

### Runtime Features

应验证：

- `Auto / Json / Binary` 模式切换
- topology 展示
- accel / gyro 双图
- 3D 姿态线框预览
- 错误列表
- 录制
- JSONL 导出
- CSV 导出
- 回放

## Binary Validation

### Build

```bash
cargo build -p esp32c3-board --no-default-features --features binary-transport --target riscv32imc-unknown-none-elf
```

### Flash

```bash
espflash flash --chip esp32c3 --port COM15 target\\riscv32imc-unknown-none-elf\\debug\\esp32c3-board
```

### Viewer

- 模式选 `Auto` 或 `Binary`
- 验证是否能正确识别二进制帧

当前状态：

- Binary 构建通过
- Viewer 端 `Binary/Auto` 解析已实现
- 真机 Binary 串口联调仍待稳定验证

## Known Issues

- `slot3 / BMI270`
  - 当前仍未识别
  - 不阻塞其余 4 路 IMU 主线验收
- 串口 `COM15`
  - 偶发被占用，导致烧录失败
  - 测试前需确保没有其它进程占用串口

## Completion Criteria

本轮可视为主线完成的标准：

- workspace 全部可构建
- JSON 模式下 4 路 IMU 稳定工作
- viewer 可完成查看、录制、导出、回放
- Binary 模式可构建，且具备 viewer 解析能力
