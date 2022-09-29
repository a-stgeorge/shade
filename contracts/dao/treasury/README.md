# Treasury
* [Introduction](#Introduction)
* [Sections](#Sections)
    * [Init](#Init)
    * [Interface](#Interface)
        * Messages
            * [Receive](#Receive)
            * [UpdateConfig](#UpdateConfig)
            * [RegisterAsset](#RegisterAsset)
            * [RegisterManager](#RegisterManager)
            * [RegisterWrap](#RegisterWrap)
            * [WrapCoins](#WrapCoins)
            * [Allowance](#Allowance)
            * [Update](#Update)
            * [SetRunLevel](#SetRunLevel)
        * Queries
            * [Config](#Config)
            * [Assets](#Assets)
            * [Allowances](#Allowances)
            * [Allowance](#Allowance)
            * [RunLevel](#RunLevel)
            * [Metrics](#Metrics)
            * [Balance](#Balance)
            * [BatchBalance](#BatchBalance)
            * [Reserves](#Reserves)
# Introduction
The treasury contract holds all protocol owned funds, receiving fees from primitives

# Sections

## Init
##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
| admin_auth | Contract | Shade admin auth contract
| utility_router | Contract | Shade utility router contract
| viewing_key | string   | viewing key for all registered snip20 assets

## Interface

### Messages

#### UpdateConfig
Updates the config for provided values
##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
| admin_auth | Contract | Shade admin auth
| utility_router | Contract | Shade utility router

##### Response
```json
{
  "update_config": {
    "status": "success"
  }
}
```

#### RegisterAsset
Registers a SNIP-20 compliant asset since [RegisterReceive](https://github.com/SecretFoundation/SNIPs/blob/master/SNIP-20.md#RegisterReceive) is called.

Note: Will return an error if there's an asset with that address already registered.
##### Request
|Name        |Type    |Description                                                                                                            | optional |
|------------|--------|-----------------------------------------------------------------------------------------------------------------------|----------|
|contract    | Contract | Snip20 asset to register
##### Response
```json
{
  "register_asset": {
    "status": "success"
  }
}
```

#### RegisterManager
Register a manager contract to begin allowing funds

Note: Will return an error if there's an asset with that address already registered.
##### Request
|Name        |Type    |Description                                                                                                            | optional |
|------------|--------|-----------------------------------------------------------------------------------------------------------------------|----------|
|contract    | Contract | Contract that implements the Manager interface
##### Response
```json
{
  "register_manager": {
    "status": "success"
  }
}
```

#### RegisterWrap
Setup Layer-1 wrapping for a given denom, wrapped using snip20 previously registered with RegisterAsset

##### Request
|Name        |Type    |Description                                                                                                            | optional |
|------------|--------|-----------------------------------------------------------------------------------------------------------------------|----------|
|denom | string | IBC denom of the Layer-1 as it exists on Secret Network
|contract    | Contract | Snip20 contract previously registered, to be used for wrapping
##### Response
```json
{
  "register_wrap": {
    "status": "success"
  }
}
```

#### WrapCoins
Wrap all non-zero Layer-1 balances that are configured for wrapping

##### Response
```json
{
  "wrap_coins": {
    "success": [ list of Coin that were successfully wrapped ],
    "failed": [ list of Coin that couldn't be wrapped ],
  }
}
```

#### Allowance
Configure an allowance to an address

##### Request
|Name        |Type    |Description                                                                                                            | optional |
|------------|--------|-----------------------------------------------------------------------------------------------------------------------|----------|
| asset | addr | Snip20 Asset to be allowed
| allowance | Allowance | Allowance object containing configuration values

#### Allowance
Set of configuration values for a specific allowance instance
|Name        |Type    |Description                                                                                                            | optional |
|------------|--------|-----------------------------------------------------------------------------------------------------------------------|----------|
| spender | addr | Address to allow funds to
| allowance_type | AllowanceType | `portion` or `amount`, portion's are percentages of total balance, amount is a strict value
| cycle | Cycle | How often to refresh the allowance e.g. `once`, `constant`, `daily`, `monthly`, `yearly` etc.
| amount | number | Amount that should be allowed, for portions this is `percent * 10^18`, amount it will be a strict token amount
| tolerance | number | `percent * 10^18` to be used as a threshold for refreshing funds. Prevents refreshing negligible amount

##### Response
```json
{
  "allowance": {
    "status": "success"
  }
}
```

#### Update
Performs routine maintenance such as rebalancing funds & refreshing allowances for a specific asset

##### Request
|Name        |Type    |Description                                                                                                            | optional |
|------------|--------|-----------------------------------------------------------------------------------------------------------------------|----------|
| asset | addr | Snip20 Asset to be updated

##### Response
```json
{
  "update": {
    "status": "success"
  }
}
```

#### SetRunLevel
Change the contract run level

##### Request
|Name        |Type    |Description                                                                                                            | optional |
|------------|--------|-----------------------------------------------------------------------------------------------------------------------|----------|
| run_level | RunLevel | `normal | deactivated | migrating`

##### Response
```json
{
  "set_run_level": {
    "status": "success"
  }
}
```

### Queries

#### Config
Gets the contract's configuration
##### Response
```json
{
  "config": {
    "config": {
      "admin_auth": Contract,
      "utility_router": Contract,
    }
  }
}
```

#### Assets
List of assets supported
##### Response
```json
{
  "assets": {
    "assets": ["asset address", ...]
  }
}
```

#### Allowances
List of configured allowances
##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
|asset | Addr |  Asset to query balance of
##### Response
```json
{
  "allowances": {
    "allowances": [
      {
        "allowance": ...
      }, 
    ...]
  }
}
```

#### Allowance
Actual current allowance to a specific spender
##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
|asset | Addr |  Asset to query allowance for
|spender | Addr |  Spender of allowance
##### Response
```json
{
  "allowance": {
    "amount": "10000",
  }
}
```

#### RunLevel
Gets the current run level
##### Response
```json
{
  "run_level": {
    "run_level": "current run level",
  }
}
```

#### Metrics
Get a list of metrics for a certain timestamp & period

##### Request
NOTE: either `date` or `epoch` should be provided
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
| date | string | Datetime string for the desired metrics
| epoch | number | epoch timestamp in seconds for the desired metrics
| period | Period | `hour | day | month` for the range of metrics to get

##### Response
```json
{
  "metrics": {
    "metrics": [
        {
            "action": "increase_allowance | decrease_allowance | unbond | claim | funds_received | send_funds | wrap",
            "context": "receive | rebalance | migration | unbond | wrap",
            "timestamp": 12345,
            "token": "secret1234...",
            "amount": "1000",
            "user": "",
        }, ...
    ],
  }
}
```

#### Balance
Get the current treasury balance of a given asset including downstream manager balances

##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
| asset | addr | asset to get the balance of

##### Response
```json
{
  "balance": {
    "amount": "1234",
  }
}
```

#### BatchBalance
Get a list of treasury balances

##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
| assets | list[addr] | assets to get the balance of

##### Response
```json
[ "1234", "5678", ... ]
```

#### Reserves
Get the current treasury reserves of a given asset, not including downstream funds (assets actually owned by treasury contract)

##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
| asset | addr | asset to get the reserves  of

##### Response
```json
{
  "reserves": {
    "amount": "1234",
  }
}
```
