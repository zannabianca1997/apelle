# Roadmap

## Features

### Feature parity with 1.0

- [x] Actions
  - [x] Get
  - [x] Events
  - [x] Enqueue
  - [x] Like
  - [x] Play
  - [x] Pause
  - [x] Next
    - [x] AutoNext
  - [x] Remove
- [x] WebUI
  - [x] Login page
  - [x] Landing page
  - [x] Queue page
    - [x] Queue model
    - [x] Player view
    - [x] Search bar
    - [x] Queued songs cards
  - [x] Players
    - [x] Youtube

### TODO

- [ ] Collect common queueu action in db functions
- [x] Events collector (returned as a response extension)
- [ ] Graceful shutdown (Cancellation token)
- [ ] Intra-services http cache

### Future

- [ ] Healthchecks and common state page
- [ ] Metrics
- [ ] Logging to common server

## CI/CD

- [ ] CI/CD pipeline
  - [ ] Run Rust tests
  - [ ] Run Bruno collection as tests

## Testing

- [ ] Fill out Bruno collection
- [ ] Rust tests