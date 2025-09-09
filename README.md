## 📌 Scaligator – Kubernetes Autoscaling on Steroids 🚀

Scaligator is an intelligent Kubernetes Horizontal Pod Autoscaler (HPA) alternative, built in Rust for speed, resilience, and extensibility.
It dynamically scales workloads based on custom Prometheus metrics, time-based rules, and business-aware logic going beyond vanilla Kubernetes autoscaling.
### ✅ Why Scaligator?

  - 🔍 More than CPU/Memory: Scale based on Prometheus metrics or any custom metric.

  - 🕐 Time-based scaling: Automatically scale down non-critical environments at night.

  - ⚡ Rust Performance: Lightweight, blazing fast, and built with safety in mind.

  - 🔒 Production-ready: Config-driven, highly observable, and Kubernetes-native.

  - 🌐 HTTP Alerts: React to external triggers for proactive scaling.

### 🔑 Features

   -  ✅ Kubernetes controller for scaling Deployments

   - ✅ Configurable via Config file or Environment variables

   - ✅ Prometheus integration for metric-driven scaling

   - ✅ REST endpoint for handling alert-based triggers

   - ✅ Graceful shutdown & robust logging

### 📦 Installation
   - Clone the repo and build the image:
   ```bash
   git clone https://github.com/p-r-a-v-i-n/scaligator.git
   cd scaligator
   docker build -t scaligator:latest .
   ```
   - Load the image into your local cluster (kind/minikube):
   ```bash
   kind load docker-image scaligator:latest
   # or
   minikube image load scaligator:latest
   ```
   - Deploy to Kubernetes:
   ```bash
   kubectl apply -f prometheus-rbac.yaml
   kubectl apply -f scaligator-serviceaccount.yaml
   kubectl apply -f scaligator-clusterrole.yaml
   kubectl apply -f scaligator-clusterrolebinding.yaml
   kubectl apply -f scaligator-deployment.yaml
   kubectl apply -f scaligator-service.yaml
   ```
   (Optionally integrate with Prometheus & Alertmanager using scaligator-rules.yaml and scaligator-alertmanager.yaml)

### ⚙️ Configuration
   - Scaligator supports:
      - Environment variables (override config file)
      - Config file (config.toml)
      - Kubernetes ConfigMap
   - Example config:
   ```bash
   prometheus_url = "http://prometheus-operated.monitoring:9090"
   watch_namespaces = ["default", "scaling", "dev"]
   scale_up_cpu_threshold = 0.7
   scale_down_cpu_threshold = 0.2
   disable_dev_after = "20:00"
   enable_dev_after = "08:00"

   ```
   - Load via ConfigMap:
   ```bash
   kubectl create configmap scaligator-config \
   --from-file=config.toml=./config/config.toml
   ```

### 🚀 Usage Examples
    Here are a few examples of how to run Scaligator with different configurations.

    1. Using a Configuration File
    You can run Scaligator with a custom configuration file by using the --config (or -c) flag.

    - Create a Config.toml file:

    ```toml
    prometheus_url = "[http://prometheus-operated.monitoring:9090](http://prometheus-operated.monitoring:9090)"
    watch_namespaces = "default,scaling,dev"
    scale_up_cpu_threshold = 0.75
    scale_down_cpu_threshold = 0.25
    reconcile_interval = 60
    ```

    - Run Scaligator with the config file:

    ```bash
    cargo run -- -c /path/to/your/Config.toml
    ```

    2. Using Environment Variables
    Scaligator can also be configured using environment variables with the SCALIGATOR_ prefix. This is especially useful in CI/CD pipelines or Kubernetes deployments.

    - Set the environment variables:

    ```bash
    export SCALIGATOR_PROMETHEUS_URL="[http://prometheus.monitoring.svc.cluster.local:9090](http://prometheus.monitoring.svc.cluster.local:9090)"
    export SCALIGATOR_WATCH_NAMESPACES="production,staging"
    export SCALIGATOR_SCALE_UP_CPU_THRESHOLD="0.8"
    export SCALIGATOR_SCALE_DOWN_CPU_THRESHOLD="0.3"
    ```

    - Run Scaligator:

    ```bash
    cargo run
    ```

    3. Running with Docker
    You can run Scaligator using the pre-built Docker image. This is the recommended way to run Scaligator in a production environment.

    - Build the Docker image:

    ```bash
    docker build -t scaligator:latest .
    ```

    - Run the Docker container with environment variables:


    ```bash
    docker run \
      -e SCALIGATOR_PROMETHEUS_URL="[http://prometheus.monitoring.svc.cluster.local:9090](http://prometheus.monitoring.svc.cluster.local:9090)" \
      -e SCALIGATOR_WATCH_NAMESPACES="default" \
      scaligator:latest
    ```

