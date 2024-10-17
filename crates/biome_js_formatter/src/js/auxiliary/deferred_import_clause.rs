use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsDeferredImportClause, JsDeferredImportClauseFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsDeferredImportClause;
impl FormatNodeRule<JsDeferredImportClause> for FormatJsDeferredImportClause {
    fn fmt_fields(&self, node: &JsDeferredImportClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDeferredImportClauseFields {
            defer_token,
            namespace_specifier,
            from_token,
            source,
            assertion,
        } = node.as_fields();

        write![
            f,
            [
                defer_token.format(),
                space(),
                namespace_specifier.format(),
                space(),
                from_token.format(),
                space(),
                source.format(),
            ]
        ]?;

        if let Some(assertion) = assertion {
            write!(f, [assertion.format()])?;
        }

        Ok(())
    }
}
