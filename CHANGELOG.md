# 0.1.1

* expanded search interface with and `iter` method that implements an `Iterator` over `Issues`
* changed `SearchListOptionsBuilder#max` to `max_results` be more consistent with the underlying api
* introduced `Error::Unauthorized` to handle invalid credentials with more grace
* replaced usage of `u32` with `u64` for a more consistent interface
* renamed `TransitionTrigger` to `TransitionTriggerOptions` for a more consistent api

# 0.1.0

* initial release
