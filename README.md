# Budget Manager CLI

## `budget.toml` file

```toml
# This is cost example
[[subscriptions]]
label = "Scaleway"
amount = -35.0
date = "2024-01-08"
recurence = "monthly"

# This is an income example
[[subscriptions]]
label = "Rent"
amount = 2000.0
date = "2024-01-08"
recurence = { number_of_months = 1 }
```

## Commands

### Show

```bash
bdg show budget.toml
```
