
use crate::text::hash_str;
use TokenType::*;
use std::collections::HashMap;

/// All the different token types than can be identified.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum TokenType {
    Instruction,
        InstrName,
            Adc,    Add,    And,    Bit,    Call,   Ccf,    Cp,     Cpl,    Daa,    Dec,
            Di,     Ei,     Halt,   Inc,    Jp,     Jr,     Ld,     Ldh,    Ldi,    Ldd,    
            Ldhl,   Or,     Pop,    Push,   Res,    Ret,    Rl,     Rla,    Rlc,    Rld,    
            Rr,     Rra,    Rrc,    Rrca,   Rrd,    Rst,    Sbc,    Scf,    Set,    Sla,    
            Sll,    Sra,    Srl,    Stop,   Sub,    Swap,   Xor,    Reti,   Rlca,   Nop,

        Argument,
            Register,       A, B, C, D, E, H, L, Af, Bc, De, Hl, Sp,
            Lit,            LitBin, LitHex, LitDec, LitStr,
            At,             At0, At1,
            Flag,           FlagZ, FlagNz, FlagC, FlagNc,
            Identifier,

    Directive,
        Define, Include,
        Macro,              MacroIdent, MacroArg, MacroBody,
        
    MacroCall,
    Marker,                 NamedMark, AnonMark, Label,
    Unknown,
    Root,
}

#[derive(Debug)]
/// Entry in the lexicon, identified by a type.
/// It can be used to find its parent and up the up hierarchy.
struct LexiconEntry {
    ty: TokenType,
    parent: usize,
}

impl LexiconEntry {
    const fn new(ty: TokenType, parent: usize) -> Self {
        Self { ty, parent }
    }
}

/// Hierarchy of the token types.
/// It is also responsible for the storing the data that helps to identify tokens.
#[derive(Debug)]
pub struct Lexicon {
    /// Contains all the different token types.
    entries: Vec<LexiconEntry>,
    /// Maps a token type to its index in 'entries'.
    types_map: HashMap<TokenType, usize>,
    names_map: HashMap<u64, usize>,
    prefixes_map: HashMap<u64, usize>,
}

impl Lexicon {