### 📊 Prometheus & Alertmanager Integration
   #### 1. Prometheus Scraping
   - Update your prometheus.yml:
   ```bash
   scrape_configs:
   - job_name: scaligator
      static_configs:
         - targets: ["scaligator.default.svc.cluster.local:8080"]
   ```

   #### 2. Prometheus Rules
   - Apply scaling rules:
   ```bash
   apiVersion: monitoring.coreos.com/v1
   kind: PrometheusRule
   metadata:
   name: scaligator-cpu-rules
   namespace: monitoring
   spec:
   groups:
      - name: scaligator.rules
         rules:
         - alert: HighCPUUsage
            expr: |
               rate(container_cpu_usage_seconds_total{namespace=~"default|scaling|dev"}[2m]) > 0.7
            for: 2m
            labels:
               severity: warning
            annotations:
               summary: "High CPU usage detected"
               description: "Pod {{ $labels.pod }} in {{ $labels.namespace }} using > 70% CPU"
   ```
   ```bash
   kubectl apply -f scaligator-rules.yaml
   ```
   #### 3. Alertmanager Config
   - Forward alerts to Scaligator:
   ```bash
      apiVersion: monitoring.coreos.com/v1alpha1
      kind: AlertmanagerConfig
      metadata:
      name: scaligator-alerts
      namespace: monitoring
      spec:
      route:
         receiver: scaligator
      receivers:
         - name: scaligator
            webhookConfigs:
            - url: "http://scaligator.default.svc.cluster.local:8080/alerts"
   ```

   ```bash
   kubectl apply -f scaligator-alertmanager.yaml
   ```

### 🔄 How Scaling Works
   - Scaligator supports two complementary modes:
      #### 1. 📊 Metric-Based Scaling
      - Runs a reconciliation loop that queries Prometheus for CPU/memory usage.
      - Adjusts Deployment replicas automatically based on thresholds.
      - Example logs:
      ```bash
      INFO scaligator::controller: 👀 Reconciling namespace: "default"
      INFO scaligator::scaler: 🔼 Scaling up myapp to 3 replicas (CPU: 0.82)
      INFO scaligator::scaler: 🔽 Scaling down myapp to 1 replicas (CPU: 0.15)
      INFO scaligator::scaler: 🟢 No scaling needed for myapp (CPU: 0.45)

      ```
      #### 2. ⚡ Alert-Based Scaling
      - Exposes a webhook for Alertmanager.
      - External alerts can trigger immediate scale actions.
      - Example:
      ```bash
      INFO scaligator::alerts: 📢 Global alert status: firing
      INFO scaligator::alerts: 📢 Individual alert status: firing
      INFO scaligator::alerts: 🚨 Alert: pod=myapp-7d9c8c9c4f-abcde, ns=dev, cpu=0.92

      ```
### 🔍 Verification
   - Check logs:
   ```bash
   kubectl logs -l app=scaligator
   ```
   - Expected output:
   ```bash
   🚀 Starting scaligator...
   INFO scaligator:🔧 Loading application config...
   INFO scaligator:✅ Config loaded: AppConfig { prometheus_url: "http://prometheus-operated.monitoring:9090", watch_namespaces: "default,scaling,dev", scale_up_cpu_threshold: 0.7, scale_down_cpu_threshold: 0.2, reconcile_interval: 30 }
   INFO scaligator:🔧 Inferring Kubernetes config...
   INFO scaligator:✅ Kubernetes client initialized
   INFO scaligator:🌐 Starting HTTP server on 0.0.0.0:8080
   INFO scaligator::controller:🚀 Controller starting reconciliation loop. Checking every 30s.
   INFO scaligator::controller:👀 Reconciling namespace: "default"
   INFO scaligator::controller:👀 Reconciling namespace: "scaling"
   INFO scaligator::controller:👀 Reconciling namespace: "dev"
   INFO scaligator::controller:✅ Reconciliation complete. Sleeping for 30s.

   ```

### 🛠️ Troubleshooting
   - 403 Forbidden on deployments → RBAC not applied → re-apply scaligator-clusterrole.yaml + scaligator-clusterrolebinding.yaml
   - Connection refused to Prometheus → update prometheus_url in config to point to your running Prometheus service
   - No scaling happening → check kubectl get events for denied patch operations

### 🤔 Scaligator vs KEDA
   - 🔍 Rust-based: tiny binary, low overhead, very fast
   - ⚡ Prometheus-native: simple queries + alert webhook, no complex CRDs
   - ⏰ Time-based scaling: scale down dev at night, scale up in morning (KEDA doesn’t support this out-of-the-box)
   - 🚨 Alert-based webhook scaling: direct integration with Alertmanager
   - 🪶 Config-driven & minimal: easier to reason about vs KEDA’s multiple CRDs
   - 🚀 Potential future: vertical scaling (planned for Kubernetes ≥1.33, KEDA doesn’t cover this yet)

### 💡 Why contribute?

   - 🦀 Written in Rust – great for learning systems programming in cloud-native environments

   - ☸️ Deep dive into Kubernetes controllers and autoscaling logic

   - 📈 Work on real-world scaling challenges in production scenarios

   -  🌍 Active community – shaping the future of autoscaling beyond HPA

   PRs welcome! Please open an issue first to discuss your ideas.
