#![allow(non_upper_case_globals)]

use bitflags::bitflags;
use oxc_index::{define_index_type, IndexVec};
use oxc_span::{Atom, Span};

use crate::{
    reference::{Reference, ReferenceId},
    scope::ScopeId,
};

define_index_type! {
    pub struct SymbolId = u32;
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SymbolFlags: u16 {
        /// Variable (var) or parameter
        const FunctionScopedVariable  = 1 << 0;
        /// A block-scoped variable (let or const)
        const BlockScopedVariable     = 1 << 1;
        /// A const variable (const)
        const ConstVariable           = 1 << 2;
        /// Is this symbol inside an import declaration
        const Import                  = 1 << 3;
        /// Is this symbol inside an export declaration
        const Export                  = 1 << 4;
        const Class                   = 1 << 5;
        const CatchVariable           = 1 << 6; // try {} catch(catch_variable) {}
        const Function                = 1 << 7;

        const Variable = Self::FunctionScopedVariable.bits() | Self::BlockScopedVariable.bits();
        const Value = Self::Variable.bits() | Self::Class.bits();

        /// Variables can be redeclared, but can not redeclare a block-scoped declaration with the
        /// same name, or any other value that is not a variable, e.g. ValueModule or Class
        const FunctionScopedVariableExcludes = Self::Value.bits() - Self::FunctionScopedVariable.bits();

        /// Block-scoped declarations are not allowed to be re-declared
        /// they can not merge with anything in the value space
        const BlockScopedVariableExcludes = Self::Value.bits();

        const ClassExcludes = Self::Value.bits();
    }
}

impl SymbolFlags {
    pub fn is_variable(&self) -> bool {
        self.intersects(Self::Variable)
    }

    pub fn is_function(&self) -> bool {
        self.contains(Self::Function)
    }

    pub fn is_catch_variable(&self) -> bool {
        self.contains(Self::CatchVariable)
    }

    pub fn is_function_scoped_declaration(&self) -> bool {
        self.contains(Self::FunctionScopedVariable)
    }
}

/// Symbol Table
///
/// `SoA` (Struct of Arrays) for memory efficiency.
#[derive(Debug)]
pub struct SymbolTable {
    pub(crate) spans: IndexVec<SymbolId, Span>,
    pub(crate) names: IndexVec<SymbolId, Atom>,
    pub(crate) flags: IndexVec<SymbolId, SymbolFlags>,
    pub(crate) scope_ids: IndexVec<SymbolId, ScopeId>,
    pub(crate) resolved_references: IndexVec<SymbolId, Vec<ReferenceId>>,
    pub(crate) references: IndexVec<ReferenceId, Reference>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            spans: IndexVec::new(),
            names: IndexVec::new(),
            flags: IndexVec::new(),
            scope_ids: IndexVec::new(),
            resolved_references: IndexVec::new(),
            references: IndexVec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.spans.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_name(&self, symbol_id: SymbolId) -> &Atom {
        &self.names[symbol_id]
    }

    pub fn get_flag(&self, symbol_id: SymbolId) -> SymbolFlags {
        self.flags[symbol_id]
    }

    pub fn get_scope_id(&self, symbol_id: SymbolId) -> ScopeId {
        self.scope_ids[symbol_id]
    }

    pub fn create_symbol(
        &mut self,
        span: Span,
        name: Atom,
        flag: SymbolFlags,
        scope_id: ScopeId,
    ) -> SymbolId {
        _ = self.spans.push(span);
        _ = self.names.push(name);
        _ = self.flags.push(flag);
        _ = self.scope_ids.push(scope_id);
        self.resolved_references.push(vec![])
    }

    pub fn create_reference(&mut self, _span: Span, name: Atom) -> ReferenceId {
        self.references.push(Reference::new(name))
    }

    pub fn get_reference(&self, reference_id: ReferenceId) -> &Reference {
        &self.references[reference_id]
    }

    pub fn is_global_reference(&self, reference_id: ReferenceId) -> bool {
        self.references[reference_id].symbol_id.is_none()
    }

    pub fn get_resolved_references(&self, symbol_id: SymbolId) -> &Vec<ReferenceId> {
        &self.resolved_references[symbol_id]
    }
}
