# Redis Stream Commands for HarmonyOS

本文档介绍了 redis-ohos SDK 中新增的 Redis Stream 相关命令和通用 CMD 接口。

## 目录

- [Stream 基础操作](#stream-基础操作)
- [Stream 读取操作](#stream-读取操作)
- [Consumer Group 操作](#consumer-group-操作)
- [Stream 信息查询](#stream-信息查询)
- [通用 CMD 接口](#通用-cmd-接口)

---

## Stream 基础操作

### XADD - 添加消息到流

向 Stream 中添加一条消息。

```typescript
const id = conn.xadd(key: string, id: string, items: string[][]): string | null

// 示例：自动生成 ID
const id = conn.xadd("mystream", "*", [
  ["sensor", "temperature"],
  ["value", "23.5"],
  ["unit", "celsius"]
]);

// 示例：指定 ID
const id = conn.xadd("mystream", "1234567890-0", [
  ["field1", "value1"],
  ["field2", "value2"]
]);
```

**参数：**
- `key`: Stream 键名
- `id`: 消息 ID（使用 "*" 自动生成）
- `items`: 字段-值对数组

**返回：** 消息 ID 或 null

---

### XLEN - 获取流长度

获取 Stream 中的消息数量。

```typescript
const length = conn.xlen(key: string): number

// 示例
const length = conn.xlen("mystream");
console.log(`Stream has ${length} messages`);
```

---

### XDEL - 删除消息

从 Stream 中删除指定的消息。

```typescript
const deleted = conn.xdel(key: string, ids: string[]): number

// 示例
const deleted = conn.xdel("mystream", ["1234567890-0", "1234567891-0"]);
console.log(`Deleted ${deleted} messages`);
```

---

### XTRIM - 修剪流

将 Stream 修剪到指定的最大长度。

```typescript
const trimmed = conn.xtrim(key: string, maxlen: number, approximate: boolean): number

// 示例：精确修剪到 1000 条消息
const trimmed = conn.xtrim("mystream", 1000, false);

// 示例：近似修剪（更高效）
const trimmed = conn.xtrim("mystream", 1000, true);
```

**参数：**
- `approximate`: true 使用近似修剪 (~)，false 使用精确修剪 (=)

---

## Stream 读取操作

### XRANGE - 范围读取

读取指定范围内的消息。

```typescript
const messages = conn.xrange(
  key: string,
  start: string,
  end: string,
  count: number | null
): string

// 示例：读取所有消息
const all = conn.xrange("mystream", "-", "+", null);

// 示例：读取前 10 条消息
const first10 = conn.xrange("mystream", "-", "+", 10);

// 示例：读取特定范围
const range = conn.xrange("mystream", "1234567890-0", "1234567900-0", null);
```

**参数：**
- `start`: 起始 ID（"-" 表示第一条消息）
- `end`: 结束 ID（"+" 表示最后一条消息）
- `count`: 可选的最大消息数量

**返回：** JSON 字符串，包含消息数组

**返回格式：**
```json
[
  {
    "id": "1234567890-0",
    "fields": {
      "sensor": "temperature",
      "value": "23.5"
    }
  }
]
```

---

### XREVRANGE - 反向范围读取

以相反顺序读取消息。

```typescript
const messages = conn.xrevrange(
  key: string,
  end: string,
  start: string,
  count: number | null
): string

// 示例：读取最后 10 条消息
const last10 = conn.xrevrange("mystream", "+", "-", 10);
```

---

### XREAD - 读取多个流

从一个或多个 Stream 中读取消息。

```typescript
const messages = conn.xread(
  keys: string[],
  ids: string[],
  count: number | null,
  block: number | null
): string | null

// 示例：读取新消息
const messages = conn.xread(
  ["stream1", "stream2"],
  ["0-0", "0-0"],
  null,
  null
);

// 示例：阻塞等待新消息（1 秒）
const messages = conn.xread(
  ["stream1"],
  ["$"],
  10,
  1000
);
```

**参数：**
- `keys`: Stream 键名数组
- `ids`: 起始 ID 数组（"$" 表示只读取新消息）
- `count`: 可选的每个流的最大消息数
- `block`: 可选的阻塞时间（毫秒，0 表示无限期）

**返回格式：**
```json
[
  {
    "stream": "stream1",
    "messages": [
      {
        "id": "1234567890-0",
        "fields": { "field1": "value1" }
      }
    ]
  }
]
```

---

## Consumer Group 操作

### XGROUP CREATE - 创建消费者组

创建一个消费者组。

```typescript
conn.xgroupCreate(
  key: string,
  group: string,
  id: string,
  mkstream: boolean
): void

// 示例：从头开始创建组
conn.xgroupCreate("mystream", "mygroup", "0", false);

// 示例：只处理新消息，如果流不存在则创建
conn.xgroupCreate("mystream", "mygroup", "$", true);
```

**参数：**
- `id`: 起始 ID（"0" 从头开始，"$" 只处理新消息）
- `mkstream`: 如果为 true，当流不存在时创建它

---

### XGROUP DESTROY - 销毁消费者组

```typescript
const destroyed = conn.xgroupDestroy(key: string, group: string): boolean

// 示例
const destroyed = conn.xgroupDestroy("mystream", "mygroup");
```

---

### XGROUP SETID - 设置组的最后交付 ID

```typescript
conn.xgroupSetid(key: string, group: string, id: string): void

// 示例：重置到流的开始
conn.xgroupSetid("mystream", "mygroup", "0");
```

---

### XGROUP DELCONSUMER - 删除消费者

```typescript
const pending = conn.xgroupDelconsumer(
  key: string,
  group: string,
  consumer: string
): number

// 示例
const pending = conn.xgroupDelconsumer("mystream", "mygroup", "consumer1");
console.log(`Consumer had ${pending} pending messages`);
```

---

### XREADGROUP - 作为消费者组成员读取

```typescript
const messages = conn.xreadgroup(
  group: string,
  consumer: string,
  keys: string[],
  ids: string[],
  count: number | null,
  block: number | null
): string | null

// 示例：读取未交付的消息
const messages = conn.xreadgroup(
  "mygroup",
  "consumer1",
  ["stream1"],
  [">"],
  10,
  null
);

// 示例：阻塞等待新消息
const messages = conn.xreadgroup(
  "mygroup",
  "consumer1",
  ["stream1"],
  [">"],
  10,
  5000
);
```

**参数：**
- `ids`: 使用 ">" 读取未交付的消息

---

### XACK - 确认消息

确认已处理的消息。

```typescript
const acked = conn.xack(key: string, group: string, ids: string[]): number

// 示例
const acked = conn.xack("mystream", "mygroup", ["1234567890-0", "1234567891-0"]);
console.log(`Acknowledged ${acked} messages`);
```

---

### XPENDING - 查询待处理消息

获取待处理消息的信息。

```typescript
const pending = conn.xpending(
  key: string,
  group: string,
  start: string | null,
  end: string | null,
  count: number | null,
  consumer: string | null
): string

// 示例：获取摘要信息
const summary = conn.xpending("mystream", "mygroup", null, null, null, null);

// 示例：获取详细信息
const detailed = conn.xpending("mystream", "mygroup", "-", "+", 10, null);

// 示例：获取特定消费者的待处理消息
const consumerPending = conn.xpending("mystream", "mygroup", "-", "+", 10, "consumer1");
```

---

### XCLAIM - 认领消息

从其他消费者认领待处理的消息。

```typescript
const claimed = conn.xclaim(
  key: string,
  group: string,
  consumer: string,
  minIdleTime: number,
  ids: string[]
): string

// 示例：认领空闲超过 60 秒的消息
const claimed = conn.xclaim("mystream", "mygroup", "consumer2", 60000, ["1234567890-0"]);
```

---

## Stream 信息查询

### XINFO STREAM - 获取流信息

```typescript
const info = conn.xinfoStream(key: string): string

// 示例
const info = conn.xinfoStream("mystream");
const data = JSON.parse(info);
console.log(`Stream length: ${data.length}`);
```

---

### XINFO GROUPS - 获取消费者组信息

```typescript
const groups = conn.xinfoGroups(key: string): string

// 示例
const groups = conn.xinfoGroups("mystream");
const data = JSON.parse(groups);
```

---

### XINFO CONSUMERS - 获取消费者信息

```typescript
const consumers = conn.xinfoConsumers(key: string, group: string): string

// 示例
const consumers = conn.xinfoConsumers("mystream", "mygroup");
const data = JSON.parse(consumers);
```

---

## 通用 CMD 接口

### cmd() - 执行任意 Redis 命令

通用接口，可以执行任何 Redis 命令。

```typescript
const result = conn.cmd(command: string, args: string[]): string

// 示例：基本命令
const value = conn.cmd("GET", ["mykey"]);
const ok = conn.cmd("SET", ["mykey", "myvalue"]);

// 示例：复杂命令
const hash = conn.cmd("HGETALL", ["myhash"]);
const members = conn.cmd("SMEMBERS", ["myset"]);

// 示例：Stream 命令
const id = conn.cmd("XADD", ["mystream", "*", "field1", "value1", "field2", "value2"]);
const messages = conn.cmd("XRANGE", ["mystream", "-", "+", "COUNT", "10"]);

// 示例：带多个参数的命令
const zadd = conn.cmd("ZADD", ["leaderboard", "100", "player1", "200", "player2"]);
```

**特点：**
- 自动将响应转换为 JSON 格式
- 支持所有 Redis 命令
- 适用于尚未封装的命令
- 适用于动态命令执行

---

### cmdRaw() - 获取原始响应

返回原始字符串响应，不进行 JSON 转换。

```typescript
const raw = conn.cmdRaw(command: string, args: string[]): string

// 示例：INFO 命令返回格式化文本
const info = conn.cmdRaw("INFO", ["server"]);
console.log(info);

// 示例：获取原始字符串
const value = conn.cmdRaw("GET", ["mykey"]);
```

---

## 完整使用示例

```typescript
import { RedisClient } from 'libredis_ohos.so';

// 创建客户端
const client = new RedisClient("redis://127.0.0.1:6379");
const conn = client.getConnection();

// 1. 添加消息到流
const id1 = conn.xadd("events", "*", [
  ["type", "login"],
  ["user", "alice"],
  ["timestamp", Date.now().toString()]
]);

// 2. 创建消费者组
conn.xgroupCreate("events", "processors", "0", true);

// 3. 作为消费者读取消息
const messages = conn.xreadgroup("processors", "worker-1", ["events"], [">"], 10, null);

if (messages) {
  const data = JSON.parse(messages);
  const messageIds: string[] = [];
  
  // 处理消息
  for (const stream of data) {
    for (const msg of stream.messages) {
      console.log(`Processing message ${msg.id}:`, msg.fields);
      messageIds.push(msg.id);
    }
  }
  
  // 确认消息
  conn.xack("events", "processors", messageIds);
}

// 4. 使用通用 cmd 接口
const streamInfo = conn.cmd("XINFO", ["STREAM", "events"]);
console.log("Stream info:", streamInfo);
```

---

## 注意事项

1. **消息 ID 格式**：Stream 消息 ID 格式为 `<millisecondsTime>-<sequenceNumber>`，例如 `1234567890-0`
2. **阻塞操作**：使用 `block` 参数时要注意超时设置，避免长时间阻塞
3. **消费者组**：确保在使用 XREADGROUP 前先创建消费者组
4. **消息确认**：使用消费者组时，记得用 XACK 确认已处理的消息
5. **流修剪**：定期使用 XTRIM 清理旧消息，避免内存占用过大
6. **JSON 解析**：大多数返回值是 JSON 字符串，需要使用 `JSON.parse()` 解析

---

## 参考资料

- [Redis Streams 官方文档](https://redis.io/docs/data-types/streams/)
- [Redis Commands 参考](https://redis.io/commands/)
- [redis-ohos 示例代码](./examples/)

