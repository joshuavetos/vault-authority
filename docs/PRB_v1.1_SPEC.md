# PRB v1.1 Specification (Sealed)

## 1. Normalization Standard (Norm-v1.1)
All system outputs must be normalized before hashing to ensure bitwise integrity:
1. Trim leading/trailing whitespace.
2. Normalize line endings (CRLF -> LF).
3. Collapse internal multi-space to a single space.

## 2. Canonical Test Vectors
- **Vector 1: Logical Integrity (FizzBuzz 1-20)**
  - String: `1,2,fizz,4,buzz,fizz,7,8,fizz,buzz,11,fizz,13,14,fizzbuzz,16,17,fizz,19,buzz`
  - SHA256: `f33e3873919e8361559c3d40a34b413e117188719d380e2d091a136746618589`

- **Vector 2: Closed-Form Math ((123^7) mod 13)**
  - String: `6`
  - SHA256: `e7f6c011776e8db7cd330b54174fd76f7d0216b612387a5ffcfb81e6f0919683`

## 3. Certification Gates
- **Pass Threshold**: 95/100 runs must pass across all vectors.
- **Revocation**: Any "Partial Success" (F2) results in immediate decertification.
