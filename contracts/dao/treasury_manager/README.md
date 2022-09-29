# Treasury Contract
* [Introduction](#Introduction)
* [Sections](#Sections)
    * [Init](#Init)
    * [Interface](#Interface)
        * [Manager](/packages/shade_protocol/src/MANAGER.md)
        * Messages
            * [Receive](#Receive)
            * [UpdateConfig](#UpdateConfig)
            * [RegisterAsset](#RegisterAsset)
            * [Allocate](#Allocate)
            * [AddHolder](#AddHolder)
            * [RemoveHolder](#RemoveHolder)
        * Queries
            * [Config](#Config)
            * [Assets](#Assets)
            * [Allocations](#Allocations)
            * [PendingAllowance](#PendingAllowance)
            * [Holders](#Holders)
            * [Holding](#Holding)
            * [Metrics](#Metrics)
# Introduction
The treasury manager manages different holders (e.g. treasury & shd staking) funds for farming.
Funds deposited by different configured "holders" will be credited to their "holding" for later withdraw by that holder.
Gains earned & losses realized from farming will be credited to the treasury by default, future implementation should allow for proportional distribution

# Sections

## Init
##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
|admin_auth     | Contract |  Shade admin auth
|viewing_key | String |  Key set on relevant SNIP-20's
|treasury    | Addr |  treasury that is owner of funds

## Interface

### Messages
#### UpdateConfig
Updates the given values
##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
|config    | Config   |  New contract config
##### Response
```json
{
  "update_config": {
    "status": "success"
  }
}
```

#### RegisterAsset
Registers a supported asset. The asset must be SNIP-20 compliant since [RegisterReceive](https://github.com/SecretFoundation/SNIPs/blob/master/SNIP-20.md#RegisterReceive) is called.

Note: Will return an error if there's an asset with that address already registered.
##### Request
|Name        |Type    |Description                                                                                                            | optional |
|------------|--------|-----------------------------------------------------------------------------------------------------------------------|----------|
|contract    | Contract |  Type explained [here](#Contract)                                                                                     |  no      |
##### Response
```json
{
  "register_asset": {
    "status": "success"
  }
}
```

#### Allocate
Registers a supported asset. The asset must be SNIP-20 compliant since [RegisterReceive](https://github.com/SecretFoundation/SNIPs/blob/master/SNIP-20.md#RegisterReceive) is called.

Note: Will return an error if there's an asset with that address already registered.
##### Request
|Name        |Type    |Description                                                                                                            | optional |
|------------|--------|-----------------------------------------------------------------------------------------------------------------------|----------|
|asset       | Addr |  Desired SNIP-20
|allocation  | Allocation | Allocation data
##### Response
```json
{
  "allocate": {
    "status": "success"
  }
}
```

### Queries

#### Config
Gets the contract's configuration variables
##### Response
```json
{
  "config": {
    "config": { .. }
  }
}
```

#### Assets
Get the list of registered assets
##### Response
```json
{
  "assets": {
    "assets": ["asset address", ..],
  }
}
```

#### Allocations
Get the allocations for a given asset

##### Request
|Name        |Type    |Description                                                                                                            | optional |
|------------|--------|-----------------------------------------------------------------------------------------------------------------------|----------|
|asset      | Addr | Address of desired SNIP-20 asset

##### Response
```json
{
  "allocations": {
    "allocations": [
      {
        "allocation": {},
      },
      ..
    ],
  }
}
```

#### PendingAllowance
Get the pending allowance for a given asset

##### Request
|Name        |Type    |Description                                                                                                            | optional |
|------------|--------|-----------------------------------------------------------------------------------------------------------------------|----------|
|asset      | Addr | Address of desired SNIP-20 asset

##### Response
```json
{
  "pending_allowance": {
    "amount": "100000",
  }
}
```
