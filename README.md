# Expenses

Simple expense tracker for general usage.

```bash
expenses list
```

```
+--------------+--------------+-------------+---------------+-----------+---------+
| expense_name | last_payment | periodicity | next_due_date | days_left | is_paid |
+--------------+--------------+-------------+---------------+-----------+---------+
| Electricity  | Not paid     | Monthly     | 2025-12-10    | 25        | ❌      |
+--------------+--------------+-------------+---------------+-----------+---------+
| Internet     | Not paid     | Monthly     | 2025-12-05    | 20        | ❌      |
+--------------+--------------+-------------+---------------+-----------+---------+
| Spotify      | Not paid     | Monthly     | 2025-12-04    | 19        | ❌      |
+--------------+--------------+-------------+---------------+-----------+---------+
| Netflix      | Not paid     | Monthly     | 2025-12-15    | 30        | ❌      |
+--------------+--------------+-------------+---------------+-----------+---------+
```

```bash
expenses add Electricity -d 2025-12-10
```

```bash
expenses pay Electricity
```

## Installing

Just grab a release from [the releases page](https://github.com/AloizioMacedo/expenses/releases) and you can use the executable directly.

## Building

You can build the executable by just running `cargo build -r` in case you have Rust installed.