    /// Build the hierarchy. (See `TokenType` declaration)
    /// The main goal is for each node to have a reference to its parent.
    /// This is why the tree hierarchy is flattened into a vec,
    /// as it requires only a single number to refer to the parent.
    pub fn new() -> Self {
        let mut lexicon = Self {
            entries: vec![LexiconEntry::new(Root, 0)],
            types_map: HashMap::new(),
            names_map: HashMap::new(),
            prefixes_map: HashMap::new(),
        };

        let root = 0;
        lexicon.types_map.insert(Root, 0);

        let instruction =   lexicon.branch(Instruction, root);
        let instr_name =        lexicon.branch(InstrName, instruction);
        let argument =          lexicon.branch(Argument, instruction);
        let lit =                   lexicon.branch(Lit, argument);
        let register =              lexicon.branch(Register, argument);
        let at =                    lexicon.branch(At, argument);
        let flag =                  lexicon.branch(Flag, argument);
        let directive =     lexicon.branch(Directive, root);
        let r#macro =           lexicon.branch(Macro, directive);
        let marker =        lexicon.branch(Marker, root);

        let i_n = vec![Adc, Add, And, Bit, Call, Ccf, Cp, Cpl, Daa, Dec, Di, Ei, Halt, 
                       Inc, Jp, Jr, Ld, Ldi, Ldd, Ldhl, Or, Pop, Push, Res, Ret, Rl, Rla, 
                       Rlc, Rld, Rr, Rra, Rrc, Rrca, Rrd, Rst, Sbc, Scf, Set, Sla, Sll, 
                       Sra, Srl, Stop, Sub, Swap, Xor, Reti, Rlca, Nop];

        let r_n = vec![A, B, C, D, E, H, L, Af, Bc, De, Hl, Sp];

        lexicon.push(instr_name,    &i_n); 
        lexicon.push(argument,      &[Identifier]);
        lexicon.push(register,      &r_n);
        lexicon.push(lit,           &[LitBin, LitHex, LitDec, LitStr]);
        lexicon.push(at,            &[At0, At1]);
        lexicon.push(flag,          &[FlagZ, FlagNz, FlagC, FlagNc]);
        lexicon.push(r#macro,       &[MacroIdent, MacroArg, MacroBody]);
        lexicon.push(directive,     &[Define, Include]);
        lexicon.push(marker,        &[NamedMark, AnonMark, Label]);
        lexicon.push(root,          &[MacroCall, Marker, Unknown]);

        // Create names map, to parse from text.
        
        lexicon.push_names(&i_n);
        lexicon.push_names(&r_n);

        let names = [
            ("(", At0), (")", At1), ("Z", FlagZ), ("NZ", FlagNz), ("C", FlagC), ("NC", FlagNc)];

        for (name, ty) in names {
            let index = lexicon.types_map[&ty];
            lexicon.names_map.insert(hash_str(name), index);
        }

        // Create prefix map. Some token are identified by their first characters.

        let prefixes = [("&", LitHex), ("#", Directive), ("%", LitBin), ("\"", LitStr),
            (".", MacroArg), (":", Label)];

        for (prefix, ty) in prefixes {
            let index = lexicon.types_map[&ty];
            lexicon.prefixes_map.insert(hash_str(prefix), index);
        }

        lexicon
    }

    /// Create an entry that will have children and return its index.
    fn branch(&mut self, ty: TokenType, parent: usize) -> usize {
        let i = self.entries.len();
        self.types_map.insert(ty, i);
        self.entries.push(LexiconEntry::new(ty, parent));

        i
    }

    /// Create leaf entries.
    fn push(&mut self, parent: usize, children: &[TokenType]) {
        for child in children {
            self.types_map.insert(*child, self.entries.len());
            self.entries.push(LexiconEntry::new(*child, parent));
        }
    }

    /// Insert names that can be found from `TokenType`'s Debug implementation.
    fn push_names(&mut self, types: &[TokenType]) {
        for ty in types {
            let name = format!("{:?}", ty).to_lowercase();
            let index = self.types_map[ty];
            self.names_map.insert(hash_str(&name), index);
        }
    }

    /// Find entry in the lexicon by its type.
    fn of_ty(&self, ty: TokenType) -> &LexiconEntry {
        &self.entries[self.types_map[&ty]]
    }

    /// Reference to the parent entry in the lexicon.
    fn parent_of(&self, entry: &LexiconEntry) -> &LexiconEntry {
        &self.entries[entry.parent]
    }

    /// Return the parent `TokenType`.
    pub fn parent_type(&self, ty: TokenType) -> TokenType {
        self.parent_of(self.of_ty(ty)).ty
    }

    /// Generalization of a type within Argument.
    /// Instruction -> Argument -> Lit -> ...   = Lit
    /// Instruction -> Argument -> Identifier   = Identifier
    pub fn argument_type(&self, ty: TokenType) -> TokenType {
        let parent = self.parent_of(self.of_ty(ty));
        if parent.ty == Argument { return ty }

        if self.parent_of(parent).ty == Argument { 
            return parent.ty 
        }

        Unknown
    }

    /// Some types are not meant to hold a value, those types will return true.
    pub fn no_value(&self, ty: TokenType) -> bool {
        let parent_ty = self.parent_of(self.of_ty(ty)).ty;
        matches!(parent_ty, InstrName|Register|Flag|At|Directive)
    }

    /// Is it one the token types that end on a newline.
    pub fn ends_on_newline(&self, ty: TokenType) -> bool {
        matches!(ty, Instruction|Argument|MacroCall|Directive|AnonMark|NamedMark) ||
            self.parent_type(ty) == Directive
    }

    /// Find a token type in the lexicon by its name.
    /// Works with instruction names and registers.
    pub fn get_by_name(&self, name: &str) -> Option<TokenType> {
        if let Some(index) = self.names_map.get(&hash_str(name)) {
            return Some(self.entries[*index].ty)
        }

        None
    }

    /// Find a token type in the lexicon by its first character.
    /// e.g. &2893 is a hexadecimal literal.
    pub fn get_by_prefix(&self, first: &str) -> Option<TokenType> {
        if let Some(index) = self.prefixes_map.get(&hash_str(first)) {
            return Some(self.entries[*index].ty)
        }

        None
    }

    /// Number of entries.
    #[cfg(test)]
    pub fn type_count(&self) -> usize {
        self.entries.len()
    }

    /// Get token type at specified index.
    #[cfg(test)]
    pub fn get_type_at(&self, index: usize) -> TokenType {
        self.entries[index].ty
    }

    /// Test finding an entry in the lexicon by its type.
    #[cfg(test)]
    pub fn try_of_ty(&self, ty: TokenType) {
        let _ = &self.entries[self.types_map[&ty]];
    }

}
