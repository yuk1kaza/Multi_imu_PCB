# 传感器识别指南（历史路径参考）

本文档中的 `src/...` 路径来自单 crate 时代，仅作为历史诊断参考。当前主实现已迁移到 workspace 结构。

## Slot 2 & 4 识别分析

### Probe 数据
```
Slot 2/4:
  Mode 0: r00=0x3E r00d1=0x3E r75=0x00
  Mode 3: r00=0x01 r01=0x3E r0f=0x00 r75=0x00
```

### 分析

#### 观察 1: Mode 3 下 r00 = 0x01
- 在 SPI Mode 3 下，寄存器 0x00 返回 0x01
- 在 SPI Mode 0 下，寄存器 0x00 返回 0x3E
- **这表明传感器可能只在特定 SPI 模式下工作**

#### 观察 2: r01 = 0x3E (在 Mode 3 下)
- 寄存器 0x01 返回 0x3E (62 十进制)
- 这可能是：
  - 某个传感器的 WHO_AM_I 值在 0x01 地址
  - 或者是状态寄存器的值

#### 观察 3: r75 = 0x00
- ICM-42688 系列的 WHO_AM_I 应该在 0x75，期望值 0x47
- 但读到 0x00，说明：
  - 不是 ICM-42688-PC
  - 或者传感器未正确初始化

### 可能的传感器型号

基于 CHIP_ID = 0x3E 和行为特征：

#### 1. LSM6DSL / LSM6DSM (STMicroelectronics)
- WHO_AM_I (0x0F) = 0x6A / 0x6C
- 但我们读到 r0f=0x00，不匹配

#### 2. ISM330DLC (STMicroelectronics)
- WHO_AM_I (0x0F) = 0x6A
- 但我们读到 r0f=0x00，不匹配

#### 3. QMI8658C (QST 的另一个版本)
- 可能有不同的 WHO_AM_I 值
- 但标准 QMI8658A 的 WHO_AM_I 应该是 0x05

#### 4. 错误焊接的传感器
- 可能焊接了不在清单中的传感器
- 需要查看 PCB 上实际焊接的芯片型号

### 建议操作

#### 步骤 1: 物理检查
```bash
# 检查 Slot 2 和 Slot 4 上实际焊接的芯片
# 查看芯片表面的标记
# 常见标记格式：
# - ICM42688: "ICM-42688" 或 "42688"
# - QMI8658: "QMI8658" 或 "8658"
# - BMI270: "BMI270" 或 "270"
```

#### 步骤 2: 尝试不同的 WHO_AM_I 地址
可能的寄存器地址组合：
- 0x00 (最常见)
- 0x01
- 0x0F (STM 系列)
- 0x75 (Invensense 系列)

#### 步骤 3: 在不同 SPI 模式下测试
从 probe 数据看，Mode 3 下的行为与 Mode 0 不同，需要：
- 在 Mode 3 下尝试读取 WHO_AM_I
- 可能需要软复位后再读取

## 快速测试命令

如果 Slot 2/4 实际上是 QMI8658A，但使用 Mode 3：

在旧实现中需要修改 board 配置里的候选驱动顺序，例如 SLOT2_CANDIDATES 和 SLOT4_CANDIDATES：
```rust
static SLOT2_CANDIDATES: [SlotCandidate; 3] = [
    SlotCandidate {
        driver: &qmi8658::DRIVER,
        profiles: &PROFILES_MODE3,  // 先尝试 Mode 3
    },
    // ...
];
```

## 0x3E 可能的含义

### 作为二进制: 0011 1110
- Bit 5-1 可能是状态位
- Bit 0 可能是就绪位

### 作为十进制: 62
- 可能是某个计数器的值
- 可能是配置寄存器的默认值

### 需要的信息
要准确识别，需要：
1. PCB 上芯片的物理标记
2. 原理图（如果有）
3. 采购清单确认实际使用的芯片型号

## Slot 2 期望传感器: ICM-42688-PC

### 标准规格
- 制造商: Tokmas (托克马斯)
- WHO_AM_I 地址: 0x75
- WHO_AM_I 值: 0x47
- SPI 模式: Mode 0 或 Mode 3
- 立创商城: https://item.szlcsc.com/50854565.html

### 可能问题
如果实际焊接的不是 ICM-42688-PC：
1. 可能是其他兼容芯片
2. 可能是山寨版本（不同的 WHO_AM_I）
3. 可能是工程样片（寄存器映射不同）

## Slot 4 期望传感器: QMI8658A

### 标准规格
- 制造商: QST (上海矽睿)
- WHO_AM_I 地址: 0x00
- WHO_AM_I 值: 0x05
- SPI 模式: Mode 0-3 都应支持
- 立创商城: https://item.szlcsc.com/3544058.html

### 可能问题
Probe 显示 Mode 3 下 r00=0x01（而非 0x05）：
1. 可能是 QMI8658C 或其他版本
2. 可能需要特殊初始化序列
3. 可能传感器处于睡眠模式

## 下一步

1. **运行改进后的代码**，查看 BMI270 (Slot 3) 是否能成功初始化
2. **物理检查** Slot 2 和 Slot 4 的芯片标记
3. **提供芯片照片**，我可以帮助识别
4. **查看采购清单**，确认实际使用的芯片型号
