# Roadmap

## Features

### Feature parity with 1.0

- [ ] Actions
  - [x] Get
  - [x] Events
  - [x] Enqueue
  - [x] Like
  - [ ] Play
  - [ ] Pause
  - [x] Next
    - [x] AutoNext
  - [ ] Remove
- [ ] WebUI
  - [ ] Login page
  - [ ] Landing page
  - [ ] Queue page
    - [ ] Queue model

### TODO

- [ ] Collect common queueu action in db functions
- [ ] Events collector (returned as a response extension)
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