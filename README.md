# Loom Video Downloader API

This is totally free API for downloading Loom videos.

> consider supporting me on [Patreon](https://www.patreon.com/abdibrokhim/membership)

## Usage

### Deployed

```bash
curl -X POST https://loom-dl-pwtf.shuttle.app/api/loom-dl \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.loom.com/share/17c3b800367e47ebaf06151f6d45447a?sid=944b8ac3-4a7a-4faf-86be-4cc3f41249f4"}'
```

### Local

```bash
curl -X POST http://localhost:8000/api/loom-dl \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.loom.com/share/17c3b800367e47ebaf06151f6d45447a?sid=944b8ac3-4a7a-4faf-86be-4cc3f41249f4"}'
```

## Testing

```bash
bash run_test.sh
```

## Deploying

```bash
shuttle deploy
```
