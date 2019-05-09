# Version next

* Fixed bug preventing usage of `IntoIterator`s which are not copy
* Added clarification to Readme.md about `Copy` requirements of
  captured objects
* Added support for `if let` clauses
* Added support for consecutive `if` and `if let` clauses such as

  ```rust
  comp!(x; for x in 0..10; if x > 3; if x < 6)
  ```

# Version 0.1.2

* Readme.md examples are now tested
* The trailing semicolon after the last expression in the macro is now optional
* Added nested-structure example to documentation

# Version 0.1.1

* Added metadata to Readme.md to show more info on crates.io

# Version 0.1.0

* Initial release
