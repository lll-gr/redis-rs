# Redis Commands 覆盖情况

## 为什么有些用 `Commands::method()` 有些用 `redis::cmd()`？

### 两种调用方式的区别

#### 1. **`Commands::method_name()` - 高级 API（推荐）**
```rust
Commands::scard(&mut self.inner, key)
```

**优点：**
- ✅ 类型安全 - 编译时检查参数类型
- ✅ 自动序列化/反序列化
- ✅ 返回类型明确
- ✅ 代码简洁易读
- ✅ IDE 自动补全支持

**适用场景：**
- redis-rs 已经提供了对应的方法
- 需要类型安全
- 标准的 Redis 命令

#### 2. **`redis::cmd("COMMAND")` - 低级 API（特殊情况）**
```rust
redis::cmd("PING")
    .query(&mut self.inner)
```

**优点：**
- ✅ 灵活 - 可以执行任何 Redis 命令
- ✅ 可以添加自定义参数
- ✅ 适合复杂或非标准命令

**适用场景：**
- redis-rs 没有提供对应的高级方法
- 需要自定义命令参数
- 特殊的 Redis 命令（如带可选参数的命令）

### 我们的实现策略

1. **优先使用 `Commands` trait** - 大部分命令都使用高级 API
2. **特殊情况使用 `redis::cmd()`**：
   - `PING` - 简单命令，不需要参数
   - `FLUSHDB` - 危险操作，明确使用低级 API
   - `ZADD` - 需要动态构建参数（可变数量的 score-member 对）
   - `ZCOUNT` - min/max 可以是特殊值（"-inf", "+inf"）
   - `SELECT` - Commands trait 没有 select() 方法（会与 Iterator::select 冲突）
   - `DBSIZE` - 简单的无参数命令
   - `INFO` - 可选参数的命令

## 已实现的命令

### String Commands (字符串命令) - 14/14
- ✅ `get` - 获取键值
- ✅ `set` - 设置键值
- ✅ `del` - 删除键
- ✅ `exists` - 检查键是否存在
- ✅ `expire` - 设置过期时间
- ✅ `ttl` - 获取剩余生存时间
- ✅ `incr` - 自增
- ✅ `decr` - 自减
- ✅ `mget` - 批量获取 ✨ NEW!
- ✅ `mset` - 批量设置 ✨ NEW!
- ✅ `setnx` - 仅当键不存在时设置 ✨ NEW!
- ✅ `setex` - 设置键值和过期时间 ✨ NEW!
- ✅ `append` - 追加字符串 ✨ NEW!
- ✅ `strlen` - 获取字符串长度 ✨ NEW!

### Hash Commands (哈希命令) - 7/7
- ✅ `hget` - 获取哈希字段值
- ✅ `hset` - 设置哈希字段值
- ✅ `hdel` - 删除哈希字段
- ✅ `hexists` - 检查哈希字段是否存在
- ✅ `hgetall` - 获取所有哈希字段和值
- ✅ `hkeys` - 获取所有哈希字段
- ✅ `hvals` - 获取所有哈希值

### List Commands (列表命令) - 9/9
- ✅ `lpush` - 从左侧推入
- ✅ `rpush` - 从右侧推入
- ✅ `lpop` - 从左侧弹出
- ✅ `rpop` - 从右侧弹出
- ✅ `llen` - 获取列表长度
- ✅ `lrange` - 获取列表范围
- ✅ `lindex` - 获取列表元素
- ✅ `lset` - 设置列表元素
- ✅ `lrem` - 删除列表元素 ✨ NEW!

### Set Commands (集合命令) - 5/5
- ✅ `sadd` - 添加集合成员
- ✅ `srem` - 删除集合成员
- ✅ `sismember` - 检查成员是否存在
- ✅ `smembers` - 获取所有成员
- ✅ `scard` - 获取集合大小

### Sorted Set Commands (有序集合命令) - 11/11
- ✅ `zadd` - 添加有序集合成员（使用 redis::cmd）
- ✅ `zrange` - 获取范围成员
- ✅ `zrem` - 删除成员
- ✅ `zscore` - 获取成员分数
- ✅ `zcard` - 获取有序集合大小
- ✅ `zcount` - 统计分数范围内的成员数（使用 redis::cmd）
- ✅ `zincrby` - 增加成员分数 ✨ NEW!
- ✅ `zrank` - 获取成员排名（升序）✨ NEW!
- ✅ `zrevrank` - 获取成员排名（降序）✨ NEW!
- ✅ `zrangebyscore` - 按分数范围获取成员 ✨ NEW!
- ✅ `zremrangebyrank` - 按排名范围删除成员 ✨ NEW!
- ✅ `zremrangebyscore` - 按分数范围删除成员 ✨ NEW!

