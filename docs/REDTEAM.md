Red-Team Test Suite

The red-team tests are executable proof, not documentation.

RT-01 Invalid Enum
Ensures intent smuggling is rejected before execution.

RT-02 Duplicate Trace
Ensures idempotence and prevents retry storms.

RT-05 Execution Failure
Ensures failed execution produces no receipt and no dedupe entry.

Passing all tests proves:
   •   The system cannot lie
   •   The system cannot partially succeed
   •   The system cannot be coerced into acting

Run:
cargo test
