# Changelog

## 0.3.1

* Use ISO8601 for time conversion [#17](https://github.com/wunderfrucht/gouqi/issues/17)

## 0.3.0

* Forked from <https://github.com/softprops/goji>
* Added several open pull request from <https://github.com/softprops/goji>
* Added additional contributions from forks on github.
* Renamed the library from goji to gouqi

## 0.2.4

* added boards issue search api interface [#30](https://github.com/softprops/goji/pull/30)
* added `Issue.permalink` convenience method [#31](https://github.com/softprops/goji/pull/31)
* fixed issue with boards iterator [#32](https://github.com/softprops/goji/pull/32)
* fix naming issue with `issue.resolutiondate` field [#34](https://github.com/softprops/goji/pull/34/files)

## 0.2.3

* fix breaking changes with agile api paths

## 0.2.2

* added sprints interfaces [#24](https://github.com/softprops/goji/pull/24)
* added boarders interfaces [#21](https://github.com/softprops/goji/pull/21)
* added agile api [#20](https://github.com/softprops/goji/pull/20)

## 0.2.1

* updated issue and attachment interfaces

## 0.2.0

* replace hyper client with reqwest

## 0.1.1

* expanded search interface with and `iter` method that implements an `Iterator` over `Issues`
* changed `SearchListOptionsBuilder#max` to `max_results` be more consistent with the underlying api
* introduced `Error::Unauthorized` to handle invalid credentials with more grace
* replaced usage of `u32` with `u64` for a more consistent interface
* renamed `TransitionTrigger` to `TransitionTriggerOptions` for a more consistent api

## 0.1.0

* initial release
