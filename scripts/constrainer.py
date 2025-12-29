#!/usr/bin/env python3
import sys
import requests
import yaml
import re

# Vault Authority v1.2+: The Constrainer Lite
# Rationale: AI output is unverified payload. Audit before delivery.

def load_config():
    with open("scripts/constraints.yaml", "r") as f:
        return yaml.safe_load(f)

def audit_output(text, config):
    for rule in config['constraints']:
        if re.search(rule['regex'], text, re.IGNORECASE):
            return False, rule['message']
    return True, text

def call_llm(prompt, api_key):
    # Pinned to Claude 3.5 Sonnet for deterministic v0.1 testing
    headers = {
        "x-api-key": api_key,
        "anthropic-version": "2023-06-01",
        "content-type": "application/json"
    }
    payload = {
        "model": "claude-3-5-sonnet-20240620",
        "max_tokens": 1024,
        "messages": [{"role": "user", "content": prompt}]
    }
    resp = requests.post("https://api.anthropic.com/v1/messages", json=payload, headers=headers)
    resp.raise_for_status()
    return resp.json()['content'][0]['text']

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: ./constrainer.py \"<prompt>\" <api_key>")
        sys.exit(1)

    config = load_config()
    prompt, key = sys.argv[1], sys.argv[2]
    
    print(f"[AUDIT] Initializing gate for prompt: '{prompt[:30]}...'")
    
    try:
        raw_output = call_llm(prompt, key)
        passed, result = audit_output(raw_output, config)

        if passed:
            print(f"--- [AUDIT PASS] ---")
            print(raw_output)
        else:
            print(f"--- [AUDIT BLOCKED] ---")
            print(f"REASON: {result}")
            # We log the violation to maintain the "Workflow Memory" (Perplexity [2])
            print("\n[EVIDENCE RETAINED FOR POSTMORTEM]:")
            print(raw_output)
            sys.exit(1)
    except Exception as e:
        print(f"[AUDIT ERROR] Fail-Closed triggered: {e}")
        sys.exit(1)
