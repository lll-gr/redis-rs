/**
 * Basic Redis Usage Example for HarmonyOS
 * 
 * This example demonstrates basic Redis operations using the redis-ohos SDK.
 * 
 * Prerequisites:
 * 1. Add dependency in entry/oh-package.json5:
 *    "dependencies": {
 *      "libredis_ohos.so": "file:./libs/Redis_sdk"
 *    }
 * 2. Copy the built SDK to entry/libs/Redis_sdk/
 */

import { RedisClient, initLogging } from 'libredis_ohos.so';

@Entry
@Component
struct BasicRedisExample {
  @State message: string = 'Redis Basic Example';
  @State output: string = '';

  aboutToAppear() {
    // Initialize logging (optional but recommended)
    initLogging(0xD001000, "RedisExample");
  }

  async runBasicExample() {
    try {
      this.output = 'Connecting to Redis...\n';

      // Create Redis client
      const client = new RedisClient("redis://127.0.0.1:6379");
      
      // Get connection
      const conn = client.getConnection();
      
      this.output += 'Connected!\n\n';

      // Test connection
      const pong = conn.ping();
      this.output += `PING: ${pong}\n\n`;

      // String operations
      this.output += '=== String Operations ===\n';
      conn.set("hello", "world");
      this.output += 'SET hello "world"\n';
      
      const value = conn.get("hello");
      this.output += `GET hello: ${value}\n\n`;

      // Number operations
      this.output += '=== Number Operations ===\n';
      conn.set("counter", "0");
      const count1 = conn.incr("counter");
      this.output += `INCR counter: ${count1}\n`;
      
      const count2 = conn.incrBy("counter", 5);
      this.output += `INCRBY counter 5: ${count2}\n\n`;

      // Key operations
      this.output += '=== Key Operations ===\n';
      const exists = conn.exists("hello");
      this.output += `EXISTS hello: ${exists}\n`;
      
      conn.expire("hello", 60);
      this.output += 'EXPIRE hello 60\n';
      
      const ttl = conn.ttl("hello");
      this.output += `TTL hello: ${ttl}\n\n`;

      // Hash operations
      this.output += '=== Hash Operations ===\n';
      conn.hset("user:1000", "name", "John Doe");
      conn.hset("user:1000", "email", "john@example.com");
      this.output += 'HSET user:1000 name "John Doe"\n';
      this.output += 'HSET user:1000 email "john@example.com"\n';
      
      const name = conn.hget("user:1000", "name");
      this.output += `HGET user:1000 name: ${name}\n\n`;

      // List operations
      this.output += '=== List Operations ===\n';
      conn.del(["mylist"]);
      conn.rpush("mylist", ["a", "b", "c"]);
      this.output += 'RPUSH mylist a b c\n';
      
      const list = conn.lrange("mylist", 0, -1);
      this.output += `LRANGE mylist 0 -1: ${JSON.stringify(list)}\n\n`;

      // Set operations
      this.output += '=== Set Operations ===\n';
      conn.sadd("myset", ["apple", "banana", "cherry"]);
      this.output += 'SADD myset apple banana cherry\n';
      
      const members = conn.smembers("myset");
      this.output += `SMEMBERS myset: ${JSON.stringify(members)}\n\n`;

      // Sorted set operations
      this.output += '=== Sorted Set Operations ===\n';
      conn.zadd("leaderboard", [["100", "player1"], ["200", "player2"], ["150", "player3"]]);
      this.output += 'ZADD leaderboard 100 player1 200 player2 150 player3\n';
      
      const topPlayers = conn.zrange("leaderboard", 0, -1);
      this.output += `ZRANGE leaderboard 0 -1: ${JSON.stringify(topPlayers)}\n\n`;

      // Cleanup
      conn.del(["hello", "counter", "user:1000", "mylist", "myset", "leaderboard"]);
      this.output += 'Cleaned up test keys\n';
      
      this.output += '\n✓ All operations completed successfully!';

    } catch (error) {
      this.output += `\n✗ Error: ${error.message}`;
      console.error('Redis error:', error);
    }
  }

  build() {
    Column() {
      Text(this.message)
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 20, bottom: 20 })

      Button('Run Basic Example')
        .onClick(() => {
          this.runBasicExample();
        })
        .margin({ bottom: 20 })

      Scroll() {
        Text(this.output)
          .fontSize(14)
          .fontFamily('monospace')
          .width('90%')
          .padding(10)
          .backgroundColor('#f5f5f5')
          .borderRadius(5)
      }
      .width('100%')
      .height('70%')
    }
    .width('100%')
    .height('100%')
    .padding(20)
  }
}

