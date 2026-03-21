## BMI270 Upstream Resource

This directory stores third-party upstream resources used to generate the
BMI270 configuration blob consumed by the firmware.

- `bmi270_upstream.c`
  - upstream C source containing `bmi270_config_file[]`
  - parsed at build time by `crates/imu-platform-esp/build.rs`
