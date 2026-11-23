# wasm-image-editor

Практическое задание для курса по Rust/Wasm.

## Предустановка

```bash
cargo install wasm-pack
cargo install cargo-watch
npm install
```

## Обычная сборка

```bash
npm run build
```

## Dev сборка (с watch файлов)

**В одном терминале:**

```bash
npm run dev:wasm
```

**В другом терминале:**

```bash
npm run dev
```

**Или можно в одном, но смешается вывод:**

```bash
npm run dev:wasm & npm run dev
```
