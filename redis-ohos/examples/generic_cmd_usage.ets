/**
 * Generic CMD Interface Usage Example for HarmonyOS
 * 
 * This example demonstrates how to use the generic cmd() interface
 * to execute any Redis command directly.
 * 
 * The cmd() interface is useful for:
 * - Executing commands not yet wrapped by specific methods
 * - Testing new Redis features
 * - Dynamic command execution
 * - Advanced Redis operations
 */

import { RedisClient, initLogging } from 'libredis_ohos.so';

@Entry
@Component
struct GenericCmdExample {
  @State message: string = 'Generic CMD Interface Example';
  @State output: string = '';

  aboutToAppear() {
    initLogging(0xD001000, "GenericCmdExample");
  }

  async runCmdExample() {
    try {
      this.output = 'Connecting to Redis...\n';

      const client = new RedisClient("redis://127.0.0.1:6379");
      const conn = client.getConnection();
      
      this.output += 'Connected!\n\n';

      // ==================== Basic Commands ====================
      this.output += '=== Basic Commands via cmd() ===\n';

      // SET command
      const setResult = conn.cmd("SET", ["mykey", "myvalue"]);
      this.output += `SET result: ${setResult}\n`;

      // GET command
      const getResult = conn.cmd("GET", ["mykey"]);
      this.output += `GET result: ${getResult}\n`;

      // INCR command
      const incrResult = conn.cmd("INCR", ["counter"]);
      this.output += `INCR result: ${incrResult}\n`;

      // DEL command
      const delResult = conn.cmd("DEL", ["mykey"]);
      this.output += `DEL result: ${delResult}\n\n`;

      // ==================== Hash Commands ====================
      this.output += '=== Hash Commands via cmd() ===\n';

      // HSET command
      conn.cmd("HSET", ["user:1000", "name", "Alice"]);
      conn.cmd("HSET", ["user:1000", "age", "30"]);
      conn.cmd("HSET", ["user:1000", "email", "alice@example.com"]);

      // HGETALL command
      const hgetallResult = conn.cmd("HGETALL", ["user:1000"]);
      this.output += `HGETALL result: ${hgetallResult}\n`;

      // Parse the result
      const userData = JSON.parse(hgetallResult);
      this.output += `Parsed user data: ${JSON.stringify(userData)}\n\n`;

      // ==================== List Commands ====================
      this.output += '=== List Commands via cmd() ===\n';

      // RPUSH command
      const rpushResult = conn.cmd("RPUSH", ["tasks", "task1", "task2", "task3"]);
      this.output += `RPUSH result: ${rpushResult}\n`;

      // LRANGE command
      const lrangeResult = conn.cmd("LRANGE", ["tasks", "0", "-1"]);
      this.output += `LRANGE result: ${lrangeResult}\n`;

      // LPOP command
      const lpopResult = conn.cmd("LPOP", ["tasks"]);
      this.output += `LPOP result: ${lpopResult}\n\n`;

      // ==================== Set Commands ====================
      this.output += '=== Set Commands via cmd() ===\n';

      // SADD command
      const saddResult = conn.cmd("SADD", ["tags", "redis", "harmonyos", "database"]);
      this.output += `SADD result: ${saddResult}\n`;

      // SMEMBERS command
      const smembersResult = conn.cmd("SMEMBERS", ["tags"]);
      this.output += `SMEMBERS result: ${smembersResult}\n`;

      // SISMEMBER command
      const sismemberResult = conn.cmd("SISMEMBER", ["tags", "redis"]);
      this.output += `SISMEMBER result: ${sismemberResult}\n\n`;

      // ==================== Sorted Set Commands ====================
      this.output += '=== Sorted Set Commands via cmd() ===\n';

      // ZADD command
      const zaddResult = conn.cmd("ZADD", [
        "leaderboard",
        "100", "player1",
        "200", "player2",
        "150", "player3"
      ]);
      this.output += `ZADD result: ${zaddResult}\n`;

      // ZRANGE with WITHSCORES
      const zrangeResult = conn.cmd("ZRANGE", ["leaderboard", "0", "-1", "WITHSCORES"]);
      this.output += `ZRANGE result: ${zrangeResult}\n`;

      // ZREVRANK command
      const zrevrankResult = conn.cmd("ZREVRANK", ["leaderboard", "player2"]);
      this.output += `ZREVRANK result: ${zrevrankResult}\n\n`;

      // ==================== Stream Commands ====================
      this.output += '=== Stream Commands via cmd() ===\n';

      // XADD command
      const xaddResult = conn.cmd("XADD", [
        "events",
        "*",
        "type", "click",
        "user", "alice",
        "timestamp", Date.now().toString()
      ]);
      this.output += `XADD result: ${xaddResult}\n`;

      // XLEN command
      const xlenResult = conn.cmd("XLEN", ["events"]);
      this.output += `XLEN result: ${xlenResult}\n`;

      // XRANGE command
      const xrangeResult = conn.cmd("XRANGE", ["events", "-", "+", "COUNT", "10"]);
      this.output += `XRANGE result: ${xrangeResult}\n\n`;

      // ==================== Advanced Commands ====================
      this.output += '=== Advanced Commands via cmd() ===\n';

      // SCAN command
      const scanResult = conn.cmd("SCAN", ["0", "MATCH", "user:*", "COUNT", "100"]);
      this.output += `SCAN result: ${scanResult}\n`;

      // KEYS command (use with caution in production)
      const keysResult = conn.cmd("KEYS", ["user:*"]);
      this.output += `KEYS result: ${keysResult}\n`;

      // TTL command
      conn.cmd("EXPIRE", ["user:1000", "3600"]);
      const ttlResult = conn.cmd("TTL", ["user:1000"]);
      this.output += `TTL result: ${ttlResult}\n\n`;

      // ==================== Server Commands ====================
      this.output += '=== Server Commands via cmd() ===\n';

      // PING command
      const pingResult = conn.cmd("PING", []);
      this.output += `PING result: ${pingResult}\n`;

      // DBSIZE command
      const dbsizeResult = conn.cmd("DBSIZE", []);
      this.output += `DBSIZE result: ${dbsizeResult}\n`;

      // TIME command
      const timeResult = conn.cmd("TIME", []);
      this.output += `TIME result: ${timeResult}\n\n`;

      // ==================== Using cmdRaw() ====================
      this.output += '=== Using cmdRaw() for Raw Responses ===\n';

      // INFO command returns formatted text
      const infoRaw = conn.cmdRaw("INFO", ["server"]);
      this.output += `INFO server (raw):\n${infoRaw}\n\n`;

      // ==================== Error Handling ====================
      this.output += '=== Error Handling ===\n';

      try {
        // Try to execute an invalid command
        conn.cmd("INVALIDCMD", ["arg1"]);
      } catch (e) {
        this.output += `Expected error: ${e.message}\n`;
      }

      try {
        // Try to get a non-existent key
        const result = conn.cmd("GET", ["nonexistent"]);
        this.output += `Non-existent key result: ${result}\n`;
      } catch (e) {
        this.output += `Error: ${e.message}\n`;
      }

      // ==================== Dynamic Command Execution ====================
      this.output += '\n=== Dynamic Command Execution ===\n';

      // Example: Execute commands based on user input
      const commands = [
        { cmd: "SET", args: ["dynamic:1", "value1"] },
        { cmd: "SET", args: ["dynamic:2", "value2"] },
        { cmd: "MGET", args: ["dynamic:1", "dynamic:2"] }
      ];

      for (const command of commands) {
        const result = conn.cmd(command.cmd, command.args);
        this.output += `${command.cmd} ${command.args.join(' ')}: ${result}\n`;
      }

      this.output += '\n✅ Generic CMD operations completed successfully!';

    } catch (error) {
      console.error('Redis error:', error);
      this.output += `\n❌ Error: ${error.message}`;
    }
  }

  build() {
    Column() {
      Text(this.message)
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ bottom: 20 })

      Button('Run CMD Example')
        .onClick(() => {
          this.runCmdExample();
        })
        .margin({ bottom: 20 })

      Scroll() {
        Text(this.output)
          .fontSize(14)
          .fontFamily('monospace')
          .width('100%')
      }
      .width('100%')
      .height('80%')
      .backgroundColor('#f5f5f5')
      .padding(10)
    }
    .width('100%')
    .height('100%')
    .padding(20)
  }
}

