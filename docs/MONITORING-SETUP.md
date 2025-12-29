# Monitoring Integration Guide

To connect **Vault Authority** to your existing Prometheus/Alertmanager stack, add the following to your `alertmanager.yml`:

```yaml
receivers:
- name: 'vault-remediator'
  webhook_configs:
  - url: 'http://<vault-server-ip>:8080/webhook'
    send_resolved: false

route:
  receiver: 'vault-remediator'
  group_by: ['alertname', 'incident_id']
  matchers:
    - severity="critical"
    - autoremove="true"
