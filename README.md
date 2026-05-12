# Craftr

Desktop prompt enhancement tool with a monorepo structure.

## Structure

*   `app/` → Rust desktop application (system tray, hotkeys, Groq API integration).
*   `web/` → Next.js website (landing page, dashboard).
*   `backend/` → FastAPI license server (validates Pro subscriptions).

## How to run

### App (Rust Desktop)
Navigate to the `app/` directory and run:
```bash
cargo build --release
./target/release/craftr.exe
```

### Web (Next.js)
Navigate to the `web/` directory and run:
```bash
npm install
npm run dev
```

### Backend (FastAPI)
Navigate to the `backend/` directory and run:
```bash
pip install -r requirements.txt
uvicorn main:app --reload
```
