### Filtering Actions by `receiver` Field

In the `actions` table, the `receiver` field represents the account notified during an inline action, while the `account` field is the contract executing the action.

An action can have multiple `receiver` values, resulting in additional rows for notifications (e.g., a token transfer creates three rows: one contract action and two notifications for the `from` and `to` accounts).

To filter only the unique contract actions and exclude notifications, use:

```sql
SELECT * FROM eos.actions WHERE receiver = account;
```