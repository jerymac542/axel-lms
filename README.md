# Axel LMS — Powered by Apexos

> **Enterprise-grade Learning Management System** — Self-hosted, multi-tenant, standards-compliant, AI-native.

## Vision

Axel LMS rivals Open edX in capability but with modern architecture:
- **Polyglot microservices** (Rust/Python/PHP) for performance, AI, and developer velocity
- **Self-hosted first** — air-gapped, on-prem, hybrid cloud
- **Standards-native** — LTI 1.3 Advantage, SCORM 2004, xAPI, Caliper, OneRoster
- **AI-integrated** — adaptive learning, auto-grading, content generation, analytics
- **Extensible** — plugin architecture, XBlock-compatible, headless API

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
                              AXEL LMS PLATFORM
└─────────────────────────────────────────────────────────────────────────────┘

┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   APEXOS     │  │   APEXOS     │  │   APEXOS     │  │   APEXOS     │
│   CORE       │  │   AI         │  │   WEB        │  │   EDGE       │
│   (Rust)     │  │   (Python)   │  │   (PHP)      │  │   (Rust)     │
├──────────────┤  ├──────────────┤  ├──────────────┤  ├──────────────┤
│ • AuthZ      │  │ • Adaptive   │  │ • LMS UI     │  │ • CDN        │
│ • Real-time  │  │ • Auto-grade │  │ • Studio     │  │ • WAF        │
│ • Search     │  │ • Analytics  │  │ • Admin      │  │ • Rate Limit │
│ • Media      │  │ • Recomm.    │  │ • LTI/SCORM  │  │ • Geo DNS    │
│ • Scheduler  │  │ • NLP        │  │ • Reports    │  │ • Edge Cache │
└──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘
        │               │               │               │
        └───────────────┼───────────────┼───────────────┘
                        ▼
         ┌──────────────────────────────┐
         │       APEXOS MESSAGE BUS     │
         │       (NATS + Kafka)         │
         └──────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        ▼               ▼               ▼
┌───────────────┐ ┌───────────────┐ ┌───────────────┐
│  PostgreSQL   │ │    Redis      │ │   MinIO/S3    │
│  (Primary)    │ │  (Cache/Queue)│ │   (Objects)   │
└───────────────┘ └───────────────┘ └───────────────┘
```

## Service Catalog

### Apexos Core (Rust)
| Service | Responsibility | Scale Target |
|---------|---------------|--------------|
| `auth-service` | OAuth2/OIDC, RBAC, SCIM, MFA, sessions | 100K+ concurrent |
| `realtime-service` | WebSocket, presence, live collab, notifications | 50K+ connections |
| `search-service` | Full-text, faceted, ML ranking (Meilisearch/Typesense) | 10M+ docs |
| `media-service` | Transcoding, streaming, DRM, signed URLs | 100TB+ |
| `scheduler-service` | Cron, delayed jobs, workflow engine | 1M+ jobs/day |
| `core-lib` | Shared types, events, validation, crypto | — |

### Apexos AI (Python)
| Service | Responsibility | Scale Target |
|---------|---------------|--------------|
| `ai-service` | LLM orchestration, RAG, fine-tuning, embeddings | 10K req/s |
| `analytics-service` | Event pipeline, OLAP, dashboards, exports | 1B+ events/day |
| `grading-service` | Auto-grading (code, math, essay), rubrics | 100K submissions/day |
| `recommendation-service` | Content recs, learning paths, spacing | 1M+ recs/day |
| `nlp-service` | Translation, summarization, accessibility | 1M+ docs/day |
| `shared` | ML ops, feature store, model registry | — |

### Apexos Web (PHP/Laravel)
| Service | Responsibility | Scale Target |
|---------|---------------|--------------|
| `web-app` | Student LMS, courseware, discussions, certificates | 100K+ concurrent |
| `admin-panel` | Multi-tenant admin, org management, branding | 1K+ admins |
| `lti-provider` | LTI 1.3 Advantage platform & tool | 10K+ launches/day |
| `scorm-engine` | SCORM 2004 4th Ed, xAPI, cmi5 player | 10K+ sessions/day |
| `shared` | Domain models, policies, API resources | — |

## Standards Compliance

| Standard | Version | Status |
|----------|---------|--------|
| LTI | 1.3 + Advantage | ✅ Platform & Tool |
| SCORM | 2004 4th Edition | ✅ Runtime & Sequencing |
| xAPI | 1.0.3 | ✅ LRS Integrated |
| Caliper | 1.2 | ✅ Event Emission |
| OneRoster | 1.2 | ✅ SIS Sync |
| QTI | 3.0 | ✅ Assessment Import/Export |
| WCAG | 2.1 AA | ✅ Accessibility |

## Quick Start (Development)

```bash
# Prerequisites: Docker, Docker Compose, Make, age (secrets)
git clone https://github.com/apexos/axel-lms
cd axel-lms

