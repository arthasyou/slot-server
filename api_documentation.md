# API 文档

## 1. 创建新的投注池

**端点**: `POST /manage/create_pool`

**描述**: 创建一个新的投注池，并将其存储到数据库中。返回创建的投注池的详细信息。

**请求体**:

```json
{
  "bet_unit": <number>,           // 每次投注的单位(万分比) 100表示一次1元
  "brokerage_ratio": <number>,    // 手续费比率 (万分比)
  "jackpot_ratio": <number>,      // 彩金比率 (万分比) 如果不需要设置成0
  "advance": <number>,            // 预付款金额
  "boundary": <number>            // 最大波动
}
```

**响应**:

- **状态码**: `200 OK`
- **响应体**:

```json
{
  "id": <number>,                 // 创建的投注池ID
  "bet_unit": <number>,           // 每次投注的单位(万分比)
  "boundary": <number>,           // 最大波动
  "brokerage_ratio": <number>,    // 手续费比率 (万分比)
  "jackpot_ratio": <number>,      // 彩金比率 (万分比
  "advance": <number>             // 预付款金额
}
```

**错误响应**:

- **状态码**: `400 Bad Request`
- **响应体**:

```json
{
  "error": "<Validation Error Message>"
}
```

## 2. 获取所有投注池

**端点**: `POST /manage/get_pools`

**描述**: 获取所有存储在数据库中的投注池的列表。

**请求体**: 无

**响应**:

- **状态码**: `200 OK`
- **响应体**:

```json
[
  {
    "id": <number>,                 // 投注池ID
    "bet_unit": <number>,           // 每次投注的单位
    "boundary": <number>,           // 边界金额
    "brokerage_ratio": <number>,    // 手续费比率 (万分比)
    "jackpot_ratio": <number>,      // 奖池比率 (万分比)
    "advance": <number>             // 预付款金额
  },
  ...
]
```

**错误响应**:

- **状态码**: `500 Internal Server Error`
- **响应体**:

```json
{
  "error": "Internal server error message"
}
```

## 3. 水果机抽奖 (Fruit Draw)

**端点**: `POST /game/fruit_draw`

**描述**: 对特定的投注池进行水果机抽奖，并返回抽奖结果。

**请求体**:

```json
{
  "pool_id": <number>,       // 要进行抽奖的投注池ID
  "bets": <array>            // 用户的投注详情
}
```

**响应**:

- **状态码**: `200 OK`
- **响应体**:

```json
{
  "result": <object>         // 抽奖结果，包括中奖信息等
}
```

**错误响应**:

- **状态码**: `400 Bad Request`
- **响应体**:

```json
{
  "error": "pool id not existed"
}
```

或

```json
{
  "error": "<Validation Error Message>"
}
```

## 4. 简单抽奖 (Simple Draw)

**端点**: `POST /game/simple_draw`

**描述**: 对特定的投注池进行简单抽奖，并返回抽奖结果。

**请求体**:

```json
{
  "pool_id": <number>,       // 要进行抽奖的投注池ID
  "bets": <array>,           // 用户的投注详情
  "odds": <number>           // 抽奖的赔率
}
```

**响应**:

- **状态码**: `200 OK`
- **响应体**:

```json
{
  "flag": <boolean>,         // 是否中奖的标志
  "reward": <number>         // 奖金金额
}
```

**错误响应**:

- **状态码**: `400 Bad Request`
- **响应体**:

```json
{
  "error": "pool id not existed"
}
```

或

```json
{
  "error": "<Validation Error Message>"
}
```

## 请求示例

**创建投注池请求示例**:

```http
POST /manage/create_pool
Content-Type: application/json

{
  "bet_unit": 100,
  "brokerage_ratio": 10,
  "jackpot_ratio": 15,
  "advance": 1000,
  "boundary": 5000
}
```

**获取投注池列表请求示例**:

```http
POST /manage/get_pools
```

**水果机抽奖请求示例**:

```http
POST /game/fruit_draw
Content-Type: application/json

{
  "pool_id": 1,
  "bets": [10, 20, 30]
}
```

**简单抽奖请求示例**:

```http
POST /game/simple_draw
Content-Type: application/json

{
  "pool_id": 1,
  "bets": [10, 20],
  "odds": 1.5
}
```

## 响应示例

**创建投注池响应示例**:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": 1,
  "bet_unit": 100,
  "boundary": 5000,
  "brokerage_ratio": 10,
  "jackpot_ratio": 15,
  "advance": 1000
}
```

**获取投注池列表响应示例**:

```http
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": 1,
    "bet_unit": 100,
    "boundary": 5000,
    "brokerage_ratio": 10,
    "jackpot_ratio": 15,
    "advance": 1000
  },
  {
    "id": 2,
    "bet_unit": 200,
    "boundary": 8000,
    "brokerage_ratio": 12,
    "jackpot_ratio": 18,
    "advance": 1500
  }
]
```

**水果机抽奖响应示例**:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "result": {
    "win": true,
    "amount": 500
  }
}
```

**简单抽奖响应示例**:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "flag": true,
  "reward": 300
}
```

## 错误示例

**创建投注池错误响应示例**:

```http
HTTP/1.1 400 Bad Request
Content-Type: application/json

{
  "error": "Invalid input: bet_unit must be greater than zero."
}
```

**水果机抽奖错误响应示例**:

```http
HTTP/1.1 400 Bad Request
Content-Type: application/json

{
  "error": "pool id not existed"
}
```

或

```http
HTTP/1.1 400 Bad Request
Content-Type: application/json

{
  "error": "Invalid input: pool_id must be greater than zero."
}
```
