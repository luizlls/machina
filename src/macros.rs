
macro_rules! as_expr {
    ($e: expr) => { $e }
}

macro_rules! bin_op {
    ($self:expr, $instruction:expr, $op:tt) => {{
        let lhs = $self.get($instruction.get(0));
        let rhs = $self.get($instruction.get(1));
        let val = if lhs.is_num() || rhs.is_num() {
            Value::from(as_expr!(lhs.as_num() $op rhs.as_num()))
        } else {
            Value::from(as_expr!(lhs.as_int() $op rhs.as_int()))
        };
        $self.set($instruction.register(0), val);
    }};
}

macro_rules! int_op {
    ($self:expr, $instruction:expr, $op:tt) => {{
        let lhs = $self.get($instruction.get(0));
        let rhs = $self.get($instruction.get(1));
        let val = Value::from(as_expr!(lhs.as_int() $op rhs.as_int()));
        $self.set($instruction.register(0), val);
    }};
}

macro_rules! unary_op {
    ($self:expr, $instruction:expr, $op:tt) => {{
        let rhs = $self.get($instruction.get(0));
        let val = Value::from(as_expr!($op rhs.as_int()));
        $self.set($instruction.register(0), val);
    }};
}

macro_rules! jmp_op {
    ($self:expr, $instruction:expr, $ip:expr, $op:tt) => {{
        let lhs = $self.get($instruction.get(1));
        let rhs = $self.get($instruction.get(2));
        if as_expr!(lhs $op rhs) {
            $ip = $instruction.position(0) as usize;
        }
    }};
}