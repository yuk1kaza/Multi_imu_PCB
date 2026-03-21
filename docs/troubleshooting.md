# 传感器检测故障排查指南（历史路径参考）

本文档中的 `src/...` 路径来自单 crate 时代，仅作为历史诊断参考。当前主实现已迁移到 workspace 结构。

## 如何解读诊断输出

运行 `cargo r` 后，程序会输出详细的 probe 信息和检测过程。

### 1. Probe 快照分析

格式：`probe{N}: m0[r00=XX r00d1=XX r75=XX] m3[r00=XX r00d1=XX r01=XX r0f=XX r75=XX]`

- `m0` = SPI Mode 0
- `m3` = SPI Mode 3
- `rXX` = 寄存器地址（十六进制）
- `rXXd1` = 带 1 个 dummy byte 的读取

#### 典型值解读：

| 读取值 | 含义 | 可能原因 |
|--------|------|----------|
| `0xFF` | 总线无响应 | 传感器未焊接、电源问题、CS 引脚错误 |
| `0x00` | 总线被拉低 | 传感器故障、短路 |
| 固定值（如 `0x3E`） | 可能读到其他寄存器 | SPI 时序问题、寄存器地址错误 |
| 正确的 CHIP_ID | 传感器正常响应 | ✓ 正常 |

### 2. 传感器期望的 CHIP_ID

| 传感器 | 寄存器地址 | 期望值 | Dummy Bytes | SPI Mode |
|--------|-----------|--------|-------------|----------|
| ICM-42688-HXY | 0x01 | 0x6A | 0 | Mode 0/3 |
| ICM-42688-PC | 0x75 | 0x47 | 0 | Mode 0 |
| BMI270 | 0x00 | 0x24 | 1 | Mode 3 |
| QMI8658A | 0x00 | 0x05 | 0 | Mode 0 |
| SC7U22/LSM6 | 0x0F/0x01 | 0x6A | 0 | Mode 3 |

### 3. 检测日志分析

格式：`Slot {N}: Trying {Driver} with {Profile} -> {Result}`

- `PROBE OK` = 传感器识别成功
- `probe failed` = WHO_AM_I 寄存器不匹配
- `init failed` = 传感器初始化失败

## 常见问题及解决方案

### 问题 1：所有读取返回 0xFF

**症状：**
```
probe2: m0[r00=Some(ff) r00d1=Some(ff) r75=Some(ff)] m3[...]
```

**可能原因：**
1. 传感器未焊接到 PCB 上
2. CS 引脚连接错误
3. 电源供电问题（5V 或 3.3V）
4. SPI 总线连接问题（MISO 引脚）

**排查步骤：**
1. 检查传感器是否已焊接到对应的插槽
2. 用万用表测量 CS 引脚在读取时是否有电平变化（应该是 HIGH→LOW→HIGH）
3. 检查 VCC 和 GND 是否正常供电
4. 检查 MISO 引脚是否正确连接到 GPIO2

### 问题 2：读取到非预期的固定值

**症状：**
```
probe2: m0[r00=Some(3e) r00d1=Some(3e) r75=Some(00)]
```

**可能原因：**
1. SPI 时序不匹配（频率过快、Mode 不对）
2. 传感器需要更长的上电延迟
3. 传感器处于未知状态，需要软复位

**排查步骤：**
1. 降低当前 board/device profile 中的 SPI 频率
2. 增加当前 board firmware 启动流程中的上电延迟
3. 检查传感器数据手册，确认读取协议

### 问题 3：Probe OK 但 Init Failed

**症状：**
```
Slot 2: Trying ICM-42688-compatible with mode0@1mhz -> PROBE OK
Slot 2: ICM-42688-compatible init failed: ConfigError
```

**可能原因：**
1. 传感器配置寄存器写入失败
2. 传感器内部状态异常
3. 电源供电不稳定

**排查步骤：**
1. 增加初始化过程中的延迟
2. 在驱动的 `init()` 函数开始处添加软复位
3. 检查电源纹波，使用示波器观察电源稳定性

### 问题 4：检测成功但数据全是 NA

**症状：**
```
1:0.069,0.564,0.905,-6.59,1.71,0.61
2:NA,NA,NA,NA,NA,NA
```

**可能原因：**
1. 数据就绪检查过于严格
2. 传感器采样率配置错误
3. 读取间隔太短，数据未更新

**排查步骤：**
1. 在对应驱动的 `read_raw()` 函数中注释掉数据就绪检查
2. 增加当前 device profile 中的采样间隔配置
3. 检查传感器配置，确认输出数据率（ODR）设置正确

## 硬件检查清单

### 电源检查
- [ ] 5V 或 3.3V 供电正常
- [ ] 电流供应充足（建议 >500mA）
- [ ] 去耦电容已焊接

### SPI 总线检查
- [ ] MOSI (GPIO7) 连接到所有传感器的 SDI/SDA 引脚
- [ ] MISO (GPIO2) 连接到所有传感器的 SDO 引脚
- [ ] SCK (GPIO6) 连接到所有传感器的 SCL/SCK 引脚
- [ ] 所有 SPI 信号线没有短路或断路

### CS 引脚检查
- [ ] ICM1 (GPIO1) 连接到 ICM-42688-HXY 的 CS 引脚
- [ ] ICM2 (GPIO3) 连接到 ICM-42688-PC 的 CS 引脚
- [ ] BM (GPIO4) 连接到 BMI270 的 CS 引脚
- [ ] QMI (GPIO5) 连接到 QMI8658A 的 CS 引脚
- [ ] SC (GPIO10) 连接到 SC7U22 的 CS 引脚

### 传感器检查
- [ ] 所有传感器已正确焊接（无虚焊）
- [ ] 传感器朝向正确（检查 Pin 1 标记）
- [ ] 没有焊接桥接（相邻引脚没有短路）

## 调整 SPI 频率

如果怀疑是 SPI 频率问题，可以尝试降低频率：

编辑当前 board/device profile 配置：
```rust
// 从 1MHz 降低到 500kHz
pub const SPI_FREQ_MHZ: u32 = 0.5;

// 或者更保守的 100kHz
pub const SPI_FREQ_MHZ: u32 = 0.1;
```

## 增加调试输出

如果需要更详细的调试信息，可以修改 `.cargo/config.toml`:
```toml
[env]
DEFMT_LOG="debug"  # 从 "off" 改为 "debug" 或 "trace"
```

## 联系支持

如果以上方法都无法解决问题，请记录：
1. 完整的 probe 输出
2. 完整的检测日志
3. 硬件版本和布线图
4. 已尝试的排查步骤

然后在项目 Issues 中提问或联系硬件提供商。
