# WCA Stacked Area Charts web app
> [!CAUTION]
> Broken after WCA updated their export to v2. Needs an update of the original WCA_SAC code.

# Setup
```sh
git submodule update --init --recursive && cd WCA_SAC && pip install -r requirements.txt
cd backend && cargo run
cd frontend && npm i && npm run dev
```
