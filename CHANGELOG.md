# Changelog

## [0.4.0](https://github.com/frender-rs/callable/compare/v0.3.0...v0.4.0) (2023-11-28)


### Features

* feature gat ([cf90660](https://github.com/frender-rs/callable/commit/cf90660d12c755da6b216bcfefb8397e8efd859b))

## [0.3.0](https://github.com/frender-rs/callable/compare/v0.2.0...v0.3.0) (2023-05-15)


### ⚠ BREAKING CHANGES

* relax bounds of `MaybeHandleEvent`

### Features

* relax bounds of `MaybeHandleEvent` ([19f536f](https://github.com/frender-rs/callable/commit/19f536f4fdba07c94c04043bbbba0c292c73cdb8))

## [0.2.0](https://github.com/frender-rs/callable/compare/v0.1.0...v0.2.0) (2023-05-14)


### ⚠ BREAKING CHANGES

* `MaybeHandleEvent` now requires `Self::Callable::Output` to be `()`

### Features

* `MaybeHandleEvent` now requires `Self::Callable::Output` to be `()` ([06c6c79](https://github.com/frender-rs/callable/commit/06c6c79c06ecc3703cae4ea0a0b3d220a868ee4c))

## 0.1.0 (2023-05-12)


### Features

* disable impl_with_macro_rules by default ([e007818](https://github.com/frender-rs/callable/commit/e0078189ceb2a851df9befc4cefc9f4ff7ae7286))
* macro ArgumentType ([995ac66](https://github.com/frender-rs/callable/commit/995ac66f938c46ad9c592e7566030124ff4a8dd7))
* make argument types un-constructable ([8f79c40](https://github.com/frender-rs/callable/commit/8f79c40a5e88715b1c7c1be1002508d1564da976))
* no_std ([f31f63b](https://github.com/frender-rs/callable/commit/f31f63be6d281f038bf41a3dff0eb54c376cf191))
* remove `'static` bounds for argument types ([6a828d1](https://github.com/frender-rs/callable/commit/6a828d1bb8b6a34107b19f5cc669bd4ece1e51f6))


### Bug Fixes

* callable![fn(_, _)] not working ([6a92f1c](https://github.com/frender-rs/callable/commit/6a92f1ce2f4658dfaba6e2b511b5cb4775aac0ed))
