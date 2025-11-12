// Redis Connection wrapper for HarmonyOS NAPI

use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::*;
use redis::{Commands, Connection, ValueType};
use std::collections::HashSet;
use crate::types::RedisValueType;

/// Redis Connection for HarmonyOS
///
/// This class represents an active connection to a Redis server.
/// It provides methods to execute Redis commands.
#[napi]
pub struct RedisConnection {
    inner: Connection,
}

impl RedisConnection {
    pub(crate) fn new(conn: Connection) -> Self {
        RedisConnection { inner: conn }
    }
}

#[napi]
impl RedisConnection {
    // ==================== String Commands ====================

    /// SET command - Set a string value
    ///
    /// # Arguments
    /// * `key` - The key to set
    /// * `value` - The value to set
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// conn.set("mykey", "myvalue");
    /// ```
    #[napi]
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        Commands::set(&mut self.inner, key, value)
            .map_err(|e| napi_ohos::Error::from_reason(format!("SET failed: {}", e)))
    }

    /// GET command - Get a string value
    ///
    /// # Arguments
    /// * `key` - The key to get
    ///
    /// # Returns
    /// The value as a string, or null if key doesn't exist
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const value = conn.get("mykey");
    /// if (value !== null) {
    ///   console.log("Value:", value);
    /// }
    /// ```
    #[napi]
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Commands::get(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("GET failed: {}", e)))
    }

    /// MSET command - Set multiple key-value pairs
    ///
    /// # Arguments
    /// * `pairs` - Array of [key, value] pairs
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// conn.mset([["key1", "value1"], ["key2", "value2"]]);
    /// ```
    #[napi]
    pub fn mset(&mut self, pairs: Vec<Vec<String>>) -> Result<()> {
        let flat_pairs: Vec<(String, String)> = pairs
            .into_iter()
            .filter_map(|pair| {
                if pair.len() == 2 {
                    Some((pair[0].clone(), pair[1].clone()))
                } else {
                    None
                }
            })
            .collect();

        Commands::mset(&mut self.inner, &flat_pairs)
            .map_err(|e| napi_ohos::Error::from_reason(format!("MSET failed: {}", e)))
    }

    /// APPEND command - Append a value to a key
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `value` - The value to append
    ///
    /// # Returns
    /// Length of the string after append
    #[napi]
    pub fn append(&mut self, key: String, value: String) -> Result<i32> {
        Commands::append(&mut self.inner, key, value)
            .map_err(|e| napi_ohos::Error::from_reason(format!("APPEND failed: {}", e)))
    }

    /// STRLEN command - Get length of string value
    ///
    /// # Arguments
    /// * `key` - The key
    ///
    /// # Returns
    /// Length of the string, or 0 if key doesn't exist
    #[napi]
    pub fn strlen(&mut self, key: String) -> Result<i32> {
        Commands::strlen(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("STRLEN failed: {}", e)))
    }

    // ==================== Key Commands ====================

    /// DEL command - Delete one or more keys
    ///
    /// # Arguments
    /// * `keys` - Array of keys to delete
    ///
    /// # Returns
    /// Number of keys deleted
    #[napi]
    pub fn del(&mut self, keys: Vec<String>) -> Result<i32> {
        Commands::del(&mut self.inner, keys)
            .map_err(|e| napi_ohos::Error::from_reason(format!("DEL failed: {}", e)))
    }

    /// EXISTS command - Check if key exists
    ///
    /// # Arguments
    /// * `key` - The key to check
    ///
    /// # Returns
    /// true if key exists, false otherwise
    #[napi]
    pub fn exists(&mut self, key: String) -> Result<bool> {
        Commands::exists(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("EXISTS failed: {}", e)))
    }

    /// EXPIRE command - Set key expiration in seconds
    ///
    /// # Arguments
    /// * `key` - The key to set expiration on
    /// * `seconds` - Expiration time in seconds
    ///
    /// # Returns
    /// true if expiration was set, false if key doesn't exist
    #[napi]
    pub fn expire(&mut self, key: String, seconds: i64) -> Result<bool> {
        Commands::expire(&mut self.inner, key, seconds as i64)
            .map_err(|e| napi_ohos::Error::from_reason(format!("EXPIRE failed: {}", e)))
    }

    /// TTL command - Get time to live in seconds
    ///
    /// # Arguments
    /// * `key` - The key to check
    ///
    /// # Returns
    /// TTL in seconds, -1 if no expiration, -2 if key doesn't exist
    #[napi]
    pub fn ttl(&mut self, key: String) -> Result<i32> {
        Commands::ttl(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("TTL failed: {}", e)))
    }

    /// PTTL command - Get time to live in milliseconds
    ///
    /// # Arguments
    /// * `key` - The key to check
    ///
    /// # Returns
    /// TTL in milliseconds, -1 if no expiration, -2 if key doesn't exist
    #[napi]
    pub fn pttl(&mut self, key: String) -> Result<i64> {
        Commands::pttl(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("PTTL failed: {}", e)))
    }

    /// PEXPIRE command - Set key expiration in milliseconds
    ///
    /// # Arguments
    /// * `key` - The key to set expiration on
    /// * `milliseconds` - Expiration time in milliseconds
    ///
    /// # Returns
    /// true if expiration was set, false if key doesn't exist
    #[napi]
    pub fn pexpire(&mut self, key: String, milliseconds: i64) -> Result<bool> {
        Commands::pexpire(&mut self.inner, key, milliseconds)
            .map_err(|e| napi_ohos::Error::from_reason(format!("PEXPIRE failed: {}", e)))
    }

    /// PERSIST command - Remove expiration from key
    ///
    /// # Arguments
    /// * `key` - The key to remove expiration from
    ///
    /// # Returns
    /// true if expiration was removed, false if key has no expiration
    #[napi]
    pub fn persist(&mut self, key: String) -> Result<bool> {
        Commands::persist(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("PERSIST failed: {}", e)))
    }

    /// TYPE command - Get the type of key
    ///
    /// # Arguments
    /// * `key` - The key to check
    ///
    /// # Returns
    /// RedisValueType enum representing the type of the key
    #[napi]
    pub fn key_type(&mut self, key: String) -> Result<RedisValueType> {
        let value_type: ValueType = Commands::key_type(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("TYPE failed: {}", e)))?;

        Ok(RedisValueType::from_redis_value_type(value_type))
    }

    /// RENAME command - Rename a key
    ///
    /// # Arguments
    /// * `key` - The current key name
    /// * `new_key` - The new key name
    ///
    /// # Returns
    /// Unit on success, error if key doesn't exist
    #[napi]
    pub fn rename(&mut self, key: String, new_key: String) -> Result<()> {
        Commands::rename(&mut self.inner, key, new_key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("RENAME failed: {}", e)))
    }

    // ==================== Number Commands ====================

    /// INCR command - Increment integer value by 1
    ///
    /// # Arguments
    /// * `key` - The key to increment
    ///
    /// # Returns
    /// The value after increment
    #[napi]
    pub fn incr(&mut self, key: String) -> Result<i64> {
        Commands::incr(&mut self.inner, key, 1)
            .map_err(|e| napi_ohos::Error::from_reason(format!("INCR failed: {}", e)))
    }

    /// INCRBY command - Increment integer value by amount
    ///
    /// # Arguments
    /// * `key` - The key to increment
    /// * `delta` - The amount to increment by
    ///
    /// # Returns
    /// The value after increment
    #[napi]
    pub fn incr_by(&mut self, key: String, delta: i64) -> Result<i64> {
        Commands::incr(&mut self.inner, key, delta)
            .map_err(|e| napi_ohos::Error::from_reason(format!("INCRBY failed: {}", e)))
    }

    /// DECR command - Decrement integer value by 1
    ///
    /// # Arguments
    /// * `key` - The key to decrement
    ///
    /// # Returns
    /// The value after decrement
    #[napi]
    pub fn decr(&mut self, key: String) -> Result<i64> {
        Commands::decr(&mut self.inner, key, 1)
            .map_err(|e| napi_ohos::Error::from_reason(format!("DECR failed: {}", e)))
    }

    /// MGET command - Get multiple values
    ///
    /// # Arguments
    /// * `keys` - Array of keys to get
    ///
    /// # Returns
    /// Array of values (null for non-existent keys)
    #[napi]
    pub fn mget(&mut self, keys: Vec<String>) -> Result<Vec<Option<String>>> {
        Commands::mget(&mut self.inner, keys)
            .map_err(|e| napi_ohos::Error::from_reason(format!("MGET failed: {}", e)))
    }

    /// SETNX command - Set key only if it doesn't exist
    ///
    /// # Arguments
    /// * `key` - The key to set
    /// * `value` - The value to set
    ///
    /// # Returns
    /// true if key was set, false if key already exists
    #[napi]
    pub fn setnx(&mut self, key: String, value: String) -> Result<bool> {
        Commands::set_nx(&mut self.inner, key, value)
            .map_err(|e| napi_ohos::Error::from_reason(format!("SETNX failed: {}", e)))
    }

    /// SETEX command - Set key with expiration in seconds
    ///
    /// # Arguments
    /// * `key` - The key to set
    /// * `value` - The value to set
    /// * `seconds` - Expiration time in seconds
    #[napi]
    pub fn setex(&mut self, key: String, value: String, seconds: u32) -> Result<()> {
        Commands::set_ex(&mut self.inner, key, value, seconds as u64)
            .map_err(|e| napi_ohos::Error::from_reason(format!("SETEX failed: {}", e)))
    }

    /// DECRBY command - Decrement integer value by amount
    ///
    /// # Arguments
    /// * `key` - The key to decrement
    /// * `delta` - The amount to decrement by
    ///
    /// # Returns
    /// The value after decrement
    #[napi]
    pub fn decr_by(&mut self, key: String, delta: i64) -> Result<i64> {
        Commands::decr(&mut self.inner, key, delta)
            .map_err(|e| napi_ohos::Error::from_reason(format!("DECRBY failed: {}", e)))
    }

    // ==================== Hash Commands ====================

    /// HSET command - Set hash field
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `field` - The field name
    /// * `value` - The field value
    ///
    /// # Returns
    /// true if field is new, false if field was updated
    #[napi]
    pub fn hset(&mut self, key: String, field: String, value: String) -> Result<bool> {
        Commands::hset(&mut self.inner, key, field, value)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HSET failed: {}", e)))
    }

    /// HGET command - Get hash field
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `field` - The field name
    ///
    /// # Returns
    /// The field value, or null if field doesn't exist
    #[napi]
    pub fn hget(&mut self, key: String, field: String) -> Result<Option<String>> {
        Commands::hget(&mut self.inner, key, field)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HGET failed: {}", e)))
    }

    /// HMSET command - Set multiple hash fields
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `pairs` - Array of [field, value] pairs
    #[napi]
    pub fn hmset(&mut self, key: String, pairs: Vec<Vec<String>>) -> Result<()> {
        let flat_pairs: Vec<(String, String)> = pairs
            .into_iter()
            .filter_map(|pair| {
                if pair.len() == 2 {
                    Some((pair[0].clone(), pair[1].clone()))
                } else {
                    None
                }
            })
            .collect();

        Commands::hset_multiple(&mut self.inner, key, &flat_pairs)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HMSET failed: {}", e)))
    }

    /// HMGET command - Get multiple hash fields
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `fields` - Array of field names
    ///
    /// # Returns
    /// Array of values (null for non-existent fields)
    #[napi]
    pub fn hmget(&mut self, key: String, fields: Vec<String>) -> Result<Vec<Option<String>>> {
        Commands::hmget(&mut self.inner, key, &fields)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HMGET failed: {}", e)))
    }

    /// HDEL command - Delete hash fields
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `fields` - Array of field names to delete
    ///
    /// # Returns
    /// Number of fields deleted
    #[napi]
    pub fn hdel(&mut self, key: String, fields: Vec<String>) -> Result<i32> {
        Commands::hdel(&mut self.inner, key, fields)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HDEL failed: {}", e)))
    }

    /// HEXISTS command - Check if hash field exists
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `field` - The field name
    ///
    /// # Returns
    /// true if field exists, false otherwise
    #[napi]
    pub fn hexists(&mut self, key: String, field: String) -> Result<bool> {
        Commands::hexists(&mut self.inner, key, field)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HEXISTS failed: {}", e)))
    }

    /// HLEN command - Get number of fields in hash
    ///
    /// # Arguments
    /// * `key` - The hash key
    ///
    /// # Returns
    /// Number of fields in the hash
    #[napi]
    pub fn hlen(&mut self, key: String) -> Result<i32> {
        Commands::hlen(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HLEN failed: {}", e)))
    }

    /// HKEYS command - Get all field names in hash
    ///
    /// # Arguments
    /// * `key` - The hash key
    ///
    /// # Returns
    /// Array of field names
    #[napi]
    pub fn hkeys(&mut self, key: String) -> Result<Vec<String>> {
        Commands::hkeys(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HKEYS failed: {}", e)))
    }

    /// HVALS command - Get all values in hash
    ///
    /// # Arguments
    /// * `key` - The hash key
    ///
    /// # Returns
    /// Array of values
    #[napi]
    pub fn hvals(&mut self, key: String) -> Result<Vec<String>> {
        Commands::hvals(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HVALS failed: {}", e)))
    }

    /// HGETALL command - Get all fields and values in hash
    ///
    /// # Arguments
    /// * `key` - The hash key
    ///
    /// # Returns
    /// HashMap containing all field-value pairs
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const allData = conn.hgetall("user:1000");
    /// // Returns: Map { "name" => "John", "age" => "30", "email" => "john@example.com" }
    /// ```
    #[napi]
    pub fn hgetall(&mut self, key: String) -> Result<std::collections::HashMap<String, String>> {
        Commands::hgetall(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HGETALL failed: {}", e)))
    }

    // ==================== List Commands ====================

    /// LPUSH command - Push values to the head of list
    ///
    /// # Arguments
    /// * `key` - The list key
    /// * `values` - Array of values to push
    ///
    /// # Returns
    /// Length of list after push
    #[napi]
    pub fn lpush(&mut self, key: String, values: Vec<String>) -> Result<i32> {
        Commands::lpush(&mut self.inner, key, values)
            .map_err(|e| napi_ohos::Error::from_reason(format!("LPUSH failed: {}", e)))
    }

    /// RPUSH command - Push values to the tail of list
    ///
    /// # Arguments
    /// * `key` - The list key
    /// * `values` - Array of values to push
    ///
    /// # Returns
    /// Length of list after push
    #[napi]
    pub fn rpush(&mut self, key: String, values: Vec<String>) -> Result<i32> {
        Commands::rpush(&mut self.inner, key, values)
            .map_err(|e| napi_ohos::Error::from_reason(format!("RPUSH failed: {}", e)))
    }

    /// LPOP command - Pop value from head of list
    ///
    /// # Arguments
    /// * `key` - The list key
    ///
    /// # Returns
    /// The popped value, or null if list is empty
    #[napi]
    pub fn lpop(&mut self, key: String) -> Result<Option<String>> {
        Commands::lpop(&mut self.inner, key, None)
            .map_err(|e| napi_ohos::Error::from_reason(format!("LPOP failed: {}", e)))
    }

    /// RPOP command - Pop value from tail of list
    ///
    /// # Arguments
    /// * `key` - The list key
    ///
    /// # Returns
    /// The popped value, or null if list is empty
    #[napi]
    pub fn rpop(&mut self, key: String) -> Result<Option<String>> {
        Commands::rpop(&mut self.inner, key, None)
            .map_err(|e| napi_ohos::Error::from_reason(format!("RPOP failed: {}", e)))
    }

    /// LLEN command - Get length of list
    ///
    /// # Arguments
    /// * `key` - The list key
    ///
    /// # Returns
    /// Length of the list
    #[napi]
    pub fn llen(&mut self, key: String) -> Result<i32> {
        Commands::llen(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("LLEN failed: {}", e)))
    }

    /// LRANGE command - Get range of elements from list
    ///
    /// # Arguments
    /// * `key` - The list key
    /// * `start` - Start index (0-based)
    /// * `stop` - Stop index (-1 for end)
    ///
    /// # Returns
    /// Array of values in the range
    #[napi]
    pub fn lrange(&mut self, key: String, start: i32, stop: i32) -> Result<Vec<String>> {
        Commands::lrange(&mut self.inner, key, start as isize, stop as isize)
            .map_err(|e| napi_ohos::Error::from_reason(format!("LRANGE failed: {}", e)))
    }

    /// LINDEX command - Get element at index
    ///
    /// # Arguments
    /// * `key` - The list key
    /// * `index` - The index (0-based, negative for from end)
    ///
    /// # Returns
    /// The value at index, or null if index is out of range
    #[napi]
    pub fn lindex(&mut self, key: String, index: i32) -> Result<Option<String>> {
        Commands::lindex(&mut self.inner, key, index as isize)
            .map_err(|e| napi_ohos::Error::from_reason(format!("LINDEX failed: {}", e)))
    }

    /// LSET command - Set element at index
    ///
    /// # Arguments
    /// * `key` - The list key
    /// * `index` - The index (0-based)
    /// * `value` - The new value
    #[napi]
    pub fn lset(&mut self, key: String, index: i32, value: String) -> Result<()> {
        Commands::lset(&mut self.inner, key, index as isize, value)
            .map_err(|e| napi_ohos::Error::from_reason(format!("LSET failed: {}", e)))
    }

    /// LREM command - Remove elements from list
    ///
    /// # Arguments
    /// * `key` - The list key
    /// * `count` - Number of elements to remove:
    ///   - count > 0: Remove elements equal to value moving from head to tail
    ///   - count < 0: Remove elements equal to value moving from tail to head
    ///   - count = 0: Remove all elements equal to value
    /// * `value` - The value to remove
    ///
    /// # Returns
    /// Number of elements removed
    #[napi]
    pub fn lrem(&mut self, key: String, count: i32, value: String) -> Result<i32> {
        Commands::lrem(&mut self.inner, key, count as isize, value)
            .map_err(|e| napi_ohos::Error::from_reason(format!("LREM failed: {}", e)))
    }

    // ==================== Set Commands ====================

    /// SADD command - Add members to set
    ///
    /// # Arguments
    /// * `key` - The set key
    /// * `members` - Array of members to add
    ///
    /// # Returns
    /// Number of members added
    #[napi]
    pub fn sadd(&mut self, key: String, members: Vec<String>) -> Result<i32> {
        Commands::sadd(&mut self.inner, key, members)
            .map_err(|e| napi_ohos::Error::from_reason(format!("SADD failed: {}", e)))
    }

    /// SREM command - Remove members from set
    ///
    /// # Arguments
    /// * `key` - The set key
    /// * `members` - Array of members to remove
    ///
    /// # Returns
    /// Number of members removed
    #[napi]
    pub fn srem(&mut self, key: String, members: Vec<String>) -> Result<i32> {
        Commands::srem(&mut self.inner, key, members)
            .map_err(|e| napi_ohos::Error::from_reason(format!("SREM failed: {}", e)))
    }

    /// SISMEMBER command - Check if member is in set
    ///
    /// # Arguments
    /// * `key` - The set key
    /// * `member` - The member to check
    ///
    /// # Returns
    /// true if member exists, false otherwise
    #[napi]
    pub fn sismember(&mut self, key: String, member: String) -> Result<bool> {
        Commands::sismember(&mut self.inner, key, member)
            .map_err(|e| napi_ohos::Error::from_reason(format!("SISMEMBER failed: {}", e)))
    }

    /// SMEMBERS command - Get all members of set
    ///
    /// # Arguments
    /// * `key` - The set key
    ///
    /// # Returns
    /// Array of all members in the set
    #[napi]
    pub fn smembers(&mut self, key: String) -> Result<Vec<String>> {
        let members: HashSet<String> = Commands::smembers(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("SMEMBERS failed: {}", e)))?;
        Ok(members.into_iter().collect())
    }

    /// SCARD command - Get number of members in set
    ///
    /// # Arguments
    /// * `key` - The set key
    ///
    /// # Returns
    /// Number of members in the set
    #[napi]
    pub fn scard(&mut self, key: String) -> Result<i32> {
        Commands::scard(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("SCARD failed: {}", e)))
    }

    // ==================== Utility Commands ====================

    /// PING command - Test connection
    ///
    /// # Returns
    /// "PONG"
    #[napi]
    pub fn ping(&mut self) -> Result<String> {
        // Using redis::cmd() because Commands::ping() doesn't exist in all versions
        // This is a simple command that doesn't benefit from the high-level API
        redis::cmd("PING")
            .query(&mut self.inner)
            .map(|_: String| "PONG".to_string())
            .map_err(|e| napi_ohos::Error::from_reason(format!("PING failed: {}", e)))
    }

    /// KEYS command - Find all keys matching pattern
    ///
    /// # Arguments
    /// * `pattern` - The pattern to match (e.g., "*", "user:*")
    ///
    /// # Returns
    /// Array of matching keys
    ///
    /// # Warning
    /// This command can be slow on large databases. Use SCAN instead for production.
    #[napi]
    pub fn keys(&mut self, pattern: String) -> Result<Vec<String>> {
        Commands::keys(&mut self.inner, pattern)
            .map_err(|e| napi_ohos::Error::from_reason(format!("KEYS failed: {}", e)))
    }

    /// FLUSHDB command - Delete all keys in current database
    ///
    /// # Warning
    /// This is a destructive operation!
    #[napi]
    pub fn flushdb(&mut self) -> Result<()> {
        // Using redis::cmd() for destructive operations to be explicit
        redis::cmd("FLUSHDB")
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("FLUSHDB failed: {}", e)))
    }

    // ==================== Sorted Set Commands ====================

    /// ZADD command - Add members to sorted set
    ///
    /// # Arguments
    /// * `key` - The sorted set key
    /// * `members` - Array of [score, member] pairs
    ///
    /// # Returns
    /// Number of members added
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// conn.zadd("leaderboard", [[100, "player1"], [200, "player2"]]);
    /// ```
    #[napi]
    pub fn zadd(&mut self, key: String, members: Vec<Vec<String>>) -> Result<i32> {
        // Using redis::cmd() because we need to build a dynamic command
        // with variable number of score-member pairs
        let mut cmd = redis::cmd("ZADD");
        cmd.arg(&key);

        for member in members {
            if member.len() == 2 {
                // Parse score as f64
                let score: f64 = member[0]
                    .parse()
                    .map_err(|_| napi_ohos::Error::from_reason("Invalid score format"))?;
                cmd.arg(score).arg(&member[1]);
            }
        }

        cmd.query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("ZADD failed: {}", e)))
    }

    /// ZRANGE command - Get range of members from sorted set
    ///
    /// # Arguments
    /// * `key` - The sorted set key
    /// * `start` - Start index
    /// * `stop` - Stop index
    ///
    /// # Returns
    /// Array of members in the range
    #[napi]
    pub fn zrange(&mut self, key: String, start: i32, stop: i32) -> Result<Vec<String>> {
        Commands::zrange(&mut self.inner, key, start as isize, stop as isize)
            .map_err(|e| napi_ohos::Error::from_reason(format!("ZRANGE failed: {}", e)))
    }

    /// ZREM command - Remove members from sorted set
    ///
    /// # Arguments
    /// * `key` - The sorted set key
    /// * `members` - Array of members to remove
    ///
    /// # Returns
    /// Number of members removed
    #[napi]
    pub fn zrem(&mut self, key: String, members: Vec<String>) -> Result<i32> {
        Commands::zrem(&mut self.inner, key, members)
            .map_err(|e| napi_ohos::Error::from_reason(format!("ZREM failed: {}", e)))
    }

    /// ZSCORE command - Get score of member in sorted set
    ///
    /// # Arguments
    /// * `key` - The sorted set key
    /// * `member` - The member
    ///
    /// # Returns
    /// Score as string, or null if member doesn't exist
    #[napi]
    pub fn zscore(&mut self, key: String, member: String) -> Result<Option<String>> {
        let score: Option<f64> = Commands::zscore(&mut self.inner, key, member)
            .map_err(|e| napi_ohos::Error::from_reason(format!("ZSCORE failed: {}", e)))?;
        Ok(score.map(|s| s.to_string()))
    }

    /// ZCARD command - Get number of members in sorted set
    ///
    /// # Arguments
    /// * `key` - The sorted set key
    ///
    /// # Returns
    /// Number of members in the sorted set
    #[napi]
    pub fn zcard(&mut self, key: String) -> Result<i32> {
        Commands::zcard(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("ZCARD failed: {}", e)))
    }

    /// ZCOUNT command - Count members in score range
    ///
    /// # Arguments
    /// * `key` - The sorted set key
    /// * `min` - Minimum score (as string, e.g., "-inf", "0", "(5")
    /// * `max` - Maximum score (as string, e.g., "+inf", "100", "(200")
    ///
    /// # Returns
    /// Number of members in the range
    #[napi]
    pub fn zcount(&mut self, key: String, min: String, max: String) -> Result<i32> {
        // Using redis::cmd() because min/max can be special values like "-inf", "+inf"
        // which are easier to handle as strings
        redis::cmd("ZCOUNT")
            .arg(&key)
            .arg(&min)
            .arg(&max)
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("ZCOUNT failed: {}", e)))
    }

    /// ZINCRBY command - Increment score of member in sorted set
    ///
    /// # Arguments
    /// * `key` - The sorted set key
    /// * `increment` - Amount to increment by
    /// * `member` - The member
    ///
    /// # Returns
    /// New score as string
    #[napi]
    pub fn zincrby(&mut self, key: String, increment: f64, member: String) -> Result<String> {
        let new_score: f64 = Commands::zincr(&mut self.inner, key, member, increment)
            .map_err(|e| napi_ohos::Error::from_reason(format!("ZINCRBY failed: {}", e)))?;
        Ok(new_score.to_string())
    }

    /// ZRANK command - Get rank of member in sorted set (ascending order)
    ///
    /// # Arguments
    /// * `key` - The sorted set key
    /// * `member` - The member
    ///
    /// # Returns
    /// Rank (0-based), or null if member doesn't exist
    #[napi]
    pub fn zrank(&mut self, key: String, member: String) -> Result<Option<i32>> {
        let rank: Option<isize> = Commands::zrank(&mut self.inner, key, member)
            .map_err(|e| napi_ohos::Error::from_reason(format!("ZRANK failed: {}", e)))?;
        Ok(rank.map(|r| r as i32))
    }

    /// ZREVRANK command - Get rank of member in sorted set (descending order)
    ///
    /// # Arguments
    /// * `key` - The sorted set key
    /// * `member` - The member
    ///
    /// # Returns
    /// Rank (0-based), or null if member doesn't exist
    #[napi]
    pub fn zrevrank(&mut self, key: String, member: String) -> Result<Option<i32>> {
        let rank: Option<isize> = Commands::zrevrank(&mut self.inner, key, member)
            .map_err(|e| napi_ohos::Error::from_reason(format!("ZREVRANK failed: {}", e)))?;
        Ok(rank.map(|r| r as i32))
    }

    /// ZRANGEBYSCORE command - Get members by score range
    ///
    /// # Arguments
    /// * `key` - The sorted set key
    /// * `min` - Minimum score (as string, e.g., "-inf", "0", "(5")
    /// * `max` - Maximum score (as string, e.g., "+inf", "100", "(200")
    ///
    /// # Returns
    /// Array of members in the score range
    #[napi]
    pub fn zrangebyscore(&mut self, key: String, min: String, max: String) -> Result<Vec<String>> {
        redis::cmd("ZRANGEBYSCORE")
            .arg(&key)
            .arg(&min)
            .arg(&max)
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("ZRANGEBYSCORE failed: {}", e)))
    }

    /// ZREMRANGEBYRANK command - Remove members by rank range
    ///
    /// # Arguments
    /// * `key` - The sorted set key
    /// * `start` - Start rank (0-based)
    /// * `stop` - Stop rank (0-based)
    ///
    /// # Returns
    /// Number of members removed
    #[napi]
    pub fn zremrangebyrank(&mut self, key: String, start: i32, stop: i32) -> Result<i32> {
        Commands::zremrangebyrank(&mut self.inner, key, start as isize, stop as isize)
            .map_err(|e| napi_ohos::Error::from_reason(format!("ZREMRANGEBYRANK failed: {}", e)))
    }

    /// ZREMRANGEBYSCORE command - Remove members by score range
    ///
    /// # Arguments
    /// * `key` - The sorted set key
    /// * `min` - Minimum score (as string, e.g., "-inf", "0", "(5")
    /// * `max` - Maximum score (as string, e.g., "+inf", "100", "(200")
    ///
    /// # Returns
    /// Number of members removed
    #[napi]
    pub fn zremrangebyscore(&mut self, key: String, min: String, max: String) -> Result<i32> {
        redis::cmd("ZREMRANGEBYSCORE")
            .arg(&key)
            .arg(&min)
            .arg(&max)
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("ZREMRANGEBYSCORE failed: {}", e)))
    }

    // ==================== Database Commands ====================

    /// SELECT command - Select database by index
    ///
    /// # Arguments
    /// * `db` - Database index (0-15 typically)
    #[napi]
    pub fn select(&mut self, db: i32) -> Result<()> {
        // Using redis::cmd() because Commands trait doesn't have a select() method
        // (it would conflict with Rust's Iterator::select)
        redis::cmd("SELECT")
            .arg(db)
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("SELECT failed: {}", e)))
    }

    /// DBSIZE command - Get number of keys in current database
    ///
    /// # Returns
    /// Number of keys
    #[napi]
    pub fn dbsize(&mut self) -> Result<i32> {
        redis::cmd("DBSIZE")
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("DBSIZE failed: {}", e)))
    }

    /// INFO command - Get server information
    ///
    /// # Arguments
    /// * `section` - Optional section name (e.g., "server", "memory", "stats", "keyspace")
    ///
    /// # Returns
    /// Server information as string
    #[napi]
    pub fn info(&mut self, section: Option<String>) -> Result<String> {
        let mut cmd = redis::cmd("INFO");
        if let Some(s) = section {
            cmd.arg(s);
        }
        cmd.query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("INFO failed: {}", e)))
    }

    /// Get keyspace statistics for all databases
    ///
    /// Returns information about keys count, expires, and avg_ttl for each database.
    /// This is a convenience method that calls INFO keyspace and parses the result.
    ///
    /// # Returns
    /// JSON string containing database statistics
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const stats = conn.getKeyspaceInfo();
    /// console.log(stats);
    /// // Output example:
    /// // {
    /// //   "db0": {"keys": 100, "expires": 10, "avg_ttl": 5000},
    /// //   "db1": {"keys": 50, "expires": 5, "avg_ttl": 3000}
    /// // }
    /// ```
    #[napi]
    pub fn get_keyspace_info(&mut self) -> Result<String> {
        let info_str: String = redis::cmd("INFO")
            .arg("keyspace")
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("INFO keyspace failed: {}", e)))?;

        // Parse the keyspace section
        // Format: db0:keys=100,expires=10,avg_ttl=5000
        let mut result = std::collections::HashMap::new();

        for line in info_str.lines() {
            if line.starts_with("db") {
                if let Some((db_name, stats)) = line.split_once(':') {
                    let mut db_stats = std::collections::HashMap::new();

                    for stat in stats.split(',') {
                        if let Some((key, value)) = stat.split_once('=') {
                            if let Ok(num) = value.parse::<i64>() {
                                db_stats.insert(key.to_string(), num);
                            }
                        }
                    }

                    result.insert(db_name.to_string(), db_stats);
                }
            }
        }

        // Convert to JSON string
        serde_json::to_string(&result).map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to serialize keyspace info: {}", e))
        })
    }

    /// Get total keys count across all databases
    ///
    /// This is a convenience method that sums up keys from all databases.
    ///
    /// # Returns
    /// Total number of keys across all databases
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const totalKeys = conn.getTotalKeysCount();
    /// console.log(`Total keys: ${totalKeys}`);
    /// ```
    #[napi]
    pub fn get_total_keys_count(&mut self) -> Result<i64> {
        let info_str: String = redis::cmd("INFO")
            .arg("keyspace")
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("INFO keyspace failed: {}", e)))?;

        let mut total = 0i64;

        for line in info_str.lines() {
            if line.starts_with("db") {
                if let Some((_, stats)) = line.split_once(':') {
                    for stat in stats.split(',') {
                        if let Some((key, value)) = stat.split_once('=') {
                            if key == "keys" {
                                if let Ok(count) = value.parse::<i64>() {
                                    total += count;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(total)
    }

    // ==================== Hash Scan Commands ====================

    /// HSCAN command - Incrementally iterate hash fields and values
    ///
    /// # Arguments
    /// * `key` - The hash key to scan
    /// * `pattern` - Optional pattern to match field names (e.g., "user:*")
    /// * `count` - Optional hint for number of elements to return per iteration
    ///
    /// # Returns
    /// HashMap with all field-value pairs
    ///
    /// # Note
    /// - The `count` parameter is a hint to Redis, not a hard limit
    /// - This method iterates through all results automatically
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// // Scan all fields
    /// const allFields = conn.hscan("myhash", null, null);
    ///
    /// // Scan with pattern
    /// const userFields = conn.hscan("myhash", "user:*", null);
    ///
    /// // Scan with count hint
    /// const fields = conn.hscan("myhash", null, 100);
    ///
    /// // Scan with both pattern and count
    /// const filtered = conn.hscan("myhash", "user:*", 100);
    /// ```
    #[napi]
    pub fn hscan(
        &mut self,
        key: String,
        pattern: Option<String>,
        count: Option<i32>
    ) -> Result<std::collections::HashMap<String, String>> {
        // Build HSCAN command with optional parameters
        let mut cmd = redis::cmd("HSCAN");
        cmd.arg(&key).arg(0); // cursor starts at 0

        if let Some(p) = pattern {
            cmd.arg("MATCH").arg(p);
        }

        if let Some(c) = count {
            cmd.arg("COUNT").arg(c);
        }

        // Execute and get iterator
        let iter: redis::Iter<(String, String)> = cmd.iter(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HSCAN failed: {}", e)))?;

        // Collect all results
        let mut result = std::collections::HashMap::new();
        for item in iter {
            let (field, value) = item
                .map_err(|e| napi_ohos::Error::from_reason(format!("HSCAN iteration failed: {}", e)))?;
            result.insert(field, value);
        }

        Ok(result)
    }

    // ==================== Hash Field Expiration Commands (Redis 7.4+) ====================

    /// HTTL command - Get hash fields' TTL in seconds
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `fields` - Array of field names to check
    ///
    /// # Returns
    /// Array of TTL values in seconds for each field:
    /// * Positive number: remaining TTL in seconds
    /// * -1: field exists but has no expiration
    /// * -2: field does not exist
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const ttls = conn.httl("myhash", ["field1", "field2"]);
    /// console.log(`field1 TTL: ${ttls[0]} seconds`);
    /// ```
    ///
    /// # Note
    /// Requires Redis 7.4.0 or later
    #[napi]
    pub fn httl(&mut self, key: String, fields: Vec<String>) -> Result<Vec<i32>> {
        use redis::Commands;

        let results: Vec<redis::IntegerReplyOrNoOp> = Commands::httl(&mut self.inner, key, fields)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HTTL failed: {}", e)))?;

        Ok(results.iter().map(|r| r.raw() as i32).collect())
    }

    /// HPTTL command - Get hash fields' TTL in milliseconds
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `fields` - Array of field names to check
    ///
    /// # Returns
    /// Array of TTL values in milliseconds for each field
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const ttls = conn.hpttl("myhash", ["field1", "field2"]);
    /// console.log(`field1 TTL: ${ttls[0]} milliseconds`);
    /// ```
    ///
    /// # Note
    /// Requires Redis 7.4.0 or later
    #[napi]
    pub fn hpttl(&mut self, key: String, fields: Vec<String>) -> Result<Vec<i32>> {
        use redis::Commands;

        let results: Vec<redis::IntegerReplyOrNoOp> = Commands::hpttl(&mut self.inner, key, fields)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HPTTL failed: {}", e)))?;

        Ok(results.iter().map(|r| r.raw() as i32).collect())
    }

    /// HEXPIRE command - Set hash fields' expiration in seconds
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `seconds` - Expiration time in seconds
    /// * `option` - Expiration option (None, NX, XX, GT, LT)
    /// * `fields` - Array of field names to set expiration on
    ///
    /// # Returns
    /// Array of results for each field:
    /// * 1: expiration was set
    /// * 0: condition was not met
    /// * 2: called with 0 seconds
    /// * -2: field does not exist
    /// * -1: field exists but has no expiration (for XX option)
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// import { RedisExpireOption } from 'libredis_ohos.so';
    /// const results = conn.hexpire("myhash", 60, RedisExpireOption.NX, ["field1", "field2"]);
    /// ```
    ///
    /// # Note
    /// Requires Redis 7.4.0 or later
    #[napi]
    pub fn hexpire(&mut self, key: String, seconds: i64, option: crate::types::RedisExpireOption, fields: Vec<String>) -> Result<Vec<i32>> {
        use redis::Commands;

        let redis_option = option.to_redis_expire_option();
        let results: Vec<redis::IntegerReplyOrNoOp> = Commands::hexpire(&mut self.inner, key, seconds, redis_option, fields)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HEXPIRE failed: {}", e)))?;

        Ok(results.iter().map(|r| r.raw() as i32).collect())
    }

    /// HPEXPIRE command - Set hash fields' expiration in milliseconds
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `milliseconds` - Expiration time in milliseconds
    /// * `option` - Expiration option (None, NX, XX, GT, LT)
    /// * `fields` - Array of field names to set expiration on
    ///
    /// # Returns
    /// Array of results for each field (same as hexpire)
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// import { RedisExpireOption } from 'libredis_ohos.so';
    /// const results = conn.hpexpire("myhash", 60000, RedisExpireOption.NX, ["field1"]);
    /// ```
    ///
    /// # Note
    /// Requires Redis 7.4.0 or later
    #[napi]
    pub fn hpexpire(&mut self, key: String, milliseconds: i64, option: crate::types::RedisExpireOption, fields: Vec<String>) -> Result<Vec<i32>> {
        use redis::Commands;

        let redis_option = option.to_redis_expire_option();
        let results: Vec<redis::IntegerReplyOrNoOp> = Commands::hpexpire(&mut self.inner, key, milliseconds, redis_option, fields)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HPEXPIRE failed: {}", e)))?;

        Ok(results.iter().map(|r| r.raw() as i32).collect())
    }

    /// HEXPIREAT command - Set hash fields' expiration as UNIX timestamp in seconds
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `timestamp` - UNIX timestamp in seconds
    /// * `option` - Expiration option (None, NX, XX, GT, LT)
    /// * `fields` - Array of field names to set expiration on
    ///
    /// # Returns
    /// Array of results for each field (same as hexpire)
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// import { RedisExpireOption } from 'libredis_ohos.so';
    /// const timestamp = Math.floor(Date.now() / 1000) + 3600; // 1 hour from now
    /// const results = conn.hexpireAt("myhash", timestamp, RedisExpireOption.None, ["field1"]);
    /// ```
    ///
    /// # Note
    /// Requires Redis 7.4.0 or later
    #[napi]
    pub fn hexpire_at(&mut self, key: String, timestamp: i64, option: crate::types::RedisExpireOption, fields: Vec<String>) -> Result<Vec<i32>> {
        use redis::Commands;

        let redis_option = option.to_redis_expire_option();
        let results: Vec<redis::IntegerReplyOrNoOp> = Commands::hexpire_at(&mut self.inner, key, timestamp, redis_option, fields)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HEXPIREAT failed: {}", e)))?;

        Ok(results.iter().map(|r| r.raw() as i32).collect())
    }

    /// HPEXPIREAT command - Set hash fields' expiration as UNIX timestamp in milliseconds
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `timestamp` - UNIX timestamp in milliseconds
    /// * `option` - Expiration option (None, NX, XX, GT, LT)
    /// * `fields` - Array of field names to set expiration on
    ///
    /// # Returns
    /// Array of results for each field (same as hexpire)
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// import { RedisExpireOption } from 'libredis_ohos.so';
    /// const timestamp = Date.now() + 3600000; // 1 hour from now
    /// const results = conn.hpexpireAt("myhash", timestamp, RedisExpireOption.None, ["field1"]);
    /// ```
    ///
    /// # Note
    /// Requires Redis 7.4.0 or later
    #[napi]
    pub fn hpexpire_at(&mut self, key: String, timestamp: i64, option: crate::types::RedisExpireOption, fields: Vec<String>) -> Result<Vec<i32>> {
        use redis::Commands;

        let redis_option = option.to_redis_expire_option();
        let results: Vec<redis::IntegerReplyOrNoOp> = Commands::hpexpire_at(&mut self.inner, key, timestamp, redis_option, fields)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HPEXPIREAT failed: {}", e)))?;

        Ok(results.iter().map(|r| r.raw() as i32).collect())
    }

    /// HEXPIRETIME command - Get hash fields' expiration timestamp in seconds
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `fields` - Array of field names to check
    ///
    /// # Returns
    /// Array of UNIX timestamps in seconds for each field:
    /// * Positive number: expiration timestamp
    /// * -1: field exists but has no expiration
    /// * -2: field does not exist
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const timestamps = conn.hexpireTime("myhash", ["field1", "field2"]);
    /// if (timestamps[0] > 0) {
    ///   console.log(`field1 expires at: ${new Date(timestamps[0] * 1000)}`);
    /// }
    /// ```
    ///
    /// # Note
    /// Requires Redis 7.4.0 or later
    #[napi]
    pub fn hexpire_time(&mut self, key: String, fields: Vec<String>) -> Result<Vec<i32>> {
        use redis::Commands;

        let results: Vec<redis::IntegerReplyOrNoOp> = Commands::hexpire_time(&mut self.inner, key, fields)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HEXPIRETIME failed: {}", e)))?;

        Ok(results.iter().map(|r| r.raw() as i32).collect())
    }

    /// HPEXPIRETIME command - Get hash fields' expiration timestamp in milliseconds
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `fields` - Array of field names to check
    ///
    /// # Returns
    /// Array of UNIX timestamps in milliseconds for each field
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const timestamps = conn.hpexpireTime("myhash", ["field1", "field2"]);
    /// if (timestamps[0] > 0) {
    ///   console.log(`field1 expires at: ${new Date(timestamps[0])}`);
    /// }
    /// ```
    ///
    /// # Note
    /// Requires Redis 7.4.0 or later
    #[napi]
    pub fn hpexpire_time(&mut self, key: String, fields: Vec<String>) -> Result<Vec<i32>> {
        use redis::Commands;

        let results: Vec<redis::IntegerReplyOrNoOp> = Commands::hpexpire_time(&mut self.inner, key, fields)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HPEXPIRETIME failed: {}", e)))?;

        Ok(results.iter().map(|r| r.raw() as i32).collect())
    }

    /// HPERSIST command - Remove expiration from hash fields
    ///
    /// # Arguments
    /// * `key` - The hash key
    /// * `fields` - Array of field names to remove expiration from
    ///
    /// # Returns
    /// Array of results for each field:
    /// * 1: expiration was removed
    /// * 0: field has no expiration
    /// * -2: field does not exist
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const results = conn.hpersist("myhash", ["field1", "field2"]);
    /// if (results[0] === 1) {
    ///   console.log("field1 expiration removed");
    /// }
    /// ```
    ///
    /// # Note
    /// Requires Redis 7.4.0 or later
    #[napi]
    pub fn hpersist(&mut self, key: String, fields: Vec<String>) -> Result<Vec<i32>> {
        use redis::Commands;

        let results: Vec<redis::IntegerReplyOrNoOp> = Commands::hpersist(&mut self.inner, key, fields)
            .map_err(|e| napi_ohos::Error::from_reason(format!("HPERSIST failed: {}", e)))?;

        Ok(results.iter().map(|r| r.raw() as i32).collect())
    }

    // ==================== Cluster Commands ====================

    /// CLUSTER INFO - Get cluster information
    ///
    /// Returns information about the cluster state, including:
    /// - cluster_state: ok or fail
    /// - cluster_slots_assigned: number of slots assigned
    /// - cluster_slots_ok: number of slots in ok state
    /// - cluster_slots_pfail: number of slots in pfail state
    /// - cluster_slots_fail: number of slots in fail state
    /// - cluster_known_nodes: number of known nodes
    /// - cluster_size: number of master nodes
    /// - cluster_current_epoch: current epoch
    /// - cluster_my_epoch: current node's epoch
    /// - cluster_stats_messages_sent: total messages sent
    /// - cluster_stats_messages_received: total messages received
    ///
    /// # Returns
    /// Cluster information as string
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const info = conn.clusterInfo();
    /// console.log(info);
    /// // Output:
    /// // cluster_state:ok
    /// // cluster_slots_assigned:16384
    /// // cluster_slots_ok:16384
    /// // ...
    /// ```
    #[napi]
    pub fn cluster_info(&mut self) -> Result<String> {
        redis::cmd("CLUSTER")
            .arg("INFO")
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("CLUSTER INFO failed: {}", e)))
    }

    /// CLUSTER NODES - Get cluster nodes information
    ///
    /// Returns information about all nodes in the cluster, including:
    /// - Node ID
    /// - IP:port
    /// - Flags (master, slave, myself, fail, etc.)
    /// - Master ID (if slave)
    /// - Ping sent timestamp
    /// - Pong received timestamp
    /// - Config epoch
    /// - Link state
    /// - Slots (if master)
    ///
    /// # Returns
    /// Cluster nodes information as string (one line per node)
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const nodes = conn.clusterNodes();
    /// console.log(nodes);
    /// // Output:
    /// // 07c37dfeb235213a872192d90877d0cd55635b91 127.0.0.1:30004@31004 slave e7d1eecce10fd6bb5eb35b9f99a514335d9ba9ca 0 1426238317239 4 connected
    /// // 67ed2db8d677e59ec4a4cefb06858cf2a1a89fa1 127.0.0.1:30002@31002 master - 0 1426238316232 2 connected 5461-10922
    /// // ...
    /// ```
    #[napi]
    pub fn cluster_nodes(&mut self) -> Result<String> {
        redis::cmd("CLUSTER")
            .arg("NODES")
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("CLUSTER NODES failed: {}", e)))
    }

    /// CLUSTER SLOTS - Get cluster slots allocation information
    ///
    /// Returns an array of slot ranges with their master and replica nodes.
    /// Each slot range contains:
    /// - Start slot
    /// - End slot
    /// - Master node (IP, port, node ID)
    /// - Replica nodes (IP, port, node ID)
    ///
    /// # Returns
    /// JSON string representing the slots allocation
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const slots = conn.clusterSlots();
    /// const slotsData = JSON.parse(slots);
    /// console.log(slotsData);
    /// // Output: Array of slot ranges with node information
    /// ```
    #[napi]
    pub fn cluster_slots(&mut self) -> Result<String> {
        let value: redis::Value = redis::cmd("CLUSTER")
            .arg("SLOTS")
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("CLUSTER SLOTS failed: {}", e)))?;

        // Convert Redis Value to JSON string
        self.redis_value_to_json(&value)
    }

    /// CLUSTER KEYSLOT - Get the hash slot of a key
    ///
    /// Returns the hash slot number for the given key.
    /// Redis Cluster uses 16384 slots (0-16383).
    ///
    /// # Arguments
    /// * `key` - The key to get the slot for
    ///
    /// # Returns
    /// The slot number (0-16383)
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const slot = conn.clusterKeyslot("mykey");
    /// console.log(`Key "mykey" belongs to slot ${slot}`);
    /// // Output: Key "mykey" belongs to slot 14687
    /// ```
    #[napi]
    pub fn cluster_keyslot(&mut self, key: String) -> Result<i32> {
        redis::cmd("CLUSTER")
            .arg("KEYSLOT")
            .arg(key)
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("CLUSTER KEYSLOT failed: {}", e)))
    }

    /// CLUSTER COUNTKEYSINSLOT - Count keys in a hash slot
    ///
    /// Returns the number of keys in the specified hash slot.
    /// Only works for slots assigned to the current node.
    ///
    /// # Arguments
    /// * `slot` - The slot number (0-16383)
    ///
    /// # Returns
    /// Number of keys in the slot
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const count = conn.clusterCountKeysInSlot(14687);
    /// console.log(`Slot 14687 has ${count} keys`);
    /// ```
    #[napi]
    pub fn cluster_count_keys_in_slot(&mut self, slot: i32) -> Result<i32> {
        redis::cmd("CLUSTER")
            .arg("COUNTKEYSINSLOT")
            .arg(slot)
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("CLUSTER COUNTKEYSINSLOT failed: {}", e)))
    }

    /// CLUSTER GETKEYSINSLOT - Get keys in a hash slot
    ///
    /// Returns an array of keys in the specified hash slot.
    /// Only works for slots assigned to the current node.
    ///
    /// # Arguments
    /// * `slot` - The slot number (0-16383)
    /// * `count` - Maximum number of keys to return
    ///
    /// # Returns
    /// Array of keys in the slot
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const keys = conn.clusterGetKeysInSlot(14687, 10);
    /// console.log(`Keys in slot 14687:`, keys);
    /// // Output: Keys in slot 14687: ["mykey", "anotherkey", ...]
    /// ```
    #[napi]
    pub fn cluster_get_keys_in_slot(&mut self, slot: i32, count: i32) -> Result<Vec<String>> {
        redis::cmd("CLUSTER")
            .arg("GETKEYSINSLOT")
            .arg(slot)
            .arg(count)
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("CLUSTER GETKEYSINSLOT failed: {}", e)))
    }

    /// CLUSTER MYID - Get the current node's ID
    ///
    /// Returns the unique ID of the current node.
    ///
    /// # Returns
    /// The node ID as a 40-character hex string
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const nodeId = conn.clusterMyId();
    /// console.log(`Current node ID: ${nodeId}`);
    /// // Output: Current node ID: 07c37dfeb235213a872192d90877d0cd55635b91
    /// ```
    #[napi]
    pub fn cluster_my_id(&mut self) -> Result<String> {
        redis::cmd("CLUSTER")
            .arg("MYID")
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("CLUSTER MYID failed: {}", e)))
    }

    /// CLUSTER REPLICAS - Get replicas of a node
    ///
    /// Returns information about all replica nodes of the specified master node.
    ///
    /// # Arguments
    /// * `node_id` - The master node ID
    ///
    /// # Returns
    /// Array of replica node information strings
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const replicas = conn.clusterReplicas("07c37dfeb235213a872192d90877d0cd55635b91");
    /// console.log(`Replicas:`, replicas);
    /// ```
    #[napi]
    pub fn cluster_replicas(&mut self, node_id: String) -> Result<Vec<String>> {
        let result: Vec<String> = redis::cmd("CLUSTER")
            .arg("REPLICAS")
            .arg(node_id)
            .query(&mut self.inner)
            .map_err(|e| napi_ohos::Error::from_reason(format!("CLUSTER REPLICAS failed: {}", e)))?;

        Ok(result)
    }

    // Helper function to convert Redis Value to JSON string
    fn redis_value_to_json(&self, value: &redis::Value) -> Result<String> {
        use redis::Value;

        let json_value = match value {
            Value::Nil => serde_json::Value::Null,
            Value::Int(i) => serde_json::Value::Number((*i).into()),
            Value::BulkString(bytes) => {
                let s = String::from_utf8_lossy(bytes);
                serde_json::Value::String(s.to_string())
            }
            Value::Array(arr) => {
                let mut json_arr = Vec::new();
                for item in arr {
                    let json_str = self.redis_value_to_json(item)?;
                    let json_val: serde_json::Value = serde_json::from_str(&json_str)
                        .map_err(|e| napi_ohos::Error::from_reason(format!("JSON parse failed: {}", e)))?;
                    json_arr.push(json_val);
                }
                serde_json::Value::Array(json_arr)
            }
            Value::SimpleString(s) => serde_json::Value::String(s.clone()),
            Value::Okay => serde_json::Value::String("OK".to_string()),
            Value::Map(map) => {
                let mut json_map = serde_json::Map::new();
                for (k, v) in map {
                    let key_str = match k {
                        Value::BulkString(bytes) => String::from_utf8_lossy(bytes).to_string(),
                        Value::SimpleString(s) => s.clone(),
                        _ => format!("{:?}", k),
                    };
                    let val_str = self.redis_value_to_json(v)?;
                    let val_json: serde_json::Value = serde_json::from_str(&val_str)
                        .map_err(|e| napi_ohos::Error::from_reason(format!("JSON parse failed: {}", e)))?;
                    json_map.insert(key_str, val_json);
                }
                serde_json::Value::Object(json_map)
            }
            Value::Attribute { data, attributes } => {
                let mut json_map = serde_json::Map::new();

                // Add data
                let data_str = self.redis_value_to_json(data)?;
                let data_json: serde_json::Value = serde_json::from_str(&data_str)
                    .map_err(|e| napi_ohos::Error::from_reason(format!("JSON parse failed: {}", e)))?;
                json_map.insert("data".to_string(), data_json);

                // Add attributes as Map
                let mut attrs_map = serde_json::Map::new();
                for (k, v) in attributes {
                    let key_str = match k {
                        Value::BulkString(bytes) => String::from_utf8_lossy(bytes).to_string(),
                        Value::SimpleString(s) => s.clone(),
                        _ => format!("{:?}", k),
                    };
                    let val_str = self.redis_value_to_json(v)?;
                    let val_json: serde_json::Value = serde_json::from_str(&val_str)
                        .map_err(|e| napi_ohos::Error::from_reason(format!("JSON parse failed: {}", e)))?;
                    attrs_map.insert(key_str, val_json);
                }
                json_map.insert("attributes".to_string(), serde_json::Value::Object(attrs_map));

                serde_json::Value::Object(json_map)
            }
            Value::Set(set) => {
                let mut json_arr = Vec::new();
                for item in set {
                    let json_str = self.redis_value_to_json(item)?;
                    let json_val: serde_json::Value = serde_json::from_str(&json_str)
                        .map_err(|e| napi_ohos::Error::from_reason(format!("JSON parse failed: {}", e)))?;
                    json_arr.push(json_val);
                }
                serde_json::Value::Array(json_arr)
            }
            Value::Double(f) => {
                serde_json::Value::Number(
                    serde_json::Number::from_f64(*f)
                        .ok_or_else(|| napi_ohos::Error::from_reason("Invalid float value"))?
                )
            }
            Value::Boolean(b) => serde_json::Value::Bool(*b),
            Value::VerbatimString { format: _, text } => {
                serde_json::Value::String(text.clone())
            }
            Value::BigNumber(n) => {
                // BigNumber is BigInt when num-bigint feature is enabled (default in redis)
                serde_json::Value::String(n.to_string())
            }
            Value::Push { kind, data } => {
                let mut json_map = serde_json::Map::new();

                let kind_str = format!("{:?}", kind);
                json_map.insert("kind".to_string(), serde_json::Value::String(kind_str));

                let mut json_arr = Vec::new();
                for item in data {
                    let json_str = self.redis_value_to_json(item)?;
                    let json_val: serde_json::Value = serde_json::from_str(&json_str)
                        .map_err(|e| napi_ohos::Error::from_reason(format!("JSON parse failed: {}", e)))?;
                    json_arr.push(json_val);
                }
                json_map.insert("data".to_string(), serde_json::Value::Array(json_arr));

                serde_json::Value::Object(json_map)
            }
            Value::ServerError(err) => {
                serde_json::Value::String(format!("ERROR: {}", err))
            }
            // Handle any future variants that might be added
            _ => {
                serde_json::Value::String(format!("{:?}", value))
            }
        };

        serde_json::to_string(&json_value)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON serialization failed: {}", e)))
    }
}