### Key Commands (键命令) - 7/7
- ✅ `keys` - 获取匹配模式的键
- ✅ `ping` - 测试连接（使用 redis::cmd）
- ✅ `pttl` - 获取毫秒级剩余时间 ✨ NEW!
- ✅ `pexpire` - 设置毫秒级过期时间 ✨ NEW!
- ✅ `persist` - 移除过期时间 ✨ NEW!
- ✅ `key_type` - 获取键类型 ✨ NEW!
- ✅ `rename` - 重命名键 ✨ NEW!

### Database Commands (数据库命令) - 3/3
- ✅ `select` - 选择数据库（使用 redis::cmd）
- ✅ `dbsize` - 获取键数量（使用 redis::cmd）
- ✅ `info` - 获取服务器信息（使用 redis::cmd）
- ✅ `flushdb` - 清空当前数据库（使用 redis::cmd）

### JSON Commands (JSON 命令) - 15/15 ✨ NEW!
- ✅ `json_set` - 设置 JSON 值
- ✅ `json_get` - 获取 JSON 值
- ✅ `json_del` - 删除 JSON 值
- ✅ `json_type` - 获取 JSON 类型
- ✅ `json_arr_append` - 追加到 JSON 数组
- ✅ `json_arr_index` - 查找数组元素索引
- ✅ `json_arr_insert` - 插入到 JSON 数组
- ✅ `json_arr_len` - 获取数组长度
- ✅ `json_arr_pop` - 弹出数组元素
- ✅ `json_arr_trim` - 修剪数组
- ✅ `json_obj_keys` - 获取对象键
- ✅ `json_obj_len` - 获取对象键数量
- ✅ `json_str_append` - 追加字符串
- ✅ `json_str_len` - 获取字符串长度
- ✅ `json_num_incr_by` - 增加数字值

## 统计

### 总体覆盖
- **已实现**: 71 个命令（56 个基础命令 + 15 个 JSON 命令）
- **使用 Commands/JsonCommands trait**: 61 个命令 (86%)
- **使用 redis::cmd()**: 10 个命令 (14%)

### 使用 redis::cmd() 的命令及原因

| 命令 | 原因 |
|------|------|
| `ping` | 简单命令，历史原因 |
| `flushdb` | 危险操作，明确使用低级 API |
| `zadd` | 需要动态构建可变数量的参数 |
| `zcount` | min/max 可以是特殊字符串值 |
| `zrangebyscore` | min/max 可以是特殊字符串值 |
| `zremrangebyscore` | min/max 可以是特殊字符串值 |
| `select` | Commands trait 没有此方法（命名冲突） |
| `dbsize` | 简单的无参数命令 |
| `info` | 带可选参数的命令 |

## 常用但未实现的命令

### List Commands
- `ltrim` - 修剪列表
- `rpoplpush` - 从一个列表弹出并推入另一个列表
- `blpop` - 阻塞式左侧弹出
- `brpop` - 阻塞式右侧弹出

### Set Commands
- `sunion` - 集合并集
- `sinter` - 集合交集
- `sdiff` - 集合差集
- `spop` - 随机弹出成员
- `srandmember` - 随机获取成员

### Key Commands
- `scan` - 迭代键（推荐替代 KEYS）

### Transaction Commands
- `multi` - 开始事务
- `exec` - 执行事务
- `discard` - 取消事务
- `watch` - 监视键

### Pub/Sub Commands
- `publish` - 发布消息
- `subscribe` - 订阅频道
- `unsubscribe` - 取消订阅
- `psubscribe` - 模式订阅

## 建议

### 对于 HarmonyOS NAPI 绑定

当前实现已经覆盖了最常用的 Redis 命令（约 38 个），足以满足大部分应用场景：

1. **基本数据操作** - String, Hash, List, Set, Sorted Set
2. **键管理** - 过期、删除、查询
3. **数据库管理** - 选择数据库、获取信息

### 如果需要扩展

可以按以下优先级添加命令：

**优先级 1（高频使用）：**
- `mget`, `mset` - 批量操作提高性能
- `setnx`, `setex` - 常用的设置操作
- `lrem`, `ltrim` - 列表维护

**优先级 2（特定场景）：**
- `sunion`, `sinter`, `sdiff` - 集合运算
- `zrank`, `zrevrank` - 排行榜功能
- `blpop`, `brpop` - 队列功能

**优先级 3（高级功能）：**
- Transaction commands - 事务支持
- Pub/Sub commands - 发布订阅
- Lua scripting - 脚本支持

## 总结

我们的实现策略是：
1. **优先使用 `Commands` trait** - 提供类型安全和更好的开发体验
2. **特殊情况使用 `redis::cmd()`** - 处理复杂参数或特殊命令
3. **覆盖核心命令** - 38 个最常用的命令已实现
4. **保持一致性** - 所有命令都遵循相同的错误处理模式

这种混合方式既保证了代码质量，又提供了必要的灵活性。

