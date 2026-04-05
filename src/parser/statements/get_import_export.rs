use std::path::PathBuf;

use crate::{
    Parser, ParserDiagnostic, ParserResult, Statement, Token,
    expressions::{Identifier, ImportedItem},
    statements::resolve_file_module_path,
};

impl Parser<'_> {
    pub fn parse_import_statement(&mut self) -> ParserResult<Statement> {
        self.consume_current_token(Token::Get);

        let (module_name, path_names, file_module_path) = self.parse_get_file_module_path()?;
        self.next_token();

        let imported_items = self.parse_import_items()?;

        self.consume_peek_token(Token::SemiColon);

        Ok(Statement::Get {
            module_name,
            path_names,
            file_module_path,
            imported_items,
        })
    }

    pub fn parse_get_file_module_path(
        &mut self,
    ) -> ParserResult<(Identifier, Vec<Identifier>, PathBuf)> {
        let mut module_path_names = vec![match self.current_token {
            Token::TwoDots => "..".into(),
            _ => self.parse_identifier()?,
        }];

        while matches!(self.peek_token, Token::Slash) {
            self.next_token();
            self.next_token();
            module_path_names.push(match self.current_token {
                Token::TwoDots => "..".into(),
                _ => self.parse_identifier()?,
            });
        }

        let module_name = module_path_names.last().cloned().unwrap();

        let path = match resolve_file_module_path(&self.working_directory, &module_path_names) {
            Ok(path) => path,
            Err(module_path_parse_error) => {
                return Err(ParserDiagnostic::FileModuleError(module_path_parse_error));
            }
        };

        Ok((module_name, module_path_names, path))
    }

    pub fn parse_import_items(&mut self) -> ParserResult<Vec<ImportedItem>> {
        let mut imported_items = Vec::new();

        if self.is_current_token(Token::LBrace) {
            self.next_token();
            loop {
                if self.is_current_token(Token::RBrace) {
                    break;
                }
                self.consume_current_token(Token::Comma);

                let name = self.parse_identifier()?;
                let as_naming = if self.is_peek_token(Token::As) {
                    self.skip_peek_token(Token::As);
                    Some(self.parse_identifier()?)
                } else {
                    None
                };
                self.next_token();

                imported_items.push(ImportedItem { name, as_naming });
            }
        }

        Ok(imported_items)
    }

    pub fn parse_export_statement(&mut self) -> ParserResult<Statement> {
        self.consume_current_token(Token::Out);

        let stmt = self.parse_statement()?;
        match stmt {
            Statement::Binding(ref binding) if binding.is_constant => {
                Ok(Statement::Out(Box::new(stmt)))
            }
            Statement::Alias { .. } => Ok(Statement::Out(Box::new(stmt))),
            stmt => Err(ParserDiagnostic::CannotExport(stmt.into())),
        }
    }
}
