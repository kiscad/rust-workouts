## Test on Windows

### 70K Files + 70K Symlinks

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run -r` | 8.870 ± 0.122 | 8.736 | 9.086 | 1.00 |

### 70K Files + 70K Copies

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run -r` | 8.094 ± 0.774 | 7.686 | 10.261 | 1.00 |

## Test on Ubuntu

### 70K Files + 70K Symlinks

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run -r` | 682.9 ± 4.1 | 679.3 | 691.9 | 1.00 |

### 70K Files + 70K Copies

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run -r` | 671.7 ± 4.1 | 665.9 | 678.0 | 1.00 |
