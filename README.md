# bswam

Ever tried to build a rust project and gotten an error because the openssl-sys
build script doesn't find openssl, because there's no way for a project
maintainer to instruct cargo to run rustc and build scripts inside the
appropriate nix-shell invocation, so you have to figure that out for yourself?

No more! Just apply this one weird trick: Put your app into a subdirectory and
invoke cargo from inside this wrapper app instead<sup>1</sup>! It will work, as
if by magic!

---

<sup>1</sup> May require some modification of all the hardcoded bits.
