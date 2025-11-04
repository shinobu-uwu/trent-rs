# trent

**trent** is a lightweight and ergonomic Rust wrapper around the
[YGOPRODeck API](https://ygoprodeck.com/api-guide/), providing an easy way to query Yu-Gi-Oh! cards

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
trent = "0.1"
```

## Example

You can get a card by name:

```rs
let client = Client::new();
let card = client.get_by_name("Trent").await.unwrap();

match card {
    Card::Normal(m) => println!("Card name: {}", m.info.name),
    _ => panic!("Unexpected variant"),
}
```

Or you can also use the `RequestBuilder` to create a better query:

```rs
let client = Client::new();
// get all dark normal monsters with 1800 atk
let request = RequestBuilder::new()
    .with_atk(1800)
    .with_attribute(Attribute::Dark)
    .with_type(CardType::NormalMonster)
    .build();
let cards = client.get(request).await.unwrap();

for card in card {
    match card {
        Card::NormalMonster(m) => println!("{}", m.info.name),
        _ => panic!("Unexpected variant"),
    }
}
```
