/**
 * Redis Streams Usage Example for HarmonyOS
 * 
 * This example demonstrates Redis Streams operations using the redis-ohos SDK.
 * Redis Streams is a powerful data structure for message queuing and event streaming.
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
struct StreamExample {
  @State message: string = 'Redis Streams Example';
  @State output: string = '';

  aboutToAppear() {
    // Initialize logging
    initLogging(0xD001000, "StreamExample");
  }

  async runStreamExample() {
    try {
      this.output = 'Connecting to Redis...\n';

      // Create Redis client
      const client = new RedisClient("redis://127.0.0.1:6379");
      const conn = client.getConnection();
      
      this.output += 'Connected!\n\n';

      // ==================== Basic Stream Operations ====================
      this.output += '=== Basic Stream Operations ===\n';

      // Add messages to stream
      const id1 = conn.xadd("sensor:temperature", "*", [
        ["sensor", "temp-01"],
        ["value", "23.5"],
        ["unit", "celsius"]
      ]);
      this.output += `Added message: ${id1}\n`;

      const id2 = conn.xadd("sensor:temperature", "*", [
        ["sensor", "temp-01"],
        ["value", "24.1"],
        ["unit", "celsius"]
      ]);
      this.output += `Added message: ${id2}\n`;

      // Get stream length
      const length = conn.xlen("sensor:temperature");
      this.output += `Stream length: ${length}\n\n`;

      // ==================== Reading Stream Messages ====================
      this.output += '=== Reading Stream Messages ===\n';

      // Read all messages
      const allMessages = conn.xrange("sensor:temperature", "-", "+", null);
      this.output += `All messages: ${allMessages}\n\n`;

      // Read last 5 messages in reverse order
      const lastMessages = conn.xrevrange("sensor:temperature", "+", "-", 5);
      this.output += `Last 5 messages: ${lastMessages}\n\n`;

      // ==================== Consumer Groups ====================
      this.output += '=== Consumer Groups ===\n';

      // Create a consumer group
      try {
        conn.xgroupCreate("sensor:temperature", "processors", "0", false);
        this.output += 'Consumer group "processors" created\n';
      } catch (e) {
        this.output += `Group may already exist: ${e.message}\n`;
      }

      // Read messages as a consumer
      const consumerMessages = conn.xreadgroup(
        "processors",
        "worker-1",
        ["sensor:temperature"],
        [">"],
        10,
        null
      );
      this.output += `Consumer messages: ${consumerMessages}\n\n`;

      // Parse and process messages
      if (consumerMessages) {
        const data = JSON.parse(consumerMessages);
        const messageIds: string[] = [];
        
        for (const stream of data) {
          this.output += `Stream: ${stream.stream}\n`;
          for (const msg of stream.messages) {
            this.output += `  ID: ${msg.id}\n`;
            this.output += `  Fields: ${JSON.stringify(msg.fields)}\n`;
            messageIds.push(msg.id);
          }
        }

        // Acknowledge processed messages
        if (messageIds.length > 0) {
          const acked = conn.xack("sensor:temperature", "processors", messageIds);
          this.output += `Acknowledged ${acked} messages\n\n`;
        }
      }

      // ==================== Pending Messages ====================
      this.output += '=== Pending Messages ===\n';

      // Get pending messages summary
      const pendingSummary = conn.xpending("sensor:temperature", "processors", null, null, null, null);
      this.output += `Pending summary: ${pendingSummary}\n\n`;

      // ==================== Stream Information ====================
      this.output += '=== Stream Information ===\n';

      // Get stream info
      const streamInfo = conn.xinfoStream("sensor:temperature");
      this.output += `Stream info: ${streamInfo}\n\n`;

      // Get consumer groups info
      const groupsInfo = conn.xinfoGroups("sensor:temperature");
      this.output += `Groups info: ${groupsInfo}\n\n`;

      // Get consumers info
      const consumersInfo = conn.xinfoConsumers("sensor:temperature", "processors");
      this.output += `Consumers info: ${consumersInfo}\n\n`;

      // ==================== Stream Trimming ====================
      this.output += '=== Stream Trimming ===\n';

      // Trim stream to keep only last 100 messages (approximate)
      const trimmed = conn.xtrim("sensor:temperature", 100, true);
      this.output += `Trimmed ${trimmed} messages\n\n`;

      // ==================== Generic CMD Interface ====================
      this.output += '=== Generic CMD Interface ===\n';

      // Use generic cmd interface for any Redis command
      const cmdResult1 = conn.cmd("XLEN", ["sensor:temperature"]);
      this.output += `XLEN via cmd: ${cmdResult1}\n`;

      // Complex command with multiple arguments
      const cmdResult2 = conn.cmd("XADD", [
        "sensor:humidity",
        "*",
        "sensor", "hum-01",
        "value", "65.5",
        "unit", "percent"
      ]);
      this.output += `XADD via cmd: ${cmdResult2}\n`;

      // XRANGE via cmd
      const cmdResult3 = conn.cmd("XRANGE", ["sensor:humidity", "-", "+", "COUNT", "10"]);
      this.output += `XRANGE via cmd: ${cmdResult3}\n\n`;

      // ==================== Advanced: Multiple Streams ====================
      this.output += '=== Multiple Streams ===\n';

      // Add messages to multiple streams
      conn.xadd("events:login", "*", [["user", "alice"], ["action", "login"]]);
      conn.xadd("events:logout", "*", [["user", "bob"], ["action", "logout"]]);

      // Read from multiple streams
      const multiStream = conn.xread(
        ["events:login", "events:logout"],
        ["0-0", "0-0"],
        5,
        null
      );
      this.output += `Multiple streams: ${multiStream}\n\n`;

      // ==================== Cleanup ====================
      this.output += '=== Cleanup ===\n';

      // Delete specific messages
      const deleted = conn.xdel("sensor:temperature", [id1]);
      this.output += `Deleted ${deleted} messages\n`;

      // Destroy consumer group
      const destroyed = conn.xgroupDestroy("sensor:temperature", "processors");
      this.output += `Group destroyed: ${destroyed}\n`;

      this.output += '\n✅ Stream operations completed successfully!';

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

      Button('Run Stream Example')
        .onClick(() => {
          this.runStreamExample();
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

