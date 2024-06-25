# Parallel-wasm-playground

Note: Cross origin isolation must be enabled.
`npm run start` does not enable COI. Instead, please follow the steps in this README below.

## Install packages

```bash
npm i
```

## Build Wasm

```bash
npm run build:wasm
```

## Build React

```bash
npm run build
```

## Start Server

```bash
cd build
npx statikk --port 8000 --coi
```
