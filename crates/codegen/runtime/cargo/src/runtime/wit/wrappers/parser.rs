use crate::diagnostic::Diagnostic;
use crate::wit::utils::{define_wrapper, FromFFI, IntoFFI};

mod ffi {
    pub use crate::wit::interface::exports::nomic_foundation::slang::cst::{
        Cursor, Node, TextRange,
    };
    pub use crate::wit::interface::exports::nomic_foundation::slang::diagnostic::Severity;
    pub use crate::wit::interface::exports::nomic_foundation::slang::parser::{
        Guest, GuestParseError, GuestParseOutput, GuestParser, NonterminalKind, ParseError,
        ParseErrorBorrow, ParseOutput, ParseOutputBorrow, Parser, ParserBorrow,
    };
}

mod rust {
    pub use crate::parser::{ParseError, ParseOutput, Parser};
}

impl ffi::Guest for crate::wit::World {
    type Parser = ParserWrapper;
    type ParseError = ParseErrorWrapper;
    type ParseOutput = ParseOutputWrapper;
}

//================================================
//
// resource parser
//
//================================================

define_wrapper! { Parser {
    fn new(version: String) -> Result<ffi::Parser, String> {
        semver::Version::parse(&version)
            .map_err(|_| format!("Invalid version: {version}"))
            .and_then(|version| rust::Parser::new(version).map_err(|e| e.to_string()))
            .map(IntoFFI::_into_ffi)
    }

    fn version(&self) -> String {
        self._borrow_ffi().version.to_string()
    }

    fn supported_versions() -> Vec<String> {
        rust::Parser::SUPPORTED_VERSIONS
            .iter()
            .map(|v| v.to_string())
            .collect()
    }

    fn parse(&self, kind: ffi::NonterminalKind, input: String) -> ffi::ParseOutput {
        self._borrow_ffi().parse(kind._from_ffi(), &input)._into_ffi()
    }
} }

//================================================
//
// resource parse-error
//
//================================================

define_wrapper! { ParseError {
  fn severity(&self) -> ffi::Severity {
      self._borrow_ffi().severity()._into_ffi()
  }

  fn text_range(&self) -> ffi::TextRange {
      self._borrow_ffi().text_range()._into_ffi()
  }

  fn message(&self) -> String {
      self._borrow_ffi().message()
  }
} }

//================================================
//
// resource parse-output
//
//================================================

define_wrapper! { ParseOutput {
  fn tree(&self) -> ffi::Node {
      self._borrow_ffi().tree()._into_ffi()
  }

  fn errors(&self) -> Vec<ffi::ParseError> {
      self._borrow_ffi().errors().iter().map(|e| e.clone()._into_ffi()).collect()
  }

  fn is_valid(&self) -> bool {
      self._borrow_ffi().is_valid()
  }

  fn create_tree_cursor(&self) -> ffi::Cursor {
      self._borrow_ffi().create_tree_cursor()._into_ffi()
  }
} }
