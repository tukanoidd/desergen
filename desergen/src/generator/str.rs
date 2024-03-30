pub trait UtilStringGenerator {
    fn space(self) -> Self;
    fn tab(self) -> Self;
    fn tabs(self, amount: usize) -> Self;
    fn new_line(self) -> Self;

    fn open_bracket(self) -> Self;
    fn close_bracket(self) -> Self;

    fn comma(self) -> Self;

    fn finish(self) -> String;
}

impl UtilStringGenerator for String {
    fn space(mut self) -> Self {
        self.push(' ');
        self
    }

    fn tab(mut self) -> Self {
        self.push('\t');
        self
    }

    fn tabs(mut self, amount: usize) -> Self {
        self.push_str(&"\t".repeat(amount));
        self
    }

    fn new_line(mut self) -> Self {
        self.push('\n');
        self
    }

    fn open_bracket(mut self) -> Self {
        self.push('{');
        self
    }

    fn close_bracket(mut self) -> Self {
        self.push('}');
        self
    }

    fn comma(mut self) -> Self {
        self.push(',');
        self
    }

    fn finish(self) -> String {
        self
    }
}

pub trait StringGenerator: UtilStringGenerator {
    fn export(self) -> Self;

    fn class(self) -> impl ClassStringGenerator;
    fn enum_(self) -> impl EnumStringGenerator;
}

impl StringGenerator for String {
    fn export(mut self) -> Self {
        self.push_str("export");
        self.space()
    }

    fn class(mut self) -> impl ClassStringGenerator {
        self.push_str("class");
        self.space()
    }

    fn enum_(mut self) -> impl EnumStringGenerator {
        self.push_str("enum");
        self.space()
    }
}

pub trait ClassStringGenerator: UtilStringGenerator {}

impl ClassStringGenerator for String {}

pub trait EnumStringGenerator: UtilStringGenerator {
    fn name(self, name: impl AsRef<str>) -> Self;
    fn variants(self, variants: &[String]) -> Self;
    fn variant(self, variant: &str) -> Self;
}

impl EnumStringGenerator for String {
    fn name(mut self, name: impl AsRef<str>) -> Self {
        self.push_str(name.as_ref());
        self.space()
    }

    fn variants(self, variants: &[String]) -> Self {
        variants
            .iter()
            .fold(self.open_bracket().new_line(), |res, variant| {
                res.tab().variant(variant).new_line()
            })
            .close_bracket()
    }

    fn variant(mut self, variant: &str) -> Self {
        self.push_str(variant);
        self.comma()
    }
}
