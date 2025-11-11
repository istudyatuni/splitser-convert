# Splitser converter

Convert data from [splitser](https://splitser.com) to CSV for selected member

## How to use

1. Create file `data/expenses.json` and put here JSON response from devtools to request for `list_items`. Note that you need all data, not only for specific page. You can copy request as cURL/Powershell, change pagination parameters, and then execute locally
2. Create file `data/config.json` and put `user_id` of the desired member:

```json
{
	"user_id": "00000000-0000-0000-0000-000000000000"
}
```

3. Then just run:

```bash
cargo r
```
