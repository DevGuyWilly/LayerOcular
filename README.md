# PersonaFinSight (e-finsight)

PersonaFinSight, also called e-finsight, is an AI-powered financial insights platform for analyzing banking transactions with RAG and multi-agent AI systems.

The original idea connects to bank accounts through TrueLayer, ingests recent transaction history, and uses AI to help with spending analysis, budget recommendations, investment advice, and natural language financial questions.

---

## API Endpoints

### Authentication
* **`POST`** `/api/v1/auth/signup`
* **`POST`** `/api/v1/auth/login`
* **`GET `** `/api/v1/auth/logout`

### Bank Connection
* **`GET `** `/api/v1/bank/connect`
* **`GET `** `/api/v1/bank/callback`

### Transactions
* **`POST`** `/api/v1/transactions/ingest`
* **`GET `** `/api/v1/transactions`
* **`GET `** `/api/v1/transactions/count`

---

> **Note:** This project is currently being rebuilt as a personal Java project in Rust.