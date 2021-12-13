//! Module containing flat program optimization
//!
//! @file mod.rs
//! @author Thibaut Schaeffer <thibaut@schaeff.fr>
//! @date 2018

mod canonicalizer;
mod directive;
mod duplicate;
mod redefinition;
mod tautology;

use self::canonicalizer::Canonicalizer;
use self::directive::DirectiveOptimizer;
use self::duplicate::DuplicateOptimizer;
use self::redefinition::RedefinitionOptimizer;
use self::tautology::TautologyOptimizer;

use crate::ir::{IntoStatements, Ir, ProgIterator};
use zokrates_field::Field;

impl<T: Field, I: IntoStatements<Ir<T>>> ProgIterator<T, I> {
    pub fn optimize(self) -> ProgIterator<T, impl IntoStatements<Ir<T>>> {
        // remove redefinitions
        log::debug!("Initialise optimizer");

        // define all optimizer steps
        let mut redefinition_optimizer = RedefinitionOptimizer::init(&self);
        let mut tautologies_optimizer = TautologyOptimizer::default();
        let mut directive_optimizer = DirectiveOptimizer::default();
        let canonicalizer = Canonicalizer::default();
        let mut duplicate_optimizer = DuplicateOptimizer::default();

        use crate::ir::folder::Folder;

        let arguments = self
            .arguments
            .into_iter()
            .map(|a| redefinition_optimizer.fold_argument(a))
            .map(|a| {
                <TautologyOptimizer as Folder<T>>::fold_argument(&mut tautologies_optimizer, a)
            })
            .map(|a| directive_optimizer.fold_argument(a))
            .map(|a| <DuplicateOptimizer as Folder<T>>::fold_argument(&mut duplicate_optimizer, a))
            .collect();

        use crate::ir::folder::fold_statements;

        let statements = fold_statements(redefinition_optimizer, self.statements);
        let statements = fold_statements(tautologies_optimizer, statements);
        let statements = fold_statements(canonicalizer, statements);
        let statements = fold_statements(directive_optimizer, statements);
        let statements = fold_statements(duplicate_optimizer, statements);

        ProgIterator::new(arguments, statements, self.return_count)
    }
}
