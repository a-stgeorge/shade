# DAO Manager Interface
* [Introduction](#Introduction)
* [Sections](#Sections)
    * [Interface](#Interface)
        * Messages
            * [Unbond](#Unbond)
            * [Claim](#Claim)
            * [Update](#Update)
        * Queries
            * [Balance](#Balance)
            * [BatchBalance](#BatchBalance)
            * [Unbonding](#Unbonding)
            * [Claimable](#Claimable)
            * [Unbondable](#Unbondable)
            * [Reserves](#Reserves)

# Introduction
This interface is how external holders (e.g. treasury) will deposit funds for use by the DAO
  - Accepting deposits as an allowance OR sent tokens
  - managing/farming its funds
  - Reporting accurate balances
  - Unbonding & returning them to the manager in Unbond/Claim

NOTE: Because of how the contract implements this, all messages will be enclosed as:
```
{
  "manager": {
    <msg>
  }
}
```

# Sections

### Messages
#### Unbond
Begin unbonding of a given amount from a given asset

##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
|asset     | Addr |  SNIP-20 asset to unbond

##### Response
```json
{
  "unbond": {
    "amount": "100"
    "status": "success"
  }
}
```

#### Claim
Claim a given amount from completed unbonding of a given asset

##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
|asset     | Addr |  SNIP-20 asset to unbond

##### Response
```json
{
  "claim": {
    "amount": "100"
    "status": "success"
  }
}
```

#### Update
Perform routine maintenance tasks on a given asset

##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
|asset     | Addr |  SNIP-20 asset to unbond

##### Response
```json
{
  "update": {
    "status": "success"
  }
}
```

### Queries

#### Balance
Get the balance of a given asset, Error if unrecognized

##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
|asset     | Addr |  SNIP-20 asset to query
|holder    | Addr |  User's balance holdings to query e.g. treasury

##### Response
```json
{
  "balance": {
    "amount": "100000",
  }
}
```

#### BatchBalance
Get the balances of a multiple assets, 0 if unrecognized

##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
|asset     | Addr |  SNIP-20 asset to query
|holder    | Addr |  User's balance holdings to query e.g. treasury

##### Response
```json
{
  "batch_balance": {
    "amounts": ["100000", ...],
  }
}
```

#### Unbonding
Get the current unbonding amount of a given asset, Error if unrecognized

##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
|asset     | Addr |  SNIP-20 asset to query
|holder    | Addr |  User's balance holdings to query e.g. treasury

##### Response
```json
{
  "unbonding": {
    "amount": "100000",
  }
}
```

#### Claimable
Get the current claimable amount of a given asset, Error if unrecognized

##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
|asset     | Addr | SNIP-20 asset to query
|holder    | Addr |  User's balance holdings to query e.g. treasury

##### Response
```json
{
  "claimable": {
    "amount": "100000",
  }
}
```

#### Unbondable
Get the current unbondable amount of a given asset, Error if unrecognized

##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
|asset     | Addr | SNIP-20 asset to query
|holder    | Addr |  User's balance holdings to query e.g. treasury

##### Response
```json
{
  "unbondable": {
    "amount": "100000",
  }
}
```

#### Reserves
Get the current reserves of a given asset, amount currently owned by manager contract

##### Request
|Name      |Type      |Description                                                                                                        | optional |
|----------|----------|-------------------------------------------------------------------------------------------------------------------|----------|
|asset     | Addr | SNIP-20 asset to query
|holder    | Addr |  User's balance holdings to query e.g. treasury

##### Response
```json
{
  "reserves": {
    "amount": "100000",
  }
}
```
