use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operator {
    name:       String,
    symbol:     String,
    fix:        OperatorFix,
    precedence: OperatorPrecedence,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OperatorFix {
    Infix,
    Prefix,
    Postfix,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OperatorPrecedence {
    Intrinsic      = 1100,
    Postfix        = 1000,
    Prefix         = 900,
    Exponential    = 800,
    Multiplicative = 700,
    Additive       = 600,
    Secondary      = 500,
    And            = 400,
    Or             = 300,
    Comparison     = 200,
    Assignment     = 100,
    None           = 0,
}

impl OperatorPrecedence {
    pub fn shifts(&self, after: OperatorPrecedence) -> bool { (*self as u32) > (after as u32) }
}

impl Operator {
    pub fn new(name: &str, symbol: &str, fix: OperatorFix, precedence: OperatorPrecedence) -> Self {
        Operator { name: name.to_string(),
                   symbol: symbol.to_string(),
                   fix,
                   precedence }
    }

    pub fn fix(&self) -> OperatorFix { self.fix }

    pub fn symbol(&self) -> &String { &self.symbol }

    pub fn precedence(&self) -> OperatorPrecedence { self.precedence }

    pub fn name(&self) -> &String { &self.name }
}

pub struct OperatorFactory {
    prefix_ops:  HashMap<String, Operator>,
    postfix_ops: HashMap<String, Operator>,
    operators:   HashMap<String, Operator>,
}

impl OperatorFactory {
    pub fn new() -> Self {
        Self { prefix_ops:  HashMap::new(),
               postfix_ops: HashMap::new(),
               operators:   HashMap::new(), }
    }

    pub fn register(&mut self, op: Operator) {
        let is_prefix = op.fix() == OperatorFix::Prefix;
        let symbol = op.symbol().clone();

        self.operators.insert(op.name.clone(), op.clone());

        if is_prefix {
            self.prefix_ops.insert(symbol, op);
        } else {
            self.postfix_ops.insert(symbol, op);
        }
    }

    pub fn register_infix(&mut self, name: &str, sym: &str, precedence: OperatorPrecedence) { self.register(Operator::new(name, sym, OperatorFix::Infix, precedence)); }

    pub fn register_prefix(&mut self, name: &str, sym: &str) { self.register(Operator::new(name, sym, OperatorFix::Prefix, OperatorPrecedence::Prefix)); }

    pub fn register_postfix(&mut self, name: &str, sym: &str) { self.register(Operator::new(name, sym, OperatorFix::Postfix, OperatorPrecedence::Postfix)); }

    pub fn register_intrinsics(&mut self) {
        self.register_infix("add", "+", OperatorPrecedence::Additive);
        self.register_infix("sub", "-", OperatorPrecedence::Additive);

        self.register_infix("mul", "*", OperatorPrecedence::Multiplicative);
        self.register_infix("div", "/", OperatorPrecedence::Multiplicative);
        self.register_infix("mod", "%", OperatorPrecedence::Multiplicative);

        self.register_infix("shiftLeft", "<<", OperatorPrecedence::Exponential);
        self.register_infix("shiftRight", ">>", OperatorPrecedence::Exponential);
        self.register_infix("bitAnd", "&", OperatorPrecedence::Multiplicative);
        self.register_infix("bitOr", "|", OperatorPrecedence::Additive);
        self.register_infix("bitXor", "^", OperatorPrecedence::Additive);

        self.register_infix("and", "&&", OperatorPrecedence::And);
        self.register_infix("or", "||", OperatorPrecedence::Or);

        // self.register_infix("instanceEq", "===", OperatorPrecedence::Comparison);
        // self.register_infix("instanceNeq", "!==", OperatorPrecedence::Comparison);
        self.register_infix("equal", "==", OperatorPrecedence::Comparison);
        self.register_infix("notEqual", "!=", OperatorPrecedence::Comparison);
        self.register_infix("greaterThan", ">", OperatorPrecedence::Comparison);
        self.register_infix("greaterThanEq", ">=", OperatorPrecedence::Comparison);
        self.register_infix("lessThan", "<", OperatorPrecedence::Comparison);
        self.register_infix("lessThanEq", "<=", OperatorPrecedence::Comparison);

        // self.register_infix("coalesce", "??", OperatorPrecedence::Comparison);
        //
        self.register_infix("openRange", "..=", OperatorPrecedence::Secondary);
        self.register_infix("closedRange", "..<", OperatorPrecedence::Secondary);
        // self.register_infix("addAssign", "+=", OperatorPrecedence::Assignment);
        // self.register_infix("subAssign", "-=", OperatorPrecedence::Assignment);
        // self.register_infix("mulAssign", "*=", OperatorPrecedence::Assignment);
        // self.register_infix("divAssign", "/=", OperatorPrecedence::Assignment);
        // self.register_infix("modAssign", "%=", OperatorPrecedence::Assignment);
        //
        // self.register_infix("shiftLeftAssign", "<<=", OperatorPrecedence::Assignment);
        // self.register_infix("shiftRightAssign", ">>=", OperatorPrecedence::Assignment);
        // self.register_infix("bitAndAssign", "&=", OperatorPrecedence::Assignment);
        // self.register_infix("bitOrAssign", "|=", OperatorPrecedence::Assignment);
        // self.register_infix("bitXorAssign", "^=", OperatorPrecedence::Assignment);
        //
        // self.register_infix("assign", "=", OperatorPrecedence::Assignment);
        self.register_postfix("upperRange", "..");
        self.register_prefix("unit", "+");
        self.register_prefix("negate", "-");

        self.register_prefix("invert", "!");

        self.register_prefix("lowerRange", "..");
        
        self.register_infix("index", "[", OperatorPrecedence::None);
    }

    pub fn get_prefix_op(&self, text: &str) -> Option<&Operator> { self.prefix_ops.get(text) }

    pub fn get_postfix_op(&self, text: &str) -> Option<&Operator> { self.postfix_ops.get(text) }

    pub fn get_op(&self, named: &str) -> Option<&Operator> { self.operators.get(named) }
}
