# Operator's Handbook

## 1. Handling Rejections
If the Vault returns a `REJECTED` status, do not attempt to bypass it. Common reasons include:
* **DuplicateTrace**: The fix was already attempted. Check the audit logs.
* **InvalidFailureID**: The incident type is not in the approved taxonomy.
* **ExecutionFailed**: The shell playbook returned a non-zero exit code. Manual intervention is required.

## 2. Verification Protocol (PRB v1.1)
To audit a remediation, use the `prb-check.sh` utility:
`./prb-check.sh "[receipt_signature]" "[expected_hash]"`



## 3. Manual Override
In the event of a Vault failure, engineers may execute playbooks manually from the `playbooks/` directory. However, manual fixes will not generate a signed receipt and must be logged manually for compliance.
