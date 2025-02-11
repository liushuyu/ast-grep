#![cfg(test)]
use ast_grep_core::source::TSParseError;

use super::*;

fn test_match(query: &str, source: &str) {
  use crate::test::test_match_lang;
  test_match_lang(query, source, D);
}

#[test]
fn test_d_pattern() {
  test_match("$A.b()", "expr.b()");
  test_match(
    "static if (a) { $$$VERYLONGNAME }",
    "static if (a) { auto a = b; a = c; }",
  );
  test_match("expr.$B()", "expr.b()");
  test_match(
    "version (Feature)",
    "version (Feature) private enum Flag = true;",
  );
  test_match(
    "private template $N($T) {}",
    "private template someTemplate(T) {}",
  );
  test_match("static if ($C) {}", "static if (13+5==18) {}");
  test_match(
    "EAX",
    "void test() { asm pure nothrow @nogc { mov EAX, 0x0B; } }",
  );
  test_match("static if ($A)", "static if (a | b) abc; }");
}

fn test_replace(src: &str, pattern: &str, replacer: &str) -> Result<String, TSParseError> {
  use crate::test::test_replace_lang;
  test_replace_lang(src, pattern, replacer, D)
}

#[test]
fn test_d_replace() -> Result<(), TSParseError> {
  let ret = test_replace("expr.b()", "$A.b()", "func($A).b()")?;
  assert_eq!(ret, "func(expr).b()");
  let ret = test_replace("static if (a) { auto a = b; a = c; }", "static if (a) { $$$A }", "$$$A")?;
  assert_eq!(ret, "auto a = b; a = c;");
  Ok(())
}
