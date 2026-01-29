<p align="center">
  <img alt="1024Chain" src="./assets/1024logo-transparent.png" width="120" />
</p>

<h1 align="center">1024Chain</h1>

<p align="center">
  <strong>基于 Solana 代码库 fork 的高性能 Rust 原生 Layer-1 区块链，<br/>
  从底层协议开始为高频金融场景深度优化。</strong>
</p>

<p align="center">
  <a href="./README.md">English</a> •
  <a href="https://testnet-scan.1024chain.com">区块浏览器</a> •
  <a href="https://docs.1024chain.com">文档</a> •
  <a href="https://discord.gg/1024chain">Discord</a>
</p>

---

## 概述

1024Chain 是一条独立的 Layer-1 区块链，源自 Solana 验证器代码库。
相比 Solana 主网，1024Chain 更加**轻量、快速、专注于执行层**，
其架构和参数选择专门针对**高吞吐量交易、低延迟撮合和快速结算**进行了优化。

> **从 Layer-1 为金融而生：**  
> 不同于在通用区块链上改造金融应用，
> 1024Chain 从协议层就为**亚秒级订单簿撮合**和**亚秒级结算** 而设计。

## 设计动机

Solana 开创了高性能 Rust 区块链架构的先河。
然而，随着 Solana 主网发展成为一个通用的大规模生态系统，
其验证器设计不可避免地为**广泛兼容性、长期稳定性和多样化工作负载**进行了优化。

1024Chain 做出了不同的权衡取舍。

我们不追求服务所有场景，而是专注于一个狭窄但要求极高的领域：

> **高频金融执行**

这包括链上订单簿、衍生品、套利、做市商以及对延迟敏感的金融基础设施。

---

## 核心优势

### 为金融场景极致优化的性能

| 能力 | 描述 |
|------|------|
| **亚秒级撮合** | 订单簿撮合引擎实现亚秒级确认 |
| **亚秒级结算** | 交易结算端到端在 1 秒内完成 |
| **高吞吐量** | 针对交易工作负载优化，持续高 TPS |
| **确定性执行** | 可预测的交易排序，确保公平交易 |

### 协议层深度优化

- **更快的出块速度**  
  更高的 Tick 频率带来更高的 TPS 和更短的确认时间。

- **优化的调度器**  
  交易调度针对交易模式和账户访问局部性进行优先级排序。

- **更低的确认延迟**  
  为时间敏感的金融操作提供更快的最终性。

- **并行执行**  
  充分利用账户隔离进行激进的并行化，最大化吞吐量。

### 开发者体验

- **100% SVM 兼容** — 所有 Solana 程序、工具和 SDK 开箱即用
- **熟悉的工具链** — 使用 Anchor、Solana CLI、web3.js 和现有的 Solana 开发工作流
- **经过实战检验的运行时** — 基于成熟的 Agave/Solana 验证器代码库构建

### 运维优势

- **更轻量的验证器** — 降低复杂度，更易部署和维护
- **更低的基础设施成本** — 优化资源使用，节点运营更具成本效益
- **简化的架构** — 专注的功能集意味着更少的组件和故障点

---

## 生态系统

1024Chain 为一系列高性能金融应用提供底层支撑，
所有应用都受益于协议层的速度和可靠性优化。

### 1024EX — 专业级交易所

原生构建于 1024Chain 的新一代去中心化交易所，
提供 CEX 级别的性能体验，同时保持完全的链上透明度。

- **永续合约 (Perpetual)** — 支持高杠杆的永续期货交易
- **现货交易 (Spot)** — 即时现货执行，深度流动性
- **期权 (Options)** — 链上期权交易，实时定价与结算
- **毫秒级撮合** — 订单簿撮合引擎极速响应
- **亚秒级结算** — 结算最终性 < 1 秒
- **专业级 API** — 支持做市商和算法交易者

### 预测市场 (Prediction Market)

基于 1024Chain 低延迟基础设施的去中心化预测市场。

- **实时赔率更新** — 新信息在毫秒内反映到价格
- **即时结算** — 事件结果确定后立即链上结算
- **支持高频交易** — 做市商可高效提供流动性
- **透明可验证** — 所有市场操作链上记录

---

## 为什么选择 1024Chain?

| 如果你需要... | 1024Chain 提供 |
|---------------|----------------|
| 快速订单撮合 | 毫秒级撮合引擎 |
| 快速交易结算 | 亚秒级链上最终性 |
| 高交易吞吐量 | 针对交易负载优化 |
| Solana 兼容性 | 100% SVM 兼容 |
| 专业交易基础设施 | 为机构级用例而建 |
| 更低运营成本 | 更轻量、更高效的验证器 |

