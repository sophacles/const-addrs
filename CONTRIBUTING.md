# How to contribute

Contributions are very welcome. If you have an issue or feature request please
report it so it can be addressed! If you have a fix for a bug, docs, tests
and/or a new feature feel free to open a PR, although for large changes,
breaking changes or new features it's recommended to open an issue first to
discuss the ramifactions before you put in the coding effort.

## PR Guidelines
* All code should be run through rustfmt before committing
* New features/macros should be accompanied by tests and documentation
* When adding a macro for a 3rd party crate, please gate it behind a feaure flag named for that crate
* Commit messages should follow [Conventional Commits](https://www.conventionalcommits.org/) format
  (in the future ther will be a CI lint for this)

# People who have helped const-addrs so far:
* [Linus Färnstrand (@faern)](https://github.com/faern)