# Generate dev secrets
make secrets

# Start infrastructure
docker compose -f infra/docker/compose.dev.yml up -d

# Run database migrations
make migrate

# Start all services (dev mode)
make dev

# Access
# LMS:        http://lms.axel.localhost
# Studio:     http://studio.axel.localhost
# Admin:      http://admin.axel.localhost
# API Docs:   http://api.axel.localhost/docs
```

## Production Deployment

```bash
# Air-gapped install (offline)
./dist/installer/axel-install.sh --airgap --config /etc/axel/config.yaml

# Kubernetes (Helm)
helm install axel ./dist/helm-charts/axel \
  --namespace axel \
  --create-namespace \
  -f values-production.yaml

# VM/Ansible
ansible-playbook -i inventory/prod infra/ansible/site.yml
```

## Project Structure

```
axel-lms/
├── .github/                 # CI/CD workflows
├── contracts/               # Shared contracts (proto, OpenAPI, JSON Schema)
│   ├── proto/               # gRPC definitions
│   ├── openapi/             # REST API specs
│   └── schemas/             # Event schemas (CloudEvents)
├── docs/                    # Architecture, API, operations
├── dist/                    # Distribution artifacts
│   ├── installer/           # Air-gap installer
│   ├── helm-charts/         # K8s deployments
│   └── airgap/              # Offline bundles
├── infra/                   # Infrastructure as Code
│   ├── terraform/           # Cloud resources
│   ├── helm/                # K8s base charts
│   ├── ansible/             # VM provisioning
│   └── docker/              # Compose files
├── php/                     # Apexos Web (Laravel)
│   ├── web-app/             # Student-facing LMS
│   ├── admin-panel/         # Multi-tenant admin
│   ├── lti-provider/        # LTI platform/tool
│   ├── scorm-engine/        # SCORM/xAPI player
│   └── shared/              # Domain kernel
├── python/                  # Apexos AI (FastAPI)
│   ├── ai-service/
│   ├── analytics-service/
│   ├── grading-service/
│   ├── recommendation-service/
│   ├── nlp-service/
│   └── shared/
├── rust/                    # Apexos Core (Rust)
│   ├── auth-service/
│   ├── realtime-service/
│   ├── search-service/
│   ├── media-service/
│   ├── scheduler-service/
│   ├── core-lib/
│   └── proto/
├── scripts/                 # Operational scripts
├── tests/                   # Integration, E2E, performance
└── Makefile                 # Top-level commands
```

## Development Principles

1. **API-first** — All services communicate via typed contracts (gRPC/REST/AsyncAPI)
2. **Observability-native** — OpenTelemetry, structured logs, metrics, traces by default
3. **Security by default** — mTLS, capability tokens, audit logs, encryption at rest/in transit
4. **Multi-tenant from day one** — Organization isolation, custom domains, data residency
5. **Upgradable** — Database migrations, schema evolution, blue-green deployments
6. **Offline-capable** — Air-gap installation, no external dependencies at runtime

## License

Axel LMS is licensed under **AGPL-3.0** with a **Commercial License Exception** for Apexos Enterprise customers.

See [LICENSE](LICENSE) and [COMMERCIAL_LICENSE](COMMERCIAL_LICENSE).

---

**Apexos** — The operating system for learning.  
https://apexos.io | https://github.com/apexos