## 核心设计原则

- **执行优先于通用性** — 优先保证确定性执行和吞吐量，而非最大化灵活性
- **低延迟优先** — 优化区块生产、调度和确认路径，实现快速反馈循环
- **高度并行化** — 积极利用并行执行和账户隔离来最大化吞吐量
- **Rust 原生、系统优先** — 将验证器视为实时分布式系统，而非应用框架

## 1024Chain 是什么（以及不是什么）

**1024Chain 是：**
- 从 Layer-1 为金融优化的 Solana 衍生 Rust 区块链
- 高性能金融执行层
- 为专业交易基础设施设计的系统
- 100% SVM (Solana 虚拟机) 兼容
- 1024EX 及其他金融应用的底层基础

**1024Chain 不是：**
- 服务所有应用的通用智能合约平台
- Solana 官方网络或升级版本
- Solana 主网的替代品

---

## 网络信息

### 测试网

```yaml
RPC 端点:       https://testnet-rpc.1024chain.com/rpc/
WebSocket:      wss://testnet-rpc.1024chain.com/ws/
区块浏览器:     https://testnet-scan.1024chain.com/
```

---

## 快速开始

### 安装 Rust

```bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup component add rustfmt
```

### 安装依赖

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install libssl-dev libudev-dev pkg-config zlib1g-dev \
    llvm clang cmake make libprotobuf-dev protobuf-compiler libclang-dev
```

**Fedora:**
```bash
sudo dnf install openssl-devel systemd-devel pkg-config zlib-devel \
    llvm clang cmake make protobuf-devel protobuf-compiler perl-core libclang-dev
```

### 克隆并构建

```bash
git clone https://github.com/1024-chain/1024Chain.git
cd 1024Chain
./cargo build --release
```

### 运行验证器

```bash
git clone https://github.com/1024-chain/1024chain-config.git
cd 1024chain-config
make bootstrap  # 启动创世验证器
```

查看 [1024chain-config](https://github.com/1024-chain/1024chain-config) 了解更多部署选项。

### 测试

```bash
./cargo test                    # 运行测试套件
cargo +nightly bench            # 运行基准测试
```

---

## 项目结构

```
1024Chain/
├── core/               # 验证器核心逻辑
├── runtime/            # 交易执行运行时
├── svm/                # Solana 虚拟机
├── programs/           # 原生程序 (System, Stake, Vote 等)
├── program-binaries/   # 预编译的 SPL 程序
├── rpc/                # JSON-RPC 服务器
├── gossip/             # 集群通信
├── ledger/             # 区块存储
├── validator/          # 验证器二进制
└── cli/                # 命令行工具
```

---

## 文档与资源

- **开发者文档:** https://docs.1024chain.com
- **API 参考:** https://docs.1024chain.com/api
- **验证器指南:** https://docs.1024chain.com/validators
- **1024EX 文档:** https://docs.1024ex.com

## 贡献与开源社区

我们欢迎贡献！请在提交 PR 前阅读 [CONTRIBUTING.md](./CONTRIBUTING.md)。

### SVM 100% 兼容承诺

1024Chain 在追求更高性能的同时，严格保持与 SVM 的完全兼容。

**允许优化的部分：**
- 网络层、调度器、执行并行度
- 存储层、共识参数

**不动的核心语义：**
- 交易/消息格式、Address Lookup Table 行为
- Syscalls / Loader / ABI / Compute 语义
- Account ownership / Rent 语义

---

## 愿景

> Solana 证明了高性能区块链是可能的。
> 1024Chain 从协议层开始，专为**高速金融场景**深度优化。

我们专注于**高频金融执行**——链上订单簿、衍生品、套利、做市商，
以及所有对延迟敏感的金融基础设施。

金融的未来在链上，而 1024Chain 就是为此而生的基础设施。

---

## 链接

**1024Chain:**
- **官网:** https://1024chain.com
- **文档:** https://docs.1024chain.com
- **区块浏览器:** https://testnet-scan.1024chain.com
- **Discord:** https://discord.gg/1024chain
- **Twitter:** https://twitter.com/1024chain

**1024EX:**
- **官网:** https://1024ex.com
- **文档:** https://docs.1024ex.com
- **Discord:** https://discord.gg/1024ex
- **Twitter:** https://x.com/1024EX

---

## 许可

1024Chain 基于 **Apache License 2.0** 开源。详见 [LICENSE](./LICENSE)。

Solana 和 Solana 标志是 Solana Foundation 的商标。
1024Chain 是一个独立项目，**与 Solana Foundation 或其关联方无关**。

---

**维护者:** 1024 Foundation
