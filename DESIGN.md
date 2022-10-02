Hydrogen contract is main hub that allows you to create, list and buy containers of hydrogen.

## Produce

```mermaid
sequenceDiagram
  participant Producer
  participant Contract
  Producer->>Contract: produce { color_spectrum, price, volume }
  note right of Contract: container with ID 0 is created and saved in key-value store on chain
  Contract->>Producer: Ok(Response)
```

## Update price

```mermaid
sequenceDiagram
  participant ContainerOwner
  participant Contract
  ContainerOwner->>Contract: update_price { container_id, new_price }
  note right of Contract: container with ID container_id saves new price in on-chain storage
  Contarct->>ContainerOwner: Ok(Response)
```

Faulty case

```mermaid
sequenceDiagram
  participant RandomUser
  participant Contract
  ContainerOwner->>Contract: update_price { container_id, new_price }
  note right of Contract: checks if user is an owner of this container
  Contarct->>ContainerOwner: Err(Unauthorized)
```

## Buy

```mermaid
sequenceDiagram
  participant Buyer
  participant Contract
  participant Producer
  Buyer->>Contract: buy { container_id, destination, coordinates } **plus tokens**
  note right of Contract: checks ownership and status (must be Created) and if proper amount was sent
  note right of Contract: changes status of container to Shipped
  Contract->>Buyer: Ok(Response)
```


No tokens are sent along with `buy` message:

```mermaid
sequenceDiagram:
  participant Buyer
  participant Contract
  participant Producer
  Buyer->>Contract: buy { container_id, destination, coordinates }
  note right of Contract: checks ownership and status (must be Created) and if proper amount was sent
  Contract->>Buyer: Err(NotEnoughTokensSent)
```


Producer trying to buy his own container:

```mermaid
sequenceDiagram
  participant Contract
  participant Producer
  Producer->>Contract: buy { container_id, destination, coordinates } plus tokens
  note right of Contract: checks ownership and sees container is already owned by Producer
  Contract->>Buyer: Err(ProducerCannotBuy)
```


## Close shipment

If container is already bought and has `Shipped` status, ownership still belongs to Producer. Both `Producer` and `Buyer` is allowed to "close shipment", which marks container as `Delivered` and transfers ownership to `Buyer`.

```mermaid
sequenceDiagram
  participant Buyer
  participant Contract
  participant Producer
  Buyer->>Contract: close_shipment { container_id }
  note right of Contract: checks if status is Shipped and if sender of message is the Buyer
  Contract->>Producer: bank_msg { to_address: Producer, amount: container.price }
  note right of Contract: changes status of container to Delivered and moves ownership to the Buyer
  Contract->>Buyer: Ok(Resposne)
```


Shippment can't be closed unless the Buyer confirms that it's been delivered and only then tokens are sent to Producer.

```mermaid
sequenceDiagram
  participant Buyer
  participant Contract
  participant Producer
  Producer->>Contract: close_shipment { container_id }
  note right of Contract: checks if status is Shipped and if sender of message is the Buyer
  Contract->>Producer: Err(ProducerCannotCloseShipment)
```


## Remove container

Container can be removed from memory at any point.

```mermaid
sequenceDiagram
  participant Owner
  participant Contract
  Owner->>Contract: remove_container { container_id }
  note right of Contract: checks if sender of message is an owner of that container
  Contract->>Owner: Ok(Response)
```

If container was already in shipped state, then tokens are being returned to the buyer.

```mermaid
sequenceDiagram
  participant Buyer
  participant Contract
  participant Producer
  Buyer->>Contract: buy { container_id, destination, coordinates } plus tokens
  note right of Contract: changes container's status to Shipped
  Producer->>Contract: remove_container { container_id }
  Contract->>Buyer: bank_msg { to_address: Buyer, amount: contract.price }
  note right of Contract: removes container from contract's storage
  Contract->>Producer: Ok(Response)
```

## Whole flow

```mermaid
sequenceDiagram
  participant Producer
  participant Contract
  participant Buyer
  Producer->>Contract: produce { color_spectrum, price, volume }
  note right of Contract: container with ID 0 is created and saved in key-value store on chain
  Contract->>Producer: Ok(Response)
  Buyer->>Contract: buy { container_id, destination, coordinates } **plus tokens**
  note right of Contract: changes status of container to Shipped
  Contract->>Buyer: Ok(Response)
  Buyer->>Contract: close_shipment { container_id }
  note right of Contract: checks if status is Shipped and if sender of message is the Buyer
  Contract->>Producer: bank_msg { to_address: Producer, amount: container.price }
  note right of Contract: changes status of container to Delivered and moves ownership to the Buyer
  Contract->>Buyer: Ok(Resposne)
  Buyer->>Contract: remove_container { container_id }
  Contract->>Buyer: Ok(Response)
```


