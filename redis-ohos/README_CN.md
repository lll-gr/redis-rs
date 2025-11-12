# Redis SDK for HarmonyOS (鸿蒙)

为 HarmonyOS 应用提供的高性能 Redis 客户端，基于 [redis-rs](https://github.com/redis-rs/redis-rs)。

> **注意：** 本项目是 redis-rs 工作空间的一部分。详见 [WORKSPACE_INTEGRATION.md](WORKSPACE_INTEGRATION.md)。

## 特性

- ✅ **完整的 Redis 命令支持**：字符串、哈希、列表、集合、有序集合操作
- ✅ **同步和异步 API**：支持同步和异步连接模式
- ✅ **连接管理**：异步连接管理器支持自动重连
- ✅ **类型安全**：自动生成 TypeScript 类型定义
- ✅ **HarmonyOS 日志**：集成 HiLog 原生日志
- ✅ **高性能**：基于久经考验的 redis-rs 库

## 快速开始

### 1. 安装构建工具

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 ohrs
cargo install ohrs

# 添加 HarmonyOS 目标平台
rustup target add aarch64-unknown-linux-ohos
```

### 2. 构建 SDK

```bash
cd redis-ohos

# 检查环境
make check

# 构建发布版本
make build-release
```

### 3. 安装到 HarmonyOS 项目

```bash
# 方式 A：使用 Make（推荐）
make install OHOS_PROJECT_PATH=/path/to/your/harmonyos/project

# 方式 B：手动复制
cp -r harmonyos-build/* /path/to/your/project/entry/libs/Redis_sdk/
```

### 4. 配置项目

编辑 `entry/oh-package.json5`：

```json5
{
  "dependencies": {
    "libredis_ohos.so": "file:./libs/Redis_sdk"
  }
}
```

### 5. 使用示例

```typescript
import { RedisClient, initLogging } from 'libredis_ohos.so';

// 初始化日志（可选）
initLogging(0xD001000, "MyApp");

// 创建客户端
const client = new RedisClient("redis://127.0.0.1:6379");

// 获取连接
const conn = client.getConnection();

// 测试连接
const pong = conn.ping();
console.log(pong); // "PONG"

// 设置和获取值
conn.set("hello", "world");
const value = conn.get("hello");
console.log(value); // "world"

// 哈希操作
conn.hset("user:1000", "name", "张三");
conn.hset("user:1000", "email", "zhangsan@example.com");
const name = conn.hget("user:1000", "name");

// 列表操作
conn.rpush("tasks", ["任务1", "任务2", "任务3"]);
const tasks = conn.lrange("tasks", 0, -1);

// 集合操作
conn.sadd("tags", ["redis", "harmonyos", "rust"]);
const tags = conn.smembers("tags");
```

### 异步使用

```typescript
async function asyncExample() {
  const client = new RedisClient("redis://127.0.0.1:6379");
  
  // 获取异步连接管理器（自动重连）
  const conn = await client.getAsyncConnection();
  
  // 所有操作都是异步的
  await conn.set("async:key", "value");
  const value = await conn.get("async:key");
  
  // 并发操作
  const [v1, v2, v3] = await Promise.all([
    conn.get("key1"),
    conn.get("key2"),
    conn.get("key3")
  ]);
}
```

## 构建命令

```bash
# 检查环境
make check

# 构建调试版本
make build-debug

# 构建发布版本
make build-release

# 构建所有架构（ARM64, ARMv7, x86_64）
make build-all

# 清理构建产物
make clean

# 创建分发包
make package VERSION=1.0.0

# 安装到项目
make install OHOS_PROJECT_PATH=/path/to/project
```

## 支持的 Redis 命令

### 字符串命令
- `set`, `get`, `mget`, `mset`, `append`, `strlen`

### 键命令
- `del`, `exists`, `expire`, `ttl`, `persist`, `rename`

### 数字命令
- `incr`, `incrBy`, `decr`, `decrBy`

### 哈希命令
- `hset`, `hget`, `hmset`, `hmget`, `hdel`, `hexists`, `hlen`, `hkeys`, `hvals`

### 列表命令
- `lpush`, `rpush`, `lpop`, `rpop`, `llen`, `lrange`, `lindex`, `lset`

### 集合命令
- `sadd`, `srem`, `sismember`, `smembers`, `scard`

### 有序集合命令
- `zadd`, `zrange`, `zrem`, `zscore`, `zcard`, `zcount`

### 数据库命令
- `select`, `dbsize`, `info`

### 工具命令
- `ping`, `keys`, `flushdb`

## 连接 URL 格式

```typescript
// 基本 TCP 连接
new RedisClient("redis://127.0.0.1:6379");

// 带密码
new RedisClient("redis://:password@127.0.0.1:6379");

// 带用户名和密码
new RedisClient("redis://username:password@127.0.0.1:6379");

// 选择数据库
new RedisClient("redis://127.0.0.1:6379/0");

// TLS 连接
new RedisClient("rediss://127.0.0.1:6379");
```

## 文档

- [README.md](README.md) - 完整英文文档
- [QUICKSTART.md](QUICKSTART.md) - 快速入门指南
- [DEVELOPMENT.md](DEVELOPMENT.md) - 开发者指南
- [WORKSPACE_INTEGRATION.md](WORKSPACE_INTEGRATION.md) - 工作空间集成说明
- [CHANGELOG.md](CHANGELOG.md) - 版本历史
- [COMPLETION_SUMMARY.md](COMPLETION_SUMMARY.md) - 完善工作总结

## 示例代码

查看 `examples/` 目录获取完整示例：
- `basic_usage.ets` - 基本同步操作
- `async_usage.ets` - 异步操作和性能测试

## 故障排除

### "找不到模块 'libredis_ohos.so'"

- 确认 SDK 在 `entry/libs/Redis_sdk/` 目录
- 检查 `oh-package.json5` 依赖配置正确
- 清理并重新构建 HarmonyOS 项目

### "连接被拒绝"

- 确保 Redis 服务器正在运行：`redis-server`
- 检查连接 URL 和端口
- 对于远程 Redis，确保设备/模拟器可以访问

### 构建错误

```bash
# 清理并重新构建
make clean
make build-release
```

## 架构支持

- **ARM64** (arm64-v8a) - 真机设备
- **ARMv7** (armeabi-v7a) - ARM 32位设备
- **x86_64** - 模拟器

## 性能

- 基于 Rust 的高性能实现
- 零拷贝操作
- 异步连接管理器支持并发操作
- 自动重连机制

## 许可证

BSD-3-Clause（与 redis-rs 相同）

## 贡献

本项目是 redis-rs 项目的一部分。如有问题和贡献，请参考主项目仓库。

## 相关链接

- [redis-rs](https://github.com/redis-rs/redis-rs) - Rust Redis 客户端
- [HarmonyOS 文档](https://developer.harmonyos.com/) - 鸿蒙开发文档
- [ohrs](https://github.com/ohos-rs/ohrs) - HarmonyOS Rust 构建工具

---

**祝您使用愉快！** 🎉

