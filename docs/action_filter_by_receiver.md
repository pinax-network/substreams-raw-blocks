## Filter by `receiver`

The `receiver` field in the `actions` TABLE is the account that is notified in the inline action. This is different from the `account` field which is the contract executing the action.

To only filter unique actions excluding the additional notification inline actions, you can use the following query:

```sql
SELECT * FROM eos.actions WHERE receiver == account
```
