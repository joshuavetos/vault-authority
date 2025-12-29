# Taxonomy Governance

## 1. Adding New Actions
To add a new remediation, follow the **Bitwise-Safe Checklist**:
1. **Idempotency**: Ensure the new script can run multiple times without causing damage.
2. **Bounds**: The script must target a specific, narrow fix (no "unbounded" deletes).
3. **Registration**: Add the `failure_id` and the absolute path of the script to `playbook.yaml`.
4. **Validation**: Run `cargo test redteam` to ensure the new gate is recognized.

## 2. Approved Failure IDs
Current authorized failures in v1.0:
* `ERR_AUTH_EXPIRED`
* `ERR_RATE_LIMIT`
* `ERR_DISK_FULL`
* `ERR_ZOMBIE_PROCESS`
* `ERR_DB_CON_LEAK`
* `ERR_NET_UNREACHABLE`
* `ERR_MEM_PRESSURE`


