<div align="center">
<p align="center">
  <a href="https://www.edgee.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.ai/img/component-dark.svg">
      <img src="https://cdn.edgee.ai/img/component.svg" height="100" alt="Edgee">
    </picture>
  </a>
</p>
</div>

<h1 align="center">GoSquared component for Edgee</h1>

[![Coverage Status](https://coveralls.io/repos/github/edgee-ai/gosquared-component/badge.svg)](https://coveralls.io/github/edgee-ai/gosquared-component)
[![GitHub issues](https://img.shields.io/github/issues/edgee-ai/gosquared-component.svg)](https://github.com/edgee-ai/gosquared-component/issues)
[![Edgee Component Registry](https://img.shields.io/badge/Edgee_Component_Registry-Public-green.svg)](https://www.edgee.ai/edgee/gosquared)

This is a Rust-based Edgee component that integrates **GoSquared analytics** using the Edgee Data Collection protocol. It enables you to track events, monitor page views, and identify users—sending data to GoSquared via their `/event`, `/pageview`, and `/identify` endpoints.

---

## ✨ Features

- ✅ Track custom events (`track`)
- ✅ Track page views (`page`)
- ✅ Identify and update users (`user`)
- ✅ Built for Edge execution: fast, secure, serverless

---

## 🔧 Settings

This component requires the following settings:

| Key            | Type   | Required | Description                                              |
|----------------|--------|----------|----------------------------------------------------------|
| `api_key`      | string | ✅       | Your GoSquared **API Key** from project settings         |
| `site_token`   | string | ✅       | Your GoSquared **Site Token** identifying the property   |

---

## 🧪 Testing Locally

### 🛠️ Build the component

```bash
edgee component build
```

### ✅ Run unit tests

```bash
edgee component build
```

### 🔍 Run a live test with simulated events
```bash
edgee components test \
  --event-type track \
  --settings api_key=YOUR_API_KEY,site_token=YOUR_SITE_TOKEN \
  --make-http-request

```

### 🚀 Deploy to Edgee Registry
```bash
edgee components push
```


### 📂 Project Structure
```
gosquared-component/
├── src/
│   └── lib.rs                 # Main component logic
├── target/
│   └── wasm32-wasip2/
│       └── release/
│           └── gosquared.wasm  # Built WebAssembly output
├── gosquared.png              # Component icon
├── Cargo.toml                 # Rust dependencies
└── edgee-component.toml       # Edgee manifest

```

### 📚 Learn More

- [GoSquared HTTP Tracking Docs](https://www.gosquared.com/docs/tracking/overview)
- [Edgee Developer Guide](https://www.edgee.ai/docs/services/registry/developer-guide)