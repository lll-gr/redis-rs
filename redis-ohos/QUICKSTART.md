# Quick Start Guide - Redis HarmonyOS SDK

Get up and running with Redis on HarmonyOS in 5 minutes!

## Prerequisites

- HarmonyOS development environment
- Rust toolchain installed
- Redis server running (local or remote)

## Step 1: Install Build Tools

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install ohrs
cargo install ohrs

# Add HarmonyOS target
rustup target add aarch64-unknown-linux-ohos
```

## Step 2: Build the SDK

```bash
cd redis-ohos

# Check your environment
make check

# Build release version
make build-release
```

The build output will be in `redis-ohos/harmonyos-build/`.

## Step 3: Install to Your HarmonyOS Project

### Option A: Using Make (Recommended)

```bash
make install OHOS_PROJECT_PATH=/path/to/your/harmonyos/project
```

### Option B: Manual Copy

```bash
cp -r harmonyos-build/* /path/to/your/project/entry/libs/Redis_sdk/
```

## Step 4: Configure Your Project

Edit `entry/oh-package.json5`:

```json5
{
  "dependencies": {
    "libredis_ohos.so": "file:./libs/Redis_sdk"
  }
}
```

## Step 5: Write Your First Redis Code

Create a new page in your HarmonyOS project:

```typescript
import { RedisClient, initLogging } from 'libredis_ohos.so';

@Entry
@Component
struct RedisDemo {
  @State message: string = 'Hello Redis!';

  async testRedis() {
    try {
      // Initialize logging (optional)
      initLogging(0xD001000, "MyApp");

      // Create client
      const client = new RedisClient("redis://127.0.0.1:6379");
      
      // Get connection
      const conn = client.getConnection();
      
      // Test connection
      const pong = conn.ping();
      console.log(pong); // "PONG"
      
      // Set and get a value
      conn.set("hello", "world");
      const value = conn.get("hello");
      console.log(value); // "world"
      
      this.message = `Redis says: ${value}`;
      
    } catch (error) {
      console.error('Redis error:', error);
      this.message = `Error: ${error.message}`;
    }
  }

  build() {
    Column() {
      Text(this.message)
        .fontSize(20)
        .margin({ top: 20, bottom: 20 })
      
      Button('Test Redis')
        .onClick(() => {
          this.testRedis();
        })
    }
    .width('100%')
    .height('100%')
    .padding(20)
  }
}
```

## Step 6: Run Your App

1. Connect your HarmonyOS device or start the emulator
2. Build and run your project
3. Click the "Test Redis" button
4. You should see "Redis says: world"

## Next Steps

### Use Async API for Better Performance

```typescript
async testAsyncRedis() {
  const client = new RedisClient("redis://127.0.0.1:6379");
  const conn = await client.getAsyncConnection();
  
  // All operations are async
  await conn.set("key", "value");
  const value = await conn.get("key");
  
  // Concurrent operations
  const [v1, v2, v3] = await Promise.all([
    conn.get("key1"),
    conn.get("key2"),
    conn.get("key3")
  ]);
}
```

### Explore More Commands

```typescript
// Hash operations
conn.hset("user:1000", "name", "John");
conn.hset("user:1000", "email", "john@example.com");
const name = conn.hget("user:1000", "name");

// List operations
conn.rpush("tasks", ["task1", "task2", "task3"]);
const tasks = conn.lrange("tasks", 0, -1);

// Set operations
conn.sadd("tags", ["redis", "harmonyos", "rust"]);
const tags = conn.smembers("tags");

// Sorted sets (leaderboards)
conn.zadd("scores", [["100", "player1"], ["200", "player2"]]);
const topScores = conn.zrange("scores", 0, 9);
```

### Check Out Examples

See the `examples/` directory for complete examples:
- `basic_usage.ets` - Comprehensive sync operations
- `async_usage.ets` - Async operations and performance testing

## Troubleshooting

### "Cannot find module 'libredis_ohos.so'"

- Verify the SDK is in `entry/libs/Redis_sdk/`
- Check `oh-package.json5` has the correct dependency
- Clean and rebuild your HarmonyOS project

### "Connection refused"

- Ensure Redis server is running: `redis-server`
- Check the connection URL and port
- For remote Redis, ensure it's accessible from your device/emulator

### Build Errors

```bash
# Clean and rebuild
make clean
make build-release
```

## Getting Help

- Read the full documentation: [README.md](README.md)
- Check the API reference in README.md
- Review example code in `examples/`
- Check [CHANGELOG.md](CHANGELOG.md) for version history

## Common Patterns

### Connection with Authentication

```typescript
const client = new RedisClient("redis://:password@127.0.0.1:6379");
```

### Connection with Timeout

```typescript
const conn = client.getConnectionWithTimeout(5000); // 5 seconds
```

### Error Handling

```typescript
try {
  conn.set("key", "value");
} catch (error) {
  console.error('Redis operation failed:', error);
  // Handle error appropriately
}
```

### Cleanup

```typescript
// Delete keys when done
conn.del(["key1", "key2", "key3"]);

// Or flush entire database (careful!)
conn.flushdb();
```

---

**Congratulations!** You're now ready to use Redis in your HarmonyOS applications! 🎉

