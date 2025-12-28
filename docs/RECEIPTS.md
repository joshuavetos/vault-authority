Receipts

A receipt is issued only after successful execution.

Receipt fields:
   •   trace_id
   •   failure_id
   •   signature

The signature is generated over the immutable payload:
trace_id:failure_id

Properties:
   •   Cannot be forged without the signing key
   •   Cannot exist without successful execution
   •   Cannot be created retroactively

Receipts are append-only and immutable.

If a receipt exists, the system acted.
If a receipt does not exist, the system did not act.